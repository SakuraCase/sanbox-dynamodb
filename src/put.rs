use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use std::collections::HashMap;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;
    let table_name = utils::get_table_name();

    let mut item = HashMap::new();
    item.insert("pk".to_string(), AttributeValue::S("1".to_string()));

    let record = utils::DynamoDBRecord {
        pk: "put".to_string(),
        sk: "1".to_string(),
        val: "Hello, world!".to_string(),
    };
    let item = to_item(record).unwrap();

    let request = client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item));

    let res = request.send().await;
    res.unwrap();
}
