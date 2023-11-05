use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn core() -> impl Responder {
    HttpResponse::Ok().body("Urban Rust!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}