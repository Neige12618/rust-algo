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
    pub lang: String,
    pub run_success: bool,
    pub status_runtime: String,
    pub memory: i64,
    pub question_id: i32,
    pub elapsed_time: Option<i32>,
    pub compare_result: Option<String>,
    pub code_output: Option<String>,
    pub std_output: Option<String>,
    pub last_testcase: Option<String>,
    pub expected_output: Option<String>,
    pub task_finish_time: i64,
    pub task_name: String,
    pub finished: bool,
    pub status_msg: String,
    pub state: String,
    pub fast_submit: bool,
    pub total_correct: Option<i32>,
    pub total_testcases: Option<i32>,
    pub submission_id: String,
    pub runtime_percentile: Option<f64>,
    pub status_memory: String,
    pub memory_percentile: Option<f64>,
    pub pretty_lang: String,
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
            "{}/{}\n测试数据:\n{}\n你的输出:\n{}\n期望输出:\n{}",
            self.total_correct.unwrap(),
            self.total_testcases.unwrap(),
            self.last_testcase.as_ref().unwrap(),
            self.code_output.as_ref().unwrap(),
            self.expected_output.as_ref().unwrap()
        )
    }

    pub fn ac_output(&self) -> String {
        format!("{}", self.runtime_percentile.unwrap())
    }
}
