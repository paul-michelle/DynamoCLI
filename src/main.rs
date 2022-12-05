use clap::{Args, Parser, Subcommand};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

use dynamodb_cli_tool as lib;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args_ {
    /// AWS region; specify 'local' to run queries against local instance of DenamoDB
    #[arg(short, long)]
    region: String,
    /// Name of the table to query
    #[arg(short, long)]
    table: String,
    /// CRUD action to perform
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Insert item into the table (will perform upsert operation)
    Insert(InsertionData),
    /// List all items stored in the table (will perform a costly scan operation)
    List,
}

#[derive(Args, Debug)]
pub struct InsertionData {
    /// First name to store
    #[arg(short, long)]
    first_name: String,
    /// Last name to store
    #[arg(short, long)]
    last_name: String,
}

#[tokio::main]
async fn main() {
    let args = Args_::parse();

    let region = Region::Custom {
        name: "local".to_owned(),
        endpoint: "http://127.0.0.1:8000".to_owned(),
    };
    let client = DynamoDbClient::new(region);

    match args.action {
        Action::Insert(data) => {
            if let Err(e) =
                lib::upsert_item(&client, args.table, data.first_name, data.last_name).await
            {
                println!("{e}")
            }
        }
        Action::List => {
            if let Err(e) = lib::scan_table(&client, args.table).await {
                println!("{e}")
            }
        }
    }
}
