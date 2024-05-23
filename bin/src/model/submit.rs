use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct SubmitRequest {
    lang: String,
    question_id: usize,
    typed_code: String,
}

impl SubmitRequest {
    pub fn new(lang: String, question_id: usize, typed_code: String) -> Self {
        Self {
            lang,
            question_id,
            typed_code,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitResponse {
    pub submission_id: usize,
}
