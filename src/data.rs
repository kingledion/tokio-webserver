use thiserror::Error;
use serde_json::{to_value};
use warp::http::StatusCode;
use super::model;
use async_trait::async_trait;

const PRODUCT_DB: &str = "product";

#[async_trait]
pub trait Repo : Send + Sync + Clone + 'static {
    async fn write_product(&self, product: model::Amount) -> Result<(), RepositoryError>;
    async fn get_product(&self, product_id: String) -> Result<model::Amount, RepositoryError>;
}

#[derive(Debug, Clone)]
pub struct Repository {
    client: couch_rs::Client
}


impl Repository{
    pub fn new(dbhost: &str, dbuser: &str, dbpass: &str) -> Repository {
        let c = couch_rs::Client::new(dbhost, dbuser, dbpass).unwrap(); 
        Repository{client: c}
    }
}

#[async_trait]
impl Repo for Repository {
    async fn write_product(&self, product: model::Amount) -> Result<(), RepositoryError> {
        let db = self.client.db(PRODUCT_DB).await?;
        let mut val = to_value(product)?;
        db.upsert(&mut val).await?;

        Ok(())
    }

    async fn get_product(&self, product_id: String) -> Result<model::Amount, RepositoryError> {
        let db = self.client.db(PRODUCT_DB).await?;
        let res = db.get(&product_id).await;

        match res {
            Ok(val) => Ok(val),
            Err(source) => {
                match source.status {
                    StatusCode::NOT_FOUND => Err(RepositoryError::NotFound),
                    _ => Err(RepositoryError::ConnectionFailed(source))
                }
            }
        }

        

    }

}

// RepositoryError enumerates all possible errors returned from intereactions with CouchDB repository
#[derive(Error, Debug)]
pub enum RepositoryError {
    /// Represents a generic connection error
    #[error("Unable to connect to database")]
    ConnectionFailed(#[from] couch_rs::error::CouchError),

    /// Represents that the specified item is not found
    #[error("Item not found")]
    NotFound,

    /// Represents failure to parse a json response from the database
    #[error("Unable to parse database response")]
    ParseFailure(#[from] serde_json::Error)
}
