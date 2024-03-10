// Hyper crate is used for handling HTTP-related functionality.
use hyper::{Body, Client, Request, Response, Server, service::{make_service_fn, service_fn}};

// Tokio crate is utilized for asynchronous programming.
use tokio::sync::{Mutex, RwLock};
use tokio::time::{self, Duration};
use tokio::net::TcpStream;

// Standard library imports.
use std::sync::Arc; // Arc (Atomic Reference Counting) for shared ownership.
use std::collections::HashMap; // HashMap for storing request rate limits.
use std::convert::Infallible; // Infallible for functions that are guaranteed to never return an error.
use std::net::{SocketAddr, IpAddr}; // SocketAddr and IpAddr for network-related operations. // SocketAddr and IpAddr for network-related operations.

// Logging library for generating logs in the application.
use log::{info, warn};




/// # Harmony Load Balancer
///
/// Welcome to the Harmony Load Balancer documentation. This project serves as a simple load balancer for distributing incoming HTTP requests among multiple backend servers.
///
/// ## Overview
/// **Author:** Mahmoud BOUJBIRI
///
/// This project, authored by **Mahmoud BOUJBIRI**, serves as the final exam submission for the Rust Programming Language Module at ESGI Paris. As part of the acknowledgments, special thanks are extended to Mr. BOURY, the Rust instructor, for his dedication and passion throughout the course.
///
/// ### Project Inspiration
/// The inspiration for this load balancer stems from Stanford's Balancebeam project. Choosing this project for the final exam was a natural decision influenced by my background in network and telecommunications. The opportunity to explore a project associated with Stanford further motivated my selection. I aim to deliver an enjoyable experience in load balancing through this endeavor.
///
/// ### Disclaimer
/// This project is submitted as part of academic coursework and serves as a demonstration of skills acquired during the Rust Programming Language Module at ESGI Paris. The primary focus is on educational purposes and practical application of concepts learned in the course. As such, it may not cover all edge cases, security considerations, or production-level optimizations.
///
/// The author and ESGI Paris are not liable for any misuse, unintended consequences, or damages arising from the use or modification of this code in a non-educational or non-experimental context. By using this software, you agree that the author and ESGI Paris are not responsible for any disruptions, losses, or harm that may occur.
///
/// **Caution:** This software is provided "as-is," without any warranty of any kind, express or implied. You are responsible for understanding the risks involved in using and modifying this code.
///
/// ### Purpose
/// The Load Balancer is designed to evenly distribute incoming HTTP requests across a group of backend servers. It monitors the health of the servers, implements basic rate limiting, and logs relevant information.
///
/// ### Components
/// 1. **Server Health Checker:** Periodically checks the health of backend servers by attempting to establish a TCP connection.
/// 2. **Request Rate Limiter:** Limits the number of requests from a single client to prevent abuse.
/// 3. **Logger:** Provides detailed logs of incoming requests, server health, and any issues encountered.
///
/// ## Code Structure
///
/// The code is organized into several main sections:
///
/// 1. **Dependencies:** The external crates and libraries used in the project are imported, including Hyper for HTTP handling and Tokio for asynchronous tasks.
/// 2. **Main Function (`main`):** The entry point of the application. It initializes the logger, defines backend servers and rate limits, and starts the server. Additionally, it spawns a background task to check the health of backend servers.
/// 3. **Service Creation Function (`make_svc`):** Creates the service to handle incoming requests. It utilizes the `service_fn` to define a closure that proxies requests to backend servers.
/// 4. **HTTP Server Setup:** Configures and starts the HTTP server, binding it to a specified address.
/// 5. **Proxy Request Function (`proxy_request`):** Handles incoming HTTP requests. It checks rate limits, logs information, rotates backend servers, and forwards requests.
///
/// ## Usage
///
/// ### Configuration
/// - The load balancer is set to run on `127.0.0.1:8080` by default. You can change this in the `addr` variable within the `main` function.
///
/// ### Backend Servers
/// - Backend servers are specified in the `backend_servers` vector within the `main` function. Add or remove server URLs as needed.
///
/// ### Rate Limiting
/// - The load balancer implements a basic rate limit of 50 requests per client IP. Adjust the limit in the `proxy_request` function if needed.
///
/// ### Logging
/// - Logs provide information about incoming requests, selected backend servers, and server health. To display logs run the following command when starting the main.rs 'RUST_LOG=info cargo run --bin harmony'
///
/// ## Running the Load Balancer
///
/// 1. Clone the repository.
/// 2. Ensure you have Rust and Cargo installed.
/// 3. Start the load balancer by running the following command :
///
///    Harmony Load Balancer : 'cargo run --bin harmony'
///
///4. Start the load balancer with logs by running the following command :
///
///   Harmony Load Balancer with logs : 'RUST_LOG=info cargo run --bin harmony'
///
/// Feel free to explore and modify the code to suit your specific requirements. If you have any questions or encounter issues, refer to the source code comments or consult the documentation for the used libraries.
///
/// We wish you happy balancing!

