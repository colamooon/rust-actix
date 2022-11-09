use std::{thread, time::Duration};

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use colamoon_api::router_config::config_router;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(conn_spec);
    let connection_timeout = Duration::new(10, 0);
    let pool = r2d2::Pool::builder()
        .max_size(100)
        .connection_timeout(connection_timeout)
        // .error_handler(error_handler)
        .build(manager)
        .expect("Failed to create pool.");

    // for _ in 0..10i32 {
    //     let pool = pool.clone();
    //     thread::spawn(move || {
    //         let connection = pool.get();

    //         assert!(connection.is_ok());
    //     });
    // }
    log::info!("starting HTTP server at http://localhost:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(config_router)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("welcome!") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(4)
    .run()
    .await
}
