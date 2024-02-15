use scraper::{Html, Selector};
use axum::extract::Path;
async fn fetch_wiki<'a>(myselector: &'a str) -> Result<String, Box<dyn std::error::Error + 'a>> {
    let response = reqwest::get("https://en.wikipedia.org/wiki/Battle_of_the_Bulge").await;
    let html_content = response?.text();

    // let's use a parser
    let document = Html::parse_document(&html_content.await?);

    let header_selector = Selector::parse(&myselector)?;
    let header = document.select(&header_selector);

    for h in header {
        println!("{}", h.inner_html());
        return Ok(h.inner_html());
    }

    return Ok("couldn't find anything".to_string());

}


pub async fn wiki(Path(ll): Path<String>) -> String {
    fetch_wiki(&ll).await.unwrap_or_else(|e| {
        tracing::error!("error fetching wiki: {:?}", e);
        format!("error fetching wiki: {:?}", e)
    })
}


