use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub success: bool,
    pub data: Option<Task>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub success: bool,
    pub data: Vec<Task>,
    pub total: usize,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update(&mut self, title: Option<String>, description: Option<String>, completed: Option<bool>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(description) = description {
            self.description = Some(description);
        }
        if let Some(completed) = completed {
            self.completed = completed;
        }
        self.updated_at = Utc::now();
    }
}

impl CreateTaskRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.title.len() > 200 {
            return Err("Title cannot exceed 200 characters".to_string());
        }
        if let Some(desc) = &self.description {
            if desc.len() > 1000 {
                return Err("Description cannot exceed 1000 characters".to_string());
            }
        }
        Ok(())
    }
}