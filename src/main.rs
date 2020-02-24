extern crate reqwest;

use reqwest::blocking::Client;
use reqwest::blocking::{RequestBuilder, Response};
use std::collections::HashMap;

fn lbc_post(c: &Client, url: &str) -> RequestBuilder {
    c.post(url)
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/76.0.3809.100 Safari/537.36")
    .header("Accept-Language", "*")
    .header("Accept", "application/json")
    .header("Accept-Encoding",  "gzip, deflate, br")
    .header("Accept-Language", "en-GB,en-US;q=0.9,en;q=0.8")
    .header("Referer", "https://www.leboncoin.fr/recherche/")
    .header("Origin", "https://www.leboncoin.fr")
    .header("api_key", "ba0c2dad52b3ec")
}

fn print_api(prefix: &str, r: Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}: status = {:?}", prefix, r.status());
    println!("{}: headers = {:?}", prefix, r.headers());
    println!("{}: body = {:?}", prefix, r.text()?);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // const SEARCH_URL : &str = "https://www.leboncoin.fr/recherche/?category=9&locations=d_38,d_69,d_1&immo_sell_type=old,new&real_estate_type=1&price=200000-400000&rooms=4-max&square=80-max";

    let api_url = "https://api.leboncoin.fr/finder/search";
    let search_params = "{\"filters\":{\"category\":{\"id\":\"9\"},\"enums\":{\"ad_type\":[\"offer\"],\"immo_sell_type\":[\"old\",\"new\"],\"real_estate_type\":[\"1\"]},\"keywords\":{},\"location\":{\"locations\":[{\"locationType\":\"department\",\"label\":\"Isère\",\"department_id\":\"38\",\"region_id\":\"22\"},{\"locationType\":\"department\",\"label\":\"Rhône\",\"department_id\":\"69\",\"region_id\":\"22\"},{\"locationType\":\"department\",\"label\":\"Ain\",\"department_id\":\"1\",\"region_id\":\"22\"}]},\"ranges\":{\"price\":{\"min\":200000,\"max\":400000},\"rooms\":{\"min\":4},\"square\":{\"min\":80}}},\"limit\":35,\"limit_alu\":3,\"user_id\":\"d2f08b09-1a54-49bc-9d96-4ad96b227df2\",\"store_id\":\"47756443\"}";
    let c = reqwest::blocking::Client::builder().gzip(true).build()?;

    let login_url = "https://api.leboncoin.fr/api/oauth/v1/token";
    let mut login_params = HashMap::new();
    let password = std::env::var("LBC_PASSWORD")?;
    login_params.insert("client_id", "frontweb");
    login_params.insert("grant_type", "password");
    login_params.insert("username", "leroi.g@gmail.com");
    login_params.insert("password", &password);
    let login = lbc_post(&c, login_url)
        .form(&login_params)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()?;
    print_api("login", login)?;

    // login user
    /*
    client_id=frontweb&grant_type=password&password=cXVa2I0L&username=leroi.g%40gmail.com
        request.post({
            uri: "https://api.leboncoin.fr/api/oauth/v1/token",
            form: {
                "client_id": 'frontweb',
                "grant_type": 'password',
                "username": self.email,
                "password": self.password
            },
            jar: utils.cookieJar,
            headers: utils.requestHeaders,
    */
    // retrieve token (field access_token)

    // to search request
    let r = lbc_post(&c, api_url)
        .header("Content-Type", "application/json")
        .body(search_params)
        .send()?;
    print_api("search", r)?;
    Ok(())
}
