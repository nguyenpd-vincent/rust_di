use actix_web::{
    web::{self, Data},
    App, Error, HttpServer, middleware::Logger, body::MessageBody, dev::{ServiceFactory, ServiceRequest, ServiceResponse}
};
use rust_di::{
    config::{app_config::AppConfig, databases::mysql::MysqlPool},
    services::{
        user_service::{interface::UserService, user_service::UserServiceImpl},
        post_service::{interface::PostService, post_service::PostServiceImpl}
    },
    repositories::{
        user_repository::interface::UserRepository,
        mysql_repository::MysqlRepository,
        post_repository::interface::PostRepository
    },
    utils::logging::init_logging,
    routes::{auth_route::init_auth_routes, post_route::init_post_routes}
};
use std::sync::Arc;

pub struct Container {
    pub user_service: Arc<dyn UserService>,
    pub post_service: Arc<dyn PostService>
}

impl Container {
    pub fn new() -> Self {

        let mysql_pool: MysqlPool = MysqlPool::new();
        // let pool = mysql_pool.get_conn();
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            MysqlRepository::new(mysql_pool.pool.clone()),
        );
        let post_repository: Arc<dyn PostRepository> = Arc::new(
            MysqlRepository::new(mysql_pool.pool.clone()),
        );
        let user_service: Arc<UserServiceImpl> = Arc::new(
            UserServiceImpl { user_repository: user_repository.clone() }
        );
        let post_service: Arc<PostServiceImpl> = Arc::new(
            PostServiceImpl {post_repository: post_repository.clone()}
        );
        Container {
            user_service,
            post_service
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}


pub fn create_app(container: Arc<Container>) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let user_service: Arc<dyn UserService> = container.user_service.clone();
    let post_service: Arc<dyn PostService> = container.post_service.clone();

    App::new()
        .app_data(Data::from(user_service))
        .app_data(Data::from(post_service))
        .wrap(Logger::default())
        // .wrap(ServiceContextMaintenanceCheck)
        .configure(init_auth_routes)
        .configure(init_post_routes)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();
    let config: AppConfig = AppConfig::new();
    let address: String = format!("{}:{}", config.host, config.port);
    println!("Server running at http://{}", address);
    let container: Arc<Container> = Arc::new(Container::new());
    let server = HttpServer::new(move || { create_app(container.clone()) })
        .bind(&address)?;
    server.run().await
}