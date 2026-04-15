# Rust Web Starter

基于 Axum + SeaORM + PostgreSQL 的 Rust Web 全栈项目脚手架。

## 技术栈

- Axum 0.8 (Web 框架)
- SeaORM 1.1 (ORM)
- PostgreSQL (数据库)
- Tokio (异步运行时)
- Vite + React (前端，静态资源嵌入二进制)

## 快速启动

### 1. 准备数据库

确保 PostgreSQL 已运行，创建 schema 并导入表结构：

```bash
# 创建 schema
psql -d postgres -c "CREATE SCHEMA IF NOT EXISTS demo;"

# 授予权限
psql -d postgres -c "GRANT ALL ON SCHEMA demo TO postgres;"

# 设置 search_path 并导入表结构和测试数据
psql -d postgres -c "SET search_path TO demo;" -f schema.sql

# 如果表已存在于 public schema，需要迁移到 demo
# psql -d postgres -c "ALTER TABLE public.sys_user SET SCHEMA demo;"
```

### 2. 修改配置（可选）

编辑 `application.yaml`：

```yaml
server:
  port: 3001

database:
  host: 127.0.0.1
  port: 5432
  user: postgres
  password: 12345678
  database: postgres
  schema: demo
```

也可通过环境变量覆盖，前缀为 `APP_`，例如：

```bash
export APP_SERVER__PORT=8080
export APP_DATABASE__PASSWORD=your_password
```

### 3. 构建前端（可选）

```bash
cd web
npm install
npm run build
cd ..
```

### 4. 启动服务

```bash
# 开发模式
cargo run

# Release 模式
cargo run --release
```

服务启动后访问：http://localhost:3001

### 5. 测试账号

| 账号 | 密码 |
|------|------|
| admin | 123456 |
| lisi | 123456 |
| zhaoliu | 123456 |

## 交叉编译

```bash
# Linux
cross build --release --target x86_64-unknown-linux-musl

# Windows
cross build --release --target x86_64-pc-windows-gnu
```

## 项目结构

```
src/
├── main.rs          # 入口
├── app/             # 核心（服务器、数据库、中间件、错误处理）
├── api/             # 路由和接口处理
├── config/          # 配置加载
├── entity/          # SeaORM 实体
└── web/             # 静态资源服务
web/                 # 前端项目 (Vite + React)
application.yaml     # 应用配置
schema.sql           # 数据库建表脚本
```
