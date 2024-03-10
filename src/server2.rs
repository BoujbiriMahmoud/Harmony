// Import necessary items from the actix_web crate
use actix_web::{web, App, HttpServer};

/// # Server 2
///
/// This module defines a simple Actix Web server (Server 2) that responds with a "Hello from Server 2!" message
/// when accessed at the root ("/") endpoint.
///
/// ## Start
/// As explained earlier to start the server, run the following command.
/// cargo run --bin server2
///
/// ## Usage
///
/// To run Server 2, include the actix-web crate in your dependencies.
///
/// ## Endpoint
///
/// - `GET /`: Returns a simple "Hello from Server 2!" message.
///
/// ## Configuration
///
/// - **Binding Address:** Server 2 is configured to bind to the address "127.0.0.1:8082".
///
/// ## Errors
///
/// The server might fail to bind or run, resulting in a panic with appropriate error messages.
///
/// ## Dependencies
///
/// The code relies on the Actix Web framework. Make sure to include it in your `Cargo.toml`.
///
/// This server is designed for demonstration purposes and can be modified or integrated into a larger system as needed.

/// The `main` function, serving as the entry point of the application.
#[actix_web::main]
async fn main() {
    // Create a new Actix Web HTTP server, using a closure to configure the app.
    HttpServer::new(|| {
        // Create a new Actix Web application and define a single service for the root ("/") endpoint, responding with the specified closure.
        App::new().service(web::resource("/").to(|| async { "Hello from Server 2!\n" }))
    })
    // Bind the server to the specified address and port.
    .bind("127.0.0.1:8082")
    // If binding fails, panic with a descriptive error message.
    .expect("Failed to bind Server 2")
    // Run the server, awaiting its completion.
    .run()
     // If the server fails to run, panic with a descriptive error message.
    .await
    .expect("Failed to run Server 2");
}
