#[macro_use]
extern crate lazy_static;

mod router;
//mod product; 
mod settings;
//mod data;
//mod model;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config loading");
}


#[tokio::main]
async fn main() {

    //let repo = data::Repository::NewRepository(&CONFIG.repo.db_host, &CONFIG.repo.db_user, &CONFIG.repo.db_pass);

    //let api = router::router(repo);
    let api = router::router("Some string".to_string());

    warp::serve(api).run(([127, 0, 0, 1], CONFIG.server.port)).await;
    
}
