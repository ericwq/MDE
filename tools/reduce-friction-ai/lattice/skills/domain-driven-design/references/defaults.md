# Domain-Driven Design：默认原则（Default Principles）

嵌入的 DDD 战术模式默认值。有观点的护栏——通过 SKILL.md Config Resolution 覆盖。

## 1. Aggregate 设计规则（Aggregate Design Rules）

### Consistency Boundary（一致性边界）

Aggregate = 对象**必须在每个事务后立即保持一致**。不是"相关的东西"。具体来说：其组合状态必须在原子检查中满足不变性的对象。

询问："*如果 entity 更改，什么必须在同一时刻有效？*"只有那些属于同一个 aggregate。

### Sizing Heuristic（大小启发式）

从**最小的可能 aggregate**开始：root entity + value objects。仅当事务不变性迫使时才添加 internal entities。正在争论是否属于？几乎肯定不。

**反模式——上帝 Aggregate（God Aggregate）**：许多 internal entities、加载慢、高争用 → 分解。仅保留共享事务不变性的内容。

### Reference by Identity（通过身份引用）

Aggregates 仅通过 ID 引用其他 aggregates——从不直接对象引用。

```
class Order
  customerId: CustomerId      // ID 引用，而非 Customer object
```

### One Transaction Rule（单一事务规则）

单个业务操作在每个事务中最多修改一个 aggregate。需要两个 aggregates 原子更新？边界错误（合并）或接受通过 domain events 的最终一致性。

**反模式——跨 Aggregate 事务**：在一个事务中更新两个 aggregates → 使用 domain event + 最终一致性。

### Invariant Ownership（不变性所有权）

每个业务规则恰好属于一个 aggregate。规则跨越两个 aggregates？
1. 边界错误——应该是一个 aggregate。
2. 一个 aggregate 通过接收另一个的状态作为 value（而非 reference）拥有规则。
3. 规则是最终一致性关注点——通过 domain events + 补偿动作强制执行。

### 代码示例：Order Aggregate（订单聚合）

```
class Order                                   // Aggregate Root
  id: OrderId
  customerId: CustomerId                      // reference by ID
  status: OrderStatus
  lineItems: List<LineItem>                   // internal——共享事务不变性

  static create(customerId, items):
    guard items is not empty
    order = new Order(OrderId.generate(), customerId, DRAFT, [])
    for each item in items:
      order.addLineItem(item.productId, item.quantity, item.unitPrice)
    return order

  addLineItem(productId, quantity, unitPrice):
    guard status is DRAFT
    guard quantity > 0
    lineItems.add(new LineItem(LineItemId.generate(), productId, quantity, unitPrice))

  confirm():
    guard lineItems is not empty
    guard status is DRAFT
    status = CONFIRMED
    raise OrderConfirmed(id, customerId, total(), now())

  total(): Money
    return lineItems.sum(item => item.subtotal())
```

---

## 2. Entity 模式（Entity Patterns）

Entity 有存活于状态更改的持久身份。两个 entities 当且仅当相同身份时相等——无论属性值如何。

### Behavior-Rich Entities（行为丰富的 Entities）

Entities 封装业务规则为方法。Guard 状态转换、触发 events、强制执行不变性。

**反模式——贫血领域模型（Anemic Domain Model）**：Entity 仅数据持有者 getter/setter；所有逻辑在 services 中 → 将业务规则移动到 entity。

```
class Account
  balance: Money
  status: AccountStatus

  withdraw(amount):
    guard status is ACTIVE
    guard balance >= amount
    balance = balance.subtract(amount)
    raise FundsWithdrawn(id, amount, balance)
```

---

## 3. Value Object 模式（Value Object Patterns）

Value objects：无身份、由属性定义、不可变、在构造时自验证。操作返回新实例。

### Self-Validation（自验证）

构造函数成功？值有效。无效状态不可表示。

```
class Email
  address: String

  constructor(raw):
    guard raw matches email pattern
    address = lowercase(trim(raw))
```

### 常见 Value Object 目录（Common Value Object Catalog）

包装携带领域意义、需要验证或防止类型混淆 bug 的 primitives。不要包装低显著性值（分页大小、重试计数）。

| 概念 | 替代 | 为什么 |
|---------|----------|--------|
| **Money** | number/decimal | 携带 currency，防止混合 currency 算术 |
| **Email** | string | 自验证格式，规范化大小写 |
| **Address** | multiple strings | 分组相关字段，验证完整性 |
| **DateRange** | two dates | 强制执行 start < end，提供 overlap 逻辑 |
| **Quantity** | integer | 强制执行非负数，提供算术 |
| **Typed ID** (OrderId, CustomerId) | string/UUID | 防止传递错误的 ID 类型 |
| **Status** | string/enum | 封装有效转换 |

**反模式——原始痴迷（Primitive Obsession）**：Raw string email、number money、UUID ID → 包装 value object。

**反模式——错误识别的 Entity/Value Object（Misidentified Entity/Value Object）**：应用身份测试——*"业务上跟踪独立实例随时间变化？"*相同属性=同一事物？Value object。每个实例不同的生命周期？Entity。

---

## 4. Domain Service 规则（Domain Service Rules）

Domain service：stateless 业务逻辑**跨越多个 entities/value objects**，在任何单个 entity 中没有自然的家。

### When to Use vs NOT（何时使用与不使用）

