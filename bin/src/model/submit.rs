use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct SubmitRequest<'a> {
    lang: &'a str,
    question_id: &'a str,
    typed_code: &'a str,
}

impl<'a> SubmitRequest<'a> {
    pub fn new(lang: &'a str, question_id: &'a str, typed_code: &'a str) -> Self {
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
