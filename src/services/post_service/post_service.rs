use std::sync::Arc;
use async_trait::async_trait;
use crate::models::error::{CommonError, ServiceResult};
use http::StatusCode;
use crate::models::post::Post;
use crate::services::post_service::interface::PostService;
use crate::repositories::post_repository::interface::PostRepository;

#[derive(Clone)]
pub struct PostServiceImpl {
    pub post_repository: Arc<dyn PostRepository>,
}

impl PostServiceImpl {
    pub fn new(p_repository: Arc<dyn PostRepository>) -> Self {
        PostServiceImpl {
            post_repository: p_repository,
        }
    }
}


#[async_trait]
impl PostService for PostServiceImpl {
    async fn find_posts_by_user_id(&self, user_id: &i32) -> ServiceResult<Vec<Post>> {
        self.post_repository.find_posts_by_user_id(user_id).await.map_err(|err| CommonError {
            message: err.message,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}