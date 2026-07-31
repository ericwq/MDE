# 整洁代码：默认原则

整洁代码的嵌入默认值。有主见的护栏——通过 SKILL.md 配置解析覆盖。

## 1. 单一职责

一个函数做一件事。一个类有一个内聚轴——一个变更原因。

**"和"测试**：用一句话描述函数的目的。需要"和"这个词吗？函数做了不止一件事。将每个职责提取为命名函数——函数名就是文档。

**类内聚**：当大多数方法使用大多数实例变量时，类是内聚的。只有部分方法触及部分字段？那部分很可能属于另一个类。

---

## 2. 小而专注的函数

### 阈值

| 指标 | 指导值 |
|--------|-----------|
| **每函数行数** | 约 20 行以下 |
| **抽象层级** | 每个函数一个层级 |
| **缩进深度** | 最多 2 层 |

这些是信号，不是硬性规则。一个有明确目的 25 行的函数比五个模糊流程的 5 行函数更好。目标：可读性，不是数行数。

### 提取模式

函数做多件事？通过命名意图来提取：

```
// 之前：一个函数混合了不同抽象层级
function renderUserProfile(userId):
  user = db.query("SELECT * FROM users WHERE id = ?", [userId])
  if user is null: return notFound()
  posts = db.query("SELECT * FROM posts WHERE author_id = ? ORDER BY date DESC LIMIT 5", [userId])
  avatar = user.avatarUrl ?? defaultAvatarUrl
  displayName = user.nickname ?? user.firstName + " " + user.lastName
  return template.render("profile", { user, posts, avatar, displayName })

// 之后：每个提取出的函数通过名称来记录意图
function renderUserProfile(userId):
  user = findUserOrFail(userId)
  posts = getRecentPosts(userId)
  profile = buildProfileViewModel(user, posts)
  return template.render("profile", profile)
```

提取出的函数名替代了原本要写的注释。`buildProfileViewModel` 记录了我们在构造视图模型——函数名就是注释。

---

## 3. 圈复杂度

### 阈值

| 复杂度 | 评估 | 操作 |
|-----------|------------|--------|
| **1-5** | 简单，易于测试 | 无需操作 |
| **6-10** | 中等，可管理 | 如果可读性受损，考虑提取 |
| **11-20** | 高，难以彻底测试 | 将子决策提取为命名函数 |
| **21+** | 非常高，可能做多件事 | 激进分解；此函数有多个职责 |

### 扁平化技术

1. **Guard 子句**：用提前返回替换嵌套条件——扁平化嵌套，减少缩进深度
2. **提取命名条件**：复杂布尔表达式 → 命名变量或函数（`canApproveOrder = isAdmin(user) or isManagerOfDepartment(user, order.department)`）
3. **管道优于循环**：当语言支持时，用 filter/map 链替换带累积的循环——每一步都显式

---

## 4. 有意义的命名

### 命名模式

| 类别 | 约定 | 好的示例 | 差的示例 |
|----------|-----------|---------------|---------------|
| **布尔变量** | `is`、`has`、`can`、`should` 前缀 | `isActive`、`hasPermission`、`canRetry` | `active`、`permission`、`retry` |
| **布尔函数** | 与布尔变量相同的前缀 | `isExpired(token)`、`hasAccess(user, resource)` | `checkExpiry(token)`、`access(user, resource)` |
| **函数（动作）** | 动词开头 | `calculateTotal`、`sendNotification`、`validateInput` | `totalCalculation`、`notification`、`inputCheck` |
| **函数（访问器）** | `get`、`find`、`fetch` 前缀 | `getUser`、`findByEmail`、`fetchLatestOrders` | `user()`、`email()`、`orders()` |
| **类** | 名词或名词短语 | `OrderValidator`、`PaymentProcessor`、`UserRepository` | `ValidateOrder`、`ProcessPayment`、`HandleUser` |
| **常量** | 大写蛇形命名或描述性名称 | `MAX_RETRY_COUNT`、`DEFAULT_PAGE_SIZE` | `MRC`、`n`、`val` |
| **集合** | 复数名词 | `activeUsers`、`pendingOrders`、`validTokens` | `list`、`data`、`items`（当存在领域上下文时） |
| **映射/字典** | `xByY` 模式 | `userById`、`priceByProductId` | `map`、`lookup`、`dict` |

### 应避免的名称

