# curd-rs 项目协作规则（rules.md）

本文件定义本仓库的工程协作规范，适用于人类开发者与 AI 代理。
目标是：小步快改、边界清晰、接口稳定、可回归验证。

## 1. 项目定位与技术边界

- 后端：Rust + Axum + Tokio + SeaORM + PostgreSQL。
- 前端：Vite + React + TypeScript + Mantine + TanStack Router/Query。
- 部署形态：前端 `web/dist` 通过 `rust-embed` 打包进后端二进制，由 Axum 提供静态资源与 SPA 回退。
- API 风格：统一以 `/api` 为前缀，响应体结构为 `ApiResponse<T>`。

不允许在本仓库引入与现有栈冲突的第二套主框架（例如新增另一套后端 Web 框架或 ORM）。

## 2. 目录职责与改动边界

- `src/main.rs`：仅负责启动，不放业务逻辑。
- `src/app/`：基础设施层（server、db、middleware、error、auth、response、validation）。
- `src/api/`：路由与 handler，承载业务编排，不承载基础设施初始化。
- `src/entity/`：SeaORM 实体定义，需与数据库 schema 同步。
- `src/config/`：配置加载与配置类型。
- `src/web/`：静态资源服务与 SPA 回退。
- `web/src/apis/`：前端接口调用层。

改动规则：
- 新增 API 时，必须同时评估并同步以下模块是否需要调整：
  - 后端路由：`src/api/mod.rs` 与对应子模块
  - 错误映射：`src/app/error.rs`
  - 参数校验：`src/app/valid.rs` / `src/app/validation.rs`
  - 前端调用：`web/src/apis/*`
- 涉及用户字段变更时，必须同步检查：
  - `src/entity/sys_user.rs`
  - `schema.sql`
  - 对应 API 入参/出参与前端类型

## 3. 后端编码规则（Rust）

### 3.1 Handler 约定

- Handler 返回类型统一使用 `ApiResult<ApiResponse<T>>`（或等价封装）。
- 成功响应使用 `ApiResponse::ok(...)`；业务失败通过 `ApiError` 返回。
- 禁止在 handler 内直接构造与规范不一致的裸 JSON 响应结构。

### 3.2 错误处理约定

- 统一使用 `src/app/error.rs` 的 `ApiError`。
- 可预期业务失败使用 `ApiError::Biz`（当前约定映射为 HTTP 200 + `code=1`）。
- 参数错误走 `Query/Path/Json/Validation` 分支，不允许手写分散的错误格式。
- 基础设施错误（DB、JWT、Bcrypt、内部错误）必须保留错误链，避免吞错。

### 3.3 参数校验约定

- 所有外部输入（Path/Query/Json）优先使用 `axum-valid` + `validator`。
- 业务规则（如手机号）统一沉淀在 `src/app/validation.rs`，避免在多个 handler 复制正则。

### 3.4 数据访问约定

- ORM 操作优先使用 SeaORM 表达，不在 API 层拼接原始 SQL（必要场景除外且需说明原因）。
- 分页查询使用统一分页模型（`PaginationParams`/`Page<T>`），避免接口间字段命名漂移。

### 3.5 安全红线

- 密码必须使用 `encode_password` / `verify_password`。
- 严禁明文存储、明文比较、明文回传密码。
- JWT 密钥不得硬编码在生产配置；必须通过环境变量注入。
- 日志中禁止输出 token、密码、密钥、完整 Authorization 头。
  - 当前 `src/api/auth.rs` 存在登录成功后打印 token 的行为，后续改动不得延续此模式。

### 3.6 可观测性

- 新增关键链路需补充 `tracing` 日志，日志内容应可定位问题但不泄露敏感信息。
- 不允许提交调试遗留（`dbg!`、临时 `println!`、无意义日志轰炸）。

## 4. 路由与鉴权规则

- API 路由统一挂载在 `/api` 下。
- 鉴权默认通过 `get_auth_layer()` 进行路由层控制，不在每个 handler 内重复解析 Bearer Token。
- 公共白名单接口（如登录）必须明确保持未鉴权；需要鉴权的接口必须显式挂载鉴权层。
- 未命中 API 的前端路由应保持 `index.html` 回退行为不被破坏。

## 5. 配置与环境规则

- 配置来源：`application.yaml` + `APP_` 前缀环境变量。
- 环境变量命名遵循双下划线层级映射（示例：`APP_SERVER__PORT`）。
- 新增配置项时必须同步更新：
  - 配置结构体（`src/config/*`）
  - `application.yaml` 默认值或说明
  - README 中必要文档

## 6. 前端协作规则（web/）

- 前端接口请求统一经 `web/src/apis/http.ts` 实例，禁止在页面组件里直接散写 fetch/axios。
- 业务 API 封装放在 `web/src/apis/*.ts`，页面层只消费封装函数。
- 路由变更需同步 `web/src/router/` 与页面目录，保持路由生成产物一致。
- 同一批改动避免同时维护两套锁文件；团队默认优先 `pnpm-lock.yaml`，除非明确切换到 npm。

## 7. 数据库与 Schema 规则

- `schema.sql` 与 SeaORM 实体必须保持一致。
- 任何字段新增/删除/类型变更，都要验证：
  - 登录流程（`/api/auth/login`）
  - 用户 CRUD（`/api/users`）
  - 列表分页与查询过滤
- 使用 `database.schema`（当前示例为 `demo`）时，SQL 与连接配置需保持一致。

## 8. 代码变更策略

- 优先最小改动，不做与需求无关的重构。
- 单次提交应聚焦一个问题域（例如“新增接口”或“修复鉴权”），避免混合大杂烩。
- 不允许“顺手”修改无关文件格式或命名，避免噪音 diff。
- 发现历史问题可在 PR 说明中记录，但不强行在当前需求中一并修复。

## 9. 提交前检查清单（必须执行）

后端改动至少执行：
- `cargo fmt`
- `cargo check`
- `cargo clippy --all-targets --all-features -D warnings`

前端改动至少执行（在 `web/` 目录）：
- `pnpm build`（或团队当前约定的等价命令）

涉及联调的改动需手动验证：
- 登录成功/失败路径
- `Authorization: Bearer <token>` 鉴权路径
- 用户列表分页、创建、修改、删除
- 刷新页面后的 SPA 路由访问与静态资源加载

## 10. AI 代理执行规则

- 检索优先级：
  - 首选 `mcp__ace-tool__search_context` 做语义检索。
  - 当 ace-tool 不可用或召回不足时，使用 `mcp__fast-context__fast_context_search` 补充。
- 修改前先定位依赖面，修改后给出受影响文件列表与验证步骤。
- 不得擅自改动密钥、密码、生产连接信息。
- 不得使用破坏性命令回滚用户未授权变更。

## 11. 文档同步要求

出现以下情况必须同步更新文档（`README.md` 或本文件）：
- 新增/删除 API
- 配置项变更
- 启动步骤变更
- 数据库初始化流程变更
- 鉴权或错误响应契约变更

---

若本规则与临时任务冲突，以用户明确指令为准；执行时需在说明中标注偏离点与原因。
