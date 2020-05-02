mod requests;

#[tokio::main]
async fn main() {
    println!("What an adventure {:?}", requests::get_data_from_request().await.unwrap());
}