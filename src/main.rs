#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod models;
mod schema;
mod controller;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        controller::index,
        controller::new_sighting,
        controller::all_sightings,
        controller::delete_sighting
    ])
}