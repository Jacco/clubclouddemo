# Pre-requisites

- cdk installed

# Part 1: cdk

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

# Part 2: rust