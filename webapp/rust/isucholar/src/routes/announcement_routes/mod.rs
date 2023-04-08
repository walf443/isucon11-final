use crate::middleware::IsAdmin;
use crate::routes::announcement_routes::add_announcement::add_announcement;
use crate::routes::announcement_routes::get_announcement_detail::get_announcement_detail;
use crate::routes::announcement_routes::get_announcement_list::get_announcement_list;
use actix_web::{web, Scope};
use isucholar_core::services::manager::{ServiceManager};

mod add_announcement;
mod get_announcement_detail;
mod get_announcement_list;

pub fn get_announcement_routes<Service: ServiceManager + 'static>() -> Scope {
    web::scope("/announcements")
        .route("", web::get().to(get_announcement_list))
        .service(
            web::resource("")
                .guard(actix_web::guard::Post())
                .wrap(IsAdmin)
                .to(add_announcement),
        )
        .route(
            "/{announcement_id}",
            web::get().to(get_announcement_detail::<Service>),
        )
}
