import { Construct } from 'constructs';
import { TerraformAsset, AssetType } from 'cdktf';
import * as aws from '@cdktf/provider-aws';

interface DynamoTableInfo {
  tableName: string;
  arn: string;
}

export interface LambdaDeploymentOptions {
  readonly path: string;
  readonly handler: 'bootstrap' | 'index.handler';
  readonly runtime: 'provided.al2' | 'nodejs14.x';
  //readonly stageName: string,
  readonly version: string;
  readonly useS3: boolean;
  readonly zip: boolean;
  readonly variables?: {
    [key: string]: string;
  };
  readonly dynamoTablesToAccess?: DynamoTableInfo[];
}

const DefaultLambdaRolePolicy = {
  Version: '2012-10-17',
  Statement: [
    {
      Action: 'sts:AssumeRole',
      Principal: {
        Service: 'lambda.amazonaws.com'
      },
      Effect: 'Allow',
      Sid: ''
    }
  ]
};

export class LambdaDeploymentConstruct extends Construct {
  private readonly lambdaFunctionUrl: aws.lambdaFunctionUrl.LambdaFunctionUrl;

  constructor(scope: Construct, id: string, options: LambdaDeploymentOptions) {
    super(scope, id);

    const asset = options.zip
      ? new TerraformAsset(scope, `${id}-lambda-asset`, {
          path: options.path,
          type: AssetType.ARCHIVE // if left empty it infers directory and file
        })
      : new TerraformAsset(scope, `${id}-lambda-asset`, {
          path: options.path,
          type: AssetType.FILE // if left empty it infers directory and file
        });

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const tables: any =
      options.dynamoTablesToAccess?.map((t) => ({
        Effect: 'Allow',
        Action: [
          'dynamodb:BatchGetItem',
          'dynamodb:GetItem',
          'dynamodb:Query',
          'dynamodb:Scan',
          'dynamodb:BatchWriteItem',
          'dynamodb:PutItem',
          'dynamodb:DeleteItem',
          'dynamodb:UpdateItem'
        ],
        Resource: t.arn
      })) || [];

    const lambdaRolePolicy = { ...DefaultLambdaRolePolicy };
    const lambdaInlinePolicy =
      tables.length > 0
        ? [
            {
              name: `${id}-dynamo-policy`,
              policy: JSON.stringify({
                Version: '2012-10-17',
                Statement: tables
              })
            }
          ]
        : undefined;

    // Create Lambda role
    const role = new aws.iamRole.IamRole(scope, `${id}-lambda-exec`, {
      name: `${id}-role`,
      assumeRolePolicy: JSON.stringify(lambdaRolePolicy),
      inlinePolicy: lambdaInlinePolicy
    });

    // Add execution role for lambda to write to CloudWatch logs
    new aws.iamRolePolicyAttachment.IamRolePolicyAttachment(scope, `${id}-lambda-managed-policy`, {
      policyArn: 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole',
      role: role.name
    });

    let lambdaConfig: aws.lambdaFunction.LambdaFunctionConfig = {
      functionName: `${id}-lambda`,
      handler: options.handler,
      runtime: options.runtime,
      role: role.arn,
      timeout: 1,
      environment: {
        variables: options.variables
      }
    };

    if (options.useS3) {
      // Create unique S3 bucket that hosts Lambda executable
      const bucket = new aws.s3Bucket.S3Bucket(scope, `${id}-lambda-bucket`, {
        //bucketPrefix: `${id}-lambda-bucket`,
        acl: 'private',
        bucket: `${id}-lambda-bucket`
      });

      // Upload Lambda zip file to newly created S3 bucket
      const lambdaArchive = new aws.s3Object.S3Object(scope, `${id}-lambda-archive`, {
        bucket: bucket.bucket,
        key: `${options.version}/${asset.fileName}`,
        source: asset.path // returns a posix path
      });

      lambdaConfig = { ...lambdaConfig, s3Bucket: bucket.bucket, s3Key: lambdaArchive.key };
    } else {
      lambdaConfig = { ...lambdaConfig, filename: asset.path };
    }

    // Create Lambda function
    const lambdaFunc = new aws.lambdaFunction.LambdaFunction(scope, `${id}-lambda`, lambdaConfig);

    this.lambdaFunctionUrl = new aws.lambdaFunctionUrl.LambdaFunctionUrl(scope, `${id}-lambda-url`, {
      functionName: lambdaFunc.functionName,
      authorizationType: 'NONE'
    });

    new aws.cloudwatchLogGroup.CloudwatchLogGroup(scope, `${id}-lambda-cloudwatch-group`, {
      name: `/aws/lambda/${lambdaFunc.functionName}`,
      retentionInDays: 30
    });
  }

  get functionUrl(): string {
    return this.lambdaFunctionUrl.functionUrl;
  }
}
