# Rust Todo List API 

这是一个使用 Rust 语言和 Axum 框架构建的简单待办事项 (Todo List) 应用。

它提供了一套完整的 REST API，可以用来创建、读取、更新和删除任务，并附带一个简单的网页界面。

## 🚀 主要功能

- **创建任务**: 添加新的待办事项。
- **查看任务**: 获取所有任务或单个任务的详情。
- **更新任务**: 修改任务的标题、描述或完成状态。
- **删除任务**: 从列表中移除任务。
- **任务统计**: 查看总任务数、已完成数和未完成数。
- **网页访问**: 提供一个简单的 HTML 页面来和 API 交互。

## 🛠️ 使用的技术

- **后端语言**: Rust
- **Web 框架**: Axum (一个现代、高效的 Rust Web 框架)
- **数据库**: SQLite (一个轻量级的本地文件数据库)
- **数据库工具**: SQLx (用于在 Rust 中安全地操作数据库)

## 📦 如何运行

### 准备工作

你需要先在你的电脑上安装 [Rust](https://www.rust-lang.org/tools/install)。

### 步骤

1.  **下载代码**
    ```bash
    git clone https://github.com/kabalach/rust-todo-list.git
    cd rust-todo-list
    ```

2.  **编译并运行**
    ```bash
    cargo run
    ```
    第一次运行会需要一些时间来下载和编译依赖库。

3.  **访问应用**
    - **网页界面**: 打开浏览器，访问 `http://localhost:3000`
    - **API 健康检查**: 访问 `http://localhost:3000/health`

## 📋 API 接口说明

你可以使用 Postman 或 curl 等工具来测试这些接口。

| 方法   | 路径                  | 描述                         | 示例 Body (请求体)                                |
| :----- | :-------------------- | :--------------------------- | :------------------------------------------------ |
| `POST` | `/api/tasks`          | 创建新任务                   | `{ "title": "学习 Rust" }`                        |
| `GET`  | `/api/tasks`          | 获取所有任务                 | -                                                 |
| `GET`  | `/api/tasks/:id`      | 获取单个任务                 | -                                                 |
| `PUT`  | `/api/tasks/:id`      | 更新任务                     | `{ "completed": true }`                           |
| `DELETE`| `/api/tasks/:id`      | 删除任务                     | -                                                 |
| `GET`  | `/api/tasks/stats`    | 获取任务统计                 | -                                                 |

**注意**:
- `:id` 是任务的唯一ID，例如 `.../api/tasks/1a2b3c4d...`。
- `POST` 和 `PUT` 请求需要设置 `Content-Type` 为 `application/json`。

## 📄 许可证

本项目使用 [MIT License](LICENSE)。