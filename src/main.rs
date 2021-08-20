#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};

mod product; 
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config loading");
}


#[tokio::main]
pub async fn main() {

    // Bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.server.port));

    // Create a service to handle connections
    let service = make_service_fn(|_conn| async {
        // service_fn converts our "product::handler" function into a `Service`
        Ok::<_, hyper::Error>(service_fn(product::handler))
    });

    let server = Server::bind(&addr).serve(service);

    // Run this server forever
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    
}
