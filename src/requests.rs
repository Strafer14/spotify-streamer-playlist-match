extern crate reqwest;
extern crate serde;

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StreamerRecord {
    viewers: u32,
    followersgained: u32,
    rownum: u32,
    id: u32,
    logo: String,
    twitchurl: String,
    url: String,
    displayname: String,
}
#[derive(Deserialize, Debug)]
pub struct Data {
    data: Vec<StreamerRecord>,
}

pub async fn get_data_from_request() -> Result<Data, Error> {
    let res = reqwest::get("https://sullygnome.com/api/general/directorybrowser/getgamechannels/2020-04-30T03:00:00.000Z/50/0/37")
        .await?;

    res.json::<Data>().await
}