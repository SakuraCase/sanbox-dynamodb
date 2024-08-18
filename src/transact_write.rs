use aws_sdk_dynamodb::types::{Put, TransactWriteItem};
use serde_dynamo::aws_sdk_dynamodb_1::to_item;
use utils::DynamoDBRecord;

mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client().await;

    let transact_items = vec![
        {
            let record = utils::DynamoDBRecord {
                pk: "batch".to_string(),
                sk: "b1".to_string(),
                val: "b1!".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "batch".to_string(),
                sk: "b2".to_string(),
                val: "b2!".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "begins".to_string(),
                sk: "hoge".to_string(),
                val: "hoge".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "begins".to_string(),
                sk: "hoge:fuga".to_string(),
                val: "hoge:fuga".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "begins".to_string(),
                sk: "hoge:piyo".to_string(),
                val: "hoge:piyo".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "begins".to_string(),
                sk: "hoge:fuga:bar".to_string(),
                val: "hoge:fuga:bar".to_string(),
            };
            build_item(record)
        },
        {
            let record = utils::DynamoDBRecord {
                pk: "begins".to_string(),
                sk: "hoge:fuga:foo".to_string(),
                val: "hoge:fuga:foo".to_string(),
            };
            build_item(record)
        },
    ];

    let response = client
        .transact_write_items()
        .set_transact_items(Some(transact_items))
        .send()
        .await
        .unwrap();

    println!("Transaction succeeded: {:?}", response);
}

fn build_item(record: DynamoDBRecord) -> TransactWriteItem {
    let item = to_item(record).unwrap();

    let put_request = Put::builder()
        .table_name(utils::get_table_name())
        .set_item(Some(item))
        .condition_expression("attribute_not_exists(pk) AND attribute_not_exists(sk)")
        .build()
        .unwrap();

    TransactWriteItem::builder().put(put_request).build()
}
