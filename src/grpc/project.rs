use tonic::{Request, Response, Status};

use self::project_proto::{
    project_server::Project,
    {FindRequest, FindResponse, ProjectProto},
};
use crate::db::{projects, Db};

pub mod project_proto {
    tonic::include_proto!("project");
}

pub struct ProjectService {
    db: Db,
}

impl ProjectService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Project for ProjectService {
    async fn find(&self, _req: Request<FindRequest>) -> Result<Response<FindResponse>, Status> {
        let projects = self.db.run(projects::find).await.unwrap();
        let projects = projects
            .into_iter()
            .map(|p| ProjectProto {
                project_id: p.project_id,
                name: p.name,
                description: p.description.unwrap_or("".to_owned()),
                url: p.url,
                github_repository: p.github_repository,
            })
            .collect::<Vec<ProjectProto>>();
        Ok(Response::new(FindResponse { projects }))
    }
}
