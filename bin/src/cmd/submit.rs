use core::panic;
use std::{env, time::Duration};

use tokio::{io::AsyncReadExt, time::sleep};

use crate::{
    action::submit,
    graphql::request::{get_question_backend_id, get_question_base_info},
    model::sub_result::SubResult,
    util::path::{get_graphql_url, get_solution_by_id},
};

use super::SubmitArgs;

pub async fn submit_and_check(_args: SubmitArgs) {
    let graphql_url = get_graphql_url();

    let base_info = get_question_base_info(&graphql_url).await;
    let backend_id = get_question_backend_id(&base_info.slug, &graphql_url).await;

    let response = submit::submit(backend_id, &base_info.slug, &get_code(base_info.id).await).await;

    loop {
        match submit::get_sub_result(response.submission_id, &base_info.slug).await {
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

async fn get_code(id: usize) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_code() {
        let code = get_code(274).await;
        println!("{}", code);
    }
}
