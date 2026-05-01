# CLAUDE.md

本文件为 Claude/Codex 等 AI 编程代理在本仓库协作时的工作说明。

## 1. 项目概览

- 项目类型: Rust 全栈脚手架（后端 API + 前端 SPA）
- 后端技术栈: Axum + Tokio + SeaORM + PostgreSQL
- 前端技术栈: Vite + React + TypeScript + Mantine + TanStack Router/Query
- 静态资源策略: 前端构建产物位于 `web/dist`，通过 `rust-embed` 打包进后端二进制并由 Axum 提供

## 2. 仓库结构

- `src/main.rs`: 程序入口，调用 `app::run(api::create_router())`
- `src/app/`: 应用基础设施
- `src/app/server.rs`: HTTP 服务器启动逻辑
- `src/app/database.rs`: 数据库连接初始化
- `src/app/error.rs`: 统一错误模型
- `src/app/middleware.rs`: 中间件（含鉴权层）
- `src/app/auth.rs`: JWT 能力
- `src/api/`: 路由与处理器
- `src/api/auth.rs`: 登录、用户信息
- `src/api/user.rs`: 用户 CRUD + 分页查询
- `src/config/`: 配置加载（`application.yaml` + `APP_` 环境变量）
- `src/entity/`: SeaORM 实体
- `src/web/mod.rs`: 前端静态资源和 `index.html` 回退处理
- `web/`: 前端工程
- `schema.sql`: PostgreSQL 初始化脚本
- `application.yaml`: 默认配置

## 3. 本地开发命令

### 3.1 后端

- 启动开发服务: `cargo run`
- Release 启动: `cargo run --release`
- 常规检查: `cargo check`
- 格式化: `cargo fmt`
- Lint: `cargo clippy --all-targets --all-features -D warnings`

说明: 当前仓库根目录缺少 `Cargo.toml`（仅存在 `Cargo.lock`）。若需执行 Rust 构建相关命令，请先补全或恢复 `Cargo.toml`。

### 3.2 前端

在 `web/` 目录执行:

- 安装依赖: `npm install`（或 `pnpm install`）
- 本地开发: `npm run dev`
- 构建: `npm run build`
- 预览: `npm run preview`

Node 版本要求: `>=22.x`（见 `web/package.json`）。

## 4. 配置与环境变量

配置来源:

1. `application.yaml`
2. 环境变量（前缀 `APP_`）

映射示例:

- `APP_SERVER__PORT=8080`
- `APP_DATABASE__HOST=127.0.0.1`
- `APP_DATABASE__PASSWORD=your_password`

数据库关键配置:

- `database.host`
- `database.port`
- `database.user`
- `database.password`
- `database.database`
- `database.schema`

## 5. API 与路由行为

- API 前缀: `/api`
- 用户接口: `/api/users`
- 认证接口: `/api/auth/login`、`/api/auth/user-info`
- 鉴权策略:
- `/api/users/**` 默认启用鉴权中间件
- `/api/auth/login` 无需鉴权
- `/api/auth/user-info` 需要鉴权
- 静态资源: `/static/{*file}`
- SPA 回退: 未命中 API/静态资源时，`GET` 回退到 `index.html`

## 6. 数据库初始化

参考 `README.md`:

1. 创建 schema（示例: `demo`）
2. 导入 `schema.sql`
3. 确保应用配置中的 `database.schema` 与实际一致

## 7. 代码修改约定（给 AI 代理）

- 优先做最小改动，避免无关重构
- 变更 API 时同步检查:
- `src/api/*`
- `src/app/error.rs`（错误响应）
- `src/app/valid.rs` / `src/app/validation.rs`（参数校验）
- 涉及认证时同步检查:
- `src/app/auth.rs`
- `src/app/middleware.rs`
- 涉及用户表结构/字段时同步检查:
- `src/entity/sys_user.rs`
- `schema.sql`
- 前端调用层 `web/src/apis/*`
- 所有密码必须走 `encode_password` / `verify_password`，禁止明文存储或比较
- 新增后端接口后，确认前端路由与 API 调用是否需要同步变更

## 8. 提交前检查清单

- Rust 侧:
- `cargo fmt`
- `cargo check`
- `cargo clippy --all-targets --all-features -D warnings`
- 前端侧（如有改动）:
- `npm run build`（在 `web/`）
- 数据库相关改动:
- `schema.sql` 与实体定义保持一致
- 行为相关改动:
- 至少手动验证登录、用户查询/增删改、静态资源访问

## 9. 已知风险与注意事项

- 当前仓库缺少 `Cargo.toml`，会阻塞 Rust 构建、检查与测试命令
- JWT token 当前在登录日志中有输出（`src/api/auth.rs`），生产环境建议移除或脱敏
- 前端依赖管理文件同时存在 `package-lock.json` 与 `pnpm-lock.yaml`，建议团队统一包管理器
