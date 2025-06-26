use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::fmt;
use std::env;

mod database;
mod error;
mod models;
mod repositories;
mod services;
mod handlers;
mod routes;

use crate::database::Database;
use crate::routes::create_routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    tracing::info!("🚀 Starting Todo List API Server");
    
    // 初始化数据库
    tracing::info!("🔧 Initializing database...");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todo.db".to_string());
    let database = Database::new(&database_url).await?;
    database.migrate().await?;
    tracing::info!("✅ Database initialized successfully");
    
    // 配置CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);
    
    // 创建静态文件服务
    let static_files = ServeDir::new("static");
    
    // 创建路由
    let app = create_routes(database)
        .nest_service("/static", static_files)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    
    // 启动服务器
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    tracing::info!("🌐 Server starting on http://{}", addr);
    tracing::info!("📋 API endpoints available:");
    tracing::info!("   GET    /                   - API information");
    tracing::info!("   POST   /api/tasks          - Create new task");
    tracing::info!("   GET    /api/tasks          - Get all tasks");
    tracing::info!("   GET    /api/tasks/:id      - Get task by ID");
    tracing::info!("   PUT    /api/tasks/:id      - Update task");
    tracing::info!("   DELETE /api/tasks/:id      - Delete task");
    tracing::info!("   GET    /api/tasks/stats    - Task statistics");
    tracing::info!("   GET    /health             - Health check");
    tracing::info!("   GET    /static/*           - Static files");
    tracing::info!("");
    tracing::info!("💡 Try opening http://localhost:{}/static/index.html for the web interface", port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}