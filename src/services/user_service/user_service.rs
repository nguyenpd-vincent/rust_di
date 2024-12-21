use std::sync::Arc;
use async_trait::async_trait;
use crate::models::error::{CommonError, ServiceResult};
use http::StatusCode;
use crate::models::user::User;
use crate::services::user_service::interface::UserService;
use crate::repositories::user_repository::interface::UserRepository;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(u_repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            user_repository: u_repository,
        }
    }
}


#[async_trait]
impl UserService for UserServiceImpl {
    async fn find_user_by_email(&self, email: &str) -> ServiceResult<User> {
        self.user_repository.find_user_by_email(email).await.map_err(|err| CommonError {
            message: err.message,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        })
    }

    async fn find_user_by_id(&self, user_id: &i32) -> ServiceResult<Option<User>> {
        self.user_repository.find_user_by_id(user_id).await.map_err(|err| CommonError {
            message: err.message,
            code: StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}