import { Construct } from 'constructs';

import { UserCredTable, UserTable, userTable, userCredTable } from './persistence/user/tables';
import { DynamoTableConstruct } from './persistence/DynamoTableConstruct';

interface DynamoUsersTableConstructOptions {
  env: 'local' | 'prod-eu';
}

export class DynamoUsersTableConstruct extends Construct {
  private readonly userTable: DynamoTableConstruct<UserTable>;
  private readonly userCredTable: DynamoTableConstruct<UserCredTable>;

  constructor(scope: Construct, id: string, options: DynamoUsersTableConstructOptions) {
    super(scope, id);

    this.userTable = new DynamoTableConstruct(scope, 'user-table', {
      table: userTable,
      env: options.env
    });

    this.userCredTable = new DynamoTableConstruct(scope, `${id}-user-cred-table`, {
      table: userCredTable,
      env: options.env
    });
  }

  get userTableName(): string {
    return this.userTable.tableName;
  }

  get userTableArn(): string {
    return this.userTable.tableArn;
  }

  get userCredTableName(): string {
    return this.userCredTable.tableName;
  }

  get userCredTableArn(): string {
    return this.userCredTable.tableArn;
  }
}
