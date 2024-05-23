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

#[cfg(test)]
mod test {
    use crate::graphql::request::get;

    use super::*;

    #[tokio::test]
    async fn test_calendar_task_schedule() {
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let url = format!("{}/graphql", url);
        let response_body =
            get::<CalendarTaskSchedule>(&url, calendar_task_schedule::Variables { days: 0 }).await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }

    #[tokio::test]
    async fn test_question_translations() {
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let url = format!("{}/graphql", url);
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
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let url = format!("{}/graphql", url);
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
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let url = format!("{}/graphql", url);
        let response_body = get::<consolePanelConfig>(
            &url,
            console_panel_config::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;

        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");
    }
}
