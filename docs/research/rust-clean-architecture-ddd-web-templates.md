# Rust Clean Architecture + DDD Web 方案研究

> 研究对象：Rust Web 框架、模板、脚手架和可复用参考实现。优先采用项目官方仓库、官方文档、发布页和维护者说明；GitHub stars 只作为传播度线索，不作为成熟度结论。

## 结论摘要

**目前没有一个可以直接替代本仓库目标的“成熟产品”。** Loco.rs 是成熟度最高、生成体验最完整的 Rust Web 框架，但它明确采用 Rails 风格的 Active Record，和本仓库要求的 Clean Architecture + DDD + 端口/适配器边界不是同一目标。`gruberb/bulletproof-rust-web` 是最接近的架构与领域建模参考，但它是指南/示例，不是可生成的生产模板；`rust10x/rust-web-app` 是有生产经验取向的 Axum 蓝图，但没有证据表明其 Cargo 边界严格表达 Clean Architecture/DDD。

建议 **build（继续建设本仓库）+ adopt（择优吸收）**：吸收 Bulletproof 的架构/领域建模内容、Loco 的生成器和运维 UX、Rust10x 的数据库/事务与开发流程经验，以及若干小型项目的端口和测试示例；不要把任何一个候选整体 fork 成本仓库的基础。

## 1. 当前仓库比较基线

已先阅读 `README.md`、`TEMPLATE.md`、`docs/guide/architecture/README.md`、`docs/guide/task-flow.md`。基线如下：

| 维度 | 当前能力与证据 |
|---|---|
| 定位与生成 | 独立的 `cargo-generate` 模板，生成可运行的五包 Axum workspace；不是 Axum 框架或通用业务框架（`README.md`、`TEMPLATE.md`）。 |
| 架构边界 | `app` 是唯一 composition root；`http`、`infrastructure` 是 sibling adapters；依赖为 `http/infrastructure -> application -> domain`，Cargo crate 边界实际强制依赖方向（`docs/guide/architecture/README.md`）。 |
| DDD 深度 | Task 参考切片有 `Task`、`TaskId`、`TaskTitle`、枚举、数量单位、私有字段、`create`/`reconstitute`、不变量和持久化重建；文档明确它是可执行架构示例而非通用任务域（`docs/guide/task-flow.md`）。 |
| 技术栈 | Axum/Tokio、SQLx/MySQL、reqwest 下游 TaskPolicy、fastrace/Logforth、环境配置和秘密保护。HTTP DTO、Application command/view、Domain 类型、DB row、下游 wire type 各自位于边界。 |
| 数据库与迁移 | SQLx checked query、MySQL migrations；`serve` 不自动迁移，`migrate` 是唯一生产迁移命令；`just sqlx-prepare` 验证离线元数据。 |
| 配置、可观测性、生命周期 | 环境驱动配置，secrecy 保护秘密；请求/下游 trace propagation；健康检查、优雅停机、shutdown timeout、生命周期验证。 |
| 测试与交付 | `just check`、`just ci`、`just verify`、`just lifecycle`；Docker Compose 启动 MySQL，覆盖格式、Clippy、数据库测试、迁移、SQLx、HTTP smoke、trace 和生命周期。 |
| 已知治理风险 | `README.md` 明确写着仓库尚未声明许可证；在复制、发布生成结果或引入上游代码前必须补齐许可决策。严重度：**中**。 |

## 2. 评价口径

### 2.1 成熟度（独立于匹配度）

- **高**：有持续维护的官方版本/发布记录、稳定的安装路径、较完整的文档和 CI，且有可重复的生成/部署/测试证据。
- **中**：有可运行模板或较完整指南、一定维护和测试证据，但缺少稳定发布产品、长期兼容承诺或完整生产验证。
- **低**：个人示例、无发布记录、无可核验 CI/部署、默认仍是内存存储或仅教程级代码。

stars、forks 和文章热度只在报告中作为背景，不替代以上证据。例如 79 stars 的项目如果只有 `0.1.0` manifest、无 release/CI 证据，成熟度仍然低；Loco 的成熟度则主要由 release、贡献者、CI 和生成器验证体现。

### 2.2 Clean + DDD 匹配度

- **高**：业务核心不依赖 HTTP/DB，依赖方向由 crate 或明确模块边界约束，Ports/Adapters 和领域类型可见。
- **中**：有 domain/application/service/repository 分层和部分依赖倒置，但边界主要靠约定，或数据库模型与领域模型仍有耦合。
- **低**：框架模型/Active Record 直接承载业务和持久化，或只是 handler/service/repository 的 CRUD 分层。

## 3. 候选总览

