#![allow(unused)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SubResult {
    Finished(FinishedResult),
    Judging(JudgeResult),
}

#[derive(Deserialize, Debug)]
pub struct JudgeResult {
    pub state: String,
}

#[derive(Deserialize, Debug)]
pub struct FinishedResult {
    pub status_code: i32,
    pub status_runtime: String,
    pub memory: i64,
    pub question_id: String,
    pub compare_result: Option<String>,
    pub code_output: Option<String>,
    pub last_testcase: Option<String>,
    pub expected_output: Option<String>,
    pub status_msg: String,
    pub state: String,
    pub total_correct: Option<i32>,
    pub total_testcases: Option<i32>,
    pub submission_id: String,
    pub runtime_percentile: Option<f64>,
    pub status_memory: String,
    pub memory_percentile: Option<f64>,
    pub compile_error: Option<String>,
    pub full_compile_error: Option<String>,
}

impl FinishedResult {
    pub fn ac(&self) -> bool {
        self.status_msg == "Accepted"
    }

    pub fn wa(&self) -> bool {
        self.status_msg == "Wrong Answer"
    }

    pub fn wa_output(&self) -> String {
        format!(
            r#"{}/{}
测试数据:
{}
你的输出:
{}
期望输出:
{}"#,
            self.total_correct.unwrap(),
            self.total_testcases.unwrap(),
            self.last_testcase.as_ref().unwrap(),
            self.code_output.as_ref().unwrap(),
            self.expected_output.as_ref().unwrap()
        )
    }

    pub fn ac_output(&self) -> String {
        format!(
            "运行时长: {} 超过了{}%\n占用内存: {} 超过了{}%",
            self.status_runtime,
            self.runtime_percentile.unwrap(),
            self.status_memory,
            self.memory_percentile.unwrap()
        )
    }
}
