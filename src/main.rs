#[macro_use]
extern crate lazy_static;

mod router;
//mod product; 
mod settings;
mod data;
mod model;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config loading");
}


#[tokio::main]
async fn main() {

    let repo = data::Repository::NewRepository(&CONFIG.repository.db_host, &CONFIG.repository.db_user, &CONFIG.repository.db_pass);

    let api = router::router(repo);

    warp::serve(api).run(([127, 0, 0, 1], CONFIG.server.port)).await;
    
}
