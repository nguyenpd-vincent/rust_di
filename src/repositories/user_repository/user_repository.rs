use mysql::*;
use mysql::prelude::*;
use async_trait::async_trait;
use crate::models::user::User;
use crate::models::error::{RepositoryError, RepositoryResult};
use crate::repositories::mysql_repository::{MysqlRepository};
use crate::repositories::user_repository::interface::UserRepository;

#[async_trait]
impl UserRepository for MysqlRepository{
    async fn find_user_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        println!("email find_user_by_email  {}", email);
        let mut conn: PooledConn = self.pool.lock().unwrap().get_conn().map_err(|err: Error| RepositoryError {
            message: err.to_string(),
        })?;
        let query: &str = "SELECT id, email, name FROM users WHERE email = :email";
        let params: Params = params! {
            "email" => email,
        };
        let result: Option<(i32, String, String)> = conn.exec_first(query, params).map_err(|err: Error| RepositoryError {
            message: err.to_string(),
        })?;

        if let Some((id, email, name)) = result {
            Ok(User { id, email, name })
        } else {
            Err(RepositoryError {
                message: "User not found".to_string(),
            })
        }
    }

    async fn find_user_by_id(&self, user_id: &i32) -> RepositoryResult<Option<User>> {
        let mut conn: PooledConn = self.pool.lock().unwrap().get_conn().map_err(|err: Error| RepositoryError {
            message: err.to_string(),
        })?;
        let query: &str = "SELECT id, name, email FROM users WHERE id = :user_id";
        let params: Params = params! {
            "user_id" => user_id,
        };
        let result: Option<(i32, String, String)> = conn.exec_first(query, params).map_err(|err: Error| RepositoryError {
            message: err.to_string(),
        })?;

        Ok(result.map(|(id, email, name)| User { id, email, name }))
    }
}