// @generated automatically by Diesel CLI.

diesel::table! {
    project_statuses (project_status_id) {
        project_status_id -> Varchar,
        name -> Varchar,
        is_healthy -> Bool,
        project_id -> Varchar,
    }
}

diesel::table! {
    projects (project_id) {
        project_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
        url -> Varchar,
        github_repository -> Varchar,
    }
}

diesel::joinable!(project_statuses -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(project_statuses, projects);
