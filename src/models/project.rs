use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::projects;

#[derive(Serialize, Queryable, Insertable, Identifiable, Debug)]
#[diesel(table_name = projects)]
#[diesel(primary_key(project_id))]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub github_repository: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ProjectCreate<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub url: &'a str,
    pub github_repository: &'a str,
}
