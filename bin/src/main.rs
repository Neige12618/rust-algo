use graphql::request::get_daily_info;
use scraper::scrape_page;

mod graphql;
mod model;
mod scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = dotenvy::var("LEETCODE_URL").unwrap();

    let daily_question_info = get_daily_info(&url).await;
    scrape_page(daily_question_info.url()).await?;

    Ok(())
}
