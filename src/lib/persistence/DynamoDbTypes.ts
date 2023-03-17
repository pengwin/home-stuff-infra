import { DynamodbTableGlobalSecondaryIndex } from '@cdktf/provider-aws/lib/dynamodb-table';

export type AttributeType = 'S' | 'N' | 'B' | 'BOOL';

export interface DynamoTableAttributes {
  [key: string]: AttributeType;
}

export type DynamoTableParam<T extends DynamoTableAttributes> = keyof T;

export interface DynamoTable<T extends DynamoTableAttributes> {
  tableName: string;
  attributes: T;
  hashKey: DynamoTableParam<T>;
  globalSecondaryIndex?: DynamodbTableGlobalSecondaryIndex[];
}

export type DynamoTableContent<T extends DynamoTableAttributes> = {
  [field in DynamoTableParam<T>]: unknown;
};
