use diesel::prelude::*;
use diesel::PgConnection;

use crate::errors::DbError;
use crate::{models::project::Project, schema::projects::dsl::*};

pub fn find(conn: &mut PgConnection) -> Result<Vec<Project>, DbError> {
    let results = projects.get_results::<Project>(conn)?;
    Ok(results)
}

pub fn find_by_id(conn: &mut PgConnection, id: &str) -> Result<Option<Project>, DbError> {
    let project = projects
        .filter(project_id.eq(id))
        .first::<Project>(conn)
        .optional()?;
    Ok(project)
}

pub fn create(conn: &mut PgConnection, new_project: Project) -> Result<Project, DbError> {
    let project = diesel::insert_into(projects)
        .values(new_project)
        .get_result::<Project>(conn)?;
    Ok(project)
}
