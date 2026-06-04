# Clean Architecture：默认原则

## 1. 层职责（Layer Responsibilities）

| 层 | 职责 | 依赖于 | 被以下依赖 |
|-------|---------|----------|------------------|
| **Controllers / Handlers** | 将外部输入（HTTP、gRPC、CLI、事件）转换为应用调用，格式化响应 | Application Services | 无（入口点） |
| **Application Services** | 编排用例：验证、调用 domain、通过接口协调基础设施 | Domain, Infrastructure interfaces | Controllers |
| **Domain** | 业务规则、entities、value objects、domain services、domain events | 无（最内部） | Application Services, Infrastructure (通过接口) |
| **Infrastructure: Repositories** | 持久化/检索 domain 对象用于状态更改操作。实现 domain 定义的接口 | Domain (对于接口) | Application Services (注入) |
| **Infrastructure: Providers** | 获取只读操作的数据。直接返回 DAOs。无 domain 接口 | 无（具体类或 app 层接口） | Application Services (注入) |
| **Infrastructure: Other** | 外部 API、文件 I/O、消息传递、缓存、通知 | Domain (对于接口，当适用时) | Application Services (注入) |

### 典型目录映射（Typical Directory Mapping）

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

注意 `infrastructure/` 下的两个同级文件夹：`repositories/`（命令流程，实现 domain 接口）和 `providers/`（查询流程，无 domain 接口）。

---

## 2. 依赖方向（Dependency Direction）

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

Infrastructure 位于外环，即使它实现内环定义的接口。源代码依赖指向内部（infrastructure 导入 domain 接口），运行时调用向外。依赖倒置——当内层需要触发外层时使用的机制。

**跨越边界的数据**简单结构——DTOs、plain objects、primitives。将外部格式映射到 app 层类型向内，domain 对象映射到响应 DTOs 向外。隔离意味着 API 契约和 DB 模式独立演进。

---

## 3. 每层规则（Per-Layer Rules）

### 3.1 Controllers / Handlers

**什么属于这里：**
- HTTP 路由定义、请求解析
- 输入验证（格式、存在性——非业务规则）
- 响应格式化、状态码映射
- Auth 中间件集成
- 请求/响应 DTOs

**什么不属于这里：**
- 业务规则评估（"如果订单总额 > 100，应用折扣"）
- 直接 DB 调用
- 从原始输入构造 domain 对象（使用 mapper/factory）

**常见违规：**
- Controller 读取 DB、应用逻辑、写回——全部在一个方法中
- 控制器操作中的业务规则条件判断
- 直接将 domain entities 作为 JSON 返回

### 3.2 Application Services

每个 domain 概念一个服务（例如，`OrderService`、`UserService`）。每个服务同时有命令方法和查询方法，使用不同的基础设施路径。

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
- 将 Repository（命令）和 Provider（查询）注入同一个服务
- 服务根据操作决定使用哪个基础设施路径

**App services vs domain services：** App services 编排工作流，协调基础设施边界。Domain services 执行跨越 entities/value objects 的纯业务逻辑，无 I/O。

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
- 来自外层的导入
- 框架注解（@Entity、@Column、@RestController）
- DB 特定类型（ResultSet、Document、Row）
- HTTP 特定类型（Request、Response、Headers）

**常见违规：**
- Entities 使用 ORM 装饰器注解
- Domain services 直接调用 repositories 而非通过 app services 接收数据

### 3.4 Infrastructure（基础设施）

两种不同的数据访问模式，以及其他技术机制：

**Repositories（`infrastructure/repositories/`）：**
- 实现 `domain/repositories/` 中定义的接口
- 接受/返回 **domain 对象**
- 在 domain 对象和 DAOs 之间内部映射
- 专门用于状态更改操作

**Providers（`infrastructure/providers/`）：**
- domain 中无接口——契约位于 app 层或具体类
- 直接返回 **DAOs**
- 专门用于读取操作
- 优化查询性能，无需 domain 构造开销

**其他基础设施：**外部 API 客户端、文件 I/O、消息队列、缓存、通知。

**常见违规：**
- Repository 方法包含业务逻辑
- 具体基础设施类型暴露给 app services
- 对只读查询使用 Repository（不必要的映射开销）
- Provider 返回 domain entities 而非 DAOs

---

## 4. 命令和查询流程（Command and Query Flows）

每个端点属于这些流程之一。选择正确的流程是在生成代码时的第一个结构决策。

单个服务处理两种流程。`OrderService` 有命令方法（`createOrder`、`updateOrder`）通过 domain 使用 Repository，查询方法（`getOrder`、`listOrders`）直接使用 Provider。命令/查询分离是服务内的*流程*区分，而非类级别拆分。

### 4.1 命令流程（创建、更新、删除）

状态更改操作启用完整栈。Domain 层在任何状态更改持久化之前强制执行不变性、业务规则。

```
Controller（请求 DTO）
  → Application Service
    → Domain（创建/填充，强制执行业务规则）
      → Repository（接受 Domain，转换为 DAO，持久化）
```

示例演示 domain/infrastructure 边界的依赖倒置——接口在 domain 中定义，在 infrastructure 中实现：

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
  constructor(orderRepo: OrderRepository, eventPublisher: EventPublisher)

  createOrder(command: CreateOrderCommand): OrderId
    order = Order.create(command.items, command.customerId)
    orderRepo.save(order)
    eventPublisher.publishAll(order.pullDomainEvents())
    return order.id
```

### 4.2 查询流程（获取、列表、搜索）

读取操作完全绕过 domain。没有不变性需要保护，所以 domain 构造是不必要的开销。

**DAO（数据访问对象）**：简单记录镜像 DB 查询结果。不是 domain entity。Service 将 DAOs 映射到响应 DTOs，仅选择 API 消费者需要的字段。

```
Controller（请求参数）
  → Application Service
    → Provider（直接返回 DAO 到服务）
  ← Service 将 DAO 映射到响应 DTO
← Controller 返回响应 DTO
```

示例展示单个服务中的两种流程，带有显式 DTO 映射：

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
| **目的** | 持久化/检索 domain 对象用于状态更改操作 | 获取只读操作的数据 |
| **接口定义于** | `domain/repositories/` | App 层或 infrastructure 中的具体类 |
| **接受** | Domain 对象（entities、aggregates） | 原始查询参数 |
| **返回** | Domain 对象 | DAOs（数据访问对象） |
| **由以下调用** | 命令方法（创建、更新、删除） | 查询方法（获取、列表、搜索） |
| **Domain 参与** | 完整——强制执行不变性 | 无——数据流动 DB → 响应 DTO |
| **映射** | Domain ↔ DAO（双向） | DAO → 响应 DTO（单向，在服务中完成） |

### 4.4 当读取需要 Domain 时

很少，读取需要 domain 逻辑——例如，取决于 domain 状态的访问控制。在这些情况下，即使没有状态更改，也使用命令流程结构。Domain 参与由业务规则证明是合理的。

---

## 5. 示例违规和修复（Example Violations and Fixes）

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
