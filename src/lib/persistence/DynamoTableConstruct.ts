import { Construct } from 'constructs';
import * as aws from '@cdktf/provider-aws';
import { DynamoTableAttributes, DynamoTable } from './DynamoDbTypes';

interface DynamoTableConstructOptions<T extends DynamoTableAttributes> {
  table: DynamoTable<T>;
}

export class DynamoTableConstruct<T extends DynamoTableAttributes> extends Construct {
  private readonly table: aws.dynamodbTable.DynamodbTable;

  constructor(scope: Construct, id: string, options: DynamoTableConstructOptions<T>) {
    super(scope, id);

    //Object.keys(options.table).map((k) => ({ name: k, type: options.table[k] }));

    const hashKey = options.table.hashKey as unknown as string;
    const attributes = [{ name: hashKey, type: options.table.attributes[hashKey] }];

    for (const index of options.table.globalSecondaryIndex ?? []) {
      attributes.push({
        name: index.hashKey,
        type: options.table.attributes[index.hashKey]
      });
    }

    this.table = new aws.dynamodbTable.DynamodbTable(scope, `${id}-dynamodb-table`, {
      name: options.table.tableName,
      billingMode: 'PAY_PER_REQUEST', // 'PROVISIONED'

      //writeCapacity: 5,
      //readCapacity: 5,
      attribute: attributes,
      hashKey,
      globalSecondaryIndex: options.table.globalSecondaryIndex,
      /*globalSecondaryIndex: [
        {
          name: 'UserIdAndTimestampSortIndex',
          hashKey: 'user_id',
          rangeKey: 'timestamp',
          projectionType: 'ALL'
        }
      ],*/
      lifecycle: {
        //ignoreChanges: ['write_capacity', 'read_capacity']
      }
    });
  }

  get tableName() {
    return this.table.name;
  }
}
