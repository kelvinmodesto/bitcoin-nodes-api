use actix_web::{App, HttpServer, Responder, get, web};
mod models;

// #[get("/nodes")]
// async fn getNodes() -> {
//     web::Json()
// }

#[tokio::main]
async fn main() {
    let nodes_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";
}
