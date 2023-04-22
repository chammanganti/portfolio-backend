use std::borrow::Cow;

use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;

use crate::{
    db::{projects, Db},
    errors::{AppError, CustomError},
    models::project::{Project, ProjectCreate},
};

#[get("/")]
pub async fn get_projects<'a>(db: Db) -> Result<Json<Vec<Project>>, AppError<'a>> {
    let projects = db.run(projects::find).await?;
    Ok(Json(projects))
}

#[get("/<id>")]
pub async fn get_project<'a>(db: Db, id: &str) -> Result<Json<Project>, AppError<'a>> {
    let project_id = (*id).to_owned();
    let project = db
        .run(move |conn| projects::find_by_id(conn, &project_id))
        .await?;
    match project {
        Some(project) => Ok(Json(project)),
        None => {
            return Err(AppError::new(
                Status::NotFound,
                Cow::from(CustomError::<'_>::RecordDoesNotExist(id).to_string()),
            ))
        }
    }
}

#[post("/", data = "<project>")]
pub async fn create_project<'a>(
    db: Db,
    project: Json<ProjectCreate<'_>>,
) -> Result<Json<Project>, AppError<'a>> {
    let new_project = Project {
        project_id: Uuid::new_v4().to_string(),
        name: project.name.to_string(),
        description: Some(project.description.unwrap_or_default().to_string()),
        url: project.url.to_string(),
        github_repository: project.github_repository.to_string(),
    };
    let project = db
        .run(move |conn| projects::create(conn, new_project))
        .await?;
    Ok(Json(project))
}
