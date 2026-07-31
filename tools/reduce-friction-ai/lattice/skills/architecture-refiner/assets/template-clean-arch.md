# 架构精炼器模板（整洁架构）

此模板定义了整洁架构风格下 `.lattice/standards/architecture.md` 输出文档的结构。它包含来自架构原子 `clean-architecture-defaults.md` 的所有默认内容，并穿插了访谈指导注释。

产出最终输出时，剥离所有 `<!-- 访谈指导： -->` 注释。最终文档是规范，不是对话记录。

---

## 前置元数据

<!-- 访谈指导：
根据用户选择的模式，选择以下两个前置元数据选项之一。
默认使用叠加模式，除非用户明确想重新定义一切。
-->

选项 A — 叠加模式（最常见）：

```yaml
---
mode: overlay
---
```

选项 B — 覆盖模式（完全替换）：

```yaml
---
mode: override
---
```

---

## 引言

<!-- 访谈指导：
包含匹配所选模式的引言。输出中只出现一个引言。
-->

**叠加引言：**

> 本文档将项目特定的自定义内容叠加在架构原子嵌入的整洁架构默认值之上。只有此处包含的章节与默认值不同——所有其他章节保持原样。
>
> 以下章节替换默认值中匹配的章节（按标题匹配）。新章节追加在默认值之后。

**覆盖引言：**

> 这些是 [项目名称] 的整洁架构原则。它们完全替换架构原子中嵌入的整洁架构默认值。

**目录**（用于覆盖模式；叠加模式只列出包含的章节）：

