use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;

use crate::{attr2str, Error};

#[derive(Debug)]
pub struct Item {
    pub uuid: String,
    pub timestamp: String,
    pub first_name: String,
    pub last_name: String,
}

impl Item {
    pub fn from_map(mut map: HashMap<String, AttributeValue>) -> Result<Self, Error> {
        Ok(Self {
            uuid: map
                .get_mut("Uid")
                .ok_or_else(|| Error::SerDeErr("No Uid".to_string()))
                .and_then(attr2str)?,
            timestamp: map
                .get_mut("TimeStamp")
                .ok_or_else(|| Error::SerDeErr("No Uid".to_string()))
                .and_then(attr2str)?,
            first_name: map
                .get_mut("first_name")
                .ok_or_else(|| Error::SerDeErr("No Uid".to_string()))
                .and_then(attr2str)?,
            last_name: map
                .get_mut("last_name")
                .ok_or_else(|| Error::SerDeErr("No Uid".to_string()))
                .and_then(attr2str)?,
        })
    }
}