- **循环计数器以外的单字母**（循环中的 `i`、`j`、`k` 可以；业务逻辑中的 `d`、`x`、`t` 不行）
- **需要项目知识的缩写**（`usr`、`txn`、`mgr`、`ctx`——除非是行业标准如 `HTTP`、`URL`、`ID`）
- **不带信息的通用名称**（`data`、`info`、`temp`、`result`、`value`、`item`——除非作用域只有 2-3 行）
- **编码类型的名称**（`strName`、`intCount`、`arrItems`——类型系统会处理这些）
- **否定式布尔名称**（`isNotActive`、`hasNoPermission`——使用肯定形式，在调用处取反）

### 作用域-长度规则

名称长度应与作用域成正比。两行循环体的循环变量可以是 `i`。跨函数使用的模块级常量应为 `MAX_LOGIN_ATTEMPTS_BEFORE_LOCKOUT`。作用域越广，名称必须承载越多的上下文。

### 魔法数字和字符串

提取测试：**读者是否停顿思考"为什么是这个特定值？"** 如果是，提取命名常量。值从上下文已不言自明？保持内联——添加常量只会增加间接性而不增加清晰度。

| 场景 | 操作 | 示例 |
|----------|--------|---------|
| 含义不显然 | 提取命名常量 | `MAX_RETRIES = 3`、`SESSION_TIMEOUT_MS = 30_000`、`DEFAULT_PAGE_SIZE = 25` |
| 在多处出现 | 提取命名常量 | 三个不同验证函数中使用的阈值 |
| 空集合字面量 | 保持内联 | `return []`、`users = []`、`new Map()` |
| 零作为起始索引 | 保持内联 | `startIndex = 0`、`offset = 0` |
| 数学恒等式 | 保持内联 | `percentage / 100`、`radians * (180 / Math.PI)` |
| 框架调用中的 HTTP 状态 | 保持内联 | `res.status(404).json(...)`、`Response(data, status=200)` |
| 布尔默认值 | 保持内联 | `enabled = false`、`verbose = true` 作为初始值 |

---

## 5. 参数设计

### 阈值

| 参数数量 | 评估 | 操作 |
|----------------|------------|--------|
| **0-2** | 理想 | 无需分组 |
| **3** | 可接受 | 如果参数相关，考虑分组 |
| **4** | 边界 | 将相关参数分组为对象 |
| **5+** | 过多 | 始终分组；函数也可能做了太多事 |

### 分组模式

```
// 差：六个参数——难读，调用处容易搞错顺序
function searchProducts(query, page, pageSize, sortBy, sortDirection, includeArchived):
  // ...

// 好：相关参数分组为对象
function searchProducts(query, options: SearchOptions):
  // ...

class SearchOptions:
  page: number = 1
  pageSize: number = 20
  sortBy: string = "relevance"
  sortDirection: "asc" | "desc" = "desc"
  includeArchived: boolean = false
```

### 布尔参数异味

布尔参数通常意味着函数做了两件事——true 时一件，false 时一件。拆分为两个具有描述性名称的函数：

```
// 差：调用处的 `true` 是什么意思？
renderUser(user, true)

// 好：意图清晰
renderUserCompact(user)
renderUserDetailed(user)
```

布尔值真正代表选项（而非行为分支）？用选项对象使调用处自说明：

```
// 可接受：布尔值作为命名选项
renderUser(user, { compact: true })
```

---

## 6. DRY 而不过早抽象

### 三法则

1. **第一次出现**：内联编写代码。不抽象。
2. **第二次出现**：注意重复。容忍它。两个实例可能服务于不同目的，之后会分化。
3. **第三次出现且有相同的变更原因**：现在提取。有足够证据表明这是真正的模式，而非巧合。

### 相同的变更原因

两个看起来相同的代码块但服务于不同的业务目的，**不是**真正的重复。它们会在各自的需求变更时分道扬镳。

```
// 这些看起来相同但不应被统一：

// 在 OrderService 中——计算订单折扣
discount = subtotal > 1000 ? subtotal * 0.1 : 0

// 在 InvoiceService 中——计算发票调整
adjustment = lineTotal > 1000 ? lineTotal * 0.1 : 0

// 原因：订单折扣和发票调整受不同的业务规则管理。
// 当折扣策略变更时，你不希望发票逻辑跟着变。
// 共享抽象会耦合不相关的关注点。
```

### 命名抽象

当提取时，以**它做什么**来命名抽象，而非以它消除了重复这一事实：

```
// 差：以提取动机命名
function commonCalculation(amount, threshold, rate): ...

// 好：以业务意图命名
function applyVolumeDiscount(amount, threshold, rate): ...
```

