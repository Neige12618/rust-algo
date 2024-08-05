#![allow(non_camel_case_types)]

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/calendar/taskScheduleS.graphql",
    query_path = "graphql/calendar/taskScheduleQ.graphql",
    response_derives = "Debug"
)]
pub struct CalendarTaskSchedule;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/question/translationS.graphql",
    query_path = "graphql/question/translationQ.graphql",
    response_derives = "Debug"
)]
pub struct questionTranslations;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/question/editorS.graphql",
    query_path = "graphql/question/editorQ.graphql",
    response_derives = "Debug"
)]
pub struct questionEditorData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/console/panelConfigS.graphql",
    query_path = "graphql/console/panelConfigQ.graphql",
    response_derives = "Debug"
)]
pub struct consolePanelConfig;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/question/listS.graphql",
    query_path = "graphql/question/listQ.graphql",
    response_derives = "Debug"
)]
pub struct problemsetQuestionList;

#[cfg(test)]
mod test {
    use problemset_question_list::QuestionListFilterInput;

    use crate::{graphql::request::get, util::path::get_graphql_url};

    use super::*;

    #[tokio::test]
    async fn test_calendar_task_schedule() {
        let url = get_graphql_url();
        let response_body =
            get::<CalendarTaskSchedule>(&url, calendar_task_schedule::Variables { days: 0 }).await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }

    #[tokio::test]
    async fn test_question_translations() {
        let url = get_graphql_url();
        let response_body = get::<questionTranslations>(
            &url,
            question_translations::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }

    #[tokio::test]
    async fn test_question_editor_data() {
        let url = get_graphql_url();
        let response_body = get::<questionEditorData>(
            &url,
            question_editor_data::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }

    #[tokio::test]
    async fn test_console_panel_config() {
        let url = get_graphql_url();
        let response_body = get::<consolePanelConfig>(
            &url,
            console_panel_config::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;

        assert_eq!(response_body.errors, None);

        println!("{response_body:#?}");
    }

    #[tokio::test]
    async fn test_problemset_question_list() {
        let url = get_graphql_url();
        let response_body = get::<problemsetQuestionList>(
            &url,
            problemset_question_list::Variables {
                category_slug: "all-code-essentials".to_string(),
                limit: 50,
                skip: 0,
                filters: Some(QuestionListFilterInput {
                    search_keywords: "two-sum".to_string(),
                }),
            },
        )
        .await;

        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }
}