| 候选 | 定位 | 成熟度 | Clean+DDD 匹配度 | 许可证 | 借鉴优先级 |
|---|---|---:|---:|---|---:|
| [gruberb/bulletproof-rust-web](https://github.com/gruberb/bulletproof-rust-web) | Axum 生产 Web 指南、架构参考、代码示例 | 中 | 高 | 官方页面未确认 LICENSE，需复核 | A |
| [Loco](https://github.com/loco-rs/loco) | Rails 风格 Web 框架、CLI、starter/scaffold | 高 | 低–中 | Apache-2.0 | A（只借生成/运维 UX） |
| [rust10x/rust-web-app](https://github.com/rust10x/rust-web-app) | Axum 生产应用蓝图/可复制 workspace | 中–高 | 中–低 | Apache-2.0 | A（数据层和开发体验） |
| [codemountains/axum-ddd-explicit-architecture](https://github.com/codemountains/axum-ddd-explicit-architecture) | Axum + SQLx + PostgreSQL 的四 workspace 示例 | 低–中 | 高（DDD 深度中） | MIT | A（边界形状） |
| [sukjaelee/clean_axum_demo](https://github.com/sukjaelee/clean_axum_demo) | Axum + SQLx 的 Clean/DDD demo/template | 低–中 | 中–高 | MIT | B |
| [microsoft/cookiecutter-rust-actix-clean-architecture](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture) | Cookiecutter Actix + Diesel Onion 模板 | 低–中 | 高（DDD 深度中） | MIT | B（Actix/Diesel 参考） |
| [n1nj4t4nuk1/rust-ddd-skeleton](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton) | Actix 微服务 DDD + CQRS + Domain Events workspace 模板 | 低 | 高 | MIT | B（DDD/CQRS 参考） |

## 4. 逐项核验

### 4.1 `gruberb/bulletproof-rust-web`

- **定位。** 官方 README 将其定义为“production-grade Rust web applications with Axum”的 opinionated guide；本地构建方式是 `mdbook serve book`，不是安装一个框架或执行模板生成命令。[官方仓库](https://github.com/gruberb/bulletproof-rust-web)；[Introduction](https://gruberb.github.io/bulletproof-rust-web/introduction.html)
- **边界与 DDD。** 讨论单 crate 分层和 workspace 的 Hexagonal/Onion 变体，强调依赖向内、Domain 不引用 Axum/SQLx、Infrastructure 实现 Ports、`main` 只做配置/组装/启动。领域建模章节覆盖实体、值对象、聚合、不变量、领域错误和仓储 Port；“Putting It All Together”给出用户注册纵向切片（Domain 类型、`UserRepository`、SQLx repo、DTO、HTTP 错误映射）。这是本研究中**领域建模深度最高的参考之一**，但仍主要是指南中的示例而非可直接生成的领域产品。[Architecture](https://gruberb.github.io/bulletproof-rust-web/architecture.html)；[Project Structure](https://gruberb.github.io/bulletproof-rust-web/project-structure.html)；[Domain Modeling](https://gruberb.github.io/bulletproof-rust-web/domain-modeling.html)；[Putting It Together](https://gruberb.github.io/bulletproof-rust-web/putting-it-together.html)
- **技术栈与复用方式。** Axum/Tokio/Tower、SQLx（示例以 PostgreSQL 为主）、`tracing`、`thiserror`/`anyhow`、JWT、Docker；通过复制章节代码和采用目录/规则复用，没有 cargo-generate CLI 或稳定生成项目。[官方 README](https://github.com/gruberb/bulletproof-rust-web)
- **数据库、配置、可观测性。** 讲解 SQLx repository、migration、offline prepare、数据库错误转换；配置章节强调环境变量和秘密保护；生产章节覆盖结构化 tracing、HTTP 安全、性能和 graceful shutdown。[Database](https://gruberb.github.io/bulletproof-rust-web/database.html)；[Configuration](https://gruberb.github.io/bulletproof-rust-web/configuration.html)；[Observability](https://gruberb.github.io/bulletproof-rust-web/observability.html)；[Deployment](https://gruberb.github.io/bulletproof-rust-web/deployment.html)
- **测试、CI、部署与成熟度。** 官方内容覆盖 unit/integration/oneshot、SQLx CI 指导、Docker 部署；仓库 workflow 的可核验部署对象是 mdBook 到 GitHub Pages，不是已发布的 Web 服务二进制。[Testing](https://gruberb.github.io/bulletproof-rust-web/testing.html)；[GitHub Pages deploy workflow](https://github.com/gruberb/bulletproof-rust-web/blob/main/.github/workflows/deploy.yml)。有 v1.0.0 发布/版本语义和持续修订迹象，文档成熟度中等；它不是框架 release，不能按框架成熟度理解。官方仓库页面未确认可复用代码的 LICENSE，复制正文或示例前需单独取得许可确认。
- **可直接采用与差距。** 直接采用领域类型、端口所有权、错误转换、HTTP 薄 handler、单 crate 到 workspace 的演进规则、SQLx offline 和测试思路。差距是没有本仓库的 `cargo-generate` 渲染、MySQL canonical slice、下游 TaskPolicy、显式 `migrate` 命令、可执行 trace/lifecycle/acceptance harness；它应当作为**设计资料**而不是替代模板。

### 4.2 Loco.rs

- **定位与边界。** Loco 是“one-person framework”，提供 starter、CLI、controller/model/worker/mailer/auth 和部署生成器；它包裹原生 Axum，但应用边界由框架的 `AppContext`、controllers、models 和 hooks 组织，而不是本仓库的 `domain -> application -> adapters -> app` Cargo 图。[官方文档](https://loco.rs/docs/)；[Starters](https://loco.rs/docs/getting-started/starters/)；[Axum users](https://loco.rs/docs/getting-started/axum-users/)
- **DDD 深度。** 官方 Models 文档明确说明模型采用 Active Record，模型是查询、写入、迁移、seed 和逻辑的中心；这能快速交付，但不是严格的 Domain/Infrastructure 分离，也没有本仓库所要求的持久化 row 到 Domain reconstitution seam。因此 Clean+DDD 匹配度低–中，而非高。[Models](https://loco.rs/docs/the-app/models/)
- **技术栈与生成/复用。** Axum、Tokio、SeaORM、YAML 配置、后台任务/队列、JWT、邮件等；`cargo loco new` 生成应用，`cargo loco generate scaffold/model/controller/worker` 生成文件、迁移和测试。它是本组候选中生成 UX 最完整的方案。[Quick Tour](https://loco.rs/docs/getting-started/tour/)；[Your Project / CLI](https://loco.rs/docs/the-app/your-project/)
- **数据库、迁移、配置。** SeaORM/Active Record；migration-first，模型生成器会生成 migration、同步 entity；支持 SQLite/PostgreSQL 等，配置可启用 `auto_migrate`、测试 truncate/recreate（生产配置需谨慎）。`config/development.yaml`、`production.yaml`、`test.yaml` 和环境变量模板提供强类型/分环境配置。[Models](https://loco.rs/docs/the-app/models/)；[Your Project](https://loco.rs/docs/the-app/your-project/)
- **可观测性、测试、CI、部署。** 内置 tracing/logger、request ID 和可配置 middleware；生成 controller/model/worker 时生成测试，`boot_test`/request helpers 支持数据库与 HTTP workflow。官方 CI 同时执行 build/check/test，并有 sanity workflow 生成不同 starter 后编译验证生成结果。部署支持编译二进制、Docker、Shuttle、Nginx 生成器。[Controllers](https://loco.rs/docs/the-app/controller/)；[Testing](https://loco.rs/docs/the-app/controller/)；[Deployment](https://loco.rs/docs/infrastructure/deployment/)；[CI](https://github.com/loco-rs/loco/blob/master/.github/workflows/loco-rs-ci.yml)；[Generator sanity CI](https://github.com/loco-rs/loco/blob/master/.github/workflows/loco-rs-ci-sanity.yml)
- **成熟度与许可证。** 官方仓库有多个正式 release（包含 1.0.x 及此前 0.16.x 系列）、Apache-2.0、较多贡献者和针对生成器的 CI；这是成熟度高的证据，9K stars 只是传播度背景。[Releases](https://github.com/loco-rs/loco/releases)；[LICENSE](https://github.com/loco-rs/loco/blob/master/LICENSE)；[Repository](https://github.com/loco-rs/loco)
- **可直接采用与差距。** 可采用 CLI 命名、starter 选择、生成器自测、分环境配置、部署模板、内置 middleware 和 `doctor` 体验。若本仓库需要严格 Clean+DDD，不应直接采用其 Active Record 模型、自动迁移默认值或框架中心生命周期；不要重复建设完整的 auth/mailer/worker/scaffold 平台。Loco 适合“快速 Rails 风格产品”，不是本仓库的替代品。

### 4.3 `rust10x/rust-web-app`

- **定位与边界。** 官方称其为 Axum 的“production Web Application Blueprint”，不是通用 Clean Architecture 框架。workspace 划分为 `lib-utils`、`lib-rpc-core`、`lib-auth`、`lib-core`（model/context/config）、`lib-web`（logging/middleware）、`web-server` 和 `gen-key`；这是良好的工程模块化，但从公开树和 Cargo manifest 未看到本仓库那种 `domain/application/http/infrastructure/app` 的硬依赖图。[官方仓库](https://github.com/rust10x/rust-web-app)；[Cargo.toml](https://github.com/rust10x/rust-web-app/blob/main/Cargo.toml)
- **DDD 深度。** 主要抽象是 `ModelManager`、BMC（business model controller）和 context/model 相关代码；E06 记录了 `Dbx` 按需 transaction 支持以及可选 CRUD `macro_rules`。这些是实用的数据/应用层经验，但不是以聚合、不变量、领域事件和独立 Domain Port 为中心的 DDD 体系；匹配度中–低。[E06/官方 README](https://github.com/rust10x/rust-web-app)
- **技术栈与复用方式。** Axum 0.8、Tokio、PostgreSQL、SQLx、SeaQuery/SeaQuery binder、modql、cookies、RPC router；通过 clone/copy blueprint、跟随官方视频和 tag 复用，没有看到 cargo-generate 入口。[Cargo.toml](https://github.com/rust10x/rust-web-app/blob/main/Cargo.toml)；[Rust10x blueprint page](https://rust10x.com/web-app)
- **数据库、迁移、配置、可观测性。** README 提供 PostgreSQL 17 Docker 启动、`ModelManager` transaction 示例、`cargo test`/`cargo watch`，Cargo 中有 `lib-core` config 和 `lib-web` logging/middleware；SQLx/SeaQuery/modql 证据充分，但官方 README 没有像本仓库那样清晰的独立 migration command、offline metadata gate 或迁移发布序列，需按具体分支复核。没有看到完整 OpenTelemetry/metrics 和 production deployment 证据，不把它们当成已提供能力。[官方仓库 README](https://github.com/rust10x/rust-web-app)
- **测试、CI、发布成熟度。** 官方 README 提供 workspace/unit test 和 quick-dev 路径；官方页面显示 Apache-2.0、2023 创建、约 3 位主要贡献者、E06 tag 和持续维护线索，但没有像框架那样的稳定 crates release/生成器兼容矩阵。故工程成熟度中–高、发布成熟度中，不能由约 637 stars 单独推导“成熟产品”。[Repository metadata/README](https://github.com/rust10x/rust-web-app)；[E06 tag](https://github.com/rust10x/rust-web-app/releases/tag/E06)
- **可直接采用与差距。** 可采用 Postgres 数据层、transaction-on-demand、BMC/ModelManager 的实际复杂度控制、workspace dependency 管理、watch/test workflow 和 auth/RPC 边界经验。差距是领域边界不够明确、未证明 Domain 无 DB/HTTP 依赖、生成方式不是 cargo-generate、迁移/observability/lifecycle 验证不如本仓库集中；应当择取实现，不应整体 fork。

### 4.4 `codemountains/axum-ddd-explicit-architecture`

- **定位与边界。** 这是一个 Todo REST API 教学/参考实现，不是框架。四个 workspace 为 `todo-driver`（Axum/controller）、`todo-app`（use case）、`todo-kernel`（domain）和 `todo-adapter`（infrastructure）；README 明确上层可调用下层、反向不允许，并在 kernel/adaptor 间使用 DIP 和 repository traits。[README](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/README.md)
- **DDD 深度。** Clean/Explicit Architecture 边界很直观，domain/kernel 和 use case/repository 可读性高；但样例主要是 Todo CRUD，公开说明没有聚合、领域事件、bounded context 或复杂不变量，故 Clean 匹配高、DDD 深度中，而不是成熟 DDD 产品。
- **技术栈与复用方式。** Axum 0.7.5、Tokio、SQLx 0.7.4、PostgreSQL 16；复制四 workspace 或按其 driver/app/kernel/adapter 命名借鉴，没有生成器。[todo-adapter Cargo.toml](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/todo-adapter/Cargo.toml)
- **数据库、迁移、配置。** Docker Compose 启动 app/PostgreSQL；通过 `sqlx database create`、`sqlx migrate run` 管理 migrations；`local.env` 使用 `DATABASE_URL`、`HOST`、`PORT` 和 `RUST_LOG`，dotenv 由 driver 读取。[README](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/README.md)；[docker-compose.yaml](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/docker-compose.yaml)；[local.env](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/local.env)
- **可观测性、测试、CI、部署。** `tracing`/`RUST_LOG` 有开发级证据；Compose/Dockerfile 是本地开发和数据库启动路径，没有在官方 README/已核验文件中找到完整生产部署、metrics/trace propagation、CI gate 或 acceptance 测试矩阵，以上项目应标为“未核实”，不能补推为能力。[Dockerfile](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/Dockerfile)；[driver startup](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/todo-driver/src/startup/mod.rs)
- **成熟度与许可证。** 官方仓库显示 MIT、2022 创建、`0.1.0` crate manifest；有一定社区关注度（约 79 stars），但未见正式 release/维护者团队/CI 证据。因此成熟度低–中，stars 不能抵消发布和验证证据不足。[Repository](https://github.com/codemountains/axum-ddd-explicit-architecture)；[Cargo.toml](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/todo-adapter/Cargo.toml)
- **可直接采用与差距。** 最值得采用的是四 crate 依赖图、repository Port 所有权和简单的 Compose + SQLx migration quickstart。差距是 Axum/SQLx 版本较旧、Todo CRUD 领域薄、错误/持久化重建/下游 I/O/observability/lifecycle/CI 不足，不能替代当前的五包可验证模板。

### 4.5 `sukjaelee/clean_axum_demo`

- **定位与边界。** 官方 README 称其为“minimalist, domain-driven Rust API server template”，使用按 feature 组织的 `api/domain/infra/dto`，`bootstrap` 构建 `AppState`；通过 repository/service traits 做依赖倒置。它是单 crate 的 demo/template，模块边界主要由约定和注册文件维护，不是 Cargo 强制的四/五 crate 图。[官方仓库](https://github.com/sukjaelee/clean_axum_demo)
- **DDD 深度。** 有实体/值对象、feature 内 repository/service trait、DTO 和显式转换，且声称支持 auth/user/device/file 模块；没有看到聚合、事件、bounded context 或复杂 reconstitution 的证据，且 README 的 model type reference 紧邻 SQLx/Postgres 类型，存在领域/持久化耦合风险。匹配度中–高，DDD 深度中。
- **技术栈与复用方式。** Axum、Tokio、SQLx/PostgreSQL、Utoipa/Swagger、validator、JWT、Docker Compose、OpenTelemetry；feature 可复制，但新增 feature 需要手动注册 `domains.rs`、`app.rs`、`app_state.rs`、`bootstrap.rs`。另有专门的 [`domain_codegen`](https://github.com/sukjaelee/domain_codegen) 从 CREATE TABLE 生成 feature skeleton，是其较独特的生成复用点。[官方 README](https://github.com/sukjaelee/clean_axum_demo)
- **数据库、迁移、配置。** 使用 SQLx offline `cargo sqlx prepare --check`，并提供 `db-seed/01-tables.sql`、`02-seed.sql`；`.env` 配置数据库、JWT secret、端口和资源，不同于本仓库显式独立 migration command 的策略。[官方 README](https://github.com/sukjaelee/clean_axum_demo)
- **可观测性、测试、CI、部署。** README 给出 Jaeger/OpenTelemetry traces/metrics、`tokio::test` 和 `tower::ServiceExt` 的 unit/integration 测试、Docker Compose quickstart；官方仓库检索未确认正式 release 或 CI workflow，因此部署成熟度低–中，不能把“production”标签当成证据。[官方 README](https://github.com/sukjaelee/clean_axum_demo)；[LICENSE](https://github.com/sukjaelee/clean_axum_demo/blob/main/LICENSE)
- **成熟度与许可证。** MIT；官方页面显示约 200 stars、2025 创建、个人维护线索，但未找到版本发布和 CI/部署验证矩阵。按“有较完整 demo、缺少产品化发布证据”评为低–中，而不是高成熟度。
- **可直接采用与差距。** 可借鉴 feature-local layout、SQLx offline、OpenAPI、Jaeger 和 HTTP integration test；不应直接复制 `common`/共享工具层或把 DB 结构生成的模型当作 Domain。差距包括单 crate 边界、PostgreSQL 而非当前 MySQL、没有 TaskPolicy 端口、迁移治理和生命周期 acceptance。

### 4.6 `microsoft/cookiecutter-rust-actix-clean-architecture`

- **定位与边界。** 这是可执行的 Cookiecutter 模板，不是教程；执行 `cookiecutter https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture` 生成 Actix + Diesel 应用。官方 README 明确 Onion Architecture、service-repository pattern、maintenance window、migrations、local PostgreSQL Docker 和 Testcontainers。[README](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)
- **DDD 深度。** 有 domain、service、repository 和 infrastructure 角色，符合 Onion/依赖向内；公开材料主要是 Todo 示例和 CRUD repository/service，未见聚合、领域事件、bounded contexts 或强类型不变量体系，故 Clean 匹配高、DDD 深度中–低。[Onion article](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/docs/onion-architecture-article.md)
- **技术栈与复用方式。** Actix Web、Diesel、PostgreSQL、Docker Compose、Testcontainers；Cookiecutter 是真正的项目生成入口。Diesel 在 async Actix 中通过额外线程执行阻塞操作，和本仓库 Tokio + SQLx 适配策略不同。[README](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)；[Onion article](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/docs/onion-architecture-article.md)
- **数据库、迁移、配置。** Diesel CLI 的 `diesel setup/migration generate/migration run`，Postgres URL 放 `.env`；maintenance window 说明是可复用的运营设计。可观测性、metrics、分布式 trace 和完整配置治理在已核验官方材料中未确认，不能补推。[README](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)
- **测试、CI、部署与成熟度。** Testcontainers integration 是明确卖点，Docker 是本地运行路径；没有正式 release，官方 Releases 页面显示“there aren’t any releases”，SUPPORT.md 仍是未编辑模板，故虽 2023 创建、约 262 stars，成熟度只评低–中。[Releases](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/releases)；[SUPPORT.md](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/SUPPORT.md)
- **许可证、采用与差距。** MIT。[LICENSE](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/LICENSE)。可采用 Onion 图、Testcontainers 和 maintenance window；不应整体 fork：Actix/Diesel、阻塞数据库路径、缺少当前 Axum/SQLx/typed boundary/trace/lifecycle gate，且无发布维护证据。

### 4.7 `n1nj4t4nuk1/rust-ddd-skeleton`

- **定位与边界。** 这是 Actix-Web 微服务 workspace 模板，包含一个 `config_api` 示例；`apps/` 放 HTTP 服务，`libs/` 放 bounded context，每个 context 自带 domain/application/infrastructure，shared 库提供 CQRS bus、event bus 和 value object。它比大多数 Axum CRUD 示例更接近完整 DDD 结构。[官方仓库](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton)
- **DDD 深度。** 官方明确列出 aggregate roots、value objects、repository traits、domain events、CQRS command/query handlers、EventBus/Event subscribers；Clean/Hexagonal 匹配高，DDD 深度高（在模板候选中仅次于 Bulletproof 的概念覆盖）。[Architecture](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/ARCHITECTURE.md)；[CQRS](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/CQRS.md)；[Domain Events](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/DOMAIN_EVENTS.md)
- **技术栈与复用方式。** Rust 2021、Actix-Web 4、Tokio、tracing；通过复制 `libs/config` 为新 bounded context、复制 `apps/config_api` 为新 app，再更新 workspace/Makefile 复用，没有生成 CLI。[官方 README](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton)
- **数据库、迁移、配置。** 默认 persistence 是 in-memory，明确写着将来可替换 PostgreSQL/Redis；因此没有真实数据库 adapter 或 migration proof。配置、Docker Compose 和本地 Make 命令有开发路径，但不是本仓库的 MySQL/显式迁移生产链。[官方 README](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton)
- **可观测性、测试、CI、部署。** tracing + tracing-subscriber；`make test`、unit/e2e、`make audit`、Docker up/down，README 声称 CI 已接入；未见云部署或正式发布包。官方页面显示 MIT、2024 创建、0 stars，成熟度低；0 stars 不是唯一依据，决定性证据是内存默认实现、无 release 和无真实 DB/deployment 验证。[官方 README](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton)；[LICENSE](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/LICENSE)
- **可直接采用与差距。** 可借鉴 bounded context 文档、CQRS/event 目录、Object Mother/测试分层和新增 app 的 checklist；不要为当前 Task 参考切片预先引入 CQRS/event bus。Actix、内存 persistence、无 MySQL/下游 I/O/正式 lifecycle gate 使它不能替代当前模板。

## 5. 其他检索结果与筛选

- [christiandoxa/axum_ddd](https://github.com/christiandoxa/axum_ddd) 是很贴题的 Axum 0.8 + Domain/Application/Infrastructure/Interface starter，有 `EmailAddress`/`Username`、`UsersRepo`、in-memory adapter、tracing 和 unit/integration tests；官方 README 还把 SQLx/Diesel、JWT、OpenTelemetry 和 property/E2E tests列为后续工作。页面显示 2025 创建、MIT、约 1 star、无 release，适合教学对照，不进入前五。
- [IltonSeixas/rust-enterprise-boilerplate](https://github.com/IltonSeixas/rust-enterprise-boilerplate) 的 README 声称 Clean + DDD + TDD、Axum + Tonic + SQLx/Postgres、OpenTelemetry、Prometheus、architecture tests、Docker/CI；但官方页面显示 2026 创建、约 1 star、无成熟发布历史，且默认仍以 in-memory adapter 立即运行，属于观察名单，不作为成熟替代。[官方 README](https://github.com/IltonSeixas/rust-enterprise-boilerplate)
- `realworld-axum-sqlx`、普通 Axum/SQLx CRUD 模板和仅有 `handler -> service -> repository` 的项目没有纳入主要比较：它们能证明 Axum/SQLx 的实战方式，但没有足够的 Clean+DDD/Hexagonal 边界证据。Loco 官方的 [Axum 对比文档](https://loco.rs/docs/getting-started/axum-users/)也把 `realworld-axum-sqlx` 作为实战代码比较对象，而不是 Clean+DDD 模板。
- 搜索阶段看到的 Reddit、Medium、YouTube、聚合站和 SEO 摘要只用于发现候选，未用作关键结论来源；正式证据以每个候选的官方 repository/README/docs/releases/CI 文件为准。

## 6. 最值得借鉴的 5 个候选

1. **Bulletproof Rust Web：架构和领域建模第一参考。** 证据是依赖向内、Domain 无 Axum/SQLx、Ports、workspace 演进和完整纵向切片；应吸收原则与文档结构，不替换当前生成器。[Architecture](https://gruberb.github.io/bulletproof-rust-web/architecture.html)
2. **Loco：生成器、starter、配置、测试和部署 UX 第一参考。** 证据是 `cargo loco generate`、生成测试、sanity CI、分环境 YAML 和 Docker/Shuttle/Nginx deployment generator；但 Active Record 使其不是 Clean+DDD 基座。[Models](https://loco.rs/docs/the-app/models/)；[Deployment](https://loco.rs/docs/infrastructure/deployment/)
3. **Rust10x：生产 workspace 与数据访问经验。** 证据是 Axum 0.8、Postgres/SQLx/SeaQuery/modql、ModelManager transaction、BMC macros 和测试/watch workflow；要补齐其公开边界中缺少的 Domain/Application/Ports 证明。[Cargo.toml](https://github.com/rust10x/rust-web-app/blob/main/Cargo.toml)
4. **codemountains：最小的显式四层依赖图。** 证据是 driver/app/kernel/adapter 四 workspace、repository traits 和 Docker+SQLx migrations；适合做边界教学，不能当生产成熟度证据。[README](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/README.md)
5. **clean_axum_demo：Axum 生态功能覆盖参考。** 证据是 SQLx offline、OpenAPI、feature-local API/domain/infra/dto、OpenTelemetry、HTTP integration tests 和 domain codegen；应只拿功能/测试片段，保留本仓库更严格的 Cargo 边界。[官方 README](https://github.com/sukjaelee/clean_axum_demo)

Actix 方向中，Microsoft Cookiecutter 和 `rust-ddd-skeleton` 分别最值得借鉴 Testcontainers/maintenance window 与 bounded context/CQRS 文档，但由于无 release、默认内存或旧的 Actix/Diesel 路径，不进入“可直接采用”名单。

## 7. buy / adopt / fork / build 建议

| 选项 | 建议 |
|---|---|
| **buy** | 不建议购买或引入商业替代；Rust 生态没有发现一个同时满足“可生成 + Axum + 严格 Clean + DDD + MySQL/迁移 + outbound/observability/lifecycle 验证”的成熟产品。若业务优先快速 CRUD，可单独选择 Loco，但那是架构取舍，不是本仓库替代。 |
| **adopt** | Adopt Bulletproof 的领域/边界写法；Loco 的 starter 选择、生成测试、`doctor`、deployment generator UX；Rust10x 的 transaction-on-demand 和 workspace 数据访问经验；clean_axum 的 SQLx offline/OpenAPI/Jaeger 测试思路，前提是转换边界和依赖方向不被破坏。 |
| **fork** | 仅在 spike 或教学对照中 fork codemountains、Microsoft 或 `rust-ddd-skeleton`；先核对许可证、依赖版本、默认安全配置和维护状态。不要把无 release/CI 证据的 demo 直接作为生产基线。 |
| **build** | 继续建设当前 `cargo-generate` 模板。当前仓库已经把“架构边界 + 可执行 Task 切片 + MySQL + 下游 Port + trace + lifecycle + quality gates”组合成候选中缺失的完整链路。 |

## 8. 本仓库应保留的差异化方向

1. **Cargo 强制的四层职责 + 独立 app composition root。** 这比单 crate 目录约定更可验证，也避免 Loco/普通 CRUD 模板把 DB model 和业务规则混为一体。
2. **可执行而非纯图示的 golden path。** `POST /api/v1/tasks` 和 `GET /api/v1/tasks/{task_id}` 同时证明 HTTP DTO、Application command/view、Domain value object、private DB row 重建和下游 wire conversion。
3. **显式 trust/representation conversion。** `FromStr`、`TryFrom`、`From` 和 corrupt persistence 分类是当前候选中很少完整展示的边界证据，应继续保持。
4. **迁移、服务启动和生命周期分离。** `serve` 不隐式迁移；`migrate`、`verify`、`lifecycle` 各自证明不同运行时契约，这比“启动时 auto-migrate”更适合生产治理。
5. **可观测性和外部 I/O 进入 acceptance path。** trace propagation、下游超时/错误分类、graceful drain/shutdown timeout 都不只是 README feature list，而是可运行验证。
6. **Task 作为参考切片而非强行通用领域。** `docs/guide/task-flow.md` 已正确限制其业务范围；后续应继续用小而真实的纵向切片证明架构，而不是堆通用 CRUD。

## 9. 不应重复建设的部分

- 不重复造一个 Loco：不要在本仓库加入 Active Record、通用 auth/mailer/worker/scaffold/全栈 starter 平台，除非出现明确产品需求。
- 不重复造 Rust10x 的通用 BMC/ModelManager 宏层：当前已有 Application Port，只有出现第二个真实实现或事务边界需求才增加抽象。
- 不复制 Bulletproof 的整本指南或上游代码：本仓库只保留与自身模板、规则和验证 harness 相关的简洁文档，并先解决许可证。
- 不因候选项目存在就预装 CQRS、事件总线、Redis、缓存、消息队列或 OpenTelemetry 全套；先有业务/负载证据，再在合适的 crate/Port 中引入。
- 不把 generic `common/shared/utils` 层作为方便的跨层依赖；继续遵守 `docs/guide/architecture/README.md` 的 inward dependency 和所有权规则。

## 10. 风险与待办（按路径和严重度）

| 严重度 | 文件/路径 | 发现 | 建议 |
|---|---|---|---|
| 中 | `README.md`、`TEMPLATE.md` | 当前仓库没有声明许可证；研究中多个上游也存在“代码可见但许可不清”情况。 | 在发布/分发模板前补充 LICENSE 和复制/归属政策；不要把未确认许可的上游正文或代码直接纳入。 |
| 低 | `docs/guide/architecture/README.md` | 当前边界比多数候选更严格，但需要持续保持 crate graph 与文档同步。 | 每次边界变更继续保留 `just architecture`/`just check` 证据。 |
| 低 | `docs/guide/task-flow.md` | Task slice 不是通用域模型；若未来被误当成业务规范会产生范围漂移。 | 保持“reference slice”措辞，并在新增真实业务时另建 Domain capability 文档。 |
| 低–中 | `README.md` 的发布定位 | 尚未发现可直接替代当前目标的成熟方案，维护和发布责任仍在本仓库。 | 优先补许可证、生成结果 smoke、版本策略和模板升级说明，而不是引入更大的框架。 |

## 11. 来源保留与排除

### 保留

- [Bulletproof Rust Web 官方仓库](https://github.com/gruberb/bulletproof-rust-web)及其 [Architecture](https://gruberb.github.io/bulletproof-rust-web/architecture.html)、[Domain Modeling](https://gruberb.github.io/bulletproof-rust-web/domain-modeling.html)、[Testing](https://gruberb.github.io/bulletproof-rust-web/testing.html)、[Deployment](https://gruberb.github.io/bulletproof-rust-web/deployment.html)。
- [Loco 官方仓库](https://github.com/loco-rs/loco)、[官方文档](https://loco.rs/docs/)、[Models](https://loco.rs/docs/the-app/models/)、[Deployment](https://loco.rs/docs/infrastructure/deployment/)、[Releases](https://github.com/loco-rs/loco/releases)及官方 CI 文件。
- [Rust10x 官方仓库](https://github.com/rust10x/rust-web-app)、[Cargo manifest](https://github.com/rust10x/rust-web-app/blob/main/Cargo.toml)、[官方 blueprint page](https://rust10x.com/web-app)。
- [codemountains 官方 README](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/README.md)、[Cargo](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/todo-adapter/Cargo.toml)、[Compose](https://github.com/codemountains/axum-ddd-explicit-architecture/blob/main/docker-compose.yaml)。
- [clean_axum_demo 官方 README](https://github.com/sukjaelee/clean_axum_demo)及其 [LICENSE](https://github.com/sukjaelee/clean_axum_demo/blob/main/LICENSE)。
- [Microsoft Cookiecutter 官方仓库](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture)、[Onion article](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/docs/onion-architecture-article.md)、[Releases](https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/releases)。
- [rust-ddd-skeleton 官方仓库](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton)及其 [Architecture](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/ARCHITECTURE.md)、[CQRS](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/CQRS.md)、[Domain Events](https://github.com/n1nj4t4nuk1/rust-ddd-skeleton/blob/main/docs/DOMAIN_EVENTS.md)。

### 排除

- Reddit、Medium、YouTube、博客、聚合站和搜索摘要：只用于发现候选，不作为关键事实来源。
- `realworld-axum-sqlx` 及普通 Axum/SQLx CRUD 模板：有实战价值，但没有足够 Clean+DDD/Hexagonal 边界证据。
- 极新的、无 release/CI/真实持久化证据的“production-ready”仓库：保留为观察名单，不冒充成熟替代品。

## 最终回答

1. **是否存在直接替代品：不存在。** Loco 的成熟度高但架构目标不同；Bulletproof 的匹配度最高但不是生成器；其余直接匹配项目缺少发布、维护或完整生产验证。
2. **最值得借鉴：** Bulletproof（架构/DDD）、Loco（生成与运维 UX）、Rust10x（workspace/数据访问）、codemountains（显式四层）、clean_axum_demo（Axum 生态功能和测试）；Actix 方向补充参考 Microsoft 和 `rust-ddd-skeleton`。
3. **策略：** 本仓库 `build`；从上述项目 `adopt` 小而明确的能力；仅对小型 demo `fork` 做 spike；不购买或整体替换。
4. **差异化：** 保留编译期依赖方向、typed conversion、MySQL + outbound Port、显式迁移、trace/lifecycle/acceptance gates 和可生成的真实 golden path；不要重复建设 Loco 的全套框架功能或 speculative CQRS/缓存/消息系统。
