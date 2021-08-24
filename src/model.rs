use serde::{Serialize, Deserialize};
use couch_rs::types::document::DocumentId;
use couch_rs::document::TypedCouchDocument;
use couch_rs::CouchDocument;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    id: String,
    name: String,
    value: u32, // value is reduced to smallest unit of currency so no fractions (i.e. US cents)
    currency_code: String,
}

impl Product {
    pub fn new(id: String, name: String, price: Amount) -> Product {
        Product{
            id: id,
            name: name,
            value: price.value,
            currency_code: price.currency_code,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone,  CouchDocument)]
pub struct Amount {
    #[serde(skip_serializing_if = "String::is_empty", skip_deserializing)]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty", skip_deserializing)]
    pub _rev: String,
    value: u32, // value is reduced to smallest unit of currency so no fractions (i.e. US cents)
    currency_code: String,
}
