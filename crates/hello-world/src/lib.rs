use reqwest::blocking;
use scraper::{selector, Html, Selector};

    let html_content = response?.text()?;
pub async fn fetch_wiki<'a>(myselector: &'a str) -> Result<String, Box<dyn std::error::Error + 'a>> {
    let response = reqwest::get("https://en.wikipedia.org/wiki/Battle_of_the_Bulge").await;

    // let's use a parser
    let document = Html::parse_document(&html_content);

    let header_selector = scraper::Selector::parse("h1 > span")?;
    let header = document.select(&header_selector);

    for h in header {
        println!("{}", h.inner_html());
        return Ok(h.inner_html());
    }

    return Ok("couldn't find anything".to_string());

}
