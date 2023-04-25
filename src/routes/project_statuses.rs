use rocket::{http::Status, serde::json::Json};
use std::borrow::Cow;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::{project_statuses, Db},
    errors::{AppError, CustomError},
    models::project_status::{ProjectStatus, ProjectStatusCreate, ProjectStatusUpdate},
};

#[get("/")]
pub async fn get_project_statuses<'a>(db: Db) -> Result<Json<Vec<ProjectStatus>>, AppError<'a>> {
    let project_statuses = db.run(project_statuses::find).await?;
    Ok(Json(project_statuses))
}

#[get("/<id>")]
pub async fn get_project_status<'a>(db: Db, id: &str) -> Result<Json<ProjectStatus>, AppError<'a>> {
    let id_find = Cow::Owned(id.to_string());
    let project_status = db
        .run(move |conn| project_statuses::find_by_id(conn, &id_find))
        .await?;
    match project_status {
        Some(project_status) => Ok(Json(project_status)),
        None => {
            return Err(AppError::new(
                Status::NotFound,
                Cow::from(CustomError::RecordDoesNotExist(id).to_string()),
            ))
        }
    }
}

#[get("/project/<project_id>")]
pub async fn get_project_statuses_by_project<'a>(
    db: Db,
    project_id: &str,
) -> Result<Json<Vec<ProjectStatus>>, AppError<'a>> {
    let project_id_find = Cow::Owned(project_id.to_string());
    let project_statuses = db
        .run(move |conn| project_statuses::find_by_project(conn, &project_id_find))
        .await?;
    Ok(Json(project_statuses))
}

#[post("/", data = "<project_status>")]
pub async fn create_project_status<'a>(
    db: Db,
    project_status: Json<ProjectStatusCreate<'_>>,
) -> Result<Json<ProjectStatus>, AppError<'a>> {
    project_status.validate()?;

    if check_project_status_by_project(&db, project_status.name, project_status.project_id).await? {
        return Err(AppError::new(
            Status::Conflict,
            Cow::from(
                CustomError::ProjectStatusAlreadyExists(
                    project_status.name,
                    project_status.project_id,
                )
                .to_string(),
            ),
        ));
    }

    let new_project_status = ProjectStatus {
        project_status_id: Uuid::new_v4().to_string(),
        name: project_status.name.to_string(),
        is_healthy: false,
        project_id: project_status.project_id.to_string(),
    };
    let project_status = db
        .run(move |conn| project_statuses::create(conn, new_project_status))
        .await?;
    Ok(Json(project_status))
}

#[put("/<id>", data = "<project_status>")]
pub async fn update_project_status<'a>(
    db: Db,
    id: &str,
    project_status: Json<ProjectStatusUpdate<'_>>,
) -> Result<Json<ProjectStatus>, AppError<'a>> {
    project_status.validate()?;

    let id_find = Cow::Owned(id.to_string());
    let id_update = Cow::Owned(id.to_string());
    let existing_project_status = match db
        .run(move |conn| project_statuses::find_by_id(conn, &id_find))
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

    let updated_name = match project_status.name {
        Some(new_name) => {
            if check_project_status_by_project(&db, new_name, &existing_project_status.project_id)
                .await?
            {
                return Err(AppError::new(
                    Status::Conflict,
                    Cow::from(
                        CustomError::ProjectStatusAlreadyExists(
                            new_name,
                            &existing_project_status.project_id,
                        )
                        .to_string(),
                    ),
                ));
            } else {
                new_name.to_string()
            }
        }
        None => existing_project_status.name,
    };

    let updated_is_healthy = match project_status.is_healthy {
        Some(new_is_healthy) => new_is_healthy,
        None => existing_project_status.is_healthy,
    };

    let updated_project_id = match project_status.project_id {
        Some(new_project_id) => {
            if check_project_status_by_project(&db, &updated_name, new_project_id).await? {
                return Err(AppError::new(
                    Status::Conflict,
                    Cow::from(
                        CustomError::ProjectStatusAlreadyExists(&updated_name, new_project_id)
                            .to_string(),
                    ),
                ));
            } else {
                new_project_id.to_string()
            }
        }
        None => existing_project_status.project_id,
    };

    let updated_project_status = ProjectStatus {
        project_status_id: existing_project_status.project_status_id,
        name: updated_name,
        is_healthy: updated_is_healthy,
        project_id: updated_project_id,
    };
    let updated_project_status = db
        .run(move |conn| project_statuses::update(conn, &id_update, updated_project_status))
        .await?;
    Ok(Json(updated_project_status))
}

#[delete("/<id>")]
pub async fn delete_project_status<'a>(db: Db, id: &str) -> Result<Status, AppError<'a>> {
    let id_find = Cow::Owned(id.to_string());
    let id_delete = Cow::Owned(id.to_string());
    if db
        .run(move |conn| project_statuses::find_by_id(conn, &id_find))
        .await?
        .is_none()
    {
        return Err(AppError::new(
            Status::NotFound,
            Cow::from(CustomError::<'_>::RecordDoesNotExist(id).to_string()),
        ));
    }

    db.run(move |conn| project_statuses::delete(conn, &id_delete))
        .await?;

    Ok(Status::NoContent)
}

async fn check_project_status_by_project<'a>(
    db: &Db,
    name: &str,
    project_id: &str,
) -> Result<bool, AppError<'a>> {
    let project_id = Cow::Owned(project_id.to_string());
    let existing_project_status = db
        .run(move |conn| project_statuses::find_by_project(conn, &project_id))
        .await?;
    Ok(existing_project_status.into_iter().any(|p| p.name == name))
}
