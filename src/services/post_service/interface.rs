use async_trait::async_trait;
use crate::models::post::Post;
use crate::models::error::ServiceResult;

#[async_trait]
pub trait PostService: 'static + Sync + Send {
    async fn find_posts_by_user_id(&self, user_id: &i32) -> ServiceResult<Vec<Post>>;
}