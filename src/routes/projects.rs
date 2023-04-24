use rocket::{http::Status, serde::json::Json};
use std::borrow::Cow;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::{projects, Db},
    errors::{AppError, CustomError},
    models::project::{Project, ProjectCreate, ProjectUpdate},
};

#[get("/")]
pub async fn get_projects<'a>(db: Db) -> Result<Json<Vec<Project>>, AppError<'a>> {
    let projects = db.run(projects::find).await?;
    Ok(Json(projects))
}

#[get("/<id>")]
pub async fn get_project<'a>(db: Db, id: &str) -> Result<Json<Project>, AppError<'a>> {
    let id_find = Cow::Owned(id.to_string());
    let project = db
        .run(move |conn| projects::find_by_id(conn, &id_find))
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

#[get("/name/<name>")]
pub async fn get_project_by_name<'a>(db: Db, name: &str) -> Result<Json<Project>, AppError<'a>> {
    let name_find = Cow::Owned(name.to_string());
    let project = db
        .run(move |conn| projects::find_by_name(conn, &name_find))
        .await?;
    match project {
        Some(project) => Ok(Json(project)),
        None => {
            return Err(AppError::new(
                Status::NotFound,
                Cow::from(CustomError::<'_>::RecordDoesNotExist(name).to_string()),
            ))
        }
    }
}

#[post("/", data = "<project>")]
pub async fn create_project<'a>(
    db: Db,
    project: Json<ProjectCreate<'_>>,
) -> Result<Json<Project>, AppError<'a>> {
    project.validate()?;

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

#[put("/<id>", data = "<project>")]
pub async fn update_project<'a>(
    db: Db,
    id: &str,
    project: Json<ProjectUpdate<'_>>,
) -> Result<Json<Project>, AppError<'a>> {
    project.validate()?;

    let id_find = Cow::Owned(id.to_string());
    let id_update = Cow::Owned(id.to_string());
    let existing_project = match db
        .run(move |conn| projects::find_by_id(conn, &id_find))
        .await?
    {
        Some(project) => project,
        None => {
            return Err(AppError::new(
                Status::NotFound,
                Cow::from(CustomError::<'_>::RecordDoesNotExist(id).to_string()),
            ))
        }
    };

    let updated_project = Project {
        project_id: existing_project.project_id,
        name: match project.name {
            Some(new_name) => new_name.to_string(),
            None => existing_project.name,
        },
        description: match project.description {
            Some(new_description) => Some(new_description.to_string()),
            None => existing_project.description,
        },
        url: match project.url {
            Some(new_url) => new_url.to_string(),
            None => existing_project.url,
        },
        github_repository: match project.github_repository {
            Some(new_github_repository) => new_github_repository.to_string(),
            None => existing_project.github_repository,
        },
    };
    let updated_project = db
        .run(move |conn| projects::update(conn, &id_update, updated_project))
        .await?;
    Ok(Json(updated_project))
}

#[delete("/<id>")]
pub async fn delete_project<'a>(db: Db, id: &str) -> Result<Status, AppError<'a>> {
    let id_find = Cow::Owned(id.to_string());
    let id_delete = Cow::Owned(id.to_string());
    if db
        .run(move |conn| projects::find_by_id(conn, &id_find))
        .await?
        .is_none()
    {
        return Err(AppError::new(
            Status::NotFound,
            Cow::from(CustomError::<'_>::RecordDoesNotExist(id).to_string()),
        ));
    }

    db.run(move |conn| projects::delete(conn, &id_delete))
        .await?;

    Ok(Status::NoContent)
}
