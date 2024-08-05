use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use serde_derive::{Deserialize, Serialize};

pub fn get_table_name() -> String {
    String::from("test")
}

pub async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .endpoint_url("http://localhost:8000")
        .region(region_provider)
        .load()
        .await;
    Client::new(&config)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DynamoDBRecord {
    pub pk: String,
    pub sk: String,
    pub val: String,
}
