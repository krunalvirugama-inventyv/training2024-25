use tikv_client::{RawClient};
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Ensure this matches your running PD server
    let pd_endpoints = vec!["172.24.131.151:2379"];

    let client = RawClient::new(pd_endpoints).await?;

    client.put("hello".to_owned(), "Hello TiKV!".to_owned()).await?;
    println!("Data written!");

    let value = client.get("hello".to_owned()).await?;
    println!("Data read: {:?}", value.map(|v| String::from_utf8(v).unwrap()));

    Ok(())
}
