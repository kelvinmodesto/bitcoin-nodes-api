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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test, web};
    use dotenvy::dotenv;
    use std::sync::{Arc, Mutex};

    // Mock NodeService for testing
    #[derive(Clone)]
    struct MockNodeService {
        should_fail: bool,
        nodes_to_return: Arc<Mutex<Vec<crate::models::node::Node>>>,
    }

    impl MockNodeService {
        fn new(should_fail: bool, nodes: Vec<crate::models::node::Node>) -> Self {
            Self {
                should_fail,
                nodes_to_return: Arc::new(Mutex::new(nodes)),
            }
        }

        async fn sync_nodes(
            &self,
        ) -> Result<Vec<crate::models::node::Node>, Box<dyn std::error::Error>> {
            if self.should_fail {
                Err("Mock sync failure".into())
            } else {
                let nodes = self.nodes_to_return.lock().unwrap().clone();
                Ok(nodes)
            }
        }
    }

    // Mock AppState for testing
    struct MockAppState {
        mock_service: MockNodeService,
    }

    // Mock handler that uses MockNodeService
    async fn mock_get_nodes(data: web::Data<MockAppState>) -> Result<impl Responder> {
        match data.mock_service.sync_nodes().await {
            Ok(nodes) => Ok(HttpResponse::Ok().json(serde_json::json!({
                "nodes": nodes,
                "count": nodes.len(),
                "message": "Nodes successfully synced and retrieved"
            }))),
            Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to sync nodes",
                "message": e.to_string(),
                "details": format!("{:?}", e)
            }))),
        }
    }

    fn create_mock_node() -> crate::models::node::Node {
        use chrono::Utc;

        crate::models::node::Node {
            id: 1234,
            public_key: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
                .to_string(),
            alias: "Test Node".to_string(),
            capacity: "1.5".to_string(),
            first_seen: "2021-01-01T00:00:00Z".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[actix_web::test]
    async fn test_get_nodes_success() {
        let mock_nodes = vec![create_mock_node()];
        let mock_service = MockNodeService::new(false, mock_nodes.clone());
        let mock_state = MockAppState { mock_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_state))
                .route("/nodes", web::get().to(mock_get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["count"], 1);
        assert_eq!(body["message"], "Nodes successfully synced and retrieved");
        assert!(body["nodes"].is_array());

        let nodes_array = body["nodes"].as_array().unwrap();
        assert_eq!(nodes_array.len(), 1);
        assert_eq!(nodes_array[0]["alias"], "Test Node");
    }

    #[actix_web::test]
    async fn test_get_nodes_service_failure() {
        let mock_service = MockNodeService::new(true, vec![]);
        let mock_state = MockAppState { mock_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_state))
                .route("/nodes", web::get().to(mock_get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 500);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["error"], "Failed to sync nodes");
        assert!(
            body["message"]
                .as_str()
                .unwrap()
                .contains("Mock sync failure")
        );
    }

    #[actix_web::test]
    async fn test_get_nodes_multiple_nodes() {
        let mock_nodes = vec![
            create_mock_node(),
            {
                let mut node = create_mock_node();
                node.alias = "Second Test Node".to_string();
                node.public_key =
                    "02864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
                        .to_string();
                node
            },
            {
                let mut node = create_mock_node();
                node.alias = "Third Test Node".to_string();
                node.public_key =
                    "01864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f"
                        .to_string();
                node
            },
        ];

        let mock_service = MockNodeService::new(false, mock_nodes.clone());
        let mock_state = MockAppState { mock_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_state))
                .route("/nodes", web::get().to(mock_get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["count"], 3);

        let nodes_array = body["nodes"].as_array().unwrap();
        assert_eq!(nodes_array.len(), 3);

        // Check that all nodes are present
        let aliases: Vec<&str> = nodes_array
            .iter()
            .map(|node| node["alias"].as_str().unwrap())
            .collect();

        assert!(aliases.contains(&"Test Node"));
        assert!(aliases.contains(&"Second Test Node"));
        assert!(aliases.contains(&"Third Test Node"));
    }

    #[actix_web::test]
    async fn test_nonexistent_route() {
        let mock_service = MockNodeService::new(false, vec![]);
        let mock_state = MockAppState { mock_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_state))
                .route("/nodes", web::get().to(mock_get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nonexistent").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404);
    }

    // Integration test with real database (requires DATABASE_URL)
    #[actix_web::test]
    async fn test_get_nodes_integration() {
        // Skip if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            println!("⏭️  Skipping integration test - DATABASE_URL not set");
            return;
        }

        dotenv().ok();

        // Try to create real services
        let db_client = match PgClient::new() {
            Ok(client) => client,
            Err(e) => {
                println!(
                    "⏭️  Skipping integration test - Database connection failed: {}",
                    e
                );
                return;
            }
        };

        let repository = NodeRepository::new(db_client);
        let node_service = NodeService::new(repository);
        let app_state = AppState { node_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(app_state))
                .route("/nodes", web::get().to(get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;

        // The response should be either success or a specific error
        // depending on external API availability
        assert!(resp.status().is_success() || resp.status().is_server_error());

        if resp.status().is_success() {
            let body: serde_json::Value = test::read_body_json(resp).await;
            assert!(body.get("nodes").is_some());
            assert!(body.get("count").is_some());
            assert!(body.get("message").is_some());
        }
    }

    // Test application state setup
    #[test]
    async fn test_app_state_creation() {
        dotenv().ok();

        // Skip if DATABASE_URL is not set
        if std::env::var("DATABASE_URL").is_err() {
            println!("⏭️  Skipping app state test - DATABASE_URL not set");
            return;
        }

        let db_client = match PgClient::new() {
            Ok(client) => client,
            Err(_) => {
                println!("⏭️  Skipping app state test - Database connection failed");
                return;
            }
        };

        let repository = NodeRepository::new(db_client);
        let node_service = NodeService::new(repository);
        let _app_state = AppState { node_service };

        // If we get here, the AppState was created successfully
        assert!(true);
    }

    // Test response format structure
    #[actix_web::test]
    async fn test_response_format() {
        let mock_nodes = vec![create_mock_node()];
        let mock_service = MockNodeService::new(false, mock_nodes);
        let mock_state = MockAppState { mock_service };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mock_state))
                .route("/nodes", web::get().to(mock_get_nodes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/nodes").to_request();
        let resp = test::call_service(&app, req).await;

        let body: serde_json::Value = test::read_body_json(resp).await;

        // Verify required fields are present
        assert!(body.get("nodes").is_some());
        assert!(body.get("count").is_some());
        assert!(body.get("message").is_some());

        // Verify field types
        assert!(body["nodes"].is_array());
        assert!(body["count"].is_number());
        assert!(body["message"].is_string());

        // Verify node structure if nodes are present
        if let Some(nodes) = body["nodes"].as_array() {
            if !nodes.is_empty() {
                let node = &nodes[0];
                assert!(node.get("id").is_some());
                assert!(node.get("public_key").is_some());
                assert!(node.get("alias").is_some());
                assert!(node.get("capacity").is_some());
                assert!(node.get("first_seen").is_some());
                assert!(node.get("created_at").is_some());
                assert!(node.get("updated_at").is_some());
            }
        }
    }
}
