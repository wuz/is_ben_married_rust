use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Response {
    person: String,
    aliases: Vec<String>,
    status: String,
    married: bool,
    legally: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://isbenmarriedyet.com/api.json";
    let response = reqwest::get(request_url).await?;

    let data: Response = response.json().await?;
    if data.married {
        println!("Yes")
    } else {
        println!("No")
    }
    Ok(())
}
