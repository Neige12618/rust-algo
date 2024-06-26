use reqwest::Client;

use crate::{
    model::{
        sub_result::SubResult,
        submit::{SubmitRequest, SubmitResponse},
    },
    util::path::{get_base_url, get_leetcode_session},
};

pub async fn submit(backend_id: &str, slug: &str, code: &str) -> SubmitResponse {
    let url = get_base_url();
    let leetcode_session = get_leetcode_session();
    let client = Client::new();

    let submit_request = SubmitRequest::new("rust", backend_id, code);

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
    let base_url = get_base_url();
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
    use crate::{graphql::request::get_question_base_info, util::path::get_graphql_url};

    use super::*;

    #[tokio::test]
    async fn test_submit() {
        let graphql_url = get_graphql_url();

        let question_base_info = get_question_base_info(&graphql_url).await;

        let submit_response = submit(
            &question_base_info.id,
            &question_base_info.slug,
            "fn main() { println!(\"Hello, world!\"); }",
        )
        .await;

        println!("{:?}", submit_response);
    }

    #[tokio::test]
    async fn test_get_submissions() {
        let graphql_url = get_graphql_url();

        let question_base_info = get_question_base_info(&graphql_url).await;
        let submission_id = 534484892;
        let submission_result = get_sub_result(submission_id, &question_base_info.slug).await;
        println!("{:#?}", submission_result);
    }
}
