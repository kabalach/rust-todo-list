use sqlx::{SqlitePool, migrate::MigrateDatabase, Row};
use anyhow::Result;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        // 如果数据库不存在则创建
        if !sqlx::Sqlite::database_exists(database_url).await? {
            tracing::info!("Creating database: {}", database_url);
            sqlx::Sqlite::create_database(database_url).await?;
        }
        
        let pool = SqlitePool::connect(database_url).await?;
        
        Ok(Self { pool })
    }
    
    pub async fn migrate(&self) -> Result<()> {
        tracing::info!("Running database migrations...");
        
        // 创建任务表
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                completed BOOLEAN NOT NULL DEFAULT FALSE,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        // 创建索引
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed)")
            .execute(&self.pool)
            .await?;
            
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)")
            .execute(&self.pool)
            .await?;
        
        tracing::info!("Database migrations completed");
        Ok(())
    }
    
    pub async fn health_check(&self) -> Result<bool> {
        let row = sqlx::query("SELECT 1 as health")
            .fetch_one(&self.pool)
            .await?;
            
        let health: i32 = row.get("health");
        Ok(health == 1)
    }
}