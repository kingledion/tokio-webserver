use super::data;
use super::model;
use super::namer;

use warp::http::StatusCode;
use warp::reply::Reply;
use std::convert::Infallible;

pub async fn get_product(id: u32, repo: data::Repository, nameclient: namer::Client) -> Result<warp::reply::Response, Infallible> {

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
        return Ok(warp::reply::with_status(warp::reply::json(&format!("Name service unavailable")), warp::http::StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }


    let result = repo.get_product(id.to_string()).await;

    match result {
        Ok(price) => {
            // This panic should not be possible due to error checking above
            let product = model::Product::new(id.to_string(), name.expect("Unexpected error resolving name"), price);
            Ok(warp::reply::json(&product).into_response())
        }
        Err(e) => {
            // add proper logging here
            println!("Error on get price {}", e);
            match e {
                data::RepositoryError::NotFound => Ok(warp::reply::with_status(warp::reply::json(&format!("Requested product does not exist")), StatusCode::BAD_REQUEST).into_response()),
                _ => Ok(warp::reply::with_status(warp::reply::json(&format!("Price service unavailable")), StatusCode::INTERNAL_SERVER_ERROR).into_response())
            }
        },
        
    }
}

pub async fn write_product(id: u32, price: model::Amount, repo: data::Repository) -> Result<warp::reply::Response, Infallible> {

    let mut product = price.clone();
    product._id = id.to_string();

    let result = repo.write_product(product.clone()).await;

    match result {
        Ok(_) => {
            Ok(warp::reply::json(&product).into_response())
        }
        Err(e) => {
            // add proper logging here
            println!("Error on get {}", e);
            Ok(warp::reply::with_status(warp::reply::json(&format!("Price service unavailable")), StatusCode::INTERNAL_SERVER_ERROR).into_response())
        },
        
    }
}