import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';

export class BackendStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

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
  }
}
