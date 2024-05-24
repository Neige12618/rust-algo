use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

use crate::{
    graphql::schema::{console_panel_config, questionEditorData, questionTranslations},
    model::{
        example::Example,
        question::QuestionBaseInfo,
        question::{FilterQuestion, TranlatedQuestion},
        template::CodeTemplate,
    },
};

use super::schema::{
    calendar_task_schedule, consolePanelConfig, problemsetQuestionList,
    problemset_question_list::{self, QuestionListFilterInput},
    question_editor_data, question_translations, CalendarTaskSchedule,
};

pub async fn get<Q: GraphQLQuery>(url: &str, variables: Q::Variables) -> Response<Q::ResponseData> {
    let client = Client::new();
    let request_body = Q::build_query(variables);

    let res = client.post(url).json(&request_body).send().await.unwrap();
    let response_body: Response<Q::ResponseData> = res.json().await.unwrap();
    response_body
}

pub async fn get_question_base_info(url: &str) -> QuestionBaseInfo {
    let data = get::<CalendarTaskSchedule>(url, calendar_task_schedule::Variables { days: 0 })
        .await
        .data
        .unwrap();

    let task_schedule = data.calendar_task_schedule.expect("No task_schedule found");
    let mut daily_questions = task_schedule.daily_questions;

    let first_question = daily_questions.pop().expect("No questions found");

    QuestionBaseInfo::new(
        first_question.id.parse().unwrap(),
        first_question.link,
        first_question.name,
        first_question.slug,
    )
}

pub async fn get_question_translations(slug: &str, url: &str) -> TranlatedQuestion {
    let data = get::<questionTranslations>(
        url,
        question_translations::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();
    let question = data.question;

    TranlatedQuestion::new(question.translated_title, question.translated_content)
}

// 此接口只需要获得代码模板
pub async fn get_question_code_template(slug: &str, url: &str) -> Option<CodeTemplate> {
    let data = get::<questionEditorData>(
        url,
        question_editor_data::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();

    let code_snippet = data.question.code_snippets;

    for i in code_snippet.into_iter() {
        if i.lang == "Rust".to_string() {
            return Some(CodeTemplate::new(i.code));
        }
    }

    None
}

// 此接口只需要获得代码模板
pub async fn get_question_backend_id(slug: &str, url: &str) -> usize {
    let data = get::<questionEditorData>(
        url,
        question_editor_data::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();

    data.question.question_id.parse().unwrap()
}

pub async fn get_example_tests(slug: &str, url: &str) -> Vec<Example> {
    let data = get::<consolePanelConfig>(
        url,
        console_panel_config::Variables {
            title_slug: slug.to_string(),
        },
    )
    .await
    .data
    .unwrap();

    let mut example_tests = vec![];

    let question = data.question;

    let input_example = question.example_testcases;
    let output_example = question.sample_test_case;

    for (input, output) in input_example.split('\n').zip(output_example.split('\n')) {
        example_tests.push(Example::new(input.to_string(), output.to_string()));
    }

    example_tests
}

pub async fn search_question(keyword: &str, url: &str) -> Vec<FilterQuestion> {
    let data = get::<problemsetQuestionList>(
        url,
        problemset_question_list::Variables {
            category_slug: "all-code-essentials".to_string(),
            limit: 10,
            skip: 0,
            filters: Some(QuestionListFilterInput {
                searchKeywords: keyword.to_string(),
            }),
        },
    )
    .await
    .data
    .unwrap();

    let questions = data.problemset_question_list.questions;

    questions
        .into_iter()
        .map(|q| {
            FilterQuestion::new(
                q.difficulty,
                q.frontend_question_id,
                q.title_cn,
                q.title_slug,
            )
        })
        .collect()
}
