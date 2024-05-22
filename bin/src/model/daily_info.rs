use super::example::Example;

#[derive(Debug)]
pub struct DailyInfo {
    pub id: u32,
    pub link: String,
    pub name: String,
    pub slug: String,
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
}

#[derive(Debug)]
pub struct QuestionInfo {
    pub base_info: DailyInfo,
    pub content: String,
    pub template: String,
    pub examples: Vec<Example>,
}

impl QuestionInfo {
    pub fn new(
        base_info: DailyInfo,
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
            "/// {}.{}\n///\n/// {}\nstruct Solution;\n\n{}\n\n#[cfg(test)]\nmod test {{\n\n    #[test]\n    fn tests() {{\n\n    }}\n}}",
            self.base_info.id, self.base_info.name, comment, self.template
        )
    }
}
