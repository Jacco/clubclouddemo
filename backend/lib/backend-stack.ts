import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as iam from 'aws-cdk-lib/aws-iam';

export class BackendStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const server = new lambda.Function(this, "GraphQL-Server", {
      functionName: 'GraphQL-Server',
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset("../binaries/graphql-server/lambda"),
      handler: 'not.required',
      architecture: lambda.Architecture.ARM_64
    });

    const dynamodbAll = new iam.PolicyStatement({
      actions: ['dynamodb:*'],
      resources: ["*"],
    });

    server.role?.attachInlinePolicy(
      new iam.Policy(this, 'dynamodb-all', {
        statements: [dynamodbAll],
      }),
    );
  }
}
