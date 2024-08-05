use aws_sdk_dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};
use aws_sdk_dynamodb::Client;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;
    let table_name = utils::get_table_name();

    delete_table_if_exists(&client, &table_name).await;

    client
        .create_table()
        .table_name(table_name)
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build()
                .expect("Failed to build partition key schema element"),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
                .build()
                .expect("Failed to build sort key schema element"),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .expect("Failed to build pk attribute definition"),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .expect("Failed to build sk attribute definition"),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .expect("Failed to build provisioned throughput"),
        )
        .send()
        .await
        .unwrap();

    println!("Successfully");
}

async fn delete_table_if_exists(client: &Client, table_name: &str) {
    match client.describe_table().table_name(table_name).send().await {
        Ok(_) => {
            println!("Table exists, deleting...");
            client
                .delete_table()
                .table_name(table_name)
                .send()
                .await
                .unwrap();
        }
        Err(_) => {
            println!("Table does not exist");
        }
    }
}
