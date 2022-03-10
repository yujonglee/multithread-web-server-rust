use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tokio::time::{Duration, sleep};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .route("/", web::get().to(home_handler))
        .route("/sleep", web::get().to(sleep_handler))
        // .default_service(default_handler)
    )
        .bind(("127.0.0.1", 7879))?
        .workers(4)
        .run()
        .await
}

async fn home_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../index.html"))
}

async fn sleep_handler() -> impl Responder {
    sleep(Duration::from_secs(5)).await;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../index.html"))
}
