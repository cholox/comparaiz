// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// mod service_web;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug)]
struct Place {
    area: u32,
    price: u64,
}

fn extract_area_from_description(description: &str) -> Option<u32> {
    let area_keyword = "mts2";
    if let Some(pos) = description.trim().find(area_keyword) {
        if let Ok(number) = description[..pos].trim().parse::<u32>() {
            return Some(number);
        }
    }
    None
}

fn convert_price_to_number(price_str: &str) -> Option<u64> {
    let cleaned_price_str: String = price_str.chars().filter(|c| c.is_digit(10)).collect();
    cleaned_price_str.parse::<u64>().ok()
}

#[tauri::command]
fn parse_page_finca_raiz() -> String {
    let url = "https://fincaraiz.elpais.com.co/avisos/venta/apartamentos/cali/cuarto-de-legua";
    let response = reqwest::blocking::get(url,)
    .unwrap()
    .text()
    .unwrap();
    let document = scraper::Html::parse_document(&response);
    let article_selector = scraper::Selector::parse("article.flexArticle").unwrap();
    let selector_description = scraper::Selector::parse("div.description").unwrap();
    let selector_price = scraper::Selector::parse("div.price").unwrap();
    // let articles = document.select(&article_selector).map(|x| x.inner_html());

    // Vec to store products
    let mut places = Vec::new();

    for art in document.select(&article_selector) {
        let description = art.select(&selector_description).next().map(|x| x.text().collect::<String>());
        let price = art.select(&selector_price).next().map(|x| x.text().collect::<String>());

        println!("Price {}", price.clone().unwrap_or_default());
        let price = price.as_ref().and_then(|p| convert_price_to_number(p)).unwrap_or_default();
        let area = description.as_ref().and_then(|desc| extract_area_from_description(desc)).unwrap_or_default();
        let place = Place {
            area,
            price,
        };
        places.push(place);
    }

    println!("{:#?}", places);
    let mut final_string = String::from("");
    if final_string.is_empty() {
        final_string = String::from("No articles found!");
    }
    println!("{}", final_string);
    final_string
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, parse_page_finca_raiz])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
