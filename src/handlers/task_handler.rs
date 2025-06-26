use axum::{
    extract::{Path, Query, State},
    response::Json,
    Json as JsonBody,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::models::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::task_service::TaskService;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    pub completed: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub total: i64,
    pub completed: i64,
    pub pending: i64,
}

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "service": "todo-list-api"
    }))
}

pub async fn create_task(
    State(service): State<TaskService>,
    JsonBody(request): JsonBody<CreateTaskRequest>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Creating new task: {}", request.title);
    
    let task = service.create_task(request).await?;
    
    Ok(Json(json!({
        "success": true,
        "data": task,
        "message": "Task created successfully"
    })))
}

pub async fn get_all_tasks(
    State(service): State<TaskService>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Fetching tasks with query: {:?}", query);
    
    let tasks = if let Some(completed) = query.completed {
        service.get_tasks_by_status(completed).await?
    } else {
        service.get_all_tasks().await?
    };
    
    // 应用分页
    let total = tasks.len();
    let tasks = if let (Some(limit), Some(offset)) = (query.limit, query.offset) {
        tasks.into_iter()
            .skip(offset)
            .take(limit)
            .collect::<Vec<_>>()
    } else {
        tasks
    };
    
    Ok(Json(json!({
        "success": true,
        "data": tasks,
        "total": total,
        "count": tasks.len()
    })))
}

pub async fn get_task(
    State(service): State<TaskService>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Fetching task by id: {}", id);
    
    match service.get_task_by_id(&id).await? {
        Some(task) => Ok(Json(json!({
            "success": true,
            "data": task
        }))),
        None => Err(AppError::NotFound(format!("Task with id '{}' not found", id))),
    }
}

pub async fn update_task(
    State(service): State<TaskService>,
    Path(id): Path<String>,
    JsonBody(request): JsonBody<UpdateTaskRequest>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Updating task: {}", id);
    
    match service.update_task(&id, request).await? {
        Some(task) => Ok(Json(json!({
            "success": true,
            "data": task,
            "message": "Task updated successfully"
        }))),
        None => Err(AppError::NotFound(format!("Task with id '{}' not found", id))),
    }
}

pub async fn delete_task(
    State(service): State<TaskService>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Deleting task: {}", id);
    
    let deleted = service.delete_task(&id).await?;
    if deleted {
        Ok(Json(json!({
            "success": true,
            "message": "Task deleted successfully"
        })))
    } else {
        Err(AppError::NotFound(format!("Task with id '{}' not found", id)))
    }
}

pub async fn get_task_statistics(
    State(service): State<TaskService>,
) -> Result<Json<Value>, AppError> {
    tracing::info!("Fetching task statistics");
    
    let (total, completed, pending) = service.get_task_statistics().await?;
    
    Ok(Json(json!({
        "success": true,
        "data": {
            "total": total,
            "completed": completed,
            "pending": pending,
            "completion_rate": if total > 0 { 
                (completed as f64 / total as f64 * 100.0).round() 
            } else { 
                0.0 
            }
        }
    })))
}