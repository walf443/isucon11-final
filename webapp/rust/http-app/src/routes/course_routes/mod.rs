use crate::routes::course_routes::add_class::add_class;
use crate::routes::course_routes::add_course::add_course;
use crate::routes::course_routes::download_submitted_assignments::download_submitted_assignments;
use crate::routes::course_routes::get_classes::get_classes;
use crate::routes::course_routes::get_course_detail::get_course_detail;
use crate::routes::course_routes::register_scores::register_scores;
use crate::routes::course_routes::search_courses::search_courses;
use crate::routes::course_routes::set_course_status::set_course_status;
use crate::routes::course_routes::submit_assignment::submit_assignment;
use actix_web::{web, Scope};
use isucholar_core::services::manager::ServiceManager;
use isucholar_http_core::middleware::IsAdmin;

mod add_class;
mod add_course;
mod download_submitted_assignments;
mod get_classes;
mod get_course_detail;
mod register_scores;
mod search_courses;
mod set_course_status;
mod submit_assignment;

pub fn get_course_routes<Service: ServiceManager + 'static>() -> Scope {
    web::scope("/courses")
        .route("", web::get().to(search_courses::<Service>))
        .service(
            web::resource("")
                .guard(actix_web::guard::Post())
                .wrap(IsAdmin)
                .to(add_course::<Service>),
        )
        .route("/{course_id}", web::get().to(get_course_detail::<Service>))
        .service(
            web::resource("/{course_id}/status")
                .guard(actix_web::guard::Put())
                .wrap(IsAdmin)
                .to(set_course_status::<Service>),
        )
        .route(
            "/{course_id}/classes",
            web::get().to(get_classes::<Service>),
        )
        .service(
            web::resource("/{course_id}/classes")
                .guard(actix_web::guard::Post())
                .wrap(IsAdmin)
                .to(add_class::<Service>),
        )
        .route(
            "/{course_id}/classes/{class_id}/assignments",
            web::post().to(submit_assignment::<Service>),
        )
        .service(
            web::resource("/{course_id}/classes/{class_id}/assignments/scores")
                .guard(actix_web::guard::Put())
                .wrap(IsAdmin)
                .to(register_scores::<Service>),
        )
        .service(
            web::resource("/{course_id}/classes/{class_id}/assignments/export")
                .guard(actix_web::guard::Get())
                .wrap(IsAdmin)
                .to(download_submitted_assignments),
        )
}
