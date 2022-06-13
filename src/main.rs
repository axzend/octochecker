use reqwest::StatusCode;
use scraper::{Html,Selector};
use colored::Colorize;

mod utils;

#[tokio::main]
async fn main() {
    
    let client = utils::get_client();
    let url = "https://www.roblox.com/bundles/201";
    let result = client.get(url).send().await.unwrap();

    let raw = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Oops! something went mad goofy"),
    };

    let document = Html::parse_document(&raw);
    let iname = Selector::parse("h1").unwrap();

    for element in document.select(&iname) {
        let inametxt = element.inner_html().to_string();
        println!("Item Name = {}", &inametxt);
    }

    if raw.contains("PurchaseButton") {
        println!("{}", "[AVAILABLE!]".green().bold());
    } else {
        println!("{}", "[UNAVAILABLE]".yellow().bold());
    }

}
