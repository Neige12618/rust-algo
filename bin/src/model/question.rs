pub struct Question {
    pub title: String,
    pub content: String,
}

impl Question {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}
