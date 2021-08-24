use super::data;
use super::model;
use super::namer;

use std::convert::Infallible;

pub async fn get_product(id: u32, repo: data::Repository, nameclient: namer::Client) -> Result<impl warp::reply::Reply, Infallible> {

    let name_result = nameclient.get_name(id.to_string()).await;

    let name = match name_result {
        Ok(n) => Some(n),
        Err(e) => {
            // add proper logging here
            println!("Error on get name {}", e);
            None
        },
    };

    if name.is_none() {
        return Ok(warp::reply::json(&format!("Unable to retrieve products")))
    }

    let result = repo.get_product(id.to_string()).await;

    match result {
        Ok(price) => {

            Ok(warp::reply::json(&model::Product::new(id.to_string(), name.expect("Unexpected error resolving name"), price)))
        }
        Err(e) => {
            // add proper logging here
            println!("Error on get price {}", e);
            //Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::BAD_REQUEST))
            Ok(warp::reply::json(&format!("Unable to retrieve products")))
        },
        
    }
}

pub async fn write_product(product: model::Amount, repo: data::Repository) -> Result<impl warp::reply::Reply, Infallible> {
    let result = repo.write_product(product).await;

    match result {
        Ok(product) => Ok(warp::reply::json(&product)),
        Err(e) => {
            // add proper logging here
            println!("Error on get {}", e);
            //Ok(warp::reply::with_status(warp::reply(), warp::http::StatusCode::BAD_REQUEST))
            Ok(warp::reply::json(&format!("Unable to write products")))
        },
        
    }
}