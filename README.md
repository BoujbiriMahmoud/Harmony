# ⚖️ Harmony Load Balancer 


Welcome to Harmony Load Balancer! This project serves as a simple load balancer for distributing incoming HTTP requests among multiple backend servers.

## 📖 Overview

**Author:** Mahmoud BOUJBIRI

This project, authored by ***Mahmoud BOUJBIRI***, serves as the final exam submission for the Rust Programming Language Module at ESGI Paris. As part of the acknowledgments, special thanks are extended to Mr. BOURY, the Rust instructor, for his dedication and passion throughout the course.

### 🌟 Project Inspiration

The inspiration for this load balancer stems from _CS 110L_ Stanford's Balancebeam project. Choosing this project for the final exam was a natural decision influenced by my background in network and telecommunications. The opportunity to explore a project associated with Stanford further motivated my selection. I aim to deliver an enjoyable experience in load balancing through this endeavor.

### ⚠️ Disclaimer

This project is submitted as part of academic coursework and serves as a demonstration of skills acquired during the Rust Programming Language Module at **ESGI Paris**. The primary focus is on educational purposes and practical application of concepts learned in the course. As such, it may not cover all edge cases, security considerations, or production-level optimizations.

**The author** and **ESGI Paris** are not liable for any misuse, unintended consequences, or damages arising from the use or modification of this code in a non-educational or non-experimental context. By using this software, ***you agree*** that the author and ESGI Paris are not responsible for any disruptions, losses, or harm that may occur.

**Caution:** This software is provided "as-is," without any warranty of any kind, express or implied. You are responsible for understanding the risks involved in using and modifying this code.

### 🎯 Purpose

The Load Balancer is designed to evenly distribute incoming HTTP requests across a group of backend servers. It monitors the health of the servers, implements basic rate limiting, and logs relevant information.

### 🧩 Components

1. **Server Health Checker:** Periodically checks the health of backend servers by attempting to establish a TCP connection.
2. **Request Rate Limiter:** Limits the number of requests from a single client to prevent abuse.
3. **Logger:** Provides detailed logs of incoming requests, server health, and any issues encountered.

## 🏗️ Code Structure

The code is organized into several main sections:

1. **Dependencies:** The external crates and libraries used in the project are imported, including Hyper for HTTP handling and Tokio for asynchronous tasks.
2. **Main Function (`main`):** The entry point of the application. It initializes the logger, defines backend servers and rate limits, and starts the server. Additionally, it spawns a background task to check the health of backend servers.
3. **Service Creation Function (`make_svc`):** Creates the service to handle incoming requests. It utilizes the `service_fn` to define a closure that proxies requests to backend servers.
4. **HTTP Server Setup:** Configures and starts the HTTP server, binding it to a specified address.
5. **Proxy Request Function (`proxy_request`):** Handles incoming HTTP requests. It checks rate limits, logs information, rotates backend servers, and forwards requests.

## 🚀 Usage

### Configuration

- The load balancer is set to run on `127.0.0.1:8080` by default. You can change this in the `addr` variable within the `main` function.

### Backend Servers

- Backend servers are specified in the `backend_servers` vector within the `main` function. Add or remove server URLs as needed.

### Rate Limiting

- The load balancer implements a basic rate limit of **50** requests per client IP. Adjust the limit in the `proxy_request` function if needed.

### Logging

- Logs provide information about incoming requests, selected backend servers, and server health. To display logs run the following command when starting the main.rs:
```
RUST_LOG=info cargo run --bin harmony
```

## 🏃 Running the Load Balancer

1. Clone the repository:
```
https://github.com/BoujbiriMahmoud/Harmony/
```
3. Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.
5. Start the webservers individually by running the following commands:  
- Server 1:
```
cargo run --bin server1
```
- Server 2:
```
cargo run --bin server2
```
- Server 3:
```
cargo run --bin server3
```
4. Start the standard load balancer by running the following command:
Harmony Load Balancer:
```   
cargo run --bin harmony
```
5. Start the load balancer with logs by running the following command:
```
RUST_LOG=info cargo run --bin harmony
```

## 🧪 Testing

To test the load balancer, you can use tools like [curl](https://curl.se/) or [Postman](https://www.postman.com/):

### Curl Testing

To test the load balancer using [curl](https://curl.se/):

1. Send a single HTTP request to the load balancer:
```
curl http://127.0.0.1:8080/
```
2. Send multiple requests (100 requests):
```
for i in {1..100}; do curl http://127.0.0.1:8080/ & done
```

### Health Update Testing
After starting the load balancer ***with logs*** and the servers, kill a back-end server and observe the load balancer detecting the server failure and adjusting accordingly. 

### Stress Testing with Apache Benchmark(ab)
1. Install Apache Benchmark:
```
sudo apt-get install apache2-utils
```
2. Run a stress test with 100 requests, concurrency of 10:
```
ab -n 100 -c 10 http://127.0.0.1:8080/
```
Observe the load balancer handling the stress test, and review the performance metrics provided by Apache Benchmark.

***Note:*** Adjust the number of requests and concurrency as needed for your specific testing scenario.

Feel free to explore and modify the code to suit your specific requirements. If you have any questions or encounter issues, refer to the source code comments or consult the documentation for the used libraries.

🎉 **We wish you happy balancing!** 🎉
