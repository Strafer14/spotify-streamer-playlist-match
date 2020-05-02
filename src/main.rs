extern crate reqwest;
extern crate serde;

use reqwest::Error;
use serde::Deserialize;


#[tokio::main]
async fn main() -> Result<(), Error> {
    #[derive(Deserialize, Debug)]
    struct Ip {
     origin: String,
    }

    let res = reqwest::get("https://sullygnome.com/api/general/directorybrowser/getgamechannels/2020-04-30T03:00:00.000Z/50/0/37")
        .await?;

    println!("Status: {}", res.status());

    let body = res.json::<Ip>().await?;

    println!("Body:\n\n{:?}", body);

    Ok(())
}