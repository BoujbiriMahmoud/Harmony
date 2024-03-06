use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Create a TcpListener to accept incoming connections
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // Print a message indicating that the load balancer is running
    println!("Load balancer is running on http://127.0.0.1:8080");

    // Create a mutable list of backend servers
    let backend_servers = Arc::new(Mutex::new(vec![
        "127.0.0.1:8001".to_string(),
        "127.0.0.1:8002".to_string(),
        "127.0.0.1:8003".to_string(),
    ]));

    // Start accepting and handling incoming connections
    while let Ok((stream, _)) = listener.accept().await {
        // Clone the Arc to pass to the handle_request function
        let backend_servers_clone = Arc::clone(&backend_servers);

        // Spawn a new asynchronous task to handle each incoming connection
        tokio::spawn(handle_request(stream, backend_servers_clone));
    }
}

async fn handle_request(mut stream: TcpStream, backend_servers: Arc<Mutex<Vec<String>>>) {
    // Read the incoming request
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).await;

    // Choose a backend server (simple round-robin strategy)
    let backend_server = {
        let mut servers = backend_servers.lock().await;
        let server = servers.pop().unwrap();
        servers.push(server.clone());
        server
    };

    // Forward the request to the chosen backend server
    let mut backend_stream = TcpStream::connect(backend_server).await.unwrap();
    backend_stream.write_all(&buffer).await.unwrap();

    // Read the response from the backend server
    let mut response = Vec::new();
    backend_stream.read_to_end(&mut response).await.unwrap();

    // Send the response back to the client
    stream.write_all(&response).await.unwrap();
}