---

## 7. 注释与自文档化

### 注释决策框架

| 情况 | 操作 |
|-----------|--------|
| 代码不清晰，注释有助于解释它做**什么** | 重构代码使其自说明（重命名、提取、简化） |
| 不显然的**为什么**——业务规则、法律要求、变通方案 | 写注释解释为什么 |
| 性能优化让代码可读性降低 | 注释解释权衡以及"显然"的做法会是什么 |
| TODO 或已知限制 | 注释以 `TODO:` 前缀，附简短上下文 |
| 公共接口的 API 文档 | 使用 doc 注释/docstring 附参数描述 |
| 正则或复杂算法 | 注释解释意图；正则尤其受益于英文描述 |

### 示例

```
// 好：注释解释不显然的业务规则
// FTC 法规要求对超过 25 美元的购买设置冷静期。
// 在此窗口内，订单可以无罚金取消。
if order.isWithinCoolingOffPeriod():

// 好：注释解释变通方案
// PostgreSQL 14 在分区表上使用 CTE 时有查询计划器回归。
// 在升级到 15+ 之前使用子查询代替 CTE。参见：postgresql.org/bugs/12345
result = db.query("SELECT * FROM (SELECT ...)")

// 好：注释解释正则意图
// 匹配带可选时区的 ISO 8601 日期：2024-01-15T10:30:00Z
datePattern = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(Z|[+-]\d{2}:\d{2})?$/
```

---

## 8. 错误处理

### 核心原则

- **快速失败**：在边界处验证；在坏数据传播到各层之前拒绝它
- **保持显式**：每个可能失败的操作都有可见的错误处理
- **保持可操作**：错误消息告诉调用者什么错了以及该做什么
- **在正确的层级处理**：不太早（丢失上下文），不太晚（失去恢复能力）
- **不用异常做控制流**：仅对真正的异常情况使用异常

### 模式

**边界处的 Guard 子句：**

```
function createUser(input):
  if not input.email: throw ValidationError("邮箱是必填的")
  if not isValidEmail(input.email): throw ValidationError("邮箱格式无效：期望 user@domain.tld")
  if not input.name: throw ValidationError("名称是必填的")
  if input.name.length > 200: throw ValidationError("名称超过 200 字符限制")
  // 以下是快乐路径——所有 guard 通过
```

**可操作的错误消息：**

```
// 差：调用者不知道该做什么
throw Error("无效输入")
throw Error("出了点问题")
throw Error("数据库错误")

// 好：调用者知道发生了什么以及该做什么
throw Error("订单总额必须为正数，实际得到：-42.50")
throw Error("邮箱为 'a@b.com' 的用户已存在。使用 updateUser() 修改现有用户。")
throw Error("连接支付 API 在 5 秒后超时。重试或检查服务状态 status.payments.io")
```

> **信任边界说明**：这些可操作的消息适用于应用级错误（服务间、服务端日志记录）。在信任边界（HTTP 响应、用户界面），剥离内部细节（邮箱、方法名），返回通用但可操作的消息附带关联 ID。参见 `framework:secure-coding`。

**在正确的层级处理**——不太早（丢失上下文，调用者无法决策），不太晚（失去恢复能力）。让错误传播到有足够上下文做出有意义决策的层级。捕获并返回 null 隐藏了失败是"未找到"、"连接错误"还是"权限拒绝"。

**不吞没错误**——空 catch 块使 bug 不可见。始终记录日志、重新抛出或显式记录为什么忽略是安全的：

```
try:
  sendNotification(user)
catch error:
  logger.warn("用户 " + user.id + " 的通知发送失败：" + error.message)
  // 通知是非关键的；继续执行而不让整个操作失败
```

---

## 9. 可测试代码

默认设计为可测试：

1. **偏好纯函数**——所有输入作为参数显式传入（不使用 `Date.now()`，不使用全局变量）。确定性输出。最容易测试。
2. **注入依赖**——构造函数/参数注入优于在方法内部 `new`。使模拟、替换实现成为可能。
3. **避免隐藏状态**——没有模块级可变变量。将状态封装在具有重置能力的显式对象中。
4. **将副作用推到边界**——将纯业务逻辑（计算、验证）与 I/O（数据库、网络、文件系统）分离。纯核心 + 薄编排外壳。

**反模式——带嵌入式 I/O 的上帝函数**：一个函数从数据库读取、应用业务逻辑、写入数据库并发送通知。提取纯计算，让编排层处理 I/O。

---
