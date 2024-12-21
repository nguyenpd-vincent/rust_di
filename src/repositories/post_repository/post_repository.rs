use mysql::*;
use mysql::prelude::*;
use async_trait::async_trait;
use crate::models::post::Post;
use crate::models::error::RepositoryError;
use crate::models::error::RepositoryResult;
use crate::repositories::mysql_repository::{MysqlRepository};
use crate::repositories::post_repository::interface::PostRepository;

#[async_trait]
impl PostRepository for MysqlRepository{
    async fn find_posts_by_user_id(&self, user_id: &i32) -> RepositoryResult<Vec<Post>> {
        println!("user_id find_posts_by_user_id  {}", user_id);
        let mut conn = self.pool.lock().unwrap().get_conn().map_err(|err| RepositoryError {
            message: err.to_string(),
        })?;
        let query = "SELECT id, title, content, user_id FROM posts WHERE user_id = :user_id";
        let params = params! {
            "user_id" => user_id,
        };
        let result: Vec<(i32, String, String, i32)> = conn.exec(query, params).map_err(|err| RepositoryError {
            message: err.to_string(),
        })?;

        Ok(result.into_iter().map(|(id, title, content, user_id)| Post { id, title, content, user_id }).collect())
    }
}