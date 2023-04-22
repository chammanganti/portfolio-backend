-- Your SQL goes here
CREATE TABLE project_statuses (
    project_status_id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    is_healthy BOOLEAN NOT NULL DEFAULT FALSE,
    project_id VARCHAR NOT NULL
);

ALTER TABLE project_statuses ADD UNIQUE (project_status_id);
ALTER TABLE project_statuses ADD CONSTRAINT fk_project_id FOREIGN KEY (project_id) REFERENCES projects(project_id);
