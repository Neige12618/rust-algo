pub struct DailyInfo {
    id: u32,
    link: String,
    name: String,
    slug: String,
}

impl DailyInfo {
    pub fn new(id: u32, link: String, name: String, slug: String) -> Self {
        DailyInfo {
            id,
            link,
            name,
            slug,
        }
    }

    pub fn url(&self) -> &str {
        &self.link
    }
}

pub struct QuestionInfo {
    base_info: DailyInfo,
    content: String,
    difficulty: String,
    desc: String,
    example: String,
    explain: String,
    similar_questions: Vec<DailyInfo>,
    title: String,
}
