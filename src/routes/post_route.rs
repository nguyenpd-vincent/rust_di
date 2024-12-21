use actix_web::{web, HttpResponse, Responder};
use crate::services::post_service::interface::PostService;
use crate::services::user_service::interface::UserService;

pub fn init_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/posts/{id}")
            .route(web::get().to(posts_user_id))
    );
}

pub async fn posts_user_id(id: web::Path<i32>,
                           user_service: web::Data<dyn UserService>,
                           post_service: web::Data<dyn PostService>) -> impl Responder {
    let id = id.into_inner();
    // check if user exists
    match user_service.find_user_by_id(&id).await {
        Ok(Some(_)) => match post_service.find_posts_by_user_id(&id).await {
            Ok(posts) => HttpResponse::Ok().json(posts),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}