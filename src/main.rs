use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[tokio::main]
async fn main() {
    let region = Region::Custom {
        name: "local".to_owned(),
        endpoint: "http://127.0.0.1:8000".to_owned(),
    };

    let client = DynamoDbClient::new(region);
    let list_tables_input: ListTablesInput = Default::default();
    let list_tables_res = client.list_tables(list_tables_input).await;

    if let Err(e) = list_tables_res {
        println!("Error: {:?}", e);
        return;
    }

    let tables_names_list = list_tables_res.unwrap().table_names;
    if tables_names_list.is_none() {
        println!("No tables in database!");
        return;
    }

    let tables_names_list = tables_names_list.unwrap();
    if tables_names_list.is_empty() {
        println!("No tables in database!");
        return;
    }

    println!("Tables in database:");
    for name in tables_names_list {
        println!("{}", name)
    }
}
