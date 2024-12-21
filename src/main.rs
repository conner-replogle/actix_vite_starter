use actix_web::{middleware::Logger, App, HttpServer};
use actix_web_lab::web::spa;
use log::info;

mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
         
            .configure(routes::factory)
            .service(spa()
            .index_file("./ui/dist/index.html")
            .static_resources_mount("/assets")
            .static_resources_location("./ui/dist/assets")
            .finish(),)
         
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}