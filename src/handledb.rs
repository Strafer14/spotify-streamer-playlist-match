mod schema;
pub(crate) mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::StreamerRecord;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &PgConnection, new_streamer_record: &StreamerRecord) -> StreamerRecord {
    use schema::streamer_records;

    diesel::insert_into(streamer_records::table)
        .values(new_streamer_record)
        .get_result(conn)
        .expect("Error saving new post")
}