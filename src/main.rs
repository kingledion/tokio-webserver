#[macro_use]
extern crate lazy_static;

mod router;
//mod product; 
mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config loading");
}


#[tokio::main]
async fn main() {


    let api = router::router("Path".to_string());

    warp::serve(api).run(([127, 0, 0, 1], CONFIG.server.port)).await;
    
}
