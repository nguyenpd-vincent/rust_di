use actix_web::{web, HttpResponse, Responder};
use crate::routes::dto::auth::LoginPayload;
use crate::services::user_service::interface::UserService;


pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth/login")
            .route(web::post().to(login))
    );
}

pub async fn login(payload: web::Json<LoginPayload>, user_service: web::Data<dyn UserService>) -> impl Responder {
    match user_service.find_user_by_email(&payload.email).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
