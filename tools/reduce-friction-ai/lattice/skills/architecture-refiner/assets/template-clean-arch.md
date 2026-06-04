# Architecture Refiner 模板

此模板定义 clean architecture 风格的 `.lattice/standards/architecture.md` 输出文档的结构。它包含来自 architecture atom 的 `clean-architecture-defaults.md` 的所有默认内容，穿插访谈指导注释。

生成输出时，剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释。最终文档是规范，而非对话日志。

---

## Frontmatter（前置元数据）

<!-- INTERVIEW GUIDANCE:
根据用户选择的模式选择两个 frontmatter 选项之一。
除非用户明确想要重新定义所有内容，否则默认使用 overlay。
-->

选项 A — Overlay 模式（最常见）：

```yaml
---
mode: overlay
---
```

选项 B — Override 模式（完全替换）：

```yaml
---
mode: override
---
```

---

## 序言（Preamble）

<!-- INTERVIEW GUIDANCE:
包含与所选模式匹配的序言。输出中只出现一个序言。
-->

**Overlay 序言：**

> 此文档在 architecture atom 的内置 clean-architecture 默认值之上叠加项目特定的自定义。此处包含的部分与默认值不同——所有其他部分保持不变。
>
> 下面的部分替换默认值中匹配的部分（按标题匹配）。新部分追加在默认值之后。

**Override 序言：**

> 这是 [项目名称] 的 clean architecture 原则。它们完全替换 architecture atom 中的内置 clean-architecture 默认值。

**目录**（对于 override 模式；overlay 模式仅列出包含的部分）：

