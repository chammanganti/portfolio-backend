use tonic::{Request, Response, Status};

use project_proto::{
    project_server::Project,
    {QueueStatusRequest, QueueStatusResponse},
};

pub mod project_proto {
    tonic::include_proto!("project");
}

#[derive(Default, Debug)]
pub struct ProjectService {}

#[tonic::async_trait]
impl Project for ProjectService {
    async fn queue_status_update(
        &self,
        req: Request<QueueStatusRequest>,
    ) -> Result<Response<QueueStatusResponse>, Status> {
        let _req = req.into_inner();
        let res = QueueStatusResponse { queued: true };
        Ok(Response::new(res))
    }
}
