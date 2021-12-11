use lambda_runtime::{handler_fn, Error};
use serde_json::{Value};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: Value, _ctx: lambda_runtime::Context) -> Result<Value, Error> {
    Ok(event)
}