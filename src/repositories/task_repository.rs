use crate::models::task::Task;
use crate::database::Database;
use crate::error::AppError;
use chrono::Utc;
use sqlx::Row; // 添加这个导入来修复第二个错误

#[derive(Clone)]
pub struct TaskRepository {
    db: Database,
}

impl TaskRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub async fn create(&self, task: &Task) -> Result<Task, AppError> {
        tracing::debug!("Creating task with id: {}", task.id);
        
        sqlx::query(
            "INSERT INTO tasks (id, title, description, completed, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.completed)
        .bind(task.created_at)
        .bind(task.updated_at)
        .execute(&self.db.pool)
        .await?;
        
        tracing::info!("Task created successfully: {}", task.id);
        Ok(task.clone())
    }
    
    pub async fn find_all(&self) -> Result<Vec<Task>, AppError> {
        tracing::debug!("Fetching all tasks");
        
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at 
             FROM tasks ORDER BY created_at DESC"
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        tracing::info!("Found {} tasks", tasks.len());
        Ok(tasks)
    }
    
    pub async fn find_by_id(&self, id: &str) -> Result<Option<Task>, AppError> {
        tracing::debug!("Fetching task by id: {}", id);
        
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at 
             FROM tasks WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;
        
        match &task {
            Some(_) => tracing::debug!("Task found: {}", id),
            None => tracing::debug!("Task not found: {}", id),
        }
        
        Ok(task)
    }
    
    pub async fn update(&self, id: &str, title: Option<String>, description: Option<String>, completed: Option<bool>) -> Result<Option<Task>, AppError> {
        tracing::debug!("Updating task: {}", id);
        
        let now = Utc::now();
        
        // 根据提供的参数执行不同的更新查询
        match (title, description, completed) {
            (Some(title), Some(description), Some(completed)) => {
                sqlx::query("UPDATE tasks SET title = ?, description = ?, completed = ?, updated_at = ? WHERE id = ?")
                    .bind(&title)
                    .bind(&description)
                    .bind(completed)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (Some(title), Some(description), None) => {
                sqlx::query("UPDATE tasks SET title = ?, description = ?, updated_at = ? WHERE id = ?")
                    .bind(&title)
                    .bind(&description)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (Some(title), None, Some(completed)) => {
                sqlx::query("UPDATE tasks SET title = ?, completed = ?, updated_at = ? WHERE id = ?")
                    .bind(&title)
                    .bind(completed)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (None, Some(description), Some(completed)) => {
                sqlx::query("UPDATE tasks SET description = ?, completed = ?, updated_at = ? WHERE id = ?")
                    .bind(&description)
                    .bind(completed)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (Some(title), None, None) => {
                sqlx::query("UPDATE tasks SET title = ?, updated_at = ? WHERE id = ?")
                    .bind(&title)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (None, Some(description), None) => {
                sqlx::query("UPDATE tasks SET description = ?, updated_at = ? WHERE id = ?")
                    .bind(&description)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (None, None, Some(completed)) => {
                sqlx::query("UPDATE tasks SET completed = ?, updated_at = ? WHERE id = ?")
                    .bind(completed)
                    .bind(now)
                    .bind(id)
                    .execute(&self.db.pool)
                    .await?;
            }
            (None, None, None) => {
                // 没有字段需要更新
                return self.find_by_id(id).await;
            }
        }
        
        tracing::info!("Task updated successfully: {}", id);
        self.find_by_id(id).await
    }
    
    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        tracing::debug!("Deleting task: {}", id);
        
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(&self.db.pool)
            .await?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            tracing::info!("Task deleted successfully: {}", id);
        } else {
            tracing::warn!("Task not found for deletion: {}", id);
        }
        
        Ok(deleted)
    }
    
    pub async fn count(&self) -> Result<i64, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM tasks")
            .fetch_one(&self.db.pool)
            .await?;
            
        Ok(row.get("count"))
    }
    
    pub async fn find_by_status(&self, completed: bool) -> Result<Vec<Task>, AppError> {
        tracing::debug!("Fetching tasks by status: completed={}", completed);
        
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, title, description, completed, created_at, updated_at 
             FROM tasks WHERE completed = ? ORDER BY created_at DESC"
        )
        .bind(completed)
        .fetch_all(&self.db.pool)
        .await?;
        
        tracing::info!("Found {} tasks with completed={}", tasks.len(), completed);
        Ok(tasks)
    }
}