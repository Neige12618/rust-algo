#![allow(unused)]
#[derive(Debug)]
pub struct Example {
    pub input: String,
    pub output: String,
}

impl Example {
    pub fn new(input: String, output: String) -> Self {
        Self { input, output }
    }
}
