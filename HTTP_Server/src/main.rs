use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct JsonResponse {
    message: String,
}

#[derive(Deserialize, Debug)]
struct PostRequestData {
    url: String,
}

async fn handle_request(data: web::Json<PostRequestData>) -> impl Responder {
    // Print the request details to the console
    println!("Received request: {:?}", data.url);

    let static_files = warp::fs::dir(data.url.to_string());

    // let routes = static_files;

    let addr: SocketAddr = ([127, 0, 0, 2], 1234).into();

    warp::serve(static_files).run(addr).await;

    

    // Respond with a JSON object
    web::Json(JsonResponse {
        message: "Server running at http://127.0.0.2:1234/".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server_address = "127.0.0.1:8080";
    println!("Server running on http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/api/endpoint", web::post().to(handle_request)) // Define the endpoint
    })
    .bind(server_address)?
    .run()
    .await
}
