use actix_web::{web, App, HttpServer};

mod base;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(base::core)
            .service(base::echo)
            .route("/hey", web::get().to(base::manual_hello))
    })
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
