use aws_sdk_dynamodb::types::AttributeValue;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;

    let response = client
        .query()
        .table_name(utils::get_table_name())
        .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk_prefix)")
        .expression_attribute_names("#pk", "pk")
        .expression_attribute_names("#sk", "sk")
        .expression_attribute_values(":pk", AttributeValue::S("begins".to_string()))
        .expression_attribute_values(":sk_prefix", AttributeValue::S("hoge:fuga".to_string()))
        .send()
        .await
        .unwrap();

    if let Some(items) = response.items {
        for item in items {
            println!("{:?}", item);
        }
    } else {
        println!("No items found.");
    }
}
