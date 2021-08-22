use serde::{Serialize, Deserialize};
use couch_rs::types::document::DocumentId;
use couch_rs::document::TypedCouchDocument;
use couch_rs::CouchDocument;

#[derive(Debug, Serialize, Deserialize, Clone, CouchDocument)]
pub struct Product {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
    name: String,
    #[serde(rename = "currentPrice")]
    current_price: Amount,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Amount {
    value: u32, // value is reduced to smallest unit of currency so no fractions (i.e. US cents)
    currency_code: String,
}