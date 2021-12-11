# Pre-requisites

- cdk installed (version > 2.0.0 (build 4b6ce31))
- rust / cargo installed

# Part 1: cdk + dummy lambda

1. setup cdk project

mkdir backend
cd backend
cdk init app --language typescript

2. get (temporary) AWS access keys into your terminal environment

3. bootstrap the cdk project

cdk bootstrap

This will create S3 Bucket + ECR + Roles/Policies + Version SSM Param

4. try a deploy

cdk deploy

This will create an empty CF deployment BackendStack

5. change /backend/lib/backed-stack.ts

- remove sqs comments
- add a lambda (initially a nodejs inline)

``` ts
import * as lambda from 'aws-cdk-lib/aws-lambda';
```

``` ts
    const server = new lambda.Function(this, "GraphQL-Server", {
      functionName: 'GraphQL-Server',
      runtime: lambda.Runtime.NODEJS_12_X,
      handler: "index.handler",
      code: lambda.Code.fromInline(`
        exports.handler = async function (event, context) {
          console.log(event);
        };
      `),
    });
```

- deploy and test (AWS console)

# Part 2: rust + change lambda

1. setup directory

mkdir binaries
cd binaries
mkdir graphql-server
cd graphql-server

cargo init

test if it works using `cargo run`

2. make building for Graviton possible

rustup target add aarch64-unknown-linux-gnu

create build.sh

``` bash
#!/bin/bash

LAMBDA_ARCH="linux/arm64" # set this to either linux/arm64 for ARM functions, or linux/amd64 for x86 functions.
RUST_TARGET="aarch64-unknown-linux-gnu" # corresponding with the above, set this to aarch64 or x86_64 -unknown-linux-gnu for ARM or x86 functions.
RUST_VERSION="latest" # Set this to a specific version of rust you want to compile for, or to latest if you want the latest stable version.
docker run \
  --platform ${LAMBDA_ARCH} \
  --rm --user "$(id -u)":"$(id -g)" \
  -v "${PWD}":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
  cargo build --release --target ${RUST_TARGET} # This line can be any cargo command

(cd target/aarch64-unknown-linux-gnu/release && mkdir -p lambda && cp graphql-server lambda/bootstrap)
```

``` bash
chmod +x build.sh
```

test it out by running `./build.sh`

3. make a lambda out of it

Add runtime dependencies

``` toml
[dependencies]
lambda_runtime = "0.4.1"
tokio = "1.14.0"
log = "0.4.14"
simple_logger = "1.15.0"
serde_json = "1.0.72"
```

Change main.rs

```
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
```

Run ./build

4. update cdk project to use the created binary

``` ts
    const server = new lambda.Function(this, "GraphQL-Server", {
      functionName: 'GraphQL-Server',
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset("../binaries/graphql-server/target/aarch64-unknown-linux-gnu/release/lambda"),
      handler: 'not.required',
      architecture: lambda.Architecture.ARM_64
    });
```

cdk deploy

and test if the lambda still echos (and source no longer visible in console)






