extern crate reqwest;

mod lbc;

use reqwest::blocking::*;

fn print_api(prefix: &str, r: Response) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}: status = {:?}", prefix, r.status());
    println!("{}: headers = {:?}", prefix, r.headers());
    // println!("{}: body = {:?}", prefix, r.text()?);
    println!(
        "{}: body = {:?}",
        prefix,
        r.json::<lbc::search::Response>()?
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = lbc::client()?;

    // login user
    // let login_url = "https://api.leboncoin.fr/api/oauth/v1/token";
    // let mut login_params = HashMap::new();
    // let password = std::env::var("LBC_PASSWORD")?;
    // login_params.insert("client_id", "frontweb");
    // login_params.insert("grant_type", "password");
    // login_params.insert("username", "leroi.g@gmail.com");
    // login_params.insert("password", &password);
    // let login = c
    //     .post(login_url)
    //     .form(&login_params)
    //     .header("Content-Type", "application/x-www-form-urlencoded")
    //     .send()?;

    // print_api("login", login)?;

    // retrieve token (field access_token)

    // to search request
    let api_url = "https://api.leboncoin.fr/finder/search";
    let search_params = "{\"filters\":{\"category\":{\"id\":\"9\"},\"enums\":{\"ad_type\":[\"offer\"],\"immo_sell_type\":[\"old\",\"new\"],\"real_estate_type\":[\"1\"]},\"keywords\":{},\"location\":{\"locations\":[{\"locationType\":\"department\",\"label\":\"Isère\",\"department_id\":\"38\",\"region_id\":\"22\"},{\"locationType\":\"department\",\"label\":\"Rhône\",\"department_id\":\"69\",\"region_id\":\"22\"},{\"locationType\":\"department\",\"label\":\"Ain\",\"department_id\":\"1\",\"region_id\":\"22\"}]},\"ranges\":{\"price\":{\"min\":200000,\"max\":400000},\"rooms\":{\"min\":4},\"square\":{\"min\":80}}},\"limit\":35,\"limit_alu\":3,\"user_id\":\"d2f08b09-1a54-49bc-9d96-4ad96b227df2\",\"store_id\":\"47756443\"}";
    let r = c.post(api_url).body(search_params).send()?;
    print_api("search", r)?;
    Ok(())
}
