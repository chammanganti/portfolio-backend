#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use routes::project_statuses;

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
                projects::get_project_by_name,
                projects::create_project,
                projects::update_project,
                projects::delete_project,
            ],
        )
        .mount(
            "/project_statuses",
            routes![
                project_statuses::get_project_statuses,
                project_statuses::get_project_status,
                project_statuses::get_project_statuses_by_project,
                project_statuses::create_project_status,
                project_statuses::update_project_status,
                project_statuses::delete_project_status,
            ],
        )
}
