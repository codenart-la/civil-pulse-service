use reqwest::blocking;
use scraper::{selector, Html, Selector};

pub fn fetch_wiki() -> Result<String, Box<dyn std::error::Error>> {
    let response = blocking::get("https://en.wikipedia.org/wiki/Battle_of_the_Bulge");
    let html_content = response?.text()?;

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
