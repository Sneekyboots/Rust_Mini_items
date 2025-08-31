use reqwest::blocking;
use scraper::{Html, Selector};
use serde::Serialize;
use std::fs::File;
use std::io::Write;


#[derive(Debug,Serialize)]
struct Book{
    title:String,
    price:String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = "https://books.toscrape.com/".to_string();
    let mut books:Vec<Book>=Vec::new();

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
            books.push(Book{title:name.to_string(),price})

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

    let json=serde_json::to_string_pretty(&books)?;
    let mut file=File::create("books.json")?;
    file.write_all(json.as_bytes())?;
    println!("Saved {} books to books.json", books.len());
    Ok(())
}
