
use hyper::{Body, Request, Response, Method, StatusCode};

pub async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Test route can be used to confirm the server is up
        (&Method::GET, "/test") => Ok(Response::new(Body::from("Product test success\n"))),

        // Default response is a 404
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }

}
