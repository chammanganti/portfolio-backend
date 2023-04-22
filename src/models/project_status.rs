use diesel::{Associations, Identifiable, Queryable};

use super::project::Project;
use crate::schema::project_statuses;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = project_statuses)]
#[diesel(primary_key(project_status_id))]
pub struct ProjectStatus {
    pub project_status_id: String,
    pub name: String,
    pub is_healthy: bool,
    pub project_id: String,
}
