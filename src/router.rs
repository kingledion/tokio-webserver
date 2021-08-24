use warp::{Reply, Rejection, Filter};
use std::convert::Infallible;
use super::data;
use super::product;
use super::model;
use super::namer;

pub fn router(repo: data::Repository, nameclient: namer::Client) ->
    impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        get_test()
            .or(get_product(repo.clone(), nameclient))
            .or(write_product(repo))
    }


pub fn get_test() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("test").map(|| "Router test success\n")
}

pub fn get_product(repo: data::Repository, nameclient: namer::Client) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("product" / u32)
        .and(warp::get())
        .and(with_repo(repo))
        .and(with_namer(nameclient))
        .and_then(product::get_product)

}

pub fn write_product(repo: data::Repository) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("product" / u32)
        .and(warp::put())
        .and(amount_body())
        .and(with_repo(repo))
        .and_then(product::write_product)

}

fn with_repo(repo: data::Repository) -> impl Filter<Extract = (data::Repository,), Error = Infallible> + Clone {
    warp::any().map(move || repo.clone())
}

fn with_namer(nameclient: namer::Client) -> impl Filter<Extract = (namer::Client,), Error = Infallible> + Clone {
    warp::any().map(move || nameclient.clone())
}

fn amount_body() -> impl Filter<Extract = (model::Amount,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


