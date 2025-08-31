use reqwest::blocking;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://books.toscrape.com/";
    let body = blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);
    let title_selector = Selector::parse("h3 a").unwrap();
    let price_selector = Selector::parse(".price_color").unwrap();
    for (title_elem, price_elem) in document
        .select(&title_selector)
        .zip(document.select(&price_selector))
    {
        let name = title_elem.value().attr("title").unwrap_or("No title");
        let price = price_elem.text().collect::<Vec<_>>().join("").to_string();
        println!("{} - {}", name, price);
    }
    Ok(())
}
