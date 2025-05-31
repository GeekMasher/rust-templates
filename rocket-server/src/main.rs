#[macro_use]
extern crate rocket;

use log::{error, info};

mod api;
mod error;
mod guards;
mod models;
mod routes;

use routes::index;

/// Application State
pub struct AppState {}

#[rocket::main]
async fn main() {
    env_logger::init();

    info!("Building Rocket");
    let rocket = rocket::build().mount("/", routes![index]);

    // TODO: Add routes

    if let Err(e) = rocket.launch().await {
        error!("Error launching Rocket: {}", e);
        drop(e);
    }

    info!("Stopping Rocket")
}
