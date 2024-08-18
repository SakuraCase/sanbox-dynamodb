use std::collections::HashMap;

use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;

    let data = vec![
        {
            let mut key = HashMap::new();
            key.insert("pk".to_string(), AttributeValue::S("batch".to_string()));
            key.insert("sk".to_string(), AttributeValue::S("b1".to_string()));
            key
        },
        {
            let mut key = HashMap::new();
            key.insert("pk".to_string(), AttributeValue::S("batch".to_string()));
            key.insert("sk".to_string(), AttributeValue::S("b2".to_string()));
            key
        },
    ];

    let keys = KeysAndAttributes::builder()
        .set_keys(Some(data))
        .build()
        .unwrap();

    let resp = client
        .batch_get_item()
        .request_items(utils::get_table_name(), keys)
        .send()
        .await
        .unwrap();

    if let Some(responses) = resp.responses {
        if let Some(items) = responses.get(&utils::get_table_name()) {
            println!("items --");
            for item in items {
                for (key, value) in item {
                    println!("  {}: {:?}", key, value);
                }
            }
        } else {
            println!("No items found");
        }
    } else {
        println!("No items found");
    }
}
