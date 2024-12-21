use async_trait::async_trait;
use crate::models::post::Post;
use crate::models::error::RepositoryResult;

#[async_trait]
pub trait PostRepository: Send + Sync{
    async fn find_posts_by_user_id(&self, user_id: &i32) -> RepositoryResult<Vec<Post>>;
}

