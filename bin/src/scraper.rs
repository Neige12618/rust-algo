pub async fn scrape_page(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    println!("{url}");

    Ok(body)
}
