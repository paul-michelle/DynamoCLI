use std::collections::HashMap;

use chrono::Utc;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput, UpdateItemInput};
use uuid::Uuid;

use crate::{str2attr, Error, Item};

pub async fn upsert_item(
    client: &DynamoDbClient,
    table: String,
    first_name: String,
    last_name: String,
) -> Result<(), Error> {
    let mut pk = HashMap::new();
    pk.insert("Uid".into(), str2attr(Uuid::new_v4().to_string()));
    pk.insert("TimeStamp".into(), str2attr(Utc::now().to_string()));

    let mut values = HashMap::new();
    values.insert(":first_name".into(), str2attr(first_name));
    values.insert(":last_name".into(), str2attr(last_name));

    let upd = UpdateItemInput {
        table_name: table,
        key: pk,
        update_expression: Some("SET first_name = :first_name, last_name = :last_name".to_string()),
        expression_attribute_values: Some(values),
        ..Default::default()
    };

    client
        .update_item(upd)
        .await
        .map_err(|e| Error::DbQueryErr(e.to_string()))?;

    Ok(())
}

pub async fn scan_table(client: &DynamoDbClient, table: String) -> Result<(), Error> {
    let input = ScanInput {
        table_name: table,
        ..Default::default()
    };

    let items = client
        .scan(input)
        .await
        .map_err(|e| Error::DbQueryErr(e.to_string()))?
        .items
        .ok_or(Error::Empty("No items in the table".to_string()))?;

    for item in items {
        println!("{:?}", Item::from_map(item)?)
    }

    Ok(())
}
