use std::collections::HashMap;
use std::time::{Instant};
use lambda_runtime::{handler_fn, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use simple_logger::SimpleLogger;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DDBError, Region, PKG_VERSION};

fn author_query(client: &Client) -> aws_sdk_dynamodb::client::fluent_builders::Query {
    client
        .query()
        .table_name("Blog")
        .index_name("GSI1")
        .limit(1)
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "SK".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S("USER".to_string()))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Author {
    id: String,
    name: String,
}

async fn get_authors(client: &Client) -> HashMap::<String, Author> {
    let mut last: Option<HashMap<String, AttributeValue>> = None;
    let mut result = HashMap::<String, Author>::new();
    loop {
        match author_query(&client)
            .set_exclusive_start_key(last)
            .send()
            .await {
                Ok(resp) => {
                    match &resp.items {
                        Some(recs) => {
                            for item in recs {
                                let auth = Author {
                                    id: item["PK"].as_s().ok().unwrap().to_string(),
                                    name: item["SRT"].as_s().ok().unwrap().to_string()
                                };
                                result.insert(auth.id.to_owned(), auth);
                            }
                        }
                        None => {

                        }
                    }
                    match resp.last_evaluated_key() {
                        Some(lev) => {
                            last = Some(lev.to_owned())
                        }
                        None => {
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("error {}", e);
                    break;
                } 
            }
    }
    return result;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();
    println!("main");
    let now = Instant::now();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let client_ref = &client;
    let author_wait = get_authors(&client);
    let authors = tokio::join!(author_wait); // just as an example
    for (key, value) in &authors.0 {
        println!("{}: {:?}", key, value);
    }
    println!("{}", now.elapsed().as_millis());

    let handler_func_closure = move |event: Value, ctx: lambda_runtime::Context| async move {
        let result = my_handler(event, ctx, client_ref).await?;
        Ok::<Value, Error>(result)
    };

    let func = handler_fn(handler_func_closure);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Value, _ctx: lambda_runtime::Context, client: &Client) -> Result<Value, Error> {
    println!("main");

    let authors = get_authors(client).await;
    Ok(event)
}
