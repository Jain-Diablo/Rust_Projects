use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define the directory to serve static files from
    let static_files = warp::fs::dir("./public");

    // Serve static files on the root route
    let routes = static_files;

    // Define the server address and port
    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    println!("Server running at http://{}/", addr);

    // Start the warp server
    warp::serve(routes).run(addr).await;
}