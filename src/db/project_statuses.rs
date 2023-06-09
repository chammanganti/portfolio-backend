use diesel::prelude::*;
use diesel::PgConnection;

use crate::schema::project_statuses::dsl::*;
use crate::{errors::DbError, models::project_status::ProjectStatus};

pub fn find<'a>(conn: &mut PgConnection) -> Result<Vec<ProjectStatus>, DbError<'a>> {
    let results = project_statuses.get_results::<ProjectStatus>(conn)?;
    Ok(results)
}

pub fn find_by_id<'a>(
    conn: &mut PgConnection,
    id: &str,
) -> Result<Option<ProjectStatus>, DbError<'a>> {
    let project_status = project_statuses
        .filter(project_status_id.eq(id))
        .first::<ProjectStatus>(conn)
        .optional()?;
    Ok(project_status)
}

pub fn find_by_project<'a>(
    conn: &mut PgConnection,
    pid: &str,
) -> Result<Vec<ProjectStatus>, DbError<'a>> {
    let results = project_statuses
        .filter(project_id.eq(pid))
        .get_results::<ProjectStatus>(conn)?;
    Ok(results)
}

pub fn create<'a>(
    conn: &mut PgConnection,
    new_project_status: ProjectStatus,
) -> Result<ProjectStatus, DbError<'a>> {
    let project_status = diesel::insert_into(project_statuses)
        .values(new_project_status)
        .get_result::<ProjectStatus>(conn)?;
    Ok(project_status)
}

pub fn update<'a>(
    conn: &mut PgConnection,
    id: &str,
    updated_project_status: ProjectStatus,
) -> Result<ProjectStatus, DbError<'a>> {
    let project_status = diesel::update(project_statuses.filter(project_status_id.eq(id)))
        .set(updated_project_status)
        .get_result::<ProjectStatus>(conn)?;
    Ok(project_status)
}

pub fn delete<'a>(conn: &mut PgConnection, id: &str) -> Result<(), DbError<'a>> {
    diesel::delete(project_statuses.filter(project_status_id.eq(id))).execute(conn)?;
    Ok(())
}
