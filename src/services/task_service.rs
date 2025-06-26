use crate::models::task::{Task, CreateTaskRequest, UpdateTaskRequest};
use crate::repositories::task_repository::TaskRepository;
use crate::error::AppError;

#[derive(Clone)]
pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(repository: TaskRepository) -> Self {
        Self { repository }
    }
    
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task, AppError> {
        // 验证请求数据
        request.validate()
            .map_err(|e| AppError::BadRequest(e))?;
        
        let task = Task::new(request.title.trim().to_string(), 
                           request.description.map(|d| d.trim().to_string()));
        
        self.repository.create(&task).await
    }
    
    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, AppError> {
        self.repository.find_all().await
    }
    
    pub async fn get_task_by_id(&self, id: &str) -> Result<Option<Task>, AppError> {
        if id.trim().is_empty() {
            return Err(AppError::BadRequest("Task ID cannot be empty".to_string()));
        }
        
        self.repository.find_by_id(id).await
    }
    
    pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Option<Task>, AppError> {
        if id.trim().is_empty() {
            return Err(AppError::BadRequest("Task ID cannot be empty".to_string()));
        }
        
        // 验证更新数据
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(AppError::BadRequest("Title cannot be empty".to_string()));
            }
            if title.len() > 200 {
                return Err(AppError::BadRequest("Title cannot exceed 200 characters".to_string()));
            }
        }
        
        if let Some(ref description) = request.description {
            if description.len() > 1000 {
                return Err(AppError::BadRequest("Description cannot exceed 1000 characters".to_string()));
            }
        }
        
        let title = request.title.map(|t| t.trim().to_string());
        let description = request.description.map(|d| d.trim().to_string());
        
        self.repository.update(id, title, description, request.completed).await
    }
    
    pub async fn delete_task(&self, id: &str) -> Result<bool, AppError> {
        if id.trim().is_empty() {
            return Err(AppError::BadRequest("Task ID cannot be empty".to_string()));
        }
        
        self.repository.delete(id).await
    }
    
    pub async fn get_tasks_by_status(&self, completed: bool) -> Result<Vec<Task>, AppError> {
        self.repository.find_by_status(completed).await
    }
    
    pub async fn get_task_statistics(&self) -> Result<(i64, i64, i64), AppError> {
        let total = self.repository.count().await?;
        let completed = self.repository.find_by_status(true).await?.len() as i64;
        let pending = total - completed;
        
        Ok((total, completed, pending))
    }
}