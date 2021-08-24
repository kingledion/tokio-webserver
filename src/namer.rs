use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Unable to connect to client")]
    ConnectionFailed {
        #[from]
        source: reqwest::Error
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client
}

impl Client {

    pub fn new() -> Result<Client, ClientError> {
        let clnt = reqwest::Client::builder().build()?;
        Ok(Client{client: clnt})
    }

    pub async fn get_name(&self, _id: String) -> Result<String, ClientError> {
        let str = self.client.get("http://random-word-api.herokuapp.com/word?number=2")
            .send()
            .await?
            .text()
            .await?
            .replace(&['[', ']', '"'][..], "")
            .replace(",", " ");

        Ok(str)
    }
}
