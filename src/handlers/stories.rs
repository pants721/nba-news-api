use actix_web::{Responder, web::{Data, Json, self, ServiceConfig}, get};
use futures::future::join_all;
use reqwest::Client;
use itertools::Itertools;
use crate::news_scraper::{site::sites, article::Article};

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(get_top_articles)
            .service(get_top_articles_from_origin);
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get top articles from all supported news sources. For supported sourced see \"/sources\".")
    )
)]
#[get("/top")]
pub async fn get_top_articles(
    reqwest_client: Data<Client>,
) -> impl Responder {
    Json(
        join_all(
            sites::get_all()
                .iter()
                .map(|site| async {
                    site.get_top_articles(reqwest_client.get_ref().clone())
                        .await.unwrap()
                })
        )
            .await
            .into_iter()
            .flatten()
            .collect_vec()
    )
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get top articles from specified supported news source. For supported sourced see \"/sources\".")
    ),
    params(
        ("source", description = "Supported news source name")
    )
)]
#[get("/top/{source}")]
pub async fn get_top_articles_from_origin(
    reqwest_client: Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let source = path.into_inner();
    Json(
        join_all(
            sites::get_all()
                .iter()
                .map(|site| async {
                    site.get_top_articles(reqwest_client.get_ref().clone())
                        .await.unwrap()
                        .into_iter()
                        .filter(|article| article.source == source)
                })
        )
            .await
            .into_iter()
            .flatten()
            .collect_vec()
    )
}


