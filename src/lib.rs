#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use crate::routes::{health, projects};

mod db;
mod errors;
mod models;
mod routes;
mod schema;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::Db::fairing())
        .mount("/", routes![health::health])
        .mount(
            "/projects",
            routes![
                projects::get_projects,
                projects::get_project,
                projects::create_project
            ],
        )
}
