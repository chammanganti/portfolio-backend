#[macro_use]
extern crate rocket;

use db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::tokio::sync::broadcast::channel;
use tonic::transport::Server;

use crate::routes::{health, project_events, project_statuses, projects};
use grpc::{
    project::{project_proto::project_server::ProjectServer, ProjectService},
    project_status::{
        project_status_proto::project_status_server::ProjectStatusServer, ProjectStatusService,
    },
};
use models::project_status::ProjectStatus;

mod db;
mod errors;
mod grpc;
mod models;
mod routes;
mod schema;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::Db::fairing())
        .attach(AdHoc::on_liftoff("gRPC", |rocket| {
            Box::pin(async move {
                let addr = "[::1]:9000".parse().unwrap();

                let project_db = Db::get_one(rocket).await.unwrap();
                let project_status_db = Db::get_one(rocket).await.unwrap();
                let project_service = ProjectService::new(project_db);
                let project_status_service = ProjectStatusService::new(project_status_db);

                let server = Server::builder()
                    .add_service(ProjectServer::new(project_service))
                    .add_service(ProjectStatusServer::new(project_status_service))
                    .serve(addr);

                tokio::spawn(server);
            })
        }))
        .manage(channel::<ProjectStatus>(1024).0)
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
        .mount(
            "/project_events",
            routes![
                project_events::project_status_events,
                project_events::publish_project_status_event
            ],
        )
}
