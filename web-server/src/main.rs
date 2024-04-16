#[macro_use] extern crate rocket;

use log::{info, error};

mod routes;
mod models;

use routes::index;

#[rocket::main]
async fn main() {
    env_logger::init();

    info!("Building Rocket");
    let rocket = rocket::build()
        .mount("/", routes![index]);
    
    // TODO: Add routes


    if let Err(e) = rocket.launch().await {
        error!("Error launching Rocket: {}", e);
        drop(e);
    }

    info!("Stopping Rocket")
}
