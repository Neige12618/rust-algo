use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

use crate::{
    graphql::schema::{questionEditorData, questionTranslations},
    model::{daily_info::DailyInfo, question::Question, template::CodeTemplate},
};

use super::schema::{
    calendar_task_schedule, question_editor_data, question_translations, CalendarTaskSchedule,
};

pub async fn get<Q: GraphQLQuery>(url: &str, variables: Q::Variables) -> Response<Q::ResponseData> {
    let client = Client::new();
    let request_body = Q::build_query(variables);

    let res = client.post(url).json(&request_body).send().await.unwrap();
    let response_body: Response<Q::ResponseData> = res.json().await.unwrap();
    response_body
}

pub async fn get_daily_info(url: &str) -> DailyInfo {
    let data = get::<CalendarTaskSchedule>(url, calendar_task_schedule::Variables { days: 0 })
        .await
        .data
        .unwrap();

    let task_schedule = data.calendar_task_schedule.expect("No task_schedule found");
    let mut daily_questions = task_schedule
        .daily_questions
        .expect("daily_questions not found");

    let first_question = daily_questions
        .pop()
        .expect("No questions found")
        .expect("No question found");

    DailyInfo::new(
        first_question.id.unwrap().parse().unwrap(),
        first_question.link.unwrap(),
        first_question.name.unwrap(),
        first_question.slug.unwrap(),
    )
}

pub async fn get_question_translations(slug: &str, url: &str) -> Question {
    let data = get::<questionTranslations>(
        url,
        question_translations::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();
    let question = data.question.unwrap();

    Question::new(
        question.translated_title.unwrap(),
        question.translated_content.unwrap(),
    )
}

async fn get_question_editor_data(slug: &str, url: &str) -> Option<CodeTemplate> {
    let data = get::<questionEditorData>(
        url,
        question_editor_data::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();

    let code_snippet = data.question.unwrap().code_snippets.unwrap();

    for i in code_snippet.into_iter().filter_map(|x| x) {
        if i.lang.unwrap() == "Rust".to_string() {
            return Some(CodeTemplate::new(i.code.unwrap()));
        }
    }

    None
}
