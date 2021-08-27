use thiserror::Error;
use serde::{Serialize, Deserialize};
use super::settings;


// **************** CLIENT DEFINITIONS *******************
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    redsky: String,
    redskyargs: String,
}

// ***************** ERROR DEFINITIONS *********************
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Unable to connect to client")]
    ConnectionFailed {#[from] source: reqwest::Error}
}

// ****************** CLIENT STRUCTURES ********************
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductResponse {
    pub product: Product,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub product_description: ProductDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDescription {
    pub title: String,
}

// ******************* CLIENT FUNCTIONALITY ******************

impl Client {

    pub fn new(cfg: settings::Nameservice) -> Result<Client, ClientError> {
        let clnt = reqwest::Client::builder().build()?;
        Ok(Client{
            client: clnt,
            redsky: cfg.getpath,
            redskyargs: cfg.getargs,
        })
    }

    pub async fn _get_name(&self, _id: String) -> Result<String, ClientError> {
        let str = self.client.get("http://random-word-api.herokuapp.com/word?number=2")
            .send()
            .await?
            .text()
            .await?
            .replace(&['[', ']', '"'][..], "")
            .replace(",", " ");

        Ok(str)
    }

    pub async fn get_name(&self, id: String) -> Result<String, ClientError> {

        println!("{}{}?{}", self.redsky, id, self.redskyargs);
        let resp = self.client.get(format!("{}{}?{}", self.redsky, id, self.redskyargs))
            .send()
            .await?
            .json::<ProductResponse>()
            .await?;

        println!("{:?}", resp);

        Ok(resp.product.item.product_description.title)
    }

}

