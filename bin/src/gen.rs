use tokio::io::AsyncWriteExt;

use crate::model::daily_info::QuestionInfo;

pub async fn create_template_file(daily_info: QuestionInfo) {
    let path_str = format!("./solution/solution{}.rs", daily_info.base_info.id);
    let file_path = std::path::Path::new(&path_str);

    if file_path.exists() {
        println!("file already exists");
        return;
    }

    let mut file = tokio::fs::File::create(file_path).await.unwrap();

    file.write_all(daily_info.to_string().as_bytes())
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn it_works() {
        let file = tokio::fs::File::create("../solution/solution.rs")
            .await
            .unwrap();
        println!("{file:?}")
    }
}
