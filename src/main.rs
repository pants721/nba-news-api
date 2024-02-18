use actix_web::{web::{Data, scope}, HttpServer, App, middleware};
use reqwest::Client;
use routes::daily;

mod news_scraper;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");
    
    HttpServer::new(move || {
        let reqwest_client = Client::new();

        App::new()
            .app_data(Data::new(reqwest_client.clone()))
            .wrap(middleware::Logger::default())
            .service(
                scope("/daily")
                    .service(daily::get_top_articles)
                    .service(daily::get_top_articles_from_origin)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
