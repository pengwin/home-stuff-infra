import { Construct } from 'constructs';
import * as random from '@cdktf/provider-random';
import * as dayjs from 'dayjs';
import * as utc from 'dayjs/plugin/utc';
import { TerraformLocal } from 'cdktf/lib/terraform-local';
import { Fn } from 'cdktf';
import { PopulateDynamoTableConstruct } from './persistence/PopulateDynamoTableConstruct';
import { userCredTable, userTable } from './persistence/user/tables';

dayjs.extend(utc);

export interface AdminUserDeploymentOptions {
  readonly userTableName: string;
  readonly credTableName: string;
  readonly adminEmail: string;
  readonly pepper: string;
}

export class AdminUserDeploymentConstruct extends Construct {
  private readonly pass: TerraformLocal;

  constructor(scope: Construct, id: string, options: AdminUserDeploymentOptions) {
    super(scope, id);

    const password = new random.password.Password(scope, `${id}-admin-password`, {
      length: 10
    });
    const userId = new random.uuid.Uuid(scope, `${id}-admin-user-id`);
    const userNameGuid = new random.uuid.Uuid(scope, `${id}-admin-username-guid`);
    const username = new TerraformLocal(scope, `${id}-admin-username-local`, `admin-${userNameGuid.result}`);
    const salt = new random.uuid.Uuid(scope, `${id}-admin-salt`);
    this.pass = new TerraformLocal(scope, `${id}-admin-pass`, Fn.nonsensitive(`${password.result}`));

    const passwordHashValue = new TerraformLocal(
      scope,
      `${id}-admin-password-value`,
      Fn.sha512(salt.result + password.result + options.pepper)
    );

    new PopulateDynamoTableConstruct(scope, `${id}-admin-user`, {
      table: userTable,
      content: {
        user_id: userId.result,
        username: username.asString,
        is_admin: true,
        created_utc: dayjs.utc().format(),
        updated_utc: dayjs.utc().format()
      }
    });

    new PopulateDynamoTableConstruct(scope, `${id}-admin-user-cred`, {
      table: userCredTable,
      content: {
        email: options.adminEmail,
        user_id: userId.result,
        salt: salt.result,
        password_hash: passwordHashValue.asString,
        created_utc: dayjs.utc().format(),
        updated_utc: dayjs.utc().format()
      }
    });
  }

  get pwd(): string {
    return this.pass.asString;
  }
}
