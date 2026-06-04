# 整洁代码：默认原则

内置的整洁代码默认值。带有偏好的护栏——通过 SKILL.md 配置解析进行覆盖。

## 1. 单一职责

函数只做一件事。类只有一个维度的内聚性——只有一个变更理由。

**"和"测试**：用一句话描述函数目的。需要用到"和"这个词吗？说明函数做了不止一件事。将每个职责提取为命名函数——函数名本身就是文档。

**类内聚性**：当大多数方法使用大多数实例变量时，类是内聚的。如果一组方法只触及一部分字段？那一部分可能属于另一个独立的类。

---

## 2. 小而专注的函数

### 阈值

| 指标 | 指导原则 | 理由 |
|--------|-----------|-----------|
| **函数行数** | 约 20 行以内 | 函数在一屏内可见，无需滚动即可推理 |
| **抽象层级** | 每个函数一个层级 | 混合高层编排与低层细节会迫使读者进行上下文切换 |
| **缩进深度** | 最多 2 层 | 每个嵌套层级增加读者需要 mentally tracked 的条件 |

信号，而非硬性规则。25 行但目的明确的函数优于五个 5 行却模糊流程的函数。目标：可读性，而非行数统计。

### 提取模式

函数做了多件事？通过命名意图来提取：

```
// 之前：一个函数混合了不同抽象层级
function renderUserProfile(userId):
  user = db.query("SELECT * FROM users WHERE id = ?", [userId])
  if user is null: return notFound()
  posts = db.query("SELECT * FROM posts WHERE author_id = ? ORDER BY date DESC LIMIT 5", [userId])
  avatar = user.avatarUrl ?? defaultAvatarUrl
  displayName = user.nickname ?? user.firstName + " " + user.lastName
  return template.render("profile", { user, posts, avatar, displayName })

// 之后：每个提取的函数通过其名称记录意图
function renderUserProfile(userId):
  user = findUserOrFail(userId)
  posts = getRecentPosts(userId)
  profile = buildProfileViewModel(user, posts)
  return template.render("profile", profile)
```

提取的函数名替代了你原本会写的注释。`buildProfileViewModel` 记录了我们构建视图模型——函数名就是注释。

---

## 3. 圈复杂度

### 阈值

| 复杂度 | 评估 | 行动 |
|-----------|------------|--------|
| **1-5** | 简单，易于测试 | 无需行动 |
| **6-10** | 中等，可管理 | 如果可读性受损则考虑提取 |
| **11-20** | 高，难以充分测试 | 将子决策提取为命名函数 |
| **21+** | 非常高，可能做了多件事 | 激进分解；函数有多个职责 |

### 展平技术

1. **卫语句**：用提前返回替代嵌套条件——展平嵌套，减少缩进深度
2. **提取命名条件**：复杂布尔表达式 → 命名变量或函数（`canApproveOrder = isAdmin(user) or isManagerOfDepartment(user, order.department)`）
3. **管道替代循环**：当语言支持时，用 filter/map 链替代带累积的循环——每一步都显式

---

## 4. 有意义的命名

### 命名模式

| 类别 | 约定 | 良好示例 | 糟糕示例 |
|----------|-----------|---------------|---------------|
| **布尔变量** | `is`、`has`、`can`、`should` 前缀 | `isActive`、`hasPermission`、`canRetry` | `active`、`permission`、`retry` |
| **布尔函数** | 与布尔变量相同前缀 | `isExpired(token)`、`hasAccess(user, resource)` | `checkExpiry(token)`、`access(user, resource)` |
| **函数（动作）** | 动词优先 | `calculateTotal`、`sendNotification`、`validateInput` | `totalCalculation`、`notification`、`inputCheck` |
| **函数（访问器）** | `get`、`find`、`fetch` 前缀 | `getUser`、`findByEmail`、`fetchLatestOrders` | `user()`、`email()`、`orders()` |
| **类** | 名词或名词短语 | `OrderValidator`、`PaymentProcessor`、`UserRepository` | `ValidateOrder`、`ProcessPayment`、`HandleUser` |
| **常量** | UPPER_SNAKE_CASE 或描述性名称 | `MAX_RETRY_COUNT`、`DEFAULT_PAGE_SIZE` | `MRC`、`n`、`val` |
| **集合** | 复数名词 | `activeUsers`、`pendingOrders`、`validTokens` | `list`、`data`、`items`（当存在领域上下文时） |
| **映射/字典** | `xByY` 模式 | `userById`、`priceByProductId` | `map`、`lookup`、`dict` |

### 应避免的命名

