-- Your SQL goes here
CREATE TABLE projects (
    project_id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    url VARCHAR NOT NULL,
    github_repository VARCHAR NOT NULL
);

ALTER TABLE projects ADD UNIQUE (project_id);
ALTER TABLE projects ADD UNIQUE (name);
