use actix_web::{web::{Json, ServiceConfig}, Responder, get};

use crate::news_scraper::site::sites;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(get_sources);
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get supported news source's names")
    )
)]
#[get("/sources")]
pub async fn get_sources() -> impl Responder {
    Json(sites::get_all())
}
