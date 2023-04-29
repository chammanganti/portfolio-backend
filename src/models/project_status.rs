use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

use super::project::Project;
use crate::schema::project_statuses;

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Insertable,
    AsChangeset,
    Identifiable,
    Associations,
    Clone,
    Debug,
)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = project_statuses)]
#[diesel(primary_key(project_status_id))]
pub struct ProjectStatus {
    pub project_status_id: String,
    pub name: String,
    pub is_healthy: bool,
    pub project_id: String,
}

#[derive(Deserialize, Validate, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProjectStatusCreate<'a> {
    #[validate(length(
        min = 3,
        max = 12,
        message = "Name must be between 3 and 12 characters"
    ))]
    pub name: &'a str,
    pub project_id: &'a str,
}

#[derive(Deserialize, Validate, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProjectStatusUpdate<'a> {
    #[validate(length(
        min = 3,
        max = 12,
        message = "Name must be between 3 and 12 characters"
    ))]
    pub name: Option<&'a str>,
    pub is_healthy: Option<bool>,
    pub project_id: Option<&'a str>,
}
