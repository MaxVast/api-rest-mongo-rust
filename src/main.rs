mod api;
mod models;
mod db;

use crate::api::handler;
use crate::db::db::MongoRepo;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web::Data, App, HttpServer};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;
use crate::models::customer;
use crate::models::seller_point;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    println!("✅ Server started successfully");

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler::customers_list,
            handler::get_customer,
            handler::seller_points_list,
            handler::get_seller_point,
        ),
        components(
            schemas(customer::Customer, seller_point::SellerPoint)
        ),
        tags(
            (name = "customer", description = "Customer management endpoints."),
            (name = "Seller Points", description = "Seller Points management endpoints.")
        )
    )]
    struct ApiDoc;
    // Make instance variable of ApiDoc so all worker threads gets the same instance.
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(db_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
