import { DynamoTableAttributes, DynamoTable } from '../DynamoDbTypes';

const userCredAttributes: DynamoTableAttributes = {
  email: 'S',
  user_id: 'S',
  password_hash: 'S',
  salt: 'S',
  created_by: 'S',
  created_utc: 'S',
  updated_utc: 'S'
};

export type UserCredTable = typeof userCredAttributes;

export const userCredTable: DynamoTable<UserCredTable> = {
  tableName: 'user_cred',
  attributes: userCredAttributes,
  hashKey: 'email',
  globalSecondaryIndex: [
    {
      name: 'UserIdIndex',
      hashKey: 'user_id',
      //rangeKey: 'timestamp',
      projectionType: 'INCLUDE',
      nonKeyAttributes: ['password_hash', 'salt', 'created_by', 'created_utc', 'updated_utc']
    }
  ]
};

const userAttributes: DynamoTableAttributes = {
  user_id: 'S',
  username: 'S',
  is_admin: 'BOOL',
  created_by: 'S',
  created_utc: 'S',
  updated_utc: 'S'
};

export type UserTable = typeof userCredAttributes;

export const userTable: DynamoTable<UserTable> = {
  tableName: 'user',
  attributes: userAttributes,
  hashKey: 'user_id'
};
