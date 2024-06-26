use core::panic;
use std::{env, time::Duration};

use tokio::{io::AsyncReadExt, time::sleep};

use crate::{
    action::submit,
    graphql::request::{get_question_backend_id, get_question_base_info, search_question},
    model::{question::QuestionBaseInfo, sub_result::SubResult},
    util::{
        file::{get_slug_by_solution_id, get_solution_by_id},
        path::{gen_link_by_slug, get_graphql_url},
    },
};

use super::SubmitArgs;

pub async fn submit_and_check(args: SubmitArgs) {
    let graphql_url = get_graphql_url();

    let (id, slug) = if let Some(id) = args.id {
        if let Ok(slug) = get_slug_by_solution_id(&id) {
            (id, slug)
        } else {
            let result = get_slug_from_remote(&id).await;
            (result.id, result.slug)
        }
    } else {
        let result = get_question_base_info(&graphql_url).await;
        (result.id, result.slug)
    };

    let backend_id = get_question_backend_id(&slug, &graphql_url).await;

    let response = submit::submit(&backend_id.to_string(), &slug, &get_code(&id).await).await;
    println!("提交 {}", get_solution_by_id(&id).unwrap().display());

    loop {
        match submit::get_sub_result(response.submission_id, &slug).await {
            SubResult::Judging(j) => {
                println!("{:?} {}", j, response.submission_id);
                sleep(Duration::from_secs(1)).await;
            }
            SubResult::Finished(e) => {
                if e.wa() {
                    println!("{}", e.wa_output());
                } else if e.ac() {
                    println!("{}", e.ac_output());
                } else {
                    println!("{:?}", e);
                }
                break;
            }
        }
    }
}

async fn get_code(id: &str) -> String {
    let file_path = get_solution_by_id(id).unwrap();

    if !file_path.exists() {
        panic!(
            "curr_path: {:?} file {:?} not exists",
            env::current_dir(),
            file_path
        );
    }

    let mut file = tokio::fs::File::open(file_path).await.unwrap();

    let mut full_code = vec![];
    file.read_to_end(&mut full_code).await.unwrap();

    let full_code = String::from_utf8(full_code).unwrap();

    let begin_str = "pub struct Solution;";
    let end_str = "#[cfg(test)]";

    let code_begin = full_code.find(&begin_str).unwrap() + begin_str.len();
    let code_end = full_code.find(&end_str).unwrap();

    full_code[code_begin..code_end].to_string()
}

async fn get_slug_from_remote(id: &str) -> QuestionBaseInfo {
    let graphql_url = get_graphql_url();

    let mut question = None;
    let mut link = None;
    let mut max_attempt_times = 5;
    let mut skip = 0;
    let step = 30;
    while max_attempt_times > 0 {
        let filter_questions = search_question(&id, &graphql_url, skip, step).await;
        skip += step;
        if filter_questions.is_empty() {
            panic!("no question found");
        }
        if let Some(q) = filter_questions
            .into_iter()
            .find(|q| q.id == id || q.slug == id)
        {
            link = Some(gen_link_by_slug(&q.slug));
            question = Some(q);
            break;
        } else {
            println!("question not found, skip {}", skip);
            max_attempt_times -= 1;
        }
    }
    if let (Some(question), Some(link)) = (question, link) {
        QuestionBaseInfo::new(question.id, link, question.title, question.slug)
    } else {
        panic!("question not found");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_code() {
        let code = get_code("274").await;
        println!("{}", code);
    }
}
