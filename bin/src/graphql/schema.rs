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

#[cfg(test)]
mod test {
    use crate::graphql::request::get;

    use super::*;

    #[tokio::test]
    async fn test_calendar_task_schedule() -> Result<(), Box<dyn std::error::Error>> {
        let url = dotenvy::var("LEETCODE_URL")?;
        let response_body =
            get::<CalendarTaskSchedule>(&url, calendar_task_schedule::Variables { days: 0 }).await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");

        Ok(())
    }

    #[tokio::test]
    async fn test_question_translations() -> Result<(), Box<dyn std::error::Error>> {
        let url = dotenvy::var("LEETCODE_URL")?;
        let response_body = get::<questionTranslations>(
            &url,
            question_translations::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");

        Ok(())
    }

    #[tokio::test]
    async fn test_question_editor_data() -> Result<(), Box<dyn std::error::Error>> {
        let url = dotenvy::var("LEETCODE_URL")?;
        let response_body = get::<questionEditorData>(
            &url,
            question_editor_data::Variables {
                title_slug: "find-players-with-zero-or-one-losses".to_string(),
            },
        )
        .await;
        assert_eq!(response_body.errors, None);

        println!("{response_body:?}");

        Ok(())
    }
}
