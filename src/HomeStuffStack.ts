import * as path from 'path';

import { Construct } from 'constructs';
import { App, TerraformStack, TerraformOutput } from 'cdktf';

import { LocalStackProvider } from './lib/LocalStackProvider';
import * as random from '@cdktf/provider-random';
import { LambdaDeploymentConstruct, LambdaDeploymentOptions } from './lib/LambdaDeploymentConstruct';
import { DynamoUsersTableConstruct } from './lib/DynamoUsersTableConstruct';
import { AdminUserDeploymentConstruct } from './lib/AdminUserDeploymentConstruct';
import { AwsProvider } from '@cdktf/provider-aws/lib/provider';

type LambdaIdConstructor = (id: string) => string;

const defaultPepper = 'DoctorPepper';

interface HomeStuffStackOptions {
  env: 'local' | 'prod-eu';
  pepper?: string;
}

export class HomeStuffStack extends TerraformStack {
  constructor(scope: Construct, id: string, options: HomeStuffStackOptions) {
    super(scope, id);

    if (options.env === 'local') {
      new LocalStackProvider(this);
    } else if (options.env === 'prod-eu') {
      new AwsProvider(this, 'AWS', {
        region: 'eu-central-1'
      });
    }

    new random.provider.RandomProvider(this, 'random');

    const randomPepper = new random.password.Password(this, `${id}-pepper`, {
      length: 10
    });

    const pepper = options.env === 'local' ? defaultPepper : randomPepper.result;

    const adminEmail = 'test@test.com';

    const userTable = new DynamoUsersTableConstruct(this, `${id}-user-table`);

    const adminItem = new AdminUserDeploymentConstruct(this, `${id}-admin-item`, {
      credTableName: userTable.userCredTableName,
      userTableName: userTable.userTableName,
      adminEmail: 'test@test.com',
      pepper
    });

    new TerraformOutput(this, 'admin_email', {
      value: adminEmail
    });

    new TerraformOutput(this, 'admin_pwd', {
      value: adminItem.pwd
    });

    /*new TerraformOutput(this, 'user_table', {
      value: userTable.userTableName
    });

    new TerraformOutput(this, 'user_cred_table', {
      value: userTable.userCredTableName
    });

    new TerraformOutput(this, 'salt_table', {
      value: userTable.saltTableName
    });*/

    const lambdaMap: { [key: string]: [LambdaIdConstructor, LambdaDeploymentOptions] } = {
      auth_lambda_url: [
        (id) => `${id}-auth-lambda`,
        {
          path: path.resolve(__dirname, '../lambdas/rust-lambda/target/lambda/auth-lambda/bootstrap.zip'),
          handler: 'bootstrap',
          runtime: 'provided.al2',
          //stageName: "rust-hello-world",
          version: 'v0.0.2',
          useS3: false,
          zip: false,
          variables: {
            APP_PEPPER: pepper,
            APP_SECRET: 'secret',
            APP_ENV: 'LocalStack'
          }
        }
      ]
    };

    for (const key in lambdaMap) {
      const [idFunc, config] = lambdaMap[key];
      const lambdaFuncUrl = new LambdaDeploymentConstruct(this, idFunc(id), config);
      new TerraformOutput(this, key, {
        value: lambdaFuncUrl.functionUrl
      });
    }
  }
}

const app = new App();

new HomeStuffStack(app, 'home-stuff-local', {
  env: 'local'
});

app.synth();
