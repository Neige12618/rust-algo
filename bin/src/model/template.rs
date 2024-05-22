pub struct CodeTemplate {
    // 模板内容
    content: String,
}

impl CodeTemplate {
    // 创建一个新的模板
    pub fn new(content: String) -> Self {
        CodeTemplate { content }
    }
}
