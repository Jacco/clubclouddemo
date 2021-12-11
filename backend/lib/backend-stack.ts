import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';

export class BackendStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const server = new lambda.Function(this, "GraphQL-Server", {
      functionName: 'GraphQL-Server',
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset("../binaries/graphql-server/target/aarch64-unknown-linux-gnu/release/lambda"),
      handler: 'not.required',
      architecture: lambda.Architecture.ARM_64
    });
  }
}
