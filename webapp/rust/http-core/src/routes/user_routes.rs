use crate::routes::user_routes::get_grades::get_grades;
use crate::routes::user_routes::get_me::get_me;
use crate::routes::user_routes::get_registered_courses::get_registered_courses;
use crate::routes::user_routes::register_courses::register_courses;
use actix_web::{web, Scope};
use isucholar_core::services::manager::ServiceManager;

mod get_grades;
mod get_me;
mod get_registered_courses;
mod register_courses;

pub fn get_user_routes<Service: ServiceManager + 'static>() -> Scope {
    web::scope("/users")
        .route("/me", web::get().to(get_me::<Service>))
        .route(
            "/me/courses",
            web::get().to(get_registered_courses::<Service>),
        )
        .route("/me/courses", web::put().to(register_courses::<Service>))
        .route("/me/grades", web::get().to(get_grades::<Service>))
}
