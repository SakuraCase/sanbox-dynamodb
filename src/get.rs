use aws_sdk_dynamodb::types::AttributeValue;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;
    let table_name = utils::get_table_name();

    let id = "1";

    let resp = client
        .get_item()
        .table_name(table_name)
        .key("id", AttributeValue::S(id.to_string()))
        .send()
        .await
        .unwrap();

    if let Some(item) = resp.item {
        println!("Retrieved item:");
        for (key, value) in item {
            println!("  {}: {:?}", key, value);
        }
    } else {
        println!("No item found with id: {}", id);
    }
}
