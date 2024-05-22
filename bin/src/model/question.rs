pub struct Question {
    title: String,
    content: String,
}

impl Question {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
