use std::{env};
use actix_web::{middleware, web, App, HttpServer};

mod base;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(base::core)
            .service(base::echo)
            .route("/hey", web::get().to(base::manual_hello))
    })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
