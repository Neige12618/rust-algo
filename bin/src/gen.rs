use std::env;

use tokio::io::AsyncWriteExt;

use crate::model::daily_info::QuestionInfo;

pub async fn create_template_file(daily_info: QuestionInfo) {
    println!("{:?}", env::current_dir());
    let mut file = tokio::fs::File::create(&format!(
        "./solution/solution{}.rs",
        daily_info.base_info.id
    ))
    .await
    .unwrap();

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
