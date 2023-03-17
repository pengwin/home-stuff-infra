import { Construct } from 'constructs';
import { TerraformAsset, AssetType } from 'cdktf';
import * as aws from '@cdktf/provider-aws';

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
}

const lambdaRolePolicy = {
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

    // Create Lambda role
    const role = new aws.iamRole.IamRole(scope, `${id}-lambda-exec`, {
      name: `${id}-role`,
      assumeRolePolicy: JSON.stringify(lambdaRolePolicy)
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
  }

  get functionUrl(): string {
    return this.lambdaFunctionUrl.functionUrl;
  }
}