- **单字母**：循环计数器除外（`i`、`j`、`k` 在循环中可以；`d`、`x`、`t` 在业务逻辑中不行）
- **缩写**：需要项目知识才能理解（`usr`、`txn`、`mgr`、`ctx`——除非是行业标准如 `HTTP`、`URL`、`ID`）
- **泛化名称**：不携带任何信息（`data`、`info`、`temp`、`result`、`value`、`item`——除非作用域仅 2-3 行）
- **类型编码名称**（`strName`、`intCount`、`arrItems`——类型系统会处理这个）
- **否定布尔值**（`isNotActive`、`hasNoPermission`——使用肯定形式，在调用处取反）

### 作用域-长度规则

名称长度与作用域成正比。循环变量、2 行代码体可以用 `i`。模块级常量、跨函数使用则应为 `MAX_LOGIN_ATTEMPTS_BEFORE_LOCKOUT`。作用域越广，名称必须独自承载的上下文越多。

### 魔法数字和字符串

提取测试：**读者是否停顿并问"为什么是这个特定值？"** 如果是，提取为命名常量。如果值从上下文中显而易见？保留内联——常量在不增加清晰度的情况下添加了间接层。

| 场景 | 行动 | 示例 |
|----------|--------|---------|
| 含义不显而易见 | 提取命名常量 | `MAX_RETRIES = 3`、`SESSION_TIMEOUT_MS = 30_000`、`DEFAULT_PAGE_SIZE = 25` |
| 多处出现 | 提取命名常量 | 阈值在三个不同的验证函数中使用 |
| 空集合字面量 | 保留内联 | `return []`、`users = []`、`new Map()` |
| 零作为起始索引 | 保留内联 | `startIndex = 0`、`offset = 0` |
| 数学恒等式 | 保留内联 | `percentage / 100`、`radians * (180 / Math.PI)` |
| 框架调用中的 HTTP 状态码 | 保留内联 | `res.status(404).json(...)`、`Response(data, status=200)` |
| 布尔默认值 | 保留内联 | `enabled = false`、`verbose = true` 作为初始值 |

---

## 5. 参数设计

### 阈值

| 参数数量 | 评估 | 行动 |
|----------------|------------|--------|
| **0-2** | 理想 | 无需分组 |
| **3** | 可接受 | 如果参数相关则考虑分组 |
| **4** | 边界 | 将相关参数分组为对象 |
| **5+** | 过多 | 始终分组；函数可能也做了太多事情 |

### 分组模式

```
// 糟糕：六个参数——难以阅读，调用处容易顺序错误
function searchProducts(query, page, pageSize, sortBy, sortDirection, includeArchived):
  // ...

// 良好：相关参数分组为对象
function searchProducts(query, options: SearchOptions):
  // ...

class SearchOptions:
  page: number = 1
  pageSize: number = 20
  sortBy: string = "relevance"
  sortDirection: "asc" | "desc" = "desc"
  includeArchived: boolean = false
```

### 布尔参数气味

布尔参数通常意味着函数做了两件事——true 时一种行为，false 时另一种行为。考虑拆分为两个具有描述性名称的函数：

```
// 糟糕：调用处的 `true` 是什么意思？
renderUser(user, true)

// 良好：意图清晰
renderUserCompact(user)
renderUserDetailed(user)
```

布尔值确实代表选项（而非行为分支）？使用 Options 对象使调用处自文档化：

```
// 可接受：布尔值作为命名选项
renderUser(user, { compact: true })
```

---

## 6. DRY 而不过早抽象

### 三次法则

1. **第一次出现**：内联编写代码。不做抽象。
2. **第二次出现**：注意重复。容忍。两个实例可能服务于不同目的，后续会分化。
3. **第三次出现且具有相同变更理由**：现在提取。有足够的证据表明这是真正的模式，而非巧合。

### 相同变更理由

两段代码看起来相同但服务于不同业务目的，并非真正的重复。当各自的需求变更时，它们会分化。

```
// 这些看起来相同但不应统一：

// OrderService 中——计算订单折扣
discount = subtotal > 1000 ? subtotal * 0.1 : 0

// InvoiceService 中——计算发票调整
adjustment = lineTotal > 1000 ? lineTotal * 0.1 : 0

// 原因：订单折扣和发票调整由不同的业务规则管理。
// 当折扣策略变更时，你不希望发票逻辑随之改变。
// 共享抽象会将不相关的关注点耦合在一起。
```

### 命名抽象

提取时，为抽象命名以反映**它做什么**，而非因为它消除了重复：

```
// 糟糕：为提取动机命名
function commonCalculation(amount, threshold, rate): ...

// 良好：为业务意图命名
function applyVolumeDiscount(amount, threshold, rate): ...
```

---

## 7. 注释与自文档化

### 注释决策框架

