use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::handlers::task_handler::{
    create_task, get_all_tasks, get_task, update_task, delete_task,
    get_task_statistics, health_check,
};
use crate::services::task_service::TaskService;

pub fn create_task_routes(service: TaskService) -> Router {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/stats", get(get_task_statistics))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", put(update_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(service)
}

pub fn create_health_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
}