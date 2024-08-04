import * as cdk from 'aws-cdk-lib';
import { AttributeType, BillingMode, Table } from 'aws-cdk-lib/aws-dynamodb';
import { Construct } from 'constructs';
import * as iam from 'aws-cdk-lib/aws-iam';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const table = new Table(this, 'DynamoDB', {
      partitionKey: { name: 'pk', type: AttributeType.STRING },
      sortKey: { name: 'sk', type: AttributeType.STRING },
      tableName: 'test',
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      billingMode: BillingMode.PAY_PER_REQUEST,
    });

    const publicPolicy = new iam.PolicyStatement({
      effect: iam.Effect.ALLOW,
      actions: ['dynamodb:PutItem', 'dynamodb:GetItem', 'dynamodb:Scan', 'dynamodb:Query'],
      resources: [table.tableArn],
      principals: [new iam.AnyPrincipal()],
    });
    table.addToResourcePolicy(publicPolicy);
  }
}
