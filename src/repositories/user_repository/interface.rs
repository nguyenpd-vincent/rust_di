use async_trait::async_trait;
use crate::models::user::User;
use crate::models::error::RepositoryResult;

#[async_trait]
pub trait UserRepository: Send + Sync{
    async fn find_user_by_email(&self, email: &str) -> RepositoryResult<User>;
    async fn find_user_by_id(&self, user_id: &i32) -> RepositoryResult<Option<User>>;
}
