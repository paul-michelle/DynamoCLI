use rusoto_dynamodb::AttributeValue;

use crate::Error;

pub fn str2attr(val: String) -> AttributeValue {
    AttributeValue {
        s: Some(val),
        ..Default::default()
    }
}

pub fn attr2str(attr: &mut AttributeValue) -> Result<String, Error> {
    let val = attr.s.take();
    if val.is_none() {
        return Err(Error::SerDeErr(
            "Failed to convert DynamoDB attr to string".to_string(),
        ));
    }
    Ok(val.unwrap())
}
