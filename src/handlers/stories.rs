use crate::news_scraper::{article::Article, site};
use actix_web::{
    get,
    web::{self, Data, Json, ServiceConfig},
    Result,
};
use futures::future::try_join_all;
use itertools::Itertools;
use reqwest::Client;

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
pub async fn get_top_articles(reqwest_client: Data<Client>) -> Result<Json<Vec<Article>>> {
    Ok(Json(
        try_join_all(
            site::get_all()
                .iter()
                .map(|site| site.get_top_articles(reqwest_client.get_ref().clone())),
        )
            .await?
            .into_iter()
            .flatten()
            .collect_vec()
    )) // TODO: there has to be a better way to flatten this, but flat_map is weird here
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
) -> Result<Json<Vec<Article>>> {
    let source = path.into_inner();
    Ok(Json(
        try_join_all(
            site::get_all()
                .iter()
                .map(|site| site.get_top_articles(reqwest_client.get_ref().clone())),
        )
            .await?
            .into_iter()
            .flatten()
            .filter(|article| article.source == source)
            .collect_vec(),
    ))
}
