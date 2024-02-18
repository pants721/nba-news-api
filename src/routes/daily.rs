use actix_web::{Responder, web::{Data, Json, self}, get};
use futures::future::join_all;
use reqwest::Client;
use itertools::Itertools;
use crate::news_scraper::site::sites;

#[get("/top-articles")]
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

#[get("/top-articles/{origin}")]
pub async fn get_top_articles_from_origin(
    reqwest_client: Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    let origin = path.into_inner();
    Json(
        join_all(
            sites::get_all()
                .iter()
                .map(|site| async {
                    site.get_top_articles(reqwest_client.get_ref().clone())
                        .await.unwrap()
                        .into_iter()
                        .filter(|article| article.origin == origin)
                })
        )
            .await
            .into_iter()
            .flatten()
            .collect_vec()
    )
}