/// # Main Function
///
/// The entry point for the Harmony Load Balancer application. It initializes the logger,
/// defines backend servers and rate limits, and starts the HTTP server. Additionally, it
/// spawns a background task to check the health of backend servers.
#[tokio::main]
async fn main() {
    // Initialize the logger for logging.
    env_logger::init();

    // Define the address for the load balancer.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    // Create shared state for backend servers and request rate limits.
    let backend_servers = Arc::new(Mutex::new(vec![
        "http://127.0.0.1:8081".to_string(),
        "http://127.0.0.1:8082".to_string(),
        "http://127.0.0.1:8083".to_string(),
    ]));
    let request_limits = Arc::new(RwLock::new(HashMap::<IpAddr, u64>::new()));

    // Background task: Check the health of backend servers.
    let backend_servers_clone = Arc::clone(&backend_servers);
    tokio::spawn(async move {
        // Create an interval of 20 seconds.
        let mut interval = time::interval(Duration::from_secs(20));
        // Continuously perform health checks.
        loop {
            // Wait for the interval to elapse.
            interval.tick().await;
            // Obtain a lock on the backend servers.
            let servers = backend_servers_clone.lock().await;
            // Iterate over each backend server.
            for server in servers.iter() {
                // Extract the server address by removing the "http://" prefix.
                let server_address = server.replace("http://", "");
                // Attempt to connect to the server.
                match TcpStream::connect(&server_address).await {
                    // If successful, log that the server is up.
                    Ok(_) => info!("Server {} is up", server),
                    // If successful, log that the server is up.
                    Err(e) => warn!("Server {} is down: {}", server, e),
                }
            }
        }
    });

    // Define the service creation function.
    let make_svc = make_service_fn(move |_conn| {
        // Clone references to backend_servers and request_limits.
        let backend_servers = Arc::clone(&backend_servers);
        let request_limits = Arc::clone(&request_limits);
        // Asynchronously create the service.
        async move {
            // Return a service that proxies requests to backend servers.
            Ok::<_, Infallible>(service_fn(move |req| {
                proxy_request(req, backend_servers.clone(), request_limits.clone())
            }))
        }
    });

    // Create the HTTP server and bind it to the specified address.
    let server = Server::bind(&addr).serve(make_svc);

    // Start the server, handling any errors.
    if let Err(e) = server.await {
        // Print an error message if the server encounters an error.
        eprintln!("server error: {}", e);
    }
}

/// Proxy incoming HTTP requests to backend servers, handling rate limiting and logging.
async fn proxy_request(
    req: Request<Body>, // Accepts an incoming HTTP request.
    backend_servers: Arc<Mutex<Vec<String>>>, // Arc-wrapped Mutex for shared ownership of backend server URLs.
    request_limits: Arc<RwLock<HashMap<IpAddr, u64>>>, // Arc-wrapped RwLock for shared ownership of client request limits.
) -> Result<Response<Body>, hyper::Error> { // Returns a Result indicating success or a Hyper error.

    // Create a Hyper client for forwarding requests.
    let client = Client::new();
    
    // Extract the client's IP address from the request.
    let ip = req.extensions().get::<SocketAddr>().map_or_else(
        || IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
        |addr| addr.ip(),
    );

    // Acquire a write lock on request rate limits.
    let mut limits = request_limits.write().await;
    // Increment the request counter for the client's IP.
    let counter = limits.entry(ip).or_insert(0);
    *counter += 1;

    // Check if the client has exceeded the rate limit.
    if *counter > 50 {
        // Log a warning if there are too many requests from the client.
        warn!("Too Many Requests from {}", ip);
        // Return a 429 status response indicating too many requests.
        return Ok(Response::builder()
            .status(429)
            .body(Body::from("Too Many Requests\n"))
            .unwrap());
    }

    // Log the incoming request.
    info!("Request received from {}", ip);

    // Rotate backend servers and select the next one.
    let backend_server = {
        let mut servers = backend_servers.lock().await;
        let server = servers.remove(0);
        servers.push(server.clone());
        server
    };

    // Log the selected backend server.
    info!("Request sent to {}", backend_server);

    // Build the forwarded URI based on the selected backend server and the original request.
    let forwarded_uri = format!("{}{}", backend_server, req.uri().path_and_query().map_or("", |x| x.as_str()));
    // Build the forwarded request using the original request's method, URI, and body.
    let forwarded_req = Request::builder()
        .method(req.method())
        .uri(forwarded_uri)
        .body(req.into_body())
        .expect("Failed to create request");

    // Forward the request to the selected backend server and return the response.
    client.request(forwarded_req).await
}