1. [层级职责](#1-层级职责)
2. [依赖方向](#2-依赖方向)
3. [各层级规则](#3-各层级规则)
4. [命令与查询流程](#4-命令与查询流程)
5. [示例违规与修复](#5-示例违规与修复)
6. [验证清单](#6-验证清单)

---

## 1. 层级职责

<!-- 访谈指导：
询问："你的代码库使用哪些层级？以下是标准的 4 层结构。这与你的项目匹配吗，还是你使用不同的层级？"

展示下表及目录映射。

追问问题：
- 你有额外的层级吗（例如，控制器和服务之间的中介/CQRS 处理器层）？
- 你对这些层级使用不同的名称吗（例如，用"适配器"代替"控制器"，用"用例"代替"服务"）？
- 除了仓库/提供者之外，还有基础设施子类别吗（例如，单独的 messaging、caching、external-api 文件夹）？
- 是否有用于横切关注点的共享/通用层？

可自定义：层级名称、职责、目录名称、额外层级。
固定：必须至少有内部（领域）和外部层级。向内的依赖规则必须成立。

跨章节影响：此处选择的层级名称必须在所有后续章节中一致使用。
如果添加了额外层级，§2（依赖图）和 §3（各层级规则）必须包含它们。
-->

| 层级 | 职责 | 依赖 | 被依赖方 |
|-------|---------------|------------|----------------|
| **控制器 / 处理器** | 将外部输入（HTTP、gRPC、CLI、事件）转换为应用调用并格式化响应 | 应用服务 | 无（入口点） |
| **应用服务** | 编排用例：验证、调用领域、通过接口协调基础设施 | 领域、基础设施接口 | 控制器 |
| **领域** | 业务规则、实体、值对象、领域服务、领域事件 | 无（最内层） | 应用服务、基础设施（通过接口） |
| **基础设施：仓库** | 为状态变更操作持久化和检索领域对象。实现领域定义的接口 | 领域（用于接口） | 应用服务（注入） |
| **基础设施：提供者** | 为只读操作获取数据。直接返回 DAO。无领域接口 | 无（具体类或应用层接口） | 应用服务（注入） |
| **基础设施：其他** | 外部 API、文件 I/O、消息、缓存、通知 | 领域（用于接口，如适用） | 应用服务（注入） |

### 典型目录映射

```
src/
├── controllers/        # 或 handlers/、routes/、api/
│   ├── UserController
│   └── OrderController
├── services/           # 或 usecases/、application/
│   ├── OrderService                # 处理命令和查询两种流程
│   └── UserService                 # 每个领域概念一个服务
├── domain/             # 或 core/、model/
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
│   └── repositories/   # 仅接口 -- 用于状态变更操作
│       ├── OrderRepository
│       └── UserRepository
└── infrastructure/     # 或 adapters/、persistence/
    ├── repositories/   # 状态变更：实现领域定义的接口
    │   ├── PostgresOrderRepository  (实现 OrderRepository)
    │   └── PostgresUserRepository   (实现 UserRepository)
    ├── providers/      # 只读：无领域接口，返回 DAO
    │   ├── UserProvider
    │   └── OrderProvider
    ├── external/
    │   └── StripePaymentGateway
    └── messaging/
        └── KafkaEventPublisher
```

注意 `infrastructure/` 下的两个同级文件夹：`repositories/`（用于命令流，实现领域接口）和 `providers/`（用于查询流，无领域接口）。

---

## 2. 依赖方向

<!-- 访谈指导：
询问："标准规则是依赖只指向内部。你的项目遵循这个规则吗，还是有任何例外？"

展示下方的 ASCII 图。

追问问题：
- 你的项目如何处理依赖反转？DI 容器、手动注入还是框架提供的？
- 跨边界的数据使用什么格式？DTO、纯对象、基本类型？
- 是否有对仅向内规则的故意例外？

可自定义：DI 机制、数据跨边界格式、额外注释。
固定：核心的仅向内依赖规则是不可协商的。

如果用户在 §1 中添加了额外层级，更新图表以包含它们。
-->

```
┌──────────────────────────────────────────────────┐
│  控制器 / 处理器                  （最外层）        │
│    │                                             │
│    ▼                                             │
│  应用服务                                        │
│    │                                             │
│    ▼                                             │
│  领域                              （最内层）      │
│    ▲                                             │
│    │ 实现接口                                    │
│  基础设施                        （外部）          │
└──────────────────────────────────────────────────┘

依赖只能向内流动。
基础设施依赖领域（它实现领域接口）。
领域不依赖自身之外的任何东西。
```

基础设施位于外环，但它实现的是内环定义的接口。源代码依赖指向内部（基础设施导入领域接口），而运行时调用指向外部。这就是依赖反转——每当内层需要触发外层时使用的机制。

**跨边界的数据**应为简单结构——DTO、纯对象、基本类型。向内将外部格式映射为应用层类型，向外将领域对象映射为响应 DTO。这种隔离意味着 API 契约和数据库模式可以独立演进。

---

## 3. 各层级规则

<!-- 访谈指导：
逐个浏览每个子章节。对于每一层，展示默认值并询问：
"这与你的团队使用这一层的方式匹配吗？有什么要添加、删除或更改的？"

此处的关键决策会波及 §4：
- §3.2 服务模式（统一服务 vs CQRS 处理器）影响 §4 中的命令和查询两种流程。
- §3.4 提供者模式（是/否）影响 §4.2、§4.3。

如果在 §1 中添加了额外层级，为每个额外层级添加新的子章节（3.5、3.6 等）。

可自定义：每层的所有项目符号。可以添加/删除项目。
固定：每层必须有"属于这里"、"不属于这里"和"常见违规"。
-->

### 3.1 控制器 / 处理器

**属于这里的内容：**
- HTTP 路由定义和请求解析
- 输入验证（格式、存在性——不是业务规则）
- 响应格式化和状态码映射
- 认证中间件集成
- 请求/响应 DTO

**不属于这里的内容：**
- 业务规则评估（"如果订单总额 > 100，应用折扣"）
- 直接数据库调用
- 从原始输入构造领域对象（使用映射器或工厂）

**常见违规：**
- 控制器读取数据库、应用逻辑、写回——全在一个方法中
- 控制器动作中的业务规则条件判断
- 直接将领域实体作为 JSON 返回

### 3.2 应用服务

<!-- 访谈指导：
关键决策：你的团队使用统一服务模式（每个领域概念一个服务，同时包含命令和查询方法）还是 CQRS 模式（分离的命令处理器和查询处理器）？

此决策影响 §4（命令和查询流程）。记录选择并传导下去。
-->

每个领域概念一个服务（例如 `OrderService`、`UserService`）。每个服务同时包含命令方法和查询方法，对每种方法使用不同的基础设施路径。

**命令方法（状态变更——创建、更新、删除）：**
- 编排：验证 → 创建/重建领域 → 通过仓库持久化 → 发布事件
- 事务边界管理
- 授权检查
- 通过领域定义的仓库接口调用基础设施

**查询方法（读取——获取、列表、搜索）：**
- 调用提供者获取 DAO 数据
- 将 DAO 映射为响应 DTO
- 不构造领域对象

**服务构造函数模式：**
- 将仓库（用于命令）和提供者（用于查询）注入同一个服务
- 服务根据操作类型决定使用哪种基础设施路径

**应用服务 vs 领域服务：** 应用服务编排工作流，协调基础设施边界。领域服务执行跨越实体/值对象的纯业务逻辑，无 I/O。

**常见违规：**
- 服务包含所有业务逻辑而实体只是数据容器（贫血领域模型）
- 导入具体仓库类而非接口
- 为读操作构造领域对象，而提供者已足够

### 3.3 领域

**属于这里的内容：**
- 具有行为的实体（不仅仅是数据）
- 值对象（Money、Email、OrderId——不可变，按属性判等）
- 领域服务（不能自然归属单个实体的业务逻辑）
- 领域事件（OrderPlaced、PaymentReceived）
- 仓库接口（基础设施实现的契约）
- 用于复杂对象创建的工厂方法

**不属于这里的内容：**
- 从任何外层导入
- 框架注解（@Entity、@Column、@RestController）
- 数据库特定类型（ResultSet、Document、Row）
- HTTP 特定类型（Request、Response、Headers）

**常见违规：**
- 实体用 ORM 装饰器注解
- 领域服务直接调用仓库，而不是通过应用服务接收数据

### 3.4 基础设施

<!-- 访谈指导：
关键决策：你的项目在读操作中使用提供者模式吗？

如果是：提供者位于 infrastructure/providers/ 中，返回 DAO，无领域接口。
如果不是：所有数据访问通过仓库。这会改变 §4.2、§4.3 和 §6。

记录选择并传导到 §4。
-->

两种不同的数据访问模式，以及其他技术机制：

**仓库（`infrastructure/repositories/`）：**
- 实现在 `domain/repositories/` 中定义的接口
- 接受并返回**领域对象**
- 内部在领域对象和 DAO 之间映射
- 仅用于状态变更操作

**提供者（`infrastructure/providers/`）：**
- 领域中没有接口——契约存在于应用层或作为具体类
- 直接返回 **DAO**
- 仅用于读操作
- 优化查询性能，无需领域构造开销

**其他基础设施：** 外部 API 客户端、文件 I/O、消息队列、缓存、通知。

**常见违规：**
- 仓库方法包含业务逻辑
- 具体的基础设施类型暴露给应用服务
- 使用仓库进行只读查询（不必要的映射开销）
- 提供者返回领域实体而非 DAO

---

## 4. 命令与查询流程

<!-- 访谈指导：
此章节取决于 §3 的决策：
- 如果 §3.2 选择了统一服务：按如下所示呈现流程。
- 如果 §3.2 选择了 CQRS：调整流程使用分离的命令/查询处理器。
- 如果 §3.4 选择了无提供者模式：调整 §4.2 和 §4.3 使用仓库进行读操作。

逐个浏览每个子章节。展示流程图和伪代码，然后询问：
"这与你的项目处理 [命令/查询] 的方式匹配吗？"
-->

每个端点归入以下两种流程之一。选择正确的流程是生成代码时的首要结构决策。

单个服务处理两种流程。`OrderService` 具有命令方法（`createOrder`、`updateOrder`），通过领域使用仓库；查询方法（`getOrder`、`listOrders`）直接使用提供者。命令/查询分离是服务内部的*流程*区分，而非类级别的拆分。

### 4.1 命令流程（创建、更新、删除）

状态变更操作使用完整技术栈。领域层在状态变更持久化前强制执行不变量和业务规则。

```
控制器（请求 DTO）
  → 应用服务
    → 领域（创建/重建，执行业务规则）
      → 仓库（接受领域对象，转换为 DAO，持久化）
```

以下示例展示了领域/基础设施边界上的依赖反转——接口在领域中定义，在基础设施中实现：

```
// domain/repositories/ -- 领域中的接口
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

读操作完全绕过领域。没有不变量需要保护，因此领域构造是不必要的开销。

```
控制器（请求参数）
  → 应用服务
    → 提供者（将 DAO 直接返回给服务）
  ← 服务将 DAO 映射为响应 DTO
← 控制器返回响应 DTO
```

以下示例展示了单个服务中两种流程以及显式的 DTO 映射：

```
// 应用服务 -- 同时使用仓库（命令）和提供者（查询）
class UserService
  constructor(userRepo: UserRepository, userProvider: UserProvider)
    // userRepo 用于状态变更操作
    // userProvider 用于读操作

  // 命令流程：经过领域，使用仓库
  registerUser(command: RegisterUserCommand): String
    user = User.create(command.email, command.name)
    userRepo.save(user)
    return user.id.value

  // 查询流程：绕过领域，使用提供者，将 DAO 映射为响应 DTO
  getUser(userId: String): UserResponse
    dao = userProvider.findById(userId)
    if dao is null: throw NotFoundError("用户未找到")
    return UserResponse.fromDAO(dao)

  listActiveUsers(page: Integer, size: Integer): List<UserResponse>
    daos = userProvider.listActive(page, size)
    return daos.map(UserResponse.fromDAO)

// 基础设施 -- 提供者（在 infrastructure/providers/ 中）
// 无领域接口。具体类或应用层接口。
class UserProvider
  findById(id: String): UserDAO or null
    return db.findOne("users", { id })

  listActive(page: Integer, size: Integer): List<UserDAO>
    return db.query(
      "SELECT * FROM users WHERE active = true LIMIT ? OFFSET ?",
      [size, page * size]
    )

// 响应 DTO -- 显式字段选择，不是 DAO 的透传。
// 数据库内部字段被剥离；字段名称根据 API 契约来塑造。
class UserResponse
  id: String
  name: String
  email: String
  active: Boolean

  static fromDAO(dao: UserDAO): UserResponse
    return new UserResponse(dao.id, dao.name, dao.email, dao.active)
    // dao.passwordHash、dao.internalFlags、dao.createdAt 被有意排除
```

### 4.3 提供者 vs 仓库：结构比较

| 方面 | 仓库 | 提供者 |
|--------|-----------|----------|
| **目的** | 持久化和检索领域对象用于状态变更操作 | 获取数据用于只读操作 |
| **接口定义位置** | `domain/repositories/` | 应用层或基础设施中的具体类 |
| **接受** | 领域对象（实体、聚合） | 基本查询参数 |
| **返回** | 领域对象 | DAO（数据访问对象） |
| **调用方** | 命令方法（创建、更新、删除） | 查询方法（获取、列表、搜索） |
| **领域参与** | 完全——强制执行不变量 | 无——数据流为 数据库 → 响应 DTO |
| **映射** | 领域 ↔ DAO（双向） | DAO → 响应 DTO（单向，在服务中完成） |

### 4.4 当读操作需要领域时

少数情况下，读操作需要领域逻辑——例如，取决于领域状态的访问控制。在这些情况下，即使没有状态变更也使用命令流结构。领域的参与由业务规则所证明是合理的。

---

## 5. 示例违规与修复

<!-- 访谈指导：
展示下面四组坏/好对照。询问：
"是否有你想添加的项目特定反模式？常见示例：
- 框架特定违规（例如，将业务逻辑放在中间件中）
- 团队特定模式，曾导致过问题
- 来自过去事故或代码评审的模式"

可自定义：可以添加新的违规/修复对照。可以修改现有示例以使用项目术语。
固定：四个核心违规应保留，除非用户有强烈理由删除。
-->

### 控制器中的业务逻辑

```
// 坏：控制器做出业务决策
class OrderController
  createOrder(request):
    items = request.body.items
    total = 0
    for each item in items:
      total = total + item.price * item.quantity
      if item.quantity > 100: total = total * 0.9   // 控制器中的业务规则
    if total > 10000: throw Error("超出订单限额")  // 业务规则
    db.query("INSERT INTO orders...", [total])       // 直接数据库访问

// 好：控制器委托给服务
class OrderController
  constructor(orderService: OrderService)

  handle(request):
    command = CreateOrderCommand.fromRequest(request)
    result = orderService.createOrder(command)
    return OrderResponse.from(result)
```

### 领域依赖基础设施

```
// 坏：领域实体依赖基础设施
class Order
  dbClient = new DatabaseClient()

  calculateTotal(): Money
    taxRate = dbClient.findConfig("tax_rate")   // 领域内的 I/O — 违规
    // ...

// 好：领域定义接口，接收数据
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

### 数据格式泄露

```
// 坏：数据库模型直接从 API 返回
class UserController
  getUser(request):
    user = db.findOne("users", { id: request.params.id })
    return user  // 暴露了 passwordHash、内部 ID、数据库列名

// 好：在边界处映射
class UserController
  constructor(userService: UserService)

  handle(request):
    return userService.getUser(request.params.id)   // 返回响应 DTO
```

### 上帝类

```
// 坏：一个类做所有事情——验证、HTTP、业务规则、持久化、邮件、消息
class OrderService
  createOrder(data):
    // 130+ 行代码覆盖 7 种职责

// 好：按职责和层级分解
// domain/entities/Order              -- 业务规则
// domain/repositories/OrderRepository -- 持久化接口
// infrastructure/repositories/PostgresOrderRepository -- 持久化实现
// infrastructure/external/InventoryClient -- 外部 API
// infrastructure/messaging/OrderEventPublisher -- 消息
// services/OrderService              -- 仅编排
```

---

## 6. 验证清单

<!-- 访谈指导：
展示下面六个分组。询问：
"AI 在生成或审查代码时应该检查所有这些吗？有什么要添加或删除的吗？"

如果用户在 §3.4 中选择了不使用提供者模式，删除或调整
"命令/查询流程分离"组以反映仅仓库的数据访问。

可自定义：可以添加或删除个别检查项。可以添加新分组。
固定：必须至少有层级位置和依赖方向分组。
-->

在生成或审查代码后使用此清单。每项映射到一个结构原则。

### 层级位置

- [ ] 业务逻辑（规则、计算、决策）在领域层
- [ ] 用例编排在应用服务中
- [ ] HTTP/传输关注点仅在控制器中
- [ ] 数据库和外部 API 细节仅在基础设施中

### 依赖方向

- [ ] 领域层对外层零导入
- [ ] 应用服务依赖领域和接口（而非具体基础设施）
- [ ] 基础设施实现领域定义的接口
- [ ] 层级之间无循环依赖

### 边界完整性

- [ ] 向内穿越边界的数据映射为领域类型（而非原始请求对象）
- [ ] 向外穿越边界的数据映射为响应 DTO（而非领域实体）
- [ ] 框架类型（ORM 模型、HTTP 请求对象）不出现在领域中

### 命令/查询流程分离

- [ ] 状态变更操作使用命令流：控制器 → 服务 → 领域 → 仓库
- [ ] 读操作使用查询流：控制器 → 服务 → 提供者 → DAO → 响应 DTO
- [ ] 仓库接口定义在 `domain/repositories/` 中
- [ ] 提供者契约不定义在领域中
- [ ] 仓库接受并返回领域对象；提供者返回 DAO
- [ ] 查询流中非经明确理由不构造领域对象
- [ ] 服务将 DAO 直接映射为响应 DTO，无需经过领域

### 单一职责

- [ ] 每个类/模块只有一个变更原因
- [ ] 没有类跨越多个层级
- [ ] 基础设施类不包含业务规则

### 可测试性

- [ ] 领域逻辑可以在不模拟 I/O 的情况下单元测试
- [ ] 应用服务可以通过模拟基础设施接口来测试
- [ ] 控制器可以独立于业务逻辑进行测试
- [ ] 查询流服务可以通过模拟提供者来测试（返回 DAO，无需领域设置）

---

## 新章节

<!-- 访谈指导：
在访谈结束时，询问：
"是否有默认值未涵盖的、你想添加的项目特定章节？
常见添加：
- 命名约定（文件命名、类命名、方法命名模式）
- 框架特定规则（例如，NestJS 模块结构、Spring Bean 约定）
- 团队协议（例如，PR 审查清单项、代码所有权规则）
- 错误处理模式（错误如何跨层传播）
- 测试模式（测试文件组织、模拟策略）"

如果用户想添加章节，从 7 开始编号。
新章节在叠加和覆盖两种模式下都可以使用。
-->

---

## 页脚

<!-- 访谈指导：
在输出中包含项目名称、生成日期和模式指示器。
示例：

---
*为 [项目名称] 于 [日期] 生成。模式：[叠加|覆盖]。*
*由架构精炼器技能生成。*
-->
