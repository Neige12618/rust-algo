use crate::{
    action::gen::create_template_file,
    graphql::request::{
        get_example_tests, get_question_base_info, get_question_code_template,
        get_question_translations, search_question,
    },
    model::question::{QuestionBaseInfo, QuestionInfo},
    util::{
        input::get_correct_number,
        path::{gen_link_by_slug, get_graphql_url},
    },
};

use super::FetchArgs;

pub async fn fetch_and_gen(args: FetchArgs) {
    let graphql_url = get_graphql_url();

    let base_info = if let Some(filter) = args.filter {
        let mut filter_questions = search_question(&filter, &graphql_url).await;
        for (index, value) in filter_questions.iter().enumerate() {
            println!("{}) {} {}", index, value.id, value.title);
        }
        println!("请输入题目前编号（默认为0）:");
        let index = get_correct_number(filter_questions.len());
        let selected = filter_questions.swap_remove(index);
        let link = gen_link_by_slug(&selected.slug);
        QuestionBaseInfo::new(selected.id, link, selected.title, selected.slug)
    } else {
        get_question_base_info(&graphql_url).await
    };

    let translation = get_question_translations(&base_info.slug, &graphql_url).await;
    let template = get_question_code_template(&base_info.slug, &graphql_url)
        .await
        .unwrap();
    let examples = get_example_tests(&base_info.slug, &graphql_url).await;

    let question_info =
        QuestionInfo::new(base_info, translation.content, template.content, examples);

    create_template_file(question_info).await;
}
