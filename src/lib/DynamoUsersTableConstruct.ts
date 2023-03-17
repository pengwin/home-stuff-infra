import { Construct } from 'constructs';

import { UserCredTable, UserTable, userTable, userCredTable } from './persistence/user/tables';
import { DynamoTableConstruct } from './persistence/DynamoTableConstruct';

export class DynamoUsersTableConstruct extends Construct {
  private readonly userTable: DynamoTableConstruct<UserTable>;
  private readonly userCredTable: DynamoTableConstruct<UserCredTable>;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    this.userTable = new DynamoTableConstruct(scope, 'user-table', {
      table: userTable
    });

    this.userCredTable = new DynamoTableConstruct(scope, `${id}-user-cred-table`, {
      table: userCredTable
    });
  }

  get userTableName(): string {
    return this.userTable.tableName;
  }

  get userCredTableName(): string {
    return this.userCredTable.tableName;
  }
}
