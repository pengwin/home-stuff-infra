use aws_sdk_dynamodb::model::AttributeValue;

pub trait ToAttribute {
    fn to_attribute(&self) -> AttributeValue;
}

impl ToAttribute for String {
    fn to_attribute(&self) -> AttributeValue {
        AttributeValue::S(self.clone())
    }
}
