import { Construct } from 'constructs';
import { AwsProvider } from '@cdktf/provider-aws/lib/provider';

export class LocalStackProvider extends AwsProvider {
  constructor(scope: Construct) {
    super(scope, 'AWS', {
      accessKey: 'mock_access_key',
      region: 'us-east-1',
      s3UsePathStyle: true,
      s3ForcePathStyle: true,
      secretKey: 'mock_secret_key',
      skipCredentialsValidation: true,
      skipMetadataApiCheck: true as any,
      skipRequestingAccountId: true,
      endpoints: [
        {
          apigateway: 'http://localhost:4566',
          cloudformation: 'http://localhost:4566',
          cloudwatch: 'http://localhost:4566',
          dynamodb: 'http://localhost:4566',
          es: 'http://localhost:4566',
          firehose: 'http://localhost:4566',
          iam: 'http://localhost:4566',
          kinesis: 'http://localhost:4566',
          lambda: 'http://localhost:4566',
          route53: 'http://localhost:4566',
          redshift: 'http://localhost:4566',
          s3: 'http://localhost:4566',
          secretsmanager: 'http://localhost:4566',
          ses: 'http://localhost:4566',
          sns: 'http://localhost:4566',
          sqs: 'http://localhost:4566',
          ssm: 'http://localhost:4566',
          stepfunctions: 'http://localhost:4566',
          sts: 'http://localhost:4566'
        }
      ]
    });
  }
}
