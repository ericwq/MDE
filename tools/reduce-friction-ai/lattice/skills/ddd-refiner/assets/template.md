# DDD Refiner 模板

此模板定义 `.lattice/standards/ddd-principles.md` 输出文档的结构。它包含来自 domain-driven-design atom 的 `defaults.md` 的所有默认内容，穿插访谈指导注释。

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

> 此文档在 domain-driven-design atom 的内置默认值之上叠加项目特定的自定义。此处包含的部分与默认值不同——所有其他部分保持不变。
>
> 下面的部分替换默认值中匹配的部分（按标题匹配）。新部分追加在默认值之后。

**Override 序言：**

> 这是 [项目名称] 的领域驱动设计原则。它们完全替换 domain-driven-design atom 中的内置默认值。

**目录**（对于 override 模式；overlay 模式仅列出包含的部分）：

1. [Aggregate 设计规则](#1-aggregate-设计规则)
2. [Entity 模式](#2-entity-模式)
3. [Value Object 模式](#3-value-object-模式)
4. [Domain Service 规则](#4-domain-service-规则)
5. [Domain Event 模式](#5-domain-event-模式)
6. [Repository 模式](#6-repository-模式)
7. [对象创建模式](#7-对象创建模式)
8. [分解指南](#8-分解指南)
9. [验证清单——详细](#9-验证清单详细)

---

## 1. Aggregate 设计规则（Aggregate Design Rules）

<!-- INTERVIEW GUIDANCE:
询问："你如何思考 aggregate 边界？以下是默认方法——aggregates 是 consistency boundaries，而非 convenience groupings。这匹配你们团队的思考吗？"

显示 consistency boundary 原则和 sizing heuristic。

追问问题：
- 你当前的 aggregates 有多大？是否有任何感觉太大或加载太慢？
- 是否有任何情况单个事务更新多个 aggregates？
- 你如何处理 aggregate 之间的最终一致性？Domain events？Sagas？
- 是否有任何 aggregates 经常被多个用户争用？

可自定义：大小阈值、引用 by ID 的例外、事务边界方法、代码示例。
固定：Consistency boundary 原则是不可协商的。Aggregates 必须通过 root 强制执行不变性。

跨部分影响：此处定义的 aggregate 边界影响§6（每个 aggregate root 一个 repository）、§5（跨 aggregate events）和§8（分解触发器）。
-->

Aggregate 边界是 DDD 中最困难的设计决策。其他一切都是从正确获取它们中衍生出来的。

### Consistency Boundary 原则（一致性边界原则）

Aggregate 是**必须在每个事务后立即保持一致的**对象集合。不是"相关的东西"。不是"共享数据库表的东西"。具体来说：其组合状态必须在原子检查中满足不变性的对象。

询问："*如果此 entity 更改，什么必须在同一时刻有效？*"只有那些对象属于同一个 aggregate。

### Sizing Heuristic（大小启发式）

从**最小的可能 aggregate**开始：一个 root entity 加上它的 value objects。仅当事务不变性迫使它们进入时才添加内部 entities。如果你正在争论某物是否应该在里面，它几乎肯定不应该。

Small aggregates 加载快、冲突少、扩展性好。Large aggregates 慢、争用多、脆弱。

**反模式——God Aggregate（上帝 Aggregate）**：许多内部 entities，加载慢、争用多 → 分解。仅保留共享事务不变性的内容。

### Reference by Identity（通过身份引用）

Aggregates 仅通过 ID 引用其他 aggregates——从不通过直接对象引用。对象引用创建隐式耦合、扩大事务范围，并使独立分布 aggregates 成为不可能。

```
// 错误：Order 持有对 Customer 的直接引用
class Order
  customer: Customer          // 将 Customer 拉入 Order 的事务范围

// 正确：Order 持有 Customer 的身份
class Order
  customerId: CustomerId      // 松耦合，独立的事务范围
```

### One Transaction Rule（单一事务规则）

每个 aggregate 定义一个事务边界。单个业务操作应该在每个事务中最多修改一个 aggregate。如果你需要两个 aggregates 原子更新，要么边界是错误的（合并它们），或者你应该通过 domain events 接受最终一致性。

**反模式——跨 Aggregate 事务**：在一个事务中更新两个 aggregates → 使用 domain event + 最终一致性。

### Invariant Ownership（不变性所有权）

每个业务规则恰好属于一个 aggregate——那个强制执行它的 root。如果一条规则真正跨越两个 aggregates，以下三种情况之一为真：
1. 边界是错误的——它们应该是一个 aggregate。
2. 一个 aggregate 可以通过接收另一个的状态作为 value（而非 reference）拥有该规则。
3. 该规则是最终一致性关注点——通过 domain events 和补偿动作强制执行。

### 代码示例：带有 LineItems 的 Order Aggregate

LineItems 在 Order aggregate 内部，因为不变性"订单总额必须等于行项目小计的总和"需要原子一致性。

```
class Order                                   // Aggregate Root
  id: OrderId
  customerId: CustomerId                      // reference by ID，而非 object
  status: OrderStatus
  lineItems: List<LineItem>                   // internal entity——在 aggregate 内部

  static create(customerId, items):
    guard items is not empty
    order = new Order(OrderId.generate(), customerId, DRAFT, [])
    for each item in items:
      order.addLineItem(item.productId, item.quantity, item.unitPrice)
    return order

  addLineItem(productId, quantity, unitPrice):
    guard status is DRAFT
    guard quantity > 0
    lineItem = new LineItem(LineItemId.generate(), productId, quantity, unitPrice)
    lineItems.add(lineItem)

  removeLineItem(lineItemId):
    guard status is DRAFT
    guard lineItems contains lineItemId
    lineItems.remove(lineItemId)

  confirm():
    guard lineItems is not empty
    guard status is DRAFT
    status = CONFIRMED
    raise OrderConfirmed(id, customerId, total(), confirmedAt: now())

  total(): Money
    return lineItems.sum(item => item.subtotal())

class LineItem                                // Internal Entity
  id: LineItemId
  productId: ProductId
  quantity: Quantity
  unitPrice: Money

  subtotal(): Money
    return unitPrice.multiply(quantity.value)
```

Customer 是一个单独的 aggregate，通过 `CustomerId` 引用。它有自己独立的生命周期、自己的不变性和自己的事务边界。加载 Order 永远不需要加载 Customer。

### 代码示例：分解 God Aggregate

之前——Shipment 错误地在 Order 内部：

```
class Order
  lineItems: List<LineItem>
  shipment: Shipment          // 与 lineItems 无共享不变性
  trackingHistory: List<TrackingEvent>  // 独立于 Order 增长
```

之后——Shipment 提取到自己的 aggregate：

```
class Order
  lineItems: List<LineItem>
  // shipment removed——无共享事务不变性

class Shipment                // separate Aggregate Root
  id: ShipmentId
  orderId: OrderId            // reference by ID 引用 Order
  trackingHistory: List<TrackingEvent>

  recordTrackingEvent(event):
    trackingHistory.add(event)
    if event.type is DELIVERED:
      raise ShipmentDelivered(id, orderId)
```

Order 和 Shipment 独立演进。当 shipment 交付时，domain event 通知 Order context（如果需要）。

---

## 2. Entity 模式（Entity Patterns）

<!-- INTERVIEW GUIDANCE:
询问："你如何处理 entity 身份和生命周期？默认值：entities 有 typed identifiers、behavior-rich methods 和强制执行的状态转换。这匹配你的项目吗？"

追问问题：
- 你当前的 entities 目前有行为还是主要是数据持有者（anemic）？
- 你如何处理身份——typed IDs、raw UUIDs、数据库生成的 IDs？
- 你使用状态机进行 entity 生命周期转换吗？
- 是否有任何领域特定的生命周期模式（例如，审批工作流、多步骤流程）？

可自定义：身份策略、生命周期模式、特定的 guard clause 约定、代码示例。
固定：Entities 必须有基于身份的相等性，必须封装行为（不能是 anemic）。

跨部分影响：Entity 身份模式影响§3（typed ID value objects）、§6（repository findById 签名）。
-->

### Identity（身份）

Entity 有一个持久的身份，它存活于状态更改。无论其状态是 DRAFT 还是 CONFIRMED，Order 始终是同一个 Order。身份通常是 typed identifier（value object 包装原始 ID）。

### Equality（相等性）

两个 entities 当且仅当它们有相同的身份时才相等——无论它们的属性值如何。id=123 的 Order 是同一个 entity，无论其总额是$50 还是$500。

```
class Entity
  equals(other):
    return this.id == other.id

  hashCode():
    return hash(this.id)
```

### Behavior-Rich Entities（行为丰富的 Entities）

Entities 封装业务规则为方法。如果 entity 只有 getters 和 setters，应该生活在它内部的逻辑已经泄漏到其他地方（通常在 application services 中）。

**反模式——贫血领域模型**：Entity 仅数据持有者 getter/setter；所有逻辑在 services 中 → 将业务规则移动到 entity 中。

```
// 错误：贫血 entity——仅数据持有者
class Account
  balance: Money
  status: AccountStatus

// Service 做所有工作
class AccountService
  withdraw(accountId, amount):
    account = repository.findById(accountId)
    if account.status != ACTIVE: throw InactiveAccountError
    if account.balance < amount: throw InsufficientFundsError
    account.balance = account.balance - amount
    repository.save(account)

// 正确：rich entity——行为和规则在内部
class Account
  balance: Money
  status: AccountStatus

  withdraw(amount):
    guard status is ACTIVE else throw InactiveAccountError
    guard balance >= amount else throw InsufficientFundsError
    balance = balance.subtract(amount)
    raise FundsWithdrawn(id, amount, balance)
```

### Lifecycle（生命周期）

Entities 有生命周期：创建→状态转换→可能的停用或完成。每个转换应该强制执行其前提条件。

```
class Order
  // 创建
  static create(customerId, items): Order

  // 状态转换——每个有前提条件
  confirm():
    guard status is DRAFT
    status = CONFIRMED

  ship(trackingNumber):
    guard status is CONFIRMED
    status = SHIPPED

  cancel():
    guard status in [DRAFT, CONFIRMED]  // 不能取消已发货的订单
    status = CANCELLED
```

---

## 3. Value Object 模式（Value Object Patterns）

<!-- INTERVIEW GUIDANCE:
询问："你的项目中哪些领域概念应该是 value objects 而非 primitives？以下是常见的。哪些适用，你想添加什么？"

显示常见 value object 目录表。

追问问题：
- 你当前是否对领域概念如 money、email、IDs 使用原始字符串/数字？
- 是否有任何领域特定的 value objects 独特于你的业务（例如，PolicyNumber、ISIN、SKU）？
- 在构造时验证应该有多严格？
- 你使用 typed IDs 用于 aggregate roots 吗？用于所有 entities？

可自定义：Value object 目录（添加/删除项目）、验证严格性、特定的 value object 实现。
固定：Value objects 必须不可变且自验证。相等性必须基于属性。

跨部分影响：此处选择的 value objects 在§1（aggregate 示例）、§2（typed IDs）中 throughout 使用。
-->

### Attributes Define It（属性定义它）

Value object 没有身份。它完全由其属性定义。两个 amount=10 和 currency=USD 的 Money 对象是相同的 Money——没有"哪一个"的概念。

### Immutability（不可变性）

Value objects 在创建后永不更改。会"修改"value object 的操作返回新实例而非更改现有实例。这消除 aliasing bugs 并使它们安全共享。

```
class Money
  amount: Decimal               // 构造后不可变
  currency: Currency

  add(other: Money): Money
    guard currency == other.currency
    return new Money(amount + other.amount, currency)

  subtract(other: Money): Money
    guard currency == other.currency
    guard amount >= other.amount
    return new Money(amount - other.amount, currency)

  multiply(factor: Number): Money
    return new Money(amount * factor, currency)
```

### Self-Validation（自验证）

Value object 在构造时自我验证。如果构造函数成功，值是有效的。无效状态是不可表示的。

```
class Email
  address: String

  constructor(raw: String):
    guard raw matches email pattern else throw InvalidEmailError
    guard length(raw) <= 254 else throw InvalidEmailError
    address = lowercase(trim(raw))

  localPart(): String
    return address.split("@")[0]

  domain(): String
    return address.split("@")[1]
```

### Equality（相等性）

当所有属性都相等时，两个 value objects 相等。无身份比较。

```
class Money
  equals(other):
    return this.amount == other.amount
       and this.currency == other.currency
```

### 常见 Value Object 目录（Common Value Object Catalog）

这些领域概念几乎总是 value objects，而非原始 primitives：

| 概念 | 替代 | 为什么 |
|---------|----------|--------|
| **Money** | number/decimal | 携带 currency，防止混合 currency 算术 |
| **Email** | string | 自验证格式，规范化大小写 |
| **PhoneNumber** | string | 验证格式，规范化国家代码 |
| **Address** | multiple strings | 分组相关字段，验证完整性 |
| **DateRange** | two dates | 强制执行 start < end，提供 overlap/contains 逻辑 |
| **TimeSlot** | two times | 强制执行 start < end，防止 overlap |
| **Quantity** | integer | 强制执行非负数，提供算术 |
| **Percentage** | number | 强制执行 0-100 范围（或 0-1），防止误用 |
| **Typed ID** (OrderId, CustomerId) | string/UUID | 防止将错误的 ID 类型传递给错误的方法 |
| **Status** | string/enum | 封装有效转换，防止无效状态 |

### 代码示例：Typed Identifier（类型化标识符）

```
class OrderId
  value: UUID

  constructor(raw: UUID):
    guard raw is not null
    value = raw

  static generate(): OrderId
    return new OrderId(UUID.random())

  static from(raw: String): OrderId
    return new OrderId(UUID.parse(raw))

  equals(other: OrderId): Boolean
    return this.value == other.value

  toString(): String
    return value.toString()
```

Typed identifiers 防止一类 bug，其中 CustomerId 意外传递给期望 OrderId 的地方。类型系统在编译时捕获这个。

**反模式——Primitive Obsession（原始痴迷）**：Raw string email、number money、UUID ID → 包装 value object。

**反模式——Misidentified Entity/Value Object（错误识别的 Entity/Value Object）**：应用身份测试——*"业务上跟踪独立实例随时间变化？"*相同属性=同一事物？Value object。每个实例不同的生命周期？Entity。

### 代码示例：作为 Value Object 的 Status（带行为）

```
class OrderStatus
  value: String                // DRAFT, CONFIRMED, SHIPPED, DELIVERED, CANCELLED

  static DRAFT = new OrderStatus("DRAFT")
  static CONFIRMED = new OrderStatus("CONFIRMED")
  static SHIPPED = new OrderStatus("SHIPPED")
  static DELIVERED = new OrderStatus("DELIVERED")
  static CANCELLED = new OrderStatus("CANCELLED")

  canTransitionTo(target: OrderStatus): Boolean
    allowed = {
      DRAFT: [CONFIRMED, CANCELLED],
      CONFIRMED: [SHIPPED, CANCELLED],
      SHIPPED: [DELIVERED],
      DELIVERED: [],
      CANCELLED: []
    }
    return target in allowed[this.value]

  transitionTo(target: OrderStatus): OrderStatus
    guard canTransitionTo(target) else throw InvalidStatusTransitionError(this, target)
    return target
```

---

## 4. Domain Service 规则（Domain Service Rules）

<!-- INTERVIEW GUIDANCE:
询问："你使用 domain services 用于跨越多个 entities 的业务逻辑吗？以下是默认规则：domain services 是 stateless、pure-computation、无 I/O。这匹配吗？"

显示 domain service vs application service 比较表。

追问问题：
- 你今天有跨越多个 entities 的业务逻辑吗？它住在哪里？
- 你的团队是否对 domain service vs application service 中什么属于有混淆？
- 你当前的任何"domain services"是否进行数据库或 API 调用？（那会使它们成为 application services。）

可自定义：示例、命名约定、项目特定的 domain service 模式。
固定：Domain services 必须是 stateless 和 pure（无 I/O）。与 application services 的区别是不可协商的。
-->

### When to Use（何时使用）

Domain service 封装**跨越多个 entities 或 value objects**的业务逻辑，且在任何单个 entity 中没有自然的家。关键测试：如果逻辑操作来自多个 aggregates 或 entities 的数据，且没有单个 entity"拥有"计算，它属于 domain service。

### When NOT to Use（何时不使用）

- **Single-entity logic** → 属于 entity 本身
- **Orchestration and workflow coordination** → 属于 application service
- **I/O operations**（数据库、HTTP、消息传递）→ 属于 infrastructure
- **Data transformation for external consumers** → 属于 application service 或 mapper

### Statelessness（无状态）

Domain services 是无状态的。它们接收所有需要的作为参数并返回结果。无内部状态，无保留对 entities 的引用。

### Pure Domain — No I/O（纯领域——无 I/O）

Domain service 执行纯业务计算。它不调用数据库、APIs 或文件系统。如果逻辑需要外部数据，application service 获取该数据并传递给 domain service。

**反模式——泄漏领域逻辑**：业务规则在 controller 或 application service 中 → 提取到 domain object 或 domain service。

### 区别：Domain Service vs Application Service（领域服务与应用服务）

| 方面 | Domain Service | Application Service |
|--------|---------------|-------------------|
| **包含** | 业务规则和计算 | 工作流编排 |
| **状态** | Stateless | Stateless |
| **I/O** | 无——纯计算 | 通过基础设施协调 I/O |
| **依赖** | 仅其他 domain objects | Domain + infrastructure interfaces |
| **示例** | 给定 product、customer tier 和 discount rules 计算价格 | 从 repo 获取 product，获取 customer，调用 pricing service，保存 order |

### 代码示例：PricingService（定价服务）

```
// Domain Service——纯业务计算，无 I/O
class PricingService

  calculatePrice(product: Product, customerTier: CustomerTier, discountRules: List<DiscountRule>): Money
    basePrice = product.basePrice()
    tierDiscount = customerTier.discountPercentage()
    priceAfterTier = basePrice.multiply(1 - tierDiscount.value)

    for each rule in discountRules:
      if rule.appliesTo(product):
        priceAfterTier = rule.apply(priceAfterTier)

    guard priceAfterTier.isPositive()
    return priceAfterTier
```

### 代码示例：什么不属于 Domain Service（领域服务）

```
// 错误：这是编排——它属于 application service
class PricingService
  constructor(productRepo, customerRepo, discountRepo)

  calculatePrice(productId, customerId):
    product = productRepo.findById(productId)       // I/O——不是 domain
    customer = customerRepo.findById(customerId)     // I/O——不是 domain
    discounts = discountRepo.findActive()            // I/O——不是 domain
    return compute(product, customer.tier, discounts)

// 正确：Application service 编排，domain service 计算
class OrderApplicationService
  constructor(productRepo, customerRepo, discountRepo, pricingService)

  createOrder(command):
    product = productRepo.findById(command.productId)
    customer = customerRepo.findById(command.customerId)
    discounts = discountRepo.findActive()
    price = pricingService.calculatePrice(product, customer.tier, discounts)
    order = Order.create(command.customerId, product, price)
    orderRepo.save(order)
```

---

## 5. Domain Event 模式（Domain Event Patterns）

<!-- INTERVIEW GUIDANCE:
询问："在你的项目中何时应该触发 domain events？默认值：过去时命名、携带足够数据、用于跨 aggregate 协调。这匹配吗？"

追问问题：
- 你当前使用 domain events 吗？如果是，什么命名约定？
- 你今天如何处理跨 aggregate 协调？直接调用？Events？Sagas？
- 你需要 event sourcing，还是仅 domain events 用于通信？
- 你的领域中哪些 events 重要？其他部分系统响应的哪些状态更改？

可自定义：命名约定、payload 结构、事件发布策略、特定的 domain events。
固定：Events 必须是过去时事实（而非 commands）。Events 必须在 domain 层定义。

跨部分影响：此处定义的 domain events 在§1（跨 aggregate 协调）中使用。
-->

### Naming Convention（命名约定）

Domain events 以**过去时**命名——它们描述领域中已经发生的事情。它们是事实，而非命令。

| 好 | 坏 |
|------|-----|
| OrderPlaced（订单已放置） | PlaceOrder（命令，不是 event） |
| PaymentReceived（付款已收到） | ProcessPayment（命令） |
| InventoryReserved（库存已保留） | ReserveInventory（命令） |
| CustomerDeactivated（客户已停用） | DeactivateCustomer（命令） |
| ShipmentDelivered（发货已交付） | DeliverShipment（命令） |

### Payload（有效载荷）

Event 携带足够的数据以描述发生的事情，而无需消费者查询回细节：

- **Aggregate ID**：哪个 aggregate 更改
- **相关值**：描述更改的数据
- **Timestamp**：何时发生
- **可选**：Correlation ID 用于追踪、actor/user ID

不要将整个 aggregate 状态放在 event 中。仅包含消费者需要的内容。

### When to Raise Events（何时触发 Events）

- **跨 aggregate 协调**：OrderConfirmed → InventoryService 保留 stock
- **通知关注点**：PaymentReceived → 发送确认邮件
- **审计跟踪**：任何业务利益相关者想要跟踪的重大状态更改
- **最终一致性**：当两个 aggregates 必须最终反映相同的业务事实时

不要为 trivial 内部状态更改触发 events，没有其他部分响应。

### Where Events Live（Events 住在哪里）

Domain events **在 domain 层定义**——它们是通用词汇的一部分。它们由 aggregate（在操作期间收集）或在持久化后由 application service 发布。

### Not Event Sourcing（不是 Event Sourcing）

默认方法是 domain events 用于**通信和协调**，而非作为持久化机制。Aggregates 通过它们的 repositories 持久化到数据库。Events 是通知系统其他部分的 side channel。

Event sourcing（将 events 持久化作为真相来源并从它们重建状态）是一个单独的架构选择，有自己的权衡。不要混淆两者。

### 代码示例：OrderConfirmed Event（订单确认事件）

```
class OrderConfirmed                          // Domain Event
  orderId: OrderId
  customerId: CustomerId
  totalAmount: Money
  confirmedAt: Timestamp

  constructor(orderId, customerId, totalAmount, confirmedAt):
    this.orderId = orderId
    this.customerId = customerId
    this.totalAmount = totalAmount
    this.confirmedAt = confirmedAt
```

### 代码示例：从 Aggregate 触发 Events（领域事件）

```
class Order
  id: OrderId
  status: OrderStatus
  domainEvents: List<DomainEvent>             // 收集，尚未发布

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

Application service 持久化 aggregate，然后发布收集的 events：

```
class OrderApplicationService
  constructor(orderRepo, eventPublisher)

  confirmOrder(orderId):
    order = orderRepo.findById(orderId)
    order.confirm()
    orderRepo.save(order)
    eventPublisher.publishAll(order.pullDomainEvents())
```

---

## 6. Repository 模式（Repository Patterns）

<!-- INTERVIEW GUIDANCE:
询问："你如何处理 aggregates 的持久化？默认值：每个 aggregate root 一个 repository、collection semantics、domain 中的接口、返回完整的 aggregates。这匹配吗？"

追问问题：
- 你有非 root entities 的 repositories 吗？（那将是违规。）
- 你的 repositories 返回部分对象或 DTOs 吗？（应该返回完整的 aggregates。）
- 你使用 Provider 模式（来自 Clean Architecture）用于读优化查询，还是所有查询通过 repositories？
- 你如何处理复杂的报告查询？

可自定义：Repository 方法约定、查询模式、特定的基础设施选择。
固定：每个 aggregate root 一个 repository。Domain 层中的接口。返回完全构成的 aggregates。

跨部分影响：此处的 Repository 模式必须与§1（每个 aggregate root 一个）对齐，并与 Clean Architecture 的 Provider 模式集成用于查询流程。
-->

### One Per Aggregate Root（每个 Aggregate Root 一个）

Repositories 仅用于 aggregate roots——不用于内部 entities 或 value objects。如果 `LineItem` 在 `Order` aggregate 内部，没有 `LineItemRepository`。你通过 `OrderRepository` 保存和加载整个 Order aggregate。

### Collection Semantics（集合语义）

将 repository 视为 aggregates 的内存集合。接口应该感觉像添加到、在集合中查找和从集合移除——而非发出 SQL 查询。

```
interface OrderRepository
  save(order: Order): void
  findById(id: OrderId): Order or null
  findByCustomerId(customerId: CustomerId): List<Order>
  remove(order: Order): void
```

### Interface in Domain, Implementation in Infrastructure（Domain 中的接口，Infrastructure 中的实现）

Repository 接口在 domain 层定义——它是一个 port。实现在 infrastructure 中，处理实际的持久化机制（SQL、ORM、document store）。这已经由 Clean Architecture 强制执行；DDD 定义语义契约。

### Returns Fully-Constituted Aggregates（返回完全构成的 Aggregates）

Repository 返回完整的 aggregates，所有内部 entities 和 value objects 正确组装。从不部分对象、从不 DTOs、从不原始数据库行。消费者接收准备好的 domain 对象，所有不变性已经满足。

```
// 错误：返回部分或原始数据
interface OrderRepository
  findById(id: OrderId): OrderDAO              // 原始数据，不是 domain
  findOrderWithoutItems(id: OrderId): Order    // 部分 aggregate

// 正确：完整 aggregate
interface OrderRepository
  findById(id: OrderId): Order or null         // 完整 aggregate
```

### What Does NOT Belong in a Repository（什么不属于 Repository）

- **复杂报告查询**：多表连接、聚合、分析 → 使用 Provider（Clean Architecture 查询流程）
- **批量操作**：大规模更新、批量删除 → 使用基础设施级操作
- **复杂过滤器的搜索**：全文搜索、分类查询 → 使用 Provider 或专用搜索基础设施

Repository 的工作是持久化和重建 aggregates 用于命令操作。读优化查询属于 Providers。

### 代码示例：Repository 接口（仓储接口）

```
interface OrderRepository
  save(order: Order): void
  findById(id: OrderId): Order or null
  remove(order: Order): void

interface CustomerRepository
  save(customer: Customer): void
  findById(id: CustomerId): Customer or null
  findByEmail(email: Email): Customer or null
```

### 代码示例：Repository vs Provider（仓储与提供者）

```
// Repository——用于命令流程，返回 domain 对象
interface OrderRepository                      // interface in domain/repositories/
  save(order: Order): void
  findById(id: OrderId): Order or null

// Provider——用于查询流程，返回 DAOs
class OrderProvider                            // concrete class in infrastructure/providers/
  findOrderSummaries(customerId, page, size): List<OrderSummaryDAO>
  findOrderDetails(orderId): OrderDetailsDAO or null
  countOrdersByStatus(status): Integer
```

---

## 7. 对象创建模式（Object Creation Patterns）

<!-- INTERVIEW GUIDANCE:
询问："你的团队如何处理 aggregate 创建？有几种有效的方法——aggregate root 上的工厂方法、独立的 factory classes、builder pattern 或普通构造函数。你的团队偏好哪个，或者你想要指导？"

这是一个 DDD atom 中的 ambiguity signal——没有规定单一模式。访谈应该浮现团队的偏好。

追问问题：
- 你的 aggregate 创建场景是简单的（构造函数/工厂方法）还是复杂的（多个数据源、多步骤组装）？
- 你的团队已经使用特定的创建模式（factory、builder）吗？
- 你需要重建以从持久化重建 aggregates 吗？
- 你有任何涉及外部验证或数据查找的创建工作流吗？

可自定义：创建模式选择（工厂方法、独立 factory、builder、普通构造函数）、创建约定、特定实现。
固定：创建必须强制执行创建时不变性。重建必须与初始创建分开。
-->

复杂的 aggregate 创建应该封装验证和组装。存在多种有效方法——创建模式选择（工厂方法、独立 factory、builder）取决于组装复杂度和团队约定。参见 SKILL.md Ambiguity Signals。

### Static Factory Method（静态工厂方法——最常见）

对于大多数情况，root 上的静态工厂方法是最简单和最好的方法。它强制执行创建不变性并返回完全有效的 aggregate。

```
class Order
  static create(customerId: CustomerId, items: List<OrderItemRequest>): Order
    guard items is not empty else throw EmptyOrderError
    guard customerId is not null

    order = new Order(
      id: OrderId.generate(),
      customerId: customerId,
      status: OrderStatus.DRAFT,
      lineItems: [],
      createdAt: now()
    )

    for each item in items:
      order.addLineItem(item.productId, item.quantity, item.unitPrice)

    return order
```

### Standalone Factory（独立工厂）

当创建需要来自多个来源的数据或组装逻辑复杂到值得其自己的类时，使用独立的 factory。

```
class LoanApplicationFactory
  constructor(creditScoreService: CreditScoreService, riskPolicy: RiskPolicy)

  create(applicant: Applicant, requestedAmount: Money, term: LoanTerm): LoanApplication
    creditScore = creditScoreService.scoreFor(applicant)
    riskLevel = riskPolicy.assess(creditScore, requestedAmount, term)

    guard riskLevel is not PROHIBITED else throw LoanProhibitedError

    return new LoanApplication(
      id: LoanApplicationId.generate(),
      applicantId: applicant.id,
      requestedAmount: requestedAmount,
      term: term,
      riskLevel: riskLevel,
      status: LoanApplicationStatus.PENDING
    )
```

注意：这里的 `creditScoreService` 是一个 domain service（从申请人数据的纯计算），而非基础设施调用。如果需要外部 I/O 来获取 credit score，application service 应该首先获取它并传入。

### Reconstitution（重建）

Repository 实现使用重建以从存储的数据重建 aggregates。这绕过创建时验证（数据在首次持久化时已经有效），但重建所有内部结构。

```
class Order
  // 由 repository 用于从持久化重建——跳过创建不变性
  static reconstitute(id, customerId, status, lineItems, createdAt): Order
    return new Order(id, customerId, status, lineItems, createdAt)
```

---

## 8. 分解指南（Decomposition Guide）

<!-- INTERVIEW GUIDANCE:
询问："你不得不打破长得太大的 aggregates 吗？默认指南提供警告信号和逐步分解过程。这匹配你的经验吗？"

追问问题：
- 是否有任何 aggregates 感觉太大或加载太慢？
- 你如何决定何时应该从 aggregate 中提取某物？
- 你有团队特定的 aggregate 大小阈值吗？

可自定义：警告信号阈值、分解步骤、特定示例。
固定：分解原则（按不变性边界分离，而非按 convenience）是不可协商的。

注意：与 aggregate 大小相关的反模式（god aggregate、跨 aggregate 事务）现在在§1 Aggregate Design Rules 中内联。
-->

### Warning Signals（警告信号）

当以下情况时，aggregate 需要分解：

1. **太多内部 entities**（超过 3-5 个）：质疑它们是否都与 root 共享事务不变性。
2. **多个不相关的不变性**：永远不引用彼此 entities 的规则可能属于单独的 aggregates。
3. **仅触及子集的方法**：如果 root 方法仅操作一些内部 entities，该子集可能是其自己的 aggregate。
4. **加载慢**："我需要加载所有内容来验证一件事"——边界太粗糙。
5. **高争用**：多个用户经常在同一 aggregate 上冲突，因为他们修改不相关的部分。
6. **增长的 entity 计数**：新功能不断向 aggregate 添加 entities 而非创建新的 aggregates。

### Step-by-Step Decomposition（逐步分解）

1. **列出所有不变性** aggregate root 当前强制执行。
2. **按不变性参与分组 entities**：哪些 entities 涉及哪些不变性？
3. **识别独立组**：参与分离、非重叠不变性的 entities 是提取的候选项。
4. **提取到新 aggregate**：为提取组创建新的 aggregate root。用 ID 引用替换直接引用。
5. **添加 domain events**：如果原始 aggregate 需要响应提取的 aggregate 中的更改（或反之），使用 domain events。
6. **验证**：每个 resulting aggregate 应该可以独立加载和保存。没有跨 aggregate 不变性需要共享事务。

### Before/After Example（前后示例）

之前——`Course` aggregate 管理 enrollment 和 grading：

```
class Course                               // God aggregate
  id: CourseId
  title: String
  maxEnrollment: Integer
  enrollments: List<Enrollment>            // invariant: count <= maxEnrollment
  gradebook: Gradebook                     // separate concern
  assignments: List<Assignment>            // separate concern

  enroll(studentId):
    guard enrollments.count < maxEnrollment
    enrollments.add(new Enrollment(studentId))

  gradeAssignment(studentId, assignmentId, score):
    // 仅触及 gradebook/assignments——从不 enrollment
    gradebook.record(studentId, assignmentId, score)
```

之后——enrollment 和 grading 是单独的 aggregates：

```
class Course                               // Enrollment aggregate
  id: CourseId
  title: String
  maxEnrollment: Integer
  enrollments: List<Enrollment>

  enroll(studentId):
    guard enrollments.count < maxEnrollment
    enrollments.add(new Enrollment(studentId))
    raise StudentEnrolled(id, studentId)

class CourseGradebook                      // Grading aggregate
  id: GradebookId
  courseId: CourseId                        // reference by ID
  assignments: List<Assignment>
  grades: List<Grade>

  gradeAssignment(studentId, assignmentId, score):
    guard assignments contains assignmentId
    grades.add(new Grade(studentId, assignmentId, score))
```

每个 aggregate 独立加载。Enrollment 争用不阻塞 grading。新的 grading 功能不会破坏 enrollment 不变性。

---

## 9. 验证清单——详细（Validation Checklist — Detailed）

<!-- INTERVIEW GUIDANCE:
显示下面的六组。询问：
"当生成或审查 domain 代码时，AI 应该检查所有这些吗？有什么要添加或删除的吗？"

可自定义：可以添加或删除单个检查。可以添加新组。
固定：必须有至少 aggregate 检查和 entity 检查组。
-->

在生成或审查 domain 代码后使用。按模式分组。

### Aggregate Checks（Aggregate 检查）

- [ ] 每个 aggregate 有明确识别的 root entity
- [ ] 仅 root 可从 aggregate 外部访问
- [ ] Internal entities 不被外部代码直接引用
- [ ] 其他 aggregates 通过 ID 引用，而非 object
- [ ] 每个 aggregate 适合单个事务
- [ ] 不超过~3-5 个内部 entities（如果更多，质疑边界）
- [ ] 每个 internal entity 参与至少一个由 root 强制执行的不变性

### Entity Checks（Entity 检查）

- [ ] 每个 entity 有 typed identifier（value object，而非原始 string/UUID）
- [ ] 相等性基于身份，而非属性
- [ ] 业务规则是 entity 上的方法，而非外部 services 中
- [ ] 状态转换强制执行前提条件（guard clauses）
- [ ] 无绕过业务规则的公共 setters

### Value Object Checks（Value Object 检查）

- [ ] Value objects 不可变——操作返回新实例
- [ ] 自验证构造函数——无效状态是不可表示的
- [ ] 相等性基于属性，而非身份
- [ ] 领域概念的 primitives 被 value objects 替换（Money、Email、OrderId）
- [ ] Value objects 上无 identity field（id）

### Domain Service Checks（Domain Service 检查）

- [ ] Stateless——调用之间不保留内部状态
- [ ] Pure domain 计算——无 I/O、无 infrastructure 依赖
- [ ] 逻辑真正跨越多个 entities 或 value objects
- [ ] 不重复属于单个 entity 的逻辑

### Domain Event Checks（Domain Event 检查）

- [ ] 以过去时命名（OrderPlaced，而非 PlaceOrder）
- [ ] 携带足够的数据以描述发生的事情（aggregate ID + 相关值）
- [ ] 不携带整个 aggregate 状态
- [ ] 为跨 aggregate 协调和重大状态更改触发
- [ ] 在 domain 层定义

### Repository Checks（Repository 检查）

- [ ] 每个 aggregate root 一个 repository——非每个 entity
- [ ] Domain 层中的接口，infrastructure 中的实现
- [ ] Collection-like 语义（save、findById、remove）
- [ ] 返回完全构成的 aggregates，而非部分对象或 DTOs
- [ ] 无复杂报告查询——那些属于 Providers

---

## 新部分（New Sections）

<!-- INTERVIEW GUIDANCE:
在访谈结束时，询问：
"是否有任何你想添加的项目特定部分不在默认值中？
常见添加：
- 通用词汇表（领域专家和开发者共享的术语定义）
- Bounded context 边界（什么是范围内，什么不是）
- Event storming artifacts（如果团队使用 event storming 进行发现）
- 领域特定不变性目录（项目特定的业务规则）
- Domain objects 的测试模式（如何测试 aggregates、value objects）"

如果用户想要添加部分，从 10 开始编号。
新部分在 overlay 和 override 模式下都有效。
-->

---

## 页脚（Footer）

<!-- INTERVIEW GUIDANCE:
在输出中包含项目名称、生成日期和模式指示器。
示例：

---
*为 [项目名称] 生成于 [日期]。模式：[overlay|override]*
*由 ddd-refiner skill 生成。*
-->
