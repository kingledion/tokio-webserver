use std::error::Error;
use serde_json::{to_value};
use crate::model::Product;


const PRODUCT_DB: &str = "product";

#[derive(Debug)]
pub struct Repository {
    client: couch_rs::Client
}


impl Repository{
    pub fn NewRepository(dbhost: &str, dbuser: &str, dbpass: &str) -> Repository {
        let c = couch_rs::Client::new(dbhost, dbuser, dbpass).unwrap(); 
        Repository{client: c}
    }

    async fn write_product(&self, product: Product) -> Result<(), Box<dyn Error>> {
        let db = self.client.db(PRODUCT_DB).await?;
        let mut val = to_value(product)?;
        db.create(&mut val).await?;

        Ok(())
    }

    async fn get_product(&self, product_id: String) -> Result<Product, Box<dyn Error>> {
        let db = self.client.db(PRODUCT_DB).await?;
        let val: Product = db.get(&product_id).await?;

        Ok(val)

    }

}