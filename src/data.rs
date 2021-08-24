use std::error::Error;
use serde_json::{to_value};
use super::model;


const PRODUCT_DB: &str = "product";

#[derive(Debug, Clone)]
pub struct Repository {
    client: couch_rs::Client
}


impl Repository{
    pub fn new(dbhost: &str, dbuser: &str, dbpass: &str) -> Repository {
        let c = couch_rs::Client::new(dbhost, dbuser, dbpass).unwrap(); 
        Repository{client: c}
    }

    pub async fn write_product(&self, product: model::Amount) -> Result<(), Box<dyn Error>> {
        let db = self.client.db(PRODUCT_DB).await?;
        let mut val = to_value(product)?;
        db.create(&mut val).await?;

        Ok(())
    }

    pub async fn get_product(&self, product_id: String) -> Result<model::Amount, Box<dyn Error>> {
        let db = self.client.db(PRODUCT_DB).await?;
        let val: model::Amount = db.get(&product_id).await?;

        Ok(val)

    }

}