# 整洁代码精炼器模板

此模板定义了 `.lattice/standards/clean-code.md` 输出文档的结构。它包含来自整洁代码原子 `defaults.md` 的所有默认内容，并穿插了访谈指导注释。

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

> 本文档将项目特定的自定义内容叠加在整洁代码原子嵌入的默认值之上。只有此处包含的章节与默认值不同——所有其他章节保持原样。
>
> 以下章节替换默认值中匹配的章节（按标题匹配）。新章节追加在默认值之后。

**覆盖引言：**

> 这些是 [项目名称] 的整洁代码原则。它们完全替换整洁代码原子中嵌入的默认值。

**目录**（用于覆盖模式；叠加模式只列出包含的章节）：

1. [单一职责](#1-单一职责)
2. [小而专注的函数](#2-小而专注的函数)
3. [圈复杂度](#3-圈复杂度)
4. [有意义的命名](#4-有意义的命名)
5. [参数设计](#5-参数设计)
6. [DRY 而不过早抽象](#6-dry-而不过早抽象)
7. [注释与自文档化](#7-注释与自文档化)
8. [错误处理](#8-错误处理)
9. [可测试代码](#9-可测试代码)
10. [验证清单](#10-验证清单)

---

## 1. 单一职责

<!-- 访谈指导：
叠加模式总结："默认规定：函数应该做一件事（'和'测试——如果需要用'和'来描述它，就提取）。对于类，内聚意味着大多数方法使用大多数字段。这与你的项目匹配吗？"

追问问题：
- 你的团队使用类还是代码库是纯函数式的？（如果是纯函数式，可以移除类内聚指导）
- 你对单一职责的理解有多严格？有些团队容忍按顺序做多件事的编排函数。
- 你的代码库中是否有单一职责被有意放宽的模式（例如，中间件链、管道阶段）？

可自定义：构成"一件事"的阈值、类 vs 函数焦点、提取目标。
固定：代码单元应有一个变更原因这一核心原则。

跨章节影响：如果团队是纯函数式的（无类），这会影响 §2（提取目标始终是函数）、§5（参数设计偏移）和 §10（类相关的清单项可以删除）。
-->

一个函数应该做一件事。一个类应该有一个内聚轴——一个变更原因。

**"和"测试**：用一句话描述函数的目的。如果你需要"和"这个词，函数做了不止一件事。

```
// 差：此函数验证、转换并持久化
function processOrder(rawInput):
  if rawInput.items is empty: throw Error("无项目")
  if rawInput.total < 0: throw Error("总额无效")
  items = rawInput.items.map(item => normalizeItem(item))
  total = items.reduce((sum, item) => sum + item.price * item.quantity, 0)
  discount = total > 1000 ? total * 0.1 : 0
  finalTotal = total - discount
  db.insert("orders", { items, total: finalTotal })
  emailService.send(rawInput.email, "订单已确认")

// 好：每个函数做一件事
function validateOrderInput(input):
  if input.items is empty: throw Error("无项目")
  if input.total < 0: throw Error("总额无效")

function calculateOrderTotal(items):
  subtotal = items.reduce((sum, item) => sum + item.price * item.quantity, 0)
  discount = subtotal > 1000 ? subtotal * 0.1 : 0
  return subtotal - discount

function createOrder(input):
  validateOrderInput(input)
  items = input.items.map(normalizeItem)
  total = calculateOrderTotal(items)
  return { items, total }
```

**类内聚**：当大多数方法使用大多数实例变量时，类是内聚的。当只有部分方法触及部分字段时，那部分很可能属于另一个类。

---

## 2. 小而专注的函数

<!-- 访谈指导：
叠加模式总结："默认规定：函数目标在约 20 行以下，每个函数一个抽象层级，最多 2 层缩进。这些是信号，不是硬性规则。这与你的项目匹配吗？"

追问问题：
- 你的团队有不同行数限制吗？（有些团队偏好 10-15，有些容忍 30）
- 你的 linter 是否强制执行最大函数长度或最大嵌套？
- 是否有更长函数可接受的模式（例如，状态机处理器、配置构建器）？

可自定义：行数阈值、缩进深度、规则的例外情况。
固定：函数应在同一抽象层级上做一件事这一原则。

跨章节影响：更短的函数限制意味着更低的复杂度预算（§3）。
-->

### 阈值

| 指标 | 指导值 | 理由 |
|--------|-----------|-----------|
| **每函数行数** | 约 20 行以下 | 一个无需滚动即可在单屏内可见的函数更容易理解 |
| **抽象层级** | 每个函数一个层级 | 混合高层编排和低层细节迫使读者在上下文中切换 |
| **缩进深度** | 最多 2 层 | 每个嵌套层级都增加了一个读者必须在心里跟踪的条件 |

这些是信号，不是硬性规则。一个有明确目的 25 行的函数比五个模糊流程的 5 行函数更好。目标是可读性，不是数行数。

### 提取模式

当函数做多件事时，通过命名意图来提取：

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

提取出的函数名替代了原本要写的注释。

---

## 3. 圈复杂度

<!-- 访谈指导：
叠加模式总结："默认规定：每个函数的圈复杂度保持在约 10 以下。三种扁平化技术：guard 子句、提取条件判断、管道操作。这与你的项目匹配吗？"

追问问题：
- 你的 linter 是否已经强制执行复杂度阈值？是多少？
- 你的团队偏好 guard 子句还是使用其他模式（例如，函数式语言中的模式匹配）？
- 是否有容忍更高复杂度的领域（例如，解析器、状态机）？

可自定义：复杂度阈值、偏好的扁平化技术、例外情况。
固定：深层嵌套和高分支计数侵蚀可读性这一原则。

跨章节影响：更低的复杂度限制可能需要更严格的函数大小（§2）。
-->

### 阈值

| 复杂度 | 评估 | 操作 |
|-----------|------------|--------|
| **1-5** | 简单，易于测试 | 无需操作 |
| **6-10** | 中等，可管理 | 如果可读性受损，考虑提取 |
| **11-20** | 高，难以彻底测试 | 将子决策提取为命名函数 |
| **21+** | 非常高，可能做多件事 | 激进分解；此函数有多个职责 |

### 扁平化技术

**Guard 子句**用提前退出替换嵌套条件：

```
// 差：深度嵌套
function getDiscount(customer, order):
  if customer is not null:
    if customer.isActive:
      if order.total > 100:
        if customer.loyaltyYears > 2:
          return 0.15
        else:
          return 0.10
      else:
        return 0.05
    else:
      return 0
  else:
    return 0

// 好：Guard 子句扁平化逻辑
function getDiscount(customer, order):
  if customer is null: return 0
  if not customer.isActive: return 0
  if order.total <= 100: return 0.05
  if customer.loyaltyYears > 2: return 0.15
  return 0.10
```

**当条件本身复杂时，提取条件分支**：

```
// 差：复杂的内联条件
if user.role == "admin" or (user.role == "manager" and user.department == order.department):
  // ... 允许

// 好：命名条件
canApproveOrder = isAdmin(user) or isManagerOfDepartment(user, order.department)
if canApproveOrder:
  // ... 允许
```

**当语言支持时，用管道操作替换循环**：

```
// 差：循环中交织累积和过滤
result = []
for item in items:
  if item.isActive:
    if item.price > threshold:
      result.push({ name: item.name, discountedPrice: item.price * 0.9 })

// 好：管道使每一步显式
result = items
  .filter(item => item.isActive)
  .filter(item => item.price > threshold)
  .map(item => ({ name: item.name, discountedPrice: item.price * 0.9 }))
```

---

## 4. 有意义的命名

<!-- 访谈指导：
叠加模式总结："默认规定：名称揭示意图，而非实现。布尔名称使用 is/has/can，函数以动词开头，类基于名词。名称长度与作用域成正比。这与你的项目匹配吗？"

追问问题：
- 你的团队有与这些不同的命名约定吗？（例如，一些 Go 团队使用更短的名称）
- 是否有领域特定的缩写是可接受的？（例如，金融代码中 `tx` 表示 transaction）
- 你的团队对类型使用特定的后缀吗？（例如，`Input`、`Output`、`DTO`、`Response`）
- 你的语言是否有覆盖通用命名规则的惯用法？（例如，Go 偏好小作用域中的短名称）

可自定义：命名模式、可接受的缩写、语言特定约定。
固定：名称应揭示意图这一原则。
-->

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
- **不带信息的通用名称**（`data`、`info`、`temp`、`result`、`value`、`item`——除非作用域只有两到三行）
- **编码类型的名称**（`strName`、`intCount`、`arrItems`——类型系统会处理这些）
- **否定式布尔名称**（`isNotActive`、`hasNoPermission`——使用肯定形式，在调用处取反）

### 作用域-长度规则

名称长度应与作用域成正比。两行循环体的循环变量可以是 `i`。跨函数使用的模块级常量应为 `MAX_LOGIN_ATTEMPTS_BEFORE_LOCKOUT`。作用域越广，名称必须承载越多的上下文。

---

## 5. 参数设计

<!-- 访谈指导：
叠加模式总结："默认规定：0-2 个参数理想，4 个是边界，5 个以上始终分组。布尔参数是异味——偏好命名选项或拆分函数。这与你的项目匹配吗？"

追问问题：
- 你的团队容忍更多参数吗？（有些函数式代码库在有良好命名时接受更高的数量）
- 你对复杂构造使用构建器模式还是选项模式？
- 你的团队如何处理配置对象——偏好显式参数还是配置对象？
- 你的语言有命名参数吗（Python、Kotlin）？这改变了是否需要参数对象。

可自定义：参数数量阈值、分组策略、语言特定模式。
固定：长参数列表造成认知负担和调用处错误这一原则。

跨章节影响：如果团队是纯函数式的（来自 §1），参数设计模式偏移——如果语言支持命名参数，更多参数可能是可接受的。
-->

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

布尔参数通常意味着函数做了两件事——true 时一件，false 时一件。考虑拆分为两个具有描述性名称的函数：

```
// 差：调用处的 `true` 是什么意思？
renderUser(user, true)

// 好：意图清晰
renderUserCompact(user)
renderUserDetailed(user)
```

当布尔值真正代表选项（而非行为分支），用选项对象使调用处自说明：

```
// 可接受：布尔值作为命名选项
renderUser(user, { compact: true })
```

---

## 6. DRY 而不过早抽象

<!-- 访谈指导：
叠加模式总结："默认规定：容忍重复直到三法则——三个具有相同变更原因的实例。错误的抽象比没有抽象代价更高。这与你的项目匹配吗？"

追问问题：
- 你的团队提取共享代码有多激进？（有些团队提取更早，有些更晚）
- 是否有重复被明确接受的领域？（例如，测试设置代码、配置）
- 你的团队是否使用代码生成使某些重复可接受？
- 你如何命名提取的抽象——有任何约定吗？

可自定义：提取阈值（二法则 vs 三法则）、抽象命名约定。
固定：过早抽象耦合不相关关注点这一原则。
-->

### 三法则

1. **第一次出现**：内联编写代码。不抽象。
2. **第二次出现**：注意重复。容忍它。两个实例可能服务于不同目的，之后会分化。
3. **第三次出现且有相同的变更原因**：现在提取。有足够证据表明这是真正的模式，而非巧合。

### 相同的变更原因

两个看起来相同的代码块但服务于不同的业务目的，**不是**真正的重复。

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

<!-- 访谈指导：
叠加模式总结："默认规定：代码应通过命名实现自文档化。注释解释为什么，不解释是什么。例外：正则模式和复杂算法值得写'是什么'注释。这与你的项目匹配吗？"

追问问题：
- 你的团队使用 doc 注释/docstring 吗？对所有公共 API 还是选择性地？
- 你的代码库中是否有"是什么"注释可接受的领域？（例如，复杂的 SQL、编码在条件中的业务规则）
- 你使用 TODO/FIXME 约定吗？有任何格式要求吗？
- 你的团队使用 JSDoc/TSDoc/Javadoc 生成文档吗？

可自定义：文档注释策略、TODO 格式、"不写'是什么'注释"规则的额外例外。
固定：解释"是什么"的注释是代码异味，表明代码应该重构这一原则。
-->

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
// 差：注释重述代码
// 将计数器加一
counter = counter + 1

// 差：注释解释是什么，而非为什么
// 检查用户是否激活
if user.isActive:

// 好：注释解释不显然的业务规则
// FTC 法规要求对超过 25 美元的购买设置冷静期。
// 在此窗口内，订单可以无罚金取消。
if order.isWithinCoolingOffPeriod():

// 好：注释解释变通方案
// PostgreSQL 14 在分区表上使用 CTE 时有查询计划器回归。
// 在升级到 15+ 之前使用子查询代替 CTE。
// 参见：https://postgresql.org/bugs/12345
result = db.query("SELECT * FROM (SELECT ...)")

// 好：注释解释正则意图
// 匹配带可选时区的 ISO 8601 日期：2024-01-15T10:30:00Z
datePattern = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(Z|[+-]\d{2}:\d{2})?$/
```

---

## 8. 错误处理

<!-- 访谈指导：
叠加模式总结："默认规定：在边界处快速失败，错误消息应可操作，在正确的层级处理（不太早，不太晚），绝不容忍吞没错误。对真正的异常情况使用异常。这与你的项目匹配吗？"

追问问题：
- 你的团队使用异常还是 Result/Either 类型进行错误处理？
- 你有自定义错误类层次结构吗？（例如，AppError、ValidationError、NotFoundError）
- 你的团队如何处理跨边界的错误？（例如，领域错误翻译为 HTTP 状态码）
- 是否有记录日志 vs 返回 vs 重新抛出错误的模式？
- 你的团队使用错误码还是仅使用消息？

可自定义：错误处理策略（异常 vs Result 类型）、自定义错误模式、错误消息格式。
固定：错误必须显式处理且绝不默默吞没这一原则。

跨章节影响：如果团队使用 Result 类型而非异常（§8），测试模式会改变（§9）——错误路径通过返回值测试，而非 catch 块。
-->

### 核心原则

| 原则 | 理由 |
|-----------|-----------|
| **快速失败** | 在边界处验证；在坏数据传播到各层之前拒绝它 |
| **保持显式** | 每个可能失败的操作应有可见的错误处理 |
| **保持可操作** | 错误消息应告诉调用者什么错了以及该做什么 |
| **在正确的层级处理** | 不太早（丢失上下文），不太晚（失去恢复能力） |
| **不用异常做控制流** | 异常模糊了正常执行路径；仅对真正的异常情况使用 |

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

**在正确的层级处理：**

```
// 差：错误被过早捕获——上下文丢失
function getUser(id):
  try:
    return db.findById("users", id)
  catch error:
    return null   // 调用者不知道为什么会失败

// 好：让它传播到能够做出决策的层级
function getUser(id):
  return db.findById("users", id)   // 如果连接失败会抛出
  // 调用者或中间件决定：重试？返回 500？记录日志并告警？
```

**不吞没错误：**

```
// 差：静默失败——bug 变得不可见
try:
  sendNotification(user)
catch error:
  // 默默忽略

// 好：关于错误的显式决策
try:
  sendNotification(user)
catch error:
  logger.warn("用户 " + user.id + " 的通知发送失败：" + error.message)
  // 通知是非关键的；继续执行而不让整个操作失败
```

---

## 9. 可测试代码

<!-- 访谈指导：
叠加模式总结："默认规定：偏好纯函数，注入依赖，避免隐藏状态，将副作用推到边界（函数式核心 / 命令式外壳）。这与你的项目匹配吗？"

追问问题：
- 你的团队使用什么测试框架？
- 你的团队实践 TDD 还是实现后写测试？
- 你的团队如何处理模拟——偏好 fakes、stubs 还是 mocks？最小模拟？
- 是否有测试数据设置的模式？（构建器、工厂、夹具）
- 你的团队对测试隔离有多严格？（有些团队在集成测试中接受共享状态）

可自定义：测试框架细节、模拟偏好、测试数据模式。
固定：可测试代码是结构良好的代码这一原则。

跨章节影响：如果 §8 使用 Result 类型而非异常，错误路径测试会改变——测试断言检查返回值，而非 catch 块。
-->

难测试的代码通常难维护。使测试成为可能的特性——显式依赖、无隐藏状态、核心为纯函数——也使代码更容易理解和修改。

**偏好纯函数：**

```
// 差：依赖全局状态——测试必须操作 Date.now()
function isExpired(token):
  return Date.now() > token.expiresAt

// 好：纯函数——所有输入显式，确定性输出
function isExpired(token, currentTime):
  return currentTime > token.expiresAt
```

**注入依赖：**

```
// 差：硬编码依赖——没有真实的邮件服务就无法测试
class OrderService:
  emailClient = new SmtpEmailClient()

  confirmOrder(order):
    emailClient.send(order.customerEmail, "订单已确认")

// 好：注入——用模拟测试，自由替换实现
class OrderService:
  constructor(emailClient: EmailClient):
    this.emailClient = emailClient

  confirmOrder(order):
    this.emailClient.send(order.customerEmail, "订单已确认")
```

**避免隐藏状态：**

```
// 差：全局可变状态——测试是顺序依赖的
requestCount = 0

function handleRequest(req):
  requestCount = requestCount + 1
  if requestCount > RATE_LIMIT: throw Error("频率限制")

// 好：状态是显式且可注入的
class RateLimiter:
  constructor(limit):
    this.limit = limit
    this.count = 0

  check():
    this.count = this.count + 1
    if this.count > this.limit: throw Error("频率限制")

  reset():
    this.count = 0
```

**将副作用推到边界：**

```
// 差：业务逻辑与 I/O 混合
function applyDiscount(orderId, discountCode):
  order = db.findById("orders", orderId)
  discount = db.findOne("discounts", { code: discountCode })
  if discount.isExpired(): throw Error("已过期")
  newTotal = order.total * (1 - discount.rate)
  db.update("orders", orderId, { total: newTotal })
  emailService.send(order.email, "折扣已应用")
  return newTotal

// 好：纯计算与 I/O 分离
function calculateDiscountedTotal(orderTotal, discountRate):
  return orderTotal * (1 - discountRate)

// 编排层处理 I/O
function applyDiscount(orderId, discountCode):
  order = orderProvider.findById(orderId)
  discount = discountProvider.findByCode(discountCode)
  if discount.isExpired(): throw Error("已过期")
  newTotal = calculateDiscountedTotal(order.total, discount.rate)
  orderRepo.updateTotal(orderId, newTotal)
  notificationService.discountApplied(order.email)
  return newTotal
```

---

## 10. 验证清单

<!-- 访谈指导：
此章节应与前述所有章节一致。如果之前更改了任何阈值、模式或策略，在此处更新对应的清单项。

询问："此清单总结了以上所有内容。AI 在生成或审查代码时应该检查所有这些吗？有什么要添加或删除的吗？"

如果 §1 移除了类指导（函数式代码库），删除类相关项。
如果 §3 更改了复杂度阈值，更新复杂度项。
如果 §8 将错误处理更改为 Result 类型，更新错误处理项。

可自定义：个别清单项、阈值、额外分组。
固定：必须至少有函数设计和错误处理分组。
-->

在生成或审查代码后使用此清单。每项映射到上述一个原则。

### 函数设计

- [ ] 每个函数做一件事（通过"和"测试）
- [ ] 函数在约 20 行以下；例外有单一明确目的
- [ ] 每个函数的圈复杂度在约 10 以下
- [ ] 缩进深度不超过两级
- [ ] 使用 guard 子句代替深层嵌套

### 命名

- [ ] 函数名以动词开头并揭示意图
- [ ] 类名基于名词
- [ ] 布尔名称使用 `is`/`has`/`can`/`should` 前缀
- [ ] 没有需要项目特定上下文才能解码的缩写
- [ ] 名称长度与作用域成正比

### 参数设计

- [ ] 函数有四个或更少的参数
- [ ] 相关参数分组为对象
- [ ] 布尔参数被避免或包装在命名选项中

### 抽象

- [ ] 重复仅在三个具有相同变更原因的实例后提取
- [ ] 提取的抽象以它做什么来命名，而非以它减少重复的事实来命名
- [ ] 没有过早抽象耦合不相关的关注点

### 注释

- [ ] 没有解释代码做"什么"的注释（改为重构以自说明）
- [ ] 注释为不显然的业务规则、变通方案和约束解释"为什么"
- [ ] 正则模式有英文描述注释
- [ ] 公共 API 有带参数描述的 doc 注释

### 错误处理

- [ ] 输入在边界处用 guard 子句验证
- [ ] 错误消息可操作（什么错了，该做什么）
- [ ] 没有吞没的错误（空 catch 块）
- [ ] 异常不用于控制流
- [ ] 错误在有足够上下文决策的层级处理

### 可测试性

- [ ] 业务逻辑尽可能在纯函数中
- [ ] 依赖被注入，而非硬编码
- [ ] 没有隐藏的可变全局状态
- [ ] 副作用在边界处，不与逻辑交织

---

## 新章节

<!-- 访谈指导：
在访谈结束时，询问：
"是否有默认值未涵盖的、你想添加的项目特定章节？
常见添加：
- 语言特定惯用法（例如，Go 的 if err != nil 错误处理、Python 上下文管理器）
- 框架特定模式（例如，React hook 规则、Express 中间件模式）
- 团队协议（例如，日志记录标准、功能标志约定）
- 性能模式（例如，缓存策略、懒加载规则）
- 并发模式（例如，async/await 约定、goroutine 生命周期规则）"

如果用户想添加章节，从 11 开始编号。
新章节在叠加和覆盖两种模式下都可以使用。
-->

---

## 页脚

<!-- 访谈指导：
在输出中包含项目名称、生成日期和模式指示器。
示例：

---
*为 [项目名称] 于 [日期] 生成。模式：[叠加|覆盖]。*
*由整洁代码精炼器技能生成。*
-->
