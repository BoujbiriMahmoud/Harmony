use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(|| async { "Hello from Server 1!" }))
    })
    .bind("127.0.0.1:8081")
    .expect("Failed to bind Server 1")
    .run()
    .await
    .expect("Failed to run Server 1");
}

