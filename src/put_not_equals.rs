use aws_sdk_dynamodb::types::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;
    let table_name = utils::get_table_name();

    let record = utils::DynamoDBRecord {
        pk: "put".to_string(),
        sk: "1".to_string(),
        val: "Hello, world".to_string(),
    };
    let item = to_item(record).unwrap();

    let request = client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .condition_expression("NOT #val = :val")
        .expression_attribute_names("#val", "val")
        .expression_attribute_values(":val", AttributeValue::S("Hello, world".to_string()));

    let res = request.send().await;
    res.unwrap();
}
