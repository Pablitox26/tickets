use actix_web::{web, HttpResponse};

use super::user::{create_user, get_user, update_user, delete_user, get_all_users};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
    .default_service(web::to(|| HttpResponse::NotFound()))
    .service(create_user)
    .service(get_user)
    .service(update_user)
    .service(delete_user)
    .service(get_all_users);

    conf.service(scope);
}