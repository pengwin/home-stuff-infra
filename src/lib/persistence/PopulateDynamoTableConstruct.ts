import { Construct } from 'constructs';
import * as aws from '@cdktf/provider-aws';
import { DynamoTableAttributes, DynamoTable, DynamoTableContent, AttributeType } from './DynamoDbTypes';
import { TerraformLocal } from 'cdktf/lib/terraform-local';

interface PopulateDynamoTableConstructOptions<T extends DynamoTableAttributes> {
  table: DynamoTable<T>;
  content: DynamoTableContent<T>;
}

export class PopulateDynamoTableConstruct<T extends DynamoTableAttributes> extends Construct {
  constructor(scope: Construct, id: string, options: PopulateDynamoTableConstructOptions<T>) {
    super(scope, id);

    const hashKey = options.table.hashKey as unknown as string;

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const content: any = {};
    const contentKeys = Object.keys(options.content);
    for (const k of contentKeys) {
      const attribute = options.table.attributes[k] as AttributeType;
      content[k] = { [attribute]: options.content[k] };
    }

    const tableItemContent = new TerraformLocal(scope, `${id}-dynamo-table-item-content`, JSON.stringify(content));

    new aws.dynamodbTableItem.DynamodbTableItem(scope, `${id}-dynamo-table-item`, {
      tableName: options.table.tableName,
      hashKey,
      item: tableItemContent.asString
    });
  }
}
