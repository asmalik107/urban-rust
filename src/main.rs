use actix_web::{middleware, web, App, HttpServer};
use env_logger:: Env;

mod base;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
