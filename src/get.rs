use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;
    let table_name = utils::get_table_name();

    let pk = "put".to_string();
    let sk = "1".to_string();

    let mut key = HashMap::new();
    key.insert("pk".to_string(), AttributeValue::S(pk));
    key.insert("sk".to_string(), AttributeValue::S(sk));

    let resp = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .unwrap();

    if let Some(item) = resp.item {
        println!("Retrieved item:");
        for (key, value) in item {
            println!("  {}: {:?}", key, value);
        }
    } else {
        println!("No item found");
    }
}
