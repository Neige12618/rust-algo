use gen::create_template_file;
use graphql::request::{
    get_daily_info, get_example_tests, get_question_editor_data, get_question_translations,
};
use model::daily_info::QuestionInfo;

mod gen;
mod graphql;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = dotenvy::var("LEETCODE_URL").unwrap();

    let daily_question_info = get_daily_info(&url).await;
    let question_translation = get_question_translations(&daily_question_info.slug, &url).await;
    let code_template = get_question_editor_data(&daily_question_info.slug, &url)
        .await
        .unwrap();
    let examples = get_example_tests(&daily_question_info.slug, &url).await;

    let question_info = QuestionInfo::new(
        daily_question_info,
        question_translation.content,
        code_template.content,
        examples,
    );

    create_template_file(question_info).await;

    Ok(())
}
