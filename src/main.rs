#[macro_use]
extern crate lazy_static;

mod router;
mod product; 
mod settings;
mod data;
mod model;
mod namer;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("config loading");
}


#[tokio::main]
async fn main() {


    let db_path = format!("http://{}:{}", CONFIG.repository.db_host, CONFIG.repository.db_port);
    let repo = data::Repository::new(&db_path, &CONFIG.repository.db_user, &CONFIG.repository.db_pass);

    let namer = match namer::Client::new(CONFIG.nameservice.clone()) {
        Ok(n) => n,
        Err(e) => panic!("Failed to start client: {}", e)
    };

    let api = router::router(repo, namer);

    warp::serve(api).run(([127, 0, 0, 1], CONFIG.server.port)).await;
    
}