| 场景 | 行动 |
|-----------|--------|
| 代码不清晰，注释帮助解释**做了什么** | 重构代码使其自文档化（重命名、提取、简化） |
| 非显而易见的**为什么**——业务规则、法律要求、变通方案 | 写注释解释原因 |
| 性能优化使代码可读性降低 | 注释解释权衡，说明"显而易见"的方法会是什么 |
| TODO 或已知限制 | 使用 `TODO:` 前缀的注释，附带简要上下文 |
| 公共接口的 API 文档 | 使用 doc comments / docstrings，附带参数描述 |
| 正则表达式或复杂算法 | 注释解释意图；正则表达式尤其受益于英文描述 |

### 示例

```
// 良好：注释解释了一个非显而易见的业务规则
// FTC 法规要求超过 $25 的购买必须有冷静期。
// 在此期间，订单可以无惩罚取消。
if order.isWithinCoolingOffPeriod():

// 良好：注释解释了一个变通方案
// PostgreSQL 14 在分区表的 CTE 上存在查询计划器回归问题。
// 使用子查询替代 CTE，直到升级到 15+。参见：postgresql.org/bugs/12345
result = db.query("SELECT * FROM (SELECT ...)")

// 良好：注释解释正则表达式的意图
// 匹配带可选时区的 ISO 8601 日期：2024-01-15T10:30:00Z
datePattern = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(Z|[+-]\d{2}:\d{2})?$/
```

---

## 8. 错误处理

### 核心原则

| 原则 | 理由 |
|-----------|-----------|
| **快速失败** | 在边界处验证；在数据传播到各层之前拒绝错误数据 |
| **显式处理** | 每个可能失败的操作都应有可见的错误处理 |
| **可操作** | 错误消息告诉调用者出了什么问题、该怎么办 |
| **在合适的层级处理** | 不要太早（丢失上下文），也不要太晚（失去恢复能力） |
| **不用异常控制流程** | 异常会模糊正常执行路径；仅用于真正异常情况 |

### 模式

**边界处的卫语句：**

```
function createUser(input):
  if not input.email: throw ValidationError("Email is required")
  if not isValidEmail(input.email): throw ValidationError("Email format is invalid: expected user@domain.tld")
  if not input.name: throw ValidationError("Name is required")
  if input.name.length > 200: throw ValidationError("Name exceeds 200-character limit")
  // 正常路径——所有卫语句已通过
```

**可操作的错误消息：**

```
// 糟糕：调用者不知道该怎么办
throw Error("Invalid input")
throw Error("Something went wrong")
throw Error("Database error")

// 良好：调用者知道发生了什么以及该怎么办
throw Error("Order total must be positive, got: -42.50")
throw Error("User with email 'a@b.com' already exists. Use updateUser() to modify existing users.")
throw Error("Connection to payments API timed out after 5s. Retry or check service status at status.payments.io")
```

> **信任边界说明**：这些可操作的消息适用于应用级错误（服务间通信、服务器端日志）。在信任边界处（HTTP 响应、面向用户的 UI），剥离内部细节（邮箱、方法名），返回泛化但可操作的消息并附带关联 ID。参见 `framework:secure-coding`。

**在合适的层级处理**——不要太早（丢失上下文，调用者无法决策），也不要太晚（失去恢复能力）。让错误传播到具有足够上下文以做出有意义决策的层级。捕获并返回 null 会隐藏失败是"未找到"、"连接错误"还是"权限不足"。

**不要吞没错误**——空的 catch 块使 bug 不可见。始终记录、重新抛出，或显式文档说明忽略是安全的：

```
try:
  sendNotification(user)
catch error:
  logger.warn("Notification failed for user " + user.id + ": " + error.message)
  // 通知是非关键的；继续而不使操作失败
```

---

## 9. 测试友好代码

难以测试的代码通常也难以维护。默认情况下为可测试性而设计：

1. **优先使用纯函数**——所有输入显式作为参数（无 `Date.now()`、无全局变量）。确定性输出。最容易测试。
2. **注入依赖**——构造函数/参数注入优于方法内的 `new`。支持模拟、替换实现。
3. **避免隐藏状态**——无模块级可变变量。将状态封装在具有重置能力的显式对象中。
4. **将副作用推至边界**——分离纯业务逻辑（计算、验证）与 I/O（数据库、网络、文件系统）。纯核心 + 薄编排外壳。

**反模式——带有内嵌 I/O 的上帝函数**：一个在单个代码体中从 DB 读取、应用业务逻辑、写入 DB 并发送通知的函数。提取纯计算，让编排层处理 I/O。

---

*默认值综合了 Robert Martin《整洁代码》（2008）、Martin Fowler《重构》（1999、2018）、Kent Beck《Smalltalk 最佳实践模式》（1996）的原则，以及软件工艺实践的集体智慧。*
