use reqwest::blocking;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = "https://books.toscrape.com/".to_string();

    loop {
        let body = blocking::get(&url)?.text()?;
        let document = Html::parse_document(&body);
        let title_selector = Selector::parse("h3 a").unwrap();
        let price_selector = Selector::parse(".price_color").unwrap();
        let next_selector = Selector::parse("li.next a").unwrap();

        for (title_elem, price_elem) in document
            .select(&title_selector)
            .zip(document.select(&price_selector))
        {
            let name = title_elem.value().attr("title").unwrap_or("No title");
            let price = price_elem.text().collect::<Vec<_>>().join("").to_string();
            println!("{} - {}", name, price);
        }

        if let Some(next_page) = document.select(&next_selector).next() {
            let next_href = next_page.value().attr("href").unwrap();
            url = if url.ends_with('/') {
                format!("{}{}", url, next_href)
            } else {
                let base = url.rsplit_once('/').unwrap().0; // everything before last /
                format!("{}{}", base, next_href)
            }
        } else {
            break; // no more pages
        }
    }
    Ok(())
}
