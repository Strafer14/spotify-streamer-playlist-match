extern crate reqwest;
extern crate serde;

use reqwest::Error;
use serde::Deserialize;
use crate::handledb::models::StreamerRecord;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub data: Vec<StreamerRecord>,
}

pub async fn get_data_from_request() -> Result<Data, Error> {
    let res = reqwest::get("https://sullygnome.com/api/general/directorybrowser/getgamechannels/2020-04-30T03:00:00.000Z/50/0/37")
        .await?;

    res.json::<Data>().await
}