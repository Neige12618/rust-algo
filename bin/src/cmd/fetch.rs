use crate::{
    action::gen::create_template_file,
    graphql::request::{
        get_example_tests, get_question_base_info, get_question_code_template,
        get_question_translations,
    },
    model::daily_info::QuestionInfo,
};

use super::FetchArgs;

pub async fn fetch_and_gen(_args: FetchArgs) {
    let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
    let graphql_url = format!("{}/graphql/", &url);

    let question_base_info = get_question_base_info(&graphql_url).await;
    let question_translation =
        get_question_translations(&question_base_info.slug, &graphql_url).await;
    let code_template = get_question_code_template(&question_base_info.slug, &graphql_url)
        .await
        .unwrap();
    let examples = get_example_tests(&question_base_info.slug, &graphql_url).await;

    let question_info = QuestionInfo::new(
        question_base_info,
        question_translation.content,
        code_template.content,
        examples,
    );

    create_template_file(question_info).await;
}
