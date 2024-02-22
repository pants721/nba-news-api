use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use handlers::{index, stories};
use news_scraper::article;
use reqwest::Client;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::Redoc;
use utoipa_swagger_ui::SwaggerUi;

mod handlers;
mod news_scraper;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            index::get_sources,
            stories::get_top_articles,
            stories::get_top_articles_from_origin,
        ),
        components(
            schemas(article::Article)
        ),
        tags(
            (name = "nba news", description = "API for NBA news")
        )
    )]
    struct ApiDoc;

    log::info!("starting HTTP server at http://localhost:8080");

    let reqwest_client = Client::new();
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(reqwest_client.clone()))
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(Redoc::new(openapi.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .configure(index::configure())
            .configure(stories::configure())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
