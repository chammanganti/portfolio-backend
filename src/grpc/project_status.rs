use std::borrow::Cow;

use tonic::{Request, Response, Status};

use self::project_status_proto::{
    project_status_server::ProjectStatus,
    {FindRequest, FindResponse, ProjectStatusProto, UpdateRequest, UpdateResponse},
};
use crate::{
    db::{project_statuses, Db},
    errors::CustomError,
};

pub mod project_status_proto {
    tonic::include_proto!("project_status");
}

pub struct ProjectStatusService {
    db: Db,
}

impl ProjectStatusService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl ProjectStatus for ProjectStatusService {
    async fn find(&self, _req: Request<FindRequest>) -> Result<Response<FindResponse>, Status> {
        let project_statuses = self.db.run(project_statuses::find).await.unwrap();
        let project_statuses = project_statuses
            .into_iter()
            .map(|p| ProjectStatusProto {
                project_status_id: p.project_status_id,
                name: p.name,
                is_healthy: p.is_healthy,
                project_id: p.project_id,
            })
            .collect::<Vec<ProjectStatusProto>>();
        Ok(Response::new(FindResponse { project_statuses }))
    }

    async fn update(
        &self,
        req: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        let req = req.into_inner();
        let id_find = Cow::Owned(req.project_status_id.clone());
        let id_update = Cow::Owned(req.project_status_id.clone());
        let existing_project_status = self
            .db
            .run(move |conn| project_statuses::find_by_id(conn, &id_find))
            .await
            .map_err(|_| Status::internal("Internal db error"))?;
        let existing_project_status = match existing_project_status {
            Some(project_status) => project_status,
            None => {
                return Err(Status::not_found(
                    CustomError::<'_>::RecordDoesNotExist(&req.project_status_id).to_string(),
                ))
            }
        };

        let mut updated_project_status = existing_project_status;
        updated_project_status.is_healthy = req.is_healthy;
        let updated_project_status = self
            .db
            .run(move |conn| project_statuses::update(conn, &id_update, updated_project_status))
            .await
            .map_err(|_| Status::internal("Project status update failed"))?;
        Ok(Response::new(UpdateResponse {
            project_status_id: updated_project_status.project_status_id,
            name: updated_project_status.name,
            is_healthy: updated_project_status.is_healthy,
            project_id: updated_project_status.project_id,
        }))
    }
}