| 使用 Domain Service | 不使用——而是 |
|---|---|
| 逻辑操作来自多个 aggregates 的数据 | Single-entity logic → entity method |
| Pure business computation，无 I/O | Orchestration/workflow → application service |
| 没有单个 entity"拥有"计算 | I/O operations → infrastructure |

### Pure Domain — No I/O（纯领域——无 I/O）

Domain service 执行纯计算。Application service 获取数据，传递给 domain service。

```
// Domain Service——纯计算
class PricingService
  calculatePrice(product, customerTier, discountRules): Money
    basePrice = product.basePrice()
    tierDiscount = customerTier.discountPercentage()
    price = basePrice.multiply(1 - tierDiscount.value)
    for each rule in discountRules:
      if rule.appliesTo(product): price = rule.apply(price)
    guard price.isPositive()
    return price
```

**反模式——泄漏领域逻辑（Leaking Domain Logic）**：业务规则在 controller 或 application service 中 → 提取到 domain object 或 domain service。

---

## 5. Domain Event 模式（Domain Event Patterns）

Domain events：过去时，描述已经发生的事情。事实，而非命令。

### Payload（有效载荷）

Event 携带 aggregate ID、描述更改的相关值、timestamp。不要将整个 aggregate 状态放在 event 中——仅包含消费者需要的内容。

### When to Raise（何时触发）

- Cross-aggregate coordination（OrderConfirmed → reserve stock）
- Notification concerns（PaymentReceived → confirmation email）
- 重大状态更改的审计跟踪
- Aggregates 之间的最终一致性

不要为 trivial 内部更改触发 events，没有其他部分响应。

### Not Event Sourcing（不是 Event Sourcing）

默认：domain events 用于通信 + 协调，而非持久化机制。Aggregates 通过 repositories 持久化。Event sourcing 单独的架构选择——不要混淆。

```
class Order
  domainEvents: List<DomainEvent>

  confirm():
    guard status is DRAFT
    guard lineItems is not empty
    status = CONFIRMED
    domainEvents.add(new OrderConfirmed(id, customerId, total(), now()))

  pullDomainEvents(): List<DomainEvent>
    events = copy(domainEvents)
    domainEvents.clear()
    return events
```

---

## 6. Repository 模式（Repository Patterns）

### Core Rules（核心规则）

- **每个 aggregate root 一个**——非 internal entities/value objects
- **Collection semantics**——接口感觉像内存集合，而非 SQL 查询
- **Domain 中的接口，Infrastructure 中的实现**
- **返回完全构成的 aggregates**——从不部分对象、DTOs、原始 db 行

### What Does NOT Belong（什么不属于）

复杂报告查询、批量操作、带复杂过滤器的搜索 → 使用 Provider（query flow）。Repository 工作：持久化 + 重建 aggregates 用于命令操作。

```
// Repository——命令流程，domain 对象
interface OrderRepository
  save(order: Order): void
  findById(id: OrderId): Order or null
  remove(order: Order): void

// Provider——查询流程，DAOs
class OrderProvider
  findOrderSummaries(customerId, page, size): List<OrderSummaryDAO>
```

---

## 7. 对象创建模式（Object Creation Patterns）

复杂 aggregate 创建应该封装验证和组装。存在多种有效方法——创建模式选择（工厂方法、独立 factory、builder）取决于组装复杂度和团队约定。参见 SKILL.md Ambiguity Signals。

### Static Factory Method（静态工厂方法——最常见）

```
class Order
  static create(customerId, items):
    guard items is not empty
    order = new Order(OrderId.generate(), customerId, DRAFT, [])
    for each item in items:
      order.addLineItem(item.productId, item.quantity, item.unitPrice)
    return order
```

### Reconstitution（重建）

Repository 实现从存储的数据重建 aggregates——绕过创建时验证（数据在首次持久化时已经有效）。

```
class Order
  static reconstitute(id, customerId, status, lineItems, createdAt): Order
    return new Order(id, customerId, status, lineItems, createdAt)
```

---

## 8. 分解指南（Decomposition Guide）

### Warning Signals（警告信号）

1. **太多 internal entities**（>3-5 个）：质疑是否都共享事务不变性。
2. **多个不相关的不变性**：规则永远不引用彼此的 entities。
3. **Methods 仅触及子集**：Root 方法仅操作一些 internal entities。
4. **加载慢/高争用**：边界太粗糙。

### Steps（步骤）

1. 列出 root 强制执行的所有不变性。
2. 按不变性参与分组 entities。
3. 识别独立组——提取候选项。
4. 提取到新 aggregate，用 ID 引用替换引用。
5. 添加 domain events 用于跨 aggregate 协调。
6. 验证每个 aggregate 可独立加载/保存。

### Example（示例）

```
// Before——Course 管理 enrollment + grading
class Course
  enrollments: List<Enrollment>    // invariant: count <= maxEnrollment
  gradebook: Gradebook             // separate concern——从不触及 enrollment

// After——separate aggregates
class Course                        // Enrollment aggregate
  enrollments: List<Enrollment>
  enroll(studentId):
    guard enrollments.count < maxEnrollment
    raise StudentEnrolled(id, studentId)

class CourseGradebook               // Grading aggregate
  courseId: CourseId                 // reference by ID
  assignments: List<Assignment>
  grades: List<Grade>
```

---

*Defaults synthesize Evans Domain-Driven Design, Vernon Implementing Domain-Driven Design, practical aggregate design heuristics.*
*默认值综合了 Evans 的领域驱动设计、Vernon 的实现领域驱动设计、实用的 aggregate 设计启发式。*
