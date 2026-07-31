# 整洁架构：默认原则

## 1. 层级职责

| 层级 | 职责 | 依赖 | 被依赖方 |
|-------|---------------|------------|----------------|
| **控制器 / 处理器** | 将外部输入（HTTP、gRPC、CLI、事件）转换为应用调用，格式化响应 | 应用服务 | 无（入口点） |
| **应用服务** | 编排用例：验证、调用领域、通过接口协调基础设施 | 领域、基础设施接口 | 控制器 |
| **领域** | 业务规则、实体、值对象、领域服务、领域事件 | 无（最内层） | 应用服务、基础设施（通过接口） |
| **基础设施：仓库（Repository）** | 持久化/检索领域对象用于状态变更操作。实现领域定义的接口 | 领域（用于接口） | 应用服务（注入） |
| **基础设施：提供者（Provider）** | 为只读操作获取数据。直接返回 DAO。无领域接口 | 无（具体类或应用层接口） | 应用服务（注入） |
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

注意 `infrastructure/` 下有两个同级文件夹：`repositories/`（命令流，实现领域接口）和 `providers/`（查询流，无领域接口）。

---

## 2. 依赖方向

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

基础设施位于外环，但它实现的是内环定义的接口。源代码依赖指向内部（基础设施导入领域接口），运行时调用指向外部。依赖反转——内层需要触发外层时使用的机制。

**跨边界的数据**使用简单结构——DTO、纯对象、基本类型。向内将外部格式映射为应用层类型，向外将领域对象映射为响应 DTO。隔离意味着 API 契约和数据库模式可以独立演进。

---

## 3. 各层级规则

### 3.1 控制器 / 处理器

**属于这里的内容：**
- HTTP 路由定义、请求解析
- 输入验证（格式、存在性——不是业务规则）
- 响应格式化、状态码映射
- 认证中间件集成
- 请求/响应 DTO

**不属于这里的内容：**
- 业务规则评估（"如果订单总额 > 100，应用折扣"）
- 直接数据库调用
- 从原始输入构造领域对象（使用映射器/工厂）

**常见违规：**
- 控制器读取数据库、应用逻辑、写回——全在一个方法中
- 控制器动作中的业务规则条件判断
- 直接将领域实体作为 JSON 返回

### 3.2 应用服务

每个领域概念一个服务（例如 `OrderService`、`UserService`）。每个服务同时包含命令方法和查询方法，使用不同的基础设施路径。

**命令方法（状态变更——创建、更新、删除）：**
- 编排：验证 → 创建/重建领域对象 → 通过仓库持久化 → 发布事件
- 事务边界管理
- 授权检查
- 通过领域定义的仓库接口调用基础设施

**查询方法（读取——获取、列表、搜索）：**
- 调用提供者获取 DAO 数据
- 将 DAO 映射为响应 DTO
- 不构造领域对象

**服务构造函数模式：**
- 将仓库（命令）和提供者（查询）注入同一个服务
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
- 从外层导入
- 框架注解（@Entity、@Column、@RestController）
- 数据库特定类型（ResultSet、Document、Row）
- HTTP 特定类型（Request、Response、Headers）

**常见违规：**
- 实体用 ORM 装饰器注解
- 领域服务直接调用仓库，而不是通过应用服务接收数据

### 3.4 基础设施

两种不同的数据访问模式，以及其他技术机制：

**仓库（`infrastructure/repositories/`）：**
- 实现在 `domain/repositories/` 中定义的接口
- 接受/返回**领域对象**
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

## 4. 命令和查询流程

每个端点归入以下流程之一。选择正确的流程是生成代码时的首要结构决策。

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
  constructor(orderRepo: OrderRepository, eventPublisher: EventPublisher)

  createOrder(command: CreateOrderCommand): OrderId
    order = Order.create(command.items, command.customerId)
    orderRepo.save(order)
    eventPublisher.publishAll(order.pullDomainEvents())
    return order.id
```

### 4.2 查询流程（获取、列表、搜索）

读操作完全绕过领域。没有不变量需要保护，因此领域构造是不必要的开销。

**DAO（数据访问对象）**：简单的记录，反映数据库查询结果。不是领域实体。服务将 DAO 映射为响应 DTO，仅选择 API 消费者需要的字段。

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
| **目的** | 持久化/检索领域对象用于状态变更操作 | 获取数据用于只读操作 |
| **接口定义位置** | `domain/repositories/` | 应用层或基础设施中的具体类 |
| **接受** | 领域对象（实体、聚合） | 基本查询参数 |
| **返回** | 领域对象 | DAO（数据访问对象） |
| **调用方** | 命令方法（创建、更新、删除） | 查询方法（获取、列表、搜索） |
| **领域参与** | 完全——强制执行不变量 | 无——数据流为 数据库 → 响应 DTO |
| **映射** | 领域 ↔ DAO（双向） | DAO → 响应 DTO（单向，在服务中完成） |

### 4.4 当读操作需要领域时

少数情况下，读操作需要领域逻辑——例如，取决于领域状态的访问控制。这些情况使用命令流结构，即使没有状态变更。领域的参与由业务规则所证明是合理的。

---

## 5. 违规示例与修复

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
