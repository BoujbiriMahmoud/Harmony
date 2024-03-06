use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(|| async { "Hello from Server 3!" }))
    })
    .bind("127.0.0.1:8083")
    .expect("Failed to bind Server 3")
    .run()
    .await
    .expect("Failed to run Server 3");
}

