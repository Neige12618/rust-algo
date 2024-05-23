use reqwest::Client;

use crate::model::{
    sub_result::SubResult,
    submit::{SubmitRequest, SubmitResponse},
};

pub async fn submit(id: usize, slug: &str, code: &str) -> SubmitResponse {
    let url = dotenvy::var("LEETCODE_BASE_URL").expect("LEETCODE_BASE_URL not set");
    let leetcode_session = dotenvy::var("LEETCODE_SESSION").expect("LEETCODE_SESSION not set");
    let client = Client::new();

    let submit_request = SubmitRequest::new("rust".to_string(), id, code.to_string());

    let response = client
        .post(format!("{}/problems/{}/submit/", &url, slug))
        .header("Origin", &url)
        .header("Referer", format!("{}/problems/{}/", &url, slug))
        .header("Cookie", format!("LEETCODE_SESSION={}", leetcode_session))
        .body(serde_json::to_string(&submit_request).unwrap())
        .send()
        .await
        .unwrap();

    serde_json::from_str(&response.text().await.unwrap()).unwrap()
}

pub async fn get_sub_result(id: usize, slug: &str) -> SubResult {
    let base_url = dotenvy::var("LEETCODE_BASE_URL").expect("LEETCODE_BASE_URL not set");
    let client = Client::new();

    let response = client
        .get(format!("{}/submissions/detail/{}/check", &base_url, id))
        .header("Referer", format!("{}/problems/{}/", &base_url, slug))
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();

    serde_json::from_str(&text).unwrap()
}

#[cfg(test)]
mod test {
    use crate::graphql::request::get_question_base_info;

    use super::*;

    #[tokio::test]
    async fn test_submit() {
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let graphql_url = format!("{}/graphql/", &url);

        let question_base_info = get_question_base_info(&graphql_url).await;

        let submit_response = submit(
            question_base_info.id,
            &question_base_info.slug,
            "fn main() { println!(\"Hello, world!\"); }",
        )
        .await;

        println!("{:?}", submit_response);
    }

    #[tokio::test]
    async fn test_get_submissions() {
        let url = dotenvy::var("LEETCODE_BASE_URL").unwrap();
        let graphql_url = format!("{}/graphql/", &url);

        let question_base_info = get_question_base_info(&graphql_url).await;
        let submission_id = 534224963;
        let submission_result = get_sub_result(submission_id, &question_base_info.slug).await;
        println!("{:?}", submission_result);
    }
}
