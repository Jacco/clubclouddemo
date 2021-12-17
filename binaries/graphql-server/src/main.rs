use std::collections::HashMap;
use std::time::{Instant};
use lambda_runtime::{handler_fn, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use simple_logger::SimpleLogger;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DDBError, Region, PKG_VERSION};

fn type_query(client: &Client, key: &String) -> aws_sdk_dynamodb::client::fluent_builders::Query {
    client
        .query()
        .table_name("Blog")
        .index_name("GSI1")
        .limit(20)
        .key_condition_expression("#key = :value".to_string())
        .expression_attribute_names("#key".to_string(), "SK".to_string())
        .expression_attribute_values(":value".to_string(), AttributeValue::S(key.to_string()))
}

fn author_query(client: &Client) -> aws_sdk_dynamodb::client::fluent_builders::Query {
    type_query(client, &"USER".to_string())
}

fn blog_query(client: &Client) -> aws_sdk_dynamodb::client::fluent_builders::Query {
    type_query(client, &"POST".to_string())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Author {
    id: String,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Blog {
    id: String,
    created: String,
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
                    if let Some(recs) = &resp.items {
                        for item in recs {
                            let auth = Author {
                                id: item["PK"].as_s().ok().unwrap().to_string(),
                                name: item["SRT"].as_s().ok().unwrap().to_string()
                            };
                            result.insert(auth.id.to_owned(), auth);
                        }
                    }
                    if let Some(lev) = resp.last_evaluated_key() {
                        last = Some(lev.to_owned())
                    } else {
                        break;
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

async fn get_blogs(client: &Client) -> HashMap::<String, Blog> {
    let mut last: Option<HashMap<String, AttributeValue>> = None;
    let mut result = HashMap::<String, Blog>::new();
    loop {
        match blog_query(&client)
            .set_exclusive_start_key(last)
            .send()
            .await {
                Ok(resp) => {
                    if let Some(recs) = &resp.items {
                        for item in recs {
                            let auth = Blog {
                                id: item["PK"].as_s().ok().unwrap().to_string(),
                                created: item["SRT"].as_s().ok().unwrap().to_string()
                            };
                            result.insert(auth.id.to_owned(), auth);
                        }
                    }
                    if let Some(lev) = resp.last_evaluated_key() {
                        last = Some(lev.to_owned())
                    } else {
                        break;
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
    println!("init");
    
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let client_ref = &client;

    let handler_func_closure = move |event: Value, ctx: lambda_runtime::Context| async move {
        let result = my_handler(event, ctx, client_ref).await?;
        Ok::<Value, Error>(result)
    };

    let func = handler_fn(handler_func_closure);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn query_parallel(client: &Client) {
    let now = Instant::now();

    let authors_w = get_authors(client);
    let blogs_w = get_blogs(client);

    let (authors, blogs) = tokio::join!(authors_w, blogs_w);

    println!("parallel {}", now.elapsed().as_millis());
    
    println!("Authors");
    for (key, value) in &authors {
        println!("{}: {:?}", key, value);
    }
    println!("Blogs");
    for (key, value) in &blogs {
        println!("{}: {:?}", key, value);
    }
}

async fn query_serial(client: &Client) {
    let now = Instant::now();

    let authors= get_authors(client).await;
    let blogs = get_blogs(client).await;

    println!("serial {}", now.elapsed().as_millis());
    
    println!("Authors");
    for (key, value) in &authors {
        println!("{}: {:?}", key, value);
    }
    println!("Blogs");
    for (key, value) in &blogs {
        println!("{}: {:?}", key, value);
    }
}

pub(crate) async fn my_handler(event: Value, _ctx: lambda_runtime::Context, client: &Client) -> Result<Value, Error> {
    println!("main");

    query_parallel(client).await;

    query_serial(client).await;

    Ok(event)
}
