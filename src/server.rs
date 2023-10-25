use actix_web::{web, App, HttpServer, HttpResponse, middleware, http::header};
use actix_cors::Cors;
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

use crate::probes;
use crate::apis;
use crate::configurations::fromenv::Configuration;

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn server() -> std::io::Result<()> {

    let listen_ipv4_addr = "localhost"; //TODO:CONFIG
    let listen_ipv4_port = 12345;       //TODO:CONFIG

    let cfg = match Configuration::init() {
        Ok(cfg) => { cfg }
        Err(e) => {
            println!("Configuration error, {}", e);
            std::process::exit(1)
        }
    };

    let database_url = format!("{}://{}:{}@{}:{}/{}", 
            cfg.db.driver.clone(),
            cfg.db.user.clone(),
            cfg.db.password.clone(),
            cfg.db.host.clone(),
            cfg.db.port.clone(),
            cfg.db.name.clone(),
            );

    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(1)             //TODO:CONFIG
        .connect(&database_url)
        .await 
    {
        Ok(pool) => {
           println!("Connection to the db is successful!");
           pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "HEAD"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ]);
        App::new()
            .app_data(actix_web::web::Data::new(AppState {db: pool.clone()}))
            //.wrap(rbac) //TODO:InDesign
            //.wrap(auth) //TODO:InDesign
            .wrap(middleware::NormalizePath::trim())
            .route("/healthz", web::get().to(probes::healthz))
            .route("/livez", web::get().to(probes::livez))
            .route("/readyz", web::get().to(probes::readyz))
            .service(
                web::scope("/{namespace}")
                .configure(apis::storage::routes::routes)
            )
            .wrap(cors)
            .default_service(web::to(|| HttpResponse::Forbidden()))
    })
    .bind((listen_ipv4_addr, listen_ipv4_port))?
    .workers(1)  //TODO:CONFIG
    .run()
    .await
}
