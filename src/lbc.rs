use reqwest::blocking::*;
use reqwest::header::*;

/// client() creates a reqwest Client configured to interact with LeBonCoin.
pub fn client() -> reqwest::Result<Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent", 
        HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36"));
    headers.insert("Accept-Language", HeaderValue::from_static("*"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-GB,en-US;q=0.9,en;q=0.8"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://www.leboncoin.fr/recherche/"),
    );
    headers.insert(
        "Origin",
        HeaderValue::from_static("https://www.leboncoin.fr"),
    );
    headers.insert("api_key", HeaderValue::from_static("ba0c2dad52b3ec"));
    reqwest::blocking::ClientBuilder::new()
        .default_headers(headers)
        .gzip(true)
        .build()
}

/// SearchResult from a call to finder/search api.
#[derive(Debug, serde::Deserialize)]
pub struct SearchResult {
    total: u32,
    total_all: u32,
    total_pro: u32,
    total_private: u32,
    total_active: u32,
    total_inactive: u32,
    max_pages: u32,
    referrer_id: String,
    pivot: String,
    ads: Vec<Ad>,
}

/// Ad is an ad as returned by finder/search api.
#[derive(Debug, serde::Deserialize)]
pub struct Ad {
    ad_type: String,
    price: Vec<u32>,
    price_calendar: serde_json::Value,
    status: String,
    subject: String,
    url: String,
    body: String,
    category_id: String,
    category_name: String,
    expiration_date: Option<String>,
    first_publication_date: String,
    has_phone: bool,
    index_date: String,
    list_id: u32,
    attributes: Vec<Attribute>,
    images: serde_json::Value,
    location: serde_json::Value,
    options: serde_json::Value,
    owner: serde_json::Value,
}

/// Attribute describes the properties of the object sold by an ad.
#[derive(Debug, serde::Deserialize)]
struct Attribute {
    generic: bool,
    key: String,
    key_label: Option<String>,
    value: String,
    value_label: String,
}
