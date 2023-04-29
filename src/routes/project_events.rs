use rocket::response::stream::{Event, EventStream};
use rocket::serde::json::Json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{error::RecvError, Sender};
use rocket::{Shutdown, State};

use crate::models::project_status::ProjectStatus;

#[get("/")]
pub async fn project_status_events(
    queue: &State<Sender<ProjectStatus>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/", data = "<project_status>")]
pub fn publish_project_status_event(
    queue: &State<Sender<ProjectStatus>>,
    project_status: Json<ProjectStatus>,
) -> Result<(), ()> {
    queue.send(project_status.into_inner()).map_err(|_| ())?;
    Ok(())
}
