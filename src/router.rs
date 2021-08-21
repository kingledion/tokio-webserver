//use super::product;
use warp::Filter;

pub fn router(repo: String) ->
    impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        get_test()
    }


pub fn get_test() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("test").map(|| "Router test success\n")
}


