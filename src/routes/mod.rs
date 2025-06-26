use axum::{Router, response::{Json, Redirect}};
use serde_json::json;
use crate::database::Database;
use crate::repositories::task_repository::TaskRepository;
use crate::services::task_service::TaskService;

pub mod task_routes;

pub fn create_routes(database: Database) -> Router {
    let task_repository = TaskRepository::new(database.clone());
    let task_service = TaskService::new(task_repository);
    
    Router::new()
        // 将根路径重定向到静态HTML文件
        .route("/", axum::routing::get(|| async { Redirect::permanent("/static/index.html") }))
        // API信息端点
        .route("/api", axum::routing::get(api_info))
        .nest("/api", task_routes::create_task_routes(task_service))
        .merge(task_routes::create_health_routes())
}

// API 信息端点
async fn api_info() -> Json<serde_json::Value> {
    Json(json!({
        "name": "Todo List API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "A RESTful API for managing todo tasks",
        "endpoints": {
            "health": "GET /health",
            "tasks": {
                "list": "GET /api/tasks",
                "create": "POST /api/tasks",
                "get": "GET /api/tasks/:id",
                "update": "PUT /api/tasks/:id",
                "delete": "DELETE /api/tasks/:id",
                "stats": "GET /api/tasks/stats"
            }
        },
        "documentation": {
            "create_task": {
                "method": "POST",
                "path": "/api/tasks",
                "body": {
                    "title": "string (required)",
                    "description": "string (optional)"
                }
            },
            "update_task": {
                "method": "PUT", 
                "path": "/api/tasks/:id",
                "body": {
                    "title": "string (optional)",
                    "description": "string (optional)",
                    "completed": "boolean (optional)"
                }
            },
            "query_parameters": {
                "GET /api/tasks": {
                    "completed": "boolean (optional) - filter by completion status",
                    "limit": "number (optional) - limit results",
                    "offset": "number (optional) - skip results"
                }
            }
        }
    }))
}