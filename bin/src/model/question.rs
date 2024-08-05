#![allow(unused)]
#[derive(Debug)]
pub struct FilterQuestion {
    pub difficulty: String,
    pub id: String,
    pub title: String,
    pub slug: String,
}

impl FilterQuestion {
    pub fn new(difficulty: String, id: String, title: String, slug: String) -> Self {
        Self {
            difficulty,
            id,
            title,
            slug,
        }
    }
}

pub struct TranlatedQuestion {
    pub title: String,
    pub content: String,
}

impl TranlatedQuestion {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

use super::example::Example;

#[derive(Debug)]
pub struct QuestionBaseInfo {
    pub id: String,
    pub link: String,
    pub name: String,
    pub slug: String,
}

impl QuestionBaseInfo {
    pub fn new(id: String, link: String, name: String, slug: String) -> Self {
        QuestionBaseInfo {
            id,
            link,
            name,
            slug,
        }
    }
}

#[derive(Debug)]
pub struct QuestionInfo {
    pub base_info: QuestionBaseInfo,
    pub content: String,
    pub template: String,
    pub examples: Vec<Example>,
}

impl QuestionInfo {
    pub fn new(
        base_info: QuestionBaseInfo,
        content: String,
        template: String,
        examples: Vec<Example>,
    ) -> Self {
        QuestionInfo {
            base_info,
            content,
            template,
            examples,
        }
    }

    pub fn to_string(self) -> String {
        let line = self.content.trim().split('\n');

        let comment = line
            .into_iter()
            .map(|s| "/// ".to_string() + s)
            .collect::<Vec<_>>()
            .join("\n")
            .to_string();
        format!(
            r#"/// {}.{}
/// {}
{}
/// <a href="{}">{}</a>
pub struct Solution;

{}

#[cfg(test)]
mod test {{
    use super::*;
    #[test]
    fn test_1() {{
        
    }}
}}"#,
            self.base_info.id,
            self.base_info.name,
            self.base_info.slug,
            comment,
            self.base_info.link,
            self.base_info.name,
            self.template
        )
    }
}
