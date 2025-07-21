use actix_web::{App, HttpResponse, HttpServer, Responder, Result, middleware::Logger, web};
use std::io;

mod api;
mod db;
mod mappers;
mod models;
mod repositories;
mod services;
mod utils;

use db::client::PgClient;
use repositories::node_repository::NodeRepository;
use services::node_service::NodeService;

// Application state
struct AppState {
    node_service: NodeService,
}

// Nodes endpoint - syncs from API and returns database data
async fn get_nodes(data: web::Data<AppState>) -> Result<impl Responder> {
    println!("📡 Starting node sync process...");

    match data.node_service.sync_nodes().await {
        Ok(nodes) => {
            println!("✅ Successfully synced {} nodes to database", nodes.len());
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "nodes": nodes,
                "count": nodes.len(),
                "message": "Nodes successfully synced and retrieved"
            })))
        }
        Err(e) => {
            eprintln!("❌ Error syncing nodes: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to sync nodes",
                "message": e.to_string(),
                "details": format!("{:?}", e)
            })))
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    env_logger::init();
    dotenvy::dotenv().ok();

    println!("🚀 Starting Bitcoin Nodes API server...");
    println!("📡 Server will be available at: http://127.0.0.1:8080");
    println!("⚡ Nodes endpoint: http://127.0.0.1:8080/nodes");

    // Initialize database client
    let db_client = match PgClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to initialize database client: {}", e);
            eprintln!("Make sure DATABASE_URL is set and the database is running");
            std::process::exit(1);
        }
    };

    println!("✅ Database connection established");

    // Initialize repository and service
    let repository = NodeRepository::new(db_client);
    let node_service = NodeService::new(repository);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                node_service: node_service.clone(),
            }))
            .wrap(Logger::default())
            .route("/nodes", web::get().to(get_nodes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
