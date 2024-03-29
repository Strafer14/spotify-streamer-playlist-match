#[macro_use]
extern crate diesel;
extern crate dotenv;

mod requests;
mod handledb;

#[tokio::main]
async fn main() {
    let connection = handledb::establish_connection();
    let request_result = requests::get_data_from_request().await.unwrap();
    let streamer_records_vec = request_result.data;
    for streamer_record in streamer_records_vec {
        let record = handledb::create_post(&connection, &streamer_record);
        match record {
            Ok(streamer_record) => println!("\nSaved draft with id {}", streamer_record.id),
            Err(e) => println!("An error has occurred, {}", e),
        }
    }
}