1. [层职责](#1-层职责)
2. [依赖方向](#2-依赖方向)
3. [每层规则](#3-每层规则)
4. [命令和查询流程](#4-命令和查询流程)
5. [示例违规和修复](#5-示例违规和修复)
6. [验证清单](#6-验证清单)

---

## 1. 层职责（Layer Responsibilities）

<!-- INTERVIEW GUIDANCE:
询问："你的代码库使用哪些层？以下是标准的 4 层层结构。这匹配你的项目吗，还是你使用不同的层？"

显示下面的表和目录映射。

追问问题：
- 是否有任何额外的层（例如，在 controllers 和 services 之间的 Mediator/CQRS handler 层）？
- 你使用这些层的不同名称吗（例如，"adapters"而非"controllers"，"usecases"而非"services"）？
- 除了 repositories/providers 之外，是否有基础设施子类别（例如，单独的消息传递、缓存、external-api 文件夹）？
- 是否有用于横切关注点的共享/公共层？

可自定义：层名称、职责、目录名称、额外层。
固定：必须至少有内部（domain）和外部层。向内依赖规则必须成立。

跨部分影响：此处选择的层名称必须在所有后续部分中一致使用。
如果添加了额外层，§2（依赖图）和§3（每层规则）必须包含它们。
-->

| 层 | 职责 | 依赖于 | 被以下依赖 |
|-------|---------|----------|------------------|
| **Controllers / Handlers** | 将外部输入（HTTP、gRPC、CLI、事件）转换为应用调用并格式化响应 | Application Services | 无（入口点） |
| **Application Services** | 编排用例：验证、调用 domain、通过接口协调基础设施 | Domain, Infrastructure interfaces | Controllers |
| **Domain** | 业务规则、entities、value objects、domain services、domain events | 无（最内部） | Application Services, Infrastructure (通过接口) |
| **Infrastructure: Repositories** | 持久化和检索 domain 对象用于状态更改操作。实现 domain 定义的接口 | Domain (对于接口) | Application Services (注入) |
| **Infrastructure: Providers** | 获取只读操作的数据。直接返回 DAOs。无 domain 接口 | 无（具体类或 app 层接口） | Application Services (注入) |
| **Infrastructure: Other** | 外部 API、文件 I/O、消息传递、缓存、通知 | Domain (对于接口，当适用时) | Application Services (注入) |

### 典型目录映射

```
src/
├── controllers/        # 或 handlers/, routes/, api/
│   ├── UserController
│   └── OrderController
├── services/           # 或 usecases/, application/
│   ├── OrderService                # 处理命令和查询流程
│   └── UserService                 # 每个 domain 概念一个服务
├── domain/             # 或 core/, model/
│   ├── entities/
│   │   ├── Order
│   │   └── User
│   ├── value-objects/
│   │   ├── Money
│   │   └── Email
│   ├── services/
│   │   └── PricingService
│   ├── events/
│   │   └── OrderPlaced
│   └── repositories/   # 仅接口——用于状态更改操作
│       ├── OrderRepository
│       └── UserRepository
└── infrastructure/     # 或 adapters/, persistence/
    ├── repositories/   # 状态更改：实现 domain 定义的接口
    │   ├── PostgresOrderRepository  (实现 OrderRepository)
    │   └── PostgresUserRepository   (实现 UserRepository)
    ├── providers/      # 只读：无 domain 接口，返回 DAOs
    │   ├── UserProvider
    │   └── OrderProvider
    ├── external/
    │   └── StripePaymentGateway
    └── messaging/
        └── KafkaEventPublisher
```

注意 `infrastructure/` 下的两个同级文件夹：`repositories/`（用于命令流程，实现 domain 接口）和 `providers/`（用于查询流程，无 domain 接口）。

---

## 2. 依赖方向（Dependency Direction）

<!-- INTERVIEW GUIDANCE:
询问："标准规则是依赖只指向内部。你的项目遵循这个规则吗，还是有任何例外？"

显示下面的 ASCII 图。

追问问题：
- 你的项目如何处理依赖倒置？DI 容器、手动注入或框架提供的？
- 你使用什么格式的数据跨越边界？DTOs、plain objects、primitives？
- 是否有任何有意违反仅向内规则的例外？

可自定义：DI 机制、数据跨越格式、额外注释。
固定：核心仅向内依赖规则是不可协商的。

如果用户在§1中添加了额外层，更新图表以包含它们。
-->

```
┌──────────────────────────────────────────────────┐
│  Controllers / Handlers         (最外层)          │
│    │                                             │
│    ▼                                             │
│  Application Services                            │
│    │                                             │
│    ▼                                             │
│  Domain                          (最内部)         │
│    ▲                                             │
│    │ 实现接口                       │
│  Infrastructure                  (外层)          │
└──────────────────────────────────────────────────┘

依赖仅向内流动。
Infrastructure 依赖于 Domain（它实现 domain 接口）。
Domain 不依赖自身之外的任何东西。
```

Infrastructure 位于外环，即使它实现内环定义的接口。源代码依赖指向内部（infrastructure 导入 domain 接口），而运行时调用向外。这是依赖倒置——当内层需要触发外层中的某些内容时使用的机制。

**跨越边界的数据**应该是简单结构——DTOs、plain objects、primitives。从外部格式映射到应用层类型向内，从 domain 对象映射到响应 DTOs 向外。这种隔离意味着 API 契约和数据库模式独立演进。

---

## 3. 每层规则（Per-Layer Rules）

<!-- INTERVIEW GUIDANCE:
一次 walkthrough 每个子部分。对于每层，显示默认值并询问：
"这匹配你们团队使用此层的方式吗？有什么要添加、删除或更改的吗？"

这里的关键决策传播到§4：
- §3.2 服务模式（统一服务 vs CQRS handlers）影响§4 中的命令和查询流程。
- §3.4 Provider 模式（是/否）影响§4.2、§4.3。

如果在§1中添加了额外层，为每个添加新的子部分（3.5、3.6 等）。

可自定义：每层的所有项目。可以添加/删除项目。
固定：每层必须有"什么属于这里"、"什么不属于这里"和"常见违规"。
-->

### 3.1 Controllers / Handlers

**什么属于这里：**
- HTTP 路由定义和请求解析
- 输入验证（格式、存在性——非业务规则）
- 响应格式化与状态码映射
- 认证中间件集成
- 请求/响应 DTOs

**什么不属于这里：**
- 业务规则评估（"如果订单总额 > 100，应用折扣"）
- 直接数据库调用
- 从原始输入构造 domain 对象（使用 mapper 或 factory）

**常见违规：**
- Controller 从 DB 读取、应用逻辑、写回——全部在一个方法中
- 控制器操作中的业务规则条件判断
- 直接将 domain entities 作为 JSON 返回

### 3.2 Application Services

<!-- INTERVIEW GUIDANCE:
关键决策：你的团队使用统一服务模式（每个 domain 概念一个服务，包含命令和查询方法）还是 CQRS 模式（单独的命令处理程序和查询处理程序）？

此决策影响§4（命令和查询流程）。记录选择并继续传递。
-->

每个 domain 概念一个服务（例如，`OrderService`、`UserService`）。每个服务包含命令方法和查询方法，为每种使用不同的基础设施路径。

**命令方法（状态更改——创建、更新、删除）：**
- 编排：验证 → 创建/填充 domain → 通过 Repository 持久化 → 发布事件
- 事务边界管理
- 授权检查
- 通过 domain 中定义的 repository 接口调用基础设施

**查询方法（读取——获取、列表、搜索）：**
- 调用 Provider 以 DAOs 形式获取数据
- 将 DAOs 映射到响应 DTOs
- 不构造 domain 对象

**服务构造函数模式：**
- 将 Repository（用于命令）和 Provider（用于查询）注入同一个服务
- 服务根据操作决定使用哪个基础设施路径

**Application services vs domain services：** Application services 编排工作流并协调基础设施边界。Domain services 执行跨越 entities/value objects 的纯业务逻辑，无 I/O。

**常见违规：**
- Service 包含所有业务逻辑而 entities 是数据持有者（贫血领域模型）
- 导入具体的 repository 类而非接口
- 在 Provider 足够的情况下为读取操作构造 domain 对象

### 3.3 Domain

**什么属于这里：**
- 带行为的 entities（不仅仅是数据）
- Value objects（Money、Email、OrderId——不可变，通过属性相等性比较）
- Domain services（不适合单个 entity 的业务逻辑）
- Domain events（OrderPlaced、PaymentReceived）
- Repository 接口（基础设施实现的契约）
- 复杂对象创建的工厂方法

**什么不属于这里：**
- 来自任何外层的导入
- 框架注解（@Entity、@Column、@RestController）
- 数据库特定类型（ResultSet、Document、Row）
- HTTP 特定类型（Request、Response、Headers）

**常见违规：**
- Entities 使用 ORM 装饰器注解
- Domain services 直接调用 repositories 而非通过 application services 接收数据

### 3.4 Infrastructure

<!-- INTERVIEW GUIDANCE:
关键决策：你的项目是否对读取操作使用 Provider 模式？

如果是：Providers 位于 infrastructure/providers/，返回 DAOs，无 domain 接口。
如果否：所有数据访问通过 Repositories。这更改§4.2、§4.3 和§6。

记录选择并继续传递到§4。
-->

两种不同的数据访问模式，以及其他技术机制：

**Repositories（`infrastructure/repositories/`）：**
- 实现 `domain/repositories/` 中定义的接口
- 接受和返回 **domain 对象**
- 在 domain 对象和 DAOs 之间内部映射
- 专门用于状态更改操作

**Providers（`infrastructure/providers/`）：**
- domain 中无接口——契约位于应用层或作为具体类
- 直接返回 **DAOs**
- 专门用于读取操作
- 优化查询性能，无需 domain 构造开销

**其他基础设施：**外部 API 客户端、文件 I/O、消息队列、缓存、通知。

**常见违规：**
- Repository 方法包含业务逻辑
- 具体基础设施类型暴露给 application services
- 对只读查询使用 Repository（不必要的映射开销）
- Provider 返回 domain entities 而非 DAOs

---

## 4. 命令和查询流程（Command and Query Flows）

<!-- INTERVIEW GUIDANCE:
此部分依赖于§3 的决策：
- 如果§3.2 选择统一服务：展示下面的流程。
- 如果§3.2 选择 CQRS：调整流程以使用单独的命令/查询处理程序。
- 如果§3.4 选择无 Provider 模式：调整§4.2 和§4.3 以使用 Repository 进行读取。

 walkthrough 每个子部分。展示流程图和伪代码，然后询问：
"这匹配你的项目处理[命令/查询]的方式吗？"
-->

每个端点属于这两种流程之一。选择正确的流程是在生成代码时的第一个结构决策。

单个服务处理两种流程。`OrderService` 有命令方法（`createOrder`、`updateOrder`）通过 domain 使用 Repository，以及查询方法（`getOrder`、`listOrders`）直接使用 Provider。命令/查询分离是服务内的*流程*区分，而非类级别拆分。

### 4.1 命令流程（创建、更新、删除）

状态更改操作启用完整栈。domain 层在任何状态更改持久化之前强制执行不变性和业务规则。

```
Controller（请求 DTO）
  → Application Service
    → Domain（创建/填充，强制执行业务规则）
      → Repository（接受 Domain，转换为 DAO，持久化）
```

以下示例演示 domain/infrastructure 边界的依赖倒置——接口在 domain 中定义，在 infrastructure 中实现：

```
// domain/repositories/ -- domain 中的接口
interface OrderRepository
  save(order: Order): void
  findById(id: OrderId): Order or null

// infrastructure/repositories/ -- 实现
class PostgresOrderRepository implements OrderRepository
  save(order: Order):
    dao = OrderDAO.fromDomain(order)
    db.insert("orders", dao)

  findById(id: OrderId): Order or null
    dao = db.findOne("orders", { id: id.value })
    return dao ? OrderDAO.toDomain(dao) : null

// services/ -- 编排
class OrderService
  constructor(orderRepo: OrderRepository)

  createOrder(command: CreateOrderCommand): OrderId
    order = Order.create(command.items, command.customerId)
    orderRepo.save(order)
    return order.id
```

### 4.2 查询流程（获取、列表、搜索）

读取操作完全绕过 domain。没有不变性需要保护，所以 domain 构造是不必要的开销。

```
Controller（请求参数）
  → Application Service
    → Provider（直接返回 DAO 到服务）
  ← 服务将 DAO 映射到响应 DTO
← Controller 返回响应 DTO
```

以下示例展示单个服务中的两种流程，带有显式 DTO 映射：

```
// Application Service -- Repository（命令）和 Provider（查询）
class UserService
  constructor(userRepo: UserRepository, userProvider: UserProvider)
    // userRepo 用于状态更改操作
    // userProvider 用于读取操作

  // 命令流程：通过 domain，使用 Repository
  registerUser(command: RegisterUserCommand): String
    user = User.create(command.email, command.name)
    userRepo.save(user)
    return user.id.value

  // 查询流程：绕过 domain，使用 Provider，将 DAO 映射到响应 DTO
  getUser(userId: String): UserResponse
    dao = userProvider.findById(userId)
    if dao is null: throw NotFoundError("User not found")
    return UserResponse.fromDAO(dao)

  listActiveUsers(page: Integer, size: Integer): List<UserResponse>
    daos = userProvider.listActive(page, size)
    return daos.map(UserResponse.fromDAO)

// Infrastructure -- Provider（在 infrastructure/providers/）
// 无 domain 接口。具体类或应用层接口。
class UserProvider
  findById(id: String): UserDAO or null
    return db.findOne("users", { id })

  listActive(page: Integer, size: Integer): List<UserDAO>
    return db.query(
      "SELECT * FROM users WHERE active = true LIMIT ? OFFSET ?",
      [size, page * size]
    )

// 响应 DTO -- 显式字段选择，非 DAO 的透传。
// DB-内部字段被剥离；名称为 API 契约塑造。
class UserResponse
  id: String
  name: String
  email: String
  active: Boolean

  static fromDAO(dao: UserDAO): UserResponse
    return new UserResponse(dao.id, dao.name, dao.email, dao.active)
    // dao.passwordHash、dao.internalFlags、dao.createdAt 被有意排除
```

### 4.3 Provider vs Repository：结构比较

| 方面 | Repository | Provider |
|--------|-----------|----------|
| **目的** | 持久化和检索 domain 对象用于状态更改操作 | 获取只读操作的数据 |
| **接口定义于** | `domain/repositories/` | 应用层或 infrastructure 中的具体类 |
| **接受** | Domain 对象（entities、aggregates） | 原始查询参数 |
| **返回** | Domain 对象 | DAOs（数据访问对象） |
| **由以下调用** | 命令方法（创建、更新、删除） | 查询方法（获取、列表、搜索） |
| **Domain 参与** | 完整——强制执行不变性 | 无——数据流动 DB → 响应 DTO |
| **映射** | Domain ↔ DAO（双向） | DAO → 响应 DTO（单向，在服务中完成） |

### 4.4 当读取需要 Domain 时

很少，读取需要 domain 逻辑——例如，取决于 domain 状态的访问控制。在这些情况下，即使没有状态更改，也使用命令流程结构。domain 参与由业务规则证明是合理的。

---

## 5. 示例违规和修复（Example Violations and Fixes）

<!-- INTERVIEW GUIDANCE:
显示下面的四个 BAD/GOOD 对。询问：
"是否有任何项目特定的反模式你想添加？常见示例：
- 框架特定违规（例如，在 middleware 中放置业务逻辑）
- 导致问题的团队特定模式
- 来自过去事件或代码审查的模式"

可自定义：可以添加新的违规/修复对。可以使用项目术语修改现有示例。
固定：除非用户有强烈理由移除一个，否则四个核心违规应保留。
-->

### 控制器中的业务逻辑

```
// BAD：Controller 做出业务决策
class OrderController
  createOrder(request):
    items = request.body.items
    total = 0
    for each item in items:
      total = total + item.price * item.quantity
      if item.quantity > 100: total = total * 0.9   // controller 中的业务规则
    if total > 10000: throw Error("Order limit exceeded")  // 业务规则
    db.query("INSERT INTO orders...", [total])       // 直接 DB 访问

// GOOD：Controller 委托给 service
class OrderController
  constructor(orderService: OrderService)

  handle(request):
    command = CreateOrderCommand.fromRequest(request)
    result = orderService.createOrder(command)
    return OrderResponse.from(result)
```

### Domain 依赖 Infrastructure

```
// BAD：Domain entity 依赖 infrastructure
class Order
  dbClient = new DatabaseClient()

  calculateTotal(): Money
    taxRate = dbClient.findConfig("tax_rate")   // domain 中的 I/O——违规
    // ...

// GOOD：Domain 定义接口，接收数据
interface TaxRateProvider
  getCurrentRate(region: Region): TaxRate

class Order
  calculateTotal(taxRate: TaxRate): Money
    // 纯业务逻辑，无 I/O

class OrderService
  constructor(taxRates: TaxRateProvider, orders: OrderRepository)

  createOrder(command: CreateOrderCommand):
    rate = taxRates.getCurrentRate(command.region)
    order = Order.create(command.items)
    total = order.calculateTotal(rate)
    // ...
```

### 泄漏数据格式

```
// BAD：数据库模型直接从 API 返回
class UserController
  getUser(request):
    user = db.findOne("users", { id: request.params.id })
    return user                 // 暴露 passwordHash、内部 IDs、DB 列名

// GOOD：在边界映射
class UserController
  constructor(userService: UserService)

  handle(request):
    return userService.getUser(request.params.id)   // 返回响应 DTO
```

### God Class（上帝类）

```
// BAD：一个类做所有事情——验证、HTTP、业务规则、持久化、邮件、消息传递
class OrderService
  createOrder(data):
    // 130+ 行覆盖 7 个职责

// GOOD：按职责和层分解
// domain/entities/Order              -- 业务规则
// domain/repositories/OrderRepository -- 持久化接口
// infrastructure/repositories/PostgresOrderRepository -- 持久化实现
// infrastructure/external/InventoryClient -- 外部 API
// infrastructure/messaging/OrderEventPublisher -- 消息传递
// services/OrderService              -- 仅编排
```

---

## 6. 验证清单（Validation Checklist）

<!-- INTERVIEW GUIDANCE:
显示下面的六组。询问：
"当生成或审查代码时，AI 应该检查所有这些吗？有什么要添加或删除的吗？"

如果用户在§3.4 中选择不使用 Provider 模式，删除或调整
"命令/查询流程分离"组以反映仅 Repository 的数据访问。

可自定义：可以添加或删除单个检查。可以添加新组。
固定：必须有层放置和依赖方向组。
-->

在生成或审查代码后使用此清单。每个项目映射到一个结构原则。

### 层放置（Layer Placement）

- [ ] 业务逻辑（规则、计算、决策）在 domain 层
- [ ] 用例编排位于 application services
- [ ] HTTP/传输关注点仅在 controllers 中
- [ ] 数据库和外部 API 细节仅在 infrastructure 中

### 依赖方向（Dependency Direction）

- [ ] Domain 层没有来自外层的零导入
- [ ] Application services 依赖 domain 和接口（而非具体基础设施）
- [ ] Infrastructure 实现 domain 定义的接口
- [ ] 层之间无循环依赖

### 边界完整性（Boundary Integrity）

- [ ] 向内跨越的数据映射到 domain 类型（而非原始请求对象）
- [ ] 向外跨越的数据映射到响应 DTOs（而非 domain entities）
- [ ] 框架类型（ORM models、HTTP 请求对象）不出现在 domain 中

### 命令/查询流程分离（Command / Query Flow Separation）

- [ ] 状态更改操作使用命令流程：Controller → Service → Domain → Repository
- [ ] 读取操作使用查询流程：Controller → Service → Provider → DAO → Response DTO
- [ ] Repository 接口在 `domain/repositories/` 中定义
- [ ] Provider 契约**不在** domain 中定义
- [ ] Repositories 接受和返回 domain 对象；Providers 返回 DAOs
- [ ] 查询流程中不构造 domain 对象，除非有明确理由
- [ ] Services 直接将 DAOs 映射到 Response DTOs，不通过 domain

### 单一职责（Single Responsibility）

- [ ] 每个类/模块只有一个更改原因
- [ ] 无类跨越多个层
- [ ] Infrastructure 类不包含业务规则

### 可测试性（Testability）

- [ ] Domain 逻辑可以在不 mock I/O 的情况下进行单元测试
- [ ] Application services 可以通过 mock 基础设施接口进行测试
- [ ] Controllers 可以在独立于业务逻辑的情况下测试
- [ ] 查询流程 services 可以通过 mock Providers（返回 DAOs，无需 domain 设置）进行测试

---

## 新部分（New Sections）

<!-- INTERVIEW GUIDANCE:
在访谈结束时，询问：
"是否有任何你想添加的项目特定部分不在默认值中？
常见添加：
- 命名约定（文件命名、类命名、方法命名模式）
- 框架特定规则（例如，NestJS 模块结构、Spring Bean 约定）
- 团队协议（例如，PR 审查检查项、代码所有权规则）
- 错误处理模式（错误如何在层之间传播）
- 测试模式（测试文件组织、mocking 策略）"

如果用户想要添加部分，从 7 开始编号。
新部分在 overlay 和 override 模式下都有效。
-->

---

## 页脚（Footer）

<!-- INTERVIEW GUIDANCE:
在输出中包含项目名称、生成日期和模式指示器。
示例：

---
*为 [项目名称] 生成于 [日期]。模式：[overlay|override]*
*由 architecture-refiner skill 生成。*
-->
