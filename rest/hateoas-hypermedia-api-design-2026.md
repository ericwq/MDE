# HATEOAS 在 2026 年：超媒体 API 设计已死？

对 HATEOAS 和超媒体驱动 API 的诚实评估。
为什么大多数 API 忽略它，它何时真正有用，以及 API 可发现性的实用替代方案。

2026年3月8日 · APIScout 团队 · [原文](https://apiscout.dev/guides/hateoas-hypermedia-api-design-2026) 

---
![top](./img/hateoas-hypermedia-api-design-2026.avif)

HATEOAS（Hypermedia as the Engine of Application State）是 REST 约束中争议最大的一个。
理论上，API 响应包含链接，告知客户端接下来可以执行哪些操作。
实践中，几乎没有人真正实现它。
让我们坦诚地探讨一下原因。

## 要点摘要

- HATEOAS 在技术上是 Roy Fielding 对 REST 定义所要求的，但几乎没有任何 “REST API” 真正实现了它
- 支持它的理由是切实存在的：状态机类资源（订单、订阅）能从运行时操作发现中获益
- 反对它的理由对大多数团队来说更强：TypeScript + OpenAPI 代码生成可以在不增加运行时开销的情况下静态地解决可发现性问题
- `available_actions` 模式 —— 返回当前哪些操作有效而不包含完整 URL 链接 —— 能以 20% 的成本获取 80% 的收益
- 对分页链接和状态机转换使用部分 HATEOAS；对标准 CRUD API 则跳过完整的超媒体导航

## HATEOAS 的实际含义

符合 HATEOAS 的响应包含指向相关资源和可用操作的链接：

```json
{
  "data": {
    "id": "order_123",
    "status": "pending",
    "total": 4999
  },
  "links": {
    "self": "/api/orders/order_123",
    "cancel": "/api/orders/order_123/cancel",
    "payment": "/api/orders/order_123/pay",
    "customer": "/api/customers/cust_456"
  }
}
```

客户端不需要硬编码 URL。
它从响应中发现可用的操作。
如果订单已付款，payment 链接就会消失。
如果订单无法取消，cancel 链接就会消失。

## 为什么大多数 API 忽略 HATEOAS

1. 客户端仍然硬编码

在实践中，前端开发者构建的组件会调用特定的端点。
他们不会编写通用的链接跟随代码。
无论 UI 是从链接中发现 URL 还是自行构造 URL，它都需要知道 `/api/orders/order_123/cancel` 是做什么的。

2. 类型安全 > 可发现性

TypeScript、基于 OpenAPI 规范的代码生成，以及类型安全的 API 客户端（tRPC、orval、openapi-typescript）提供了编译时安全性。
硬编码的、带类型的 API 调用比动态的链接跟随更加安全。

3. 带宽开销

链接给每个响应增加了字节数。
在一个发起数百次 API 调用的移动应用中，包含客户端从未使用过的导航链接会浪费带宽。

4. 复杂性且回报不明确

实现 HATEOAS 需要构建链接生成系统、维护链接关系，以及处理链接驱动的状态机。
对于大多数团队来说，很难证明为仅供自己前端使用的 API 承担这些开销是合理的。

## HATEOAS 实际有用的情况

### 1. 长生命周期 API 客户端

如果 API 客户端已部署且无法轻易更新（IoT 设备、更新周期缓慢的原生移动应用），HATEOAS 允许服务器更改 URL 结构而不会破坏客户端。

### 2. 复杂状态机

当资源具有复杂的状态转换时（订单 → 已付款 → 已发货 → 已送达 → 已退货），链接传达了哪些转换是可用的。
这可以防止客户端在已发货的订单上显示 “取消” 按钮。

### 3. API 市场和聚合器

通用 API 客户端（如 Postman、API 探索器）能从可发现性中获益。
HATEOAS 让这些工具无需预先了解即可浏览 API。

### 4. 多服务架构

当一个 API 聚合了多个服务时，链接可以指向不同的服务 URL。
客户端无需知道哪个服务处理什么，只需跟随链接即可。

## 实用替代方案

### 1. OpenAPI 规范

使用 OpenAPI/Swagger 来记录你的 API。
客户端根据规范生成类型化的代码。
变更通过规范更新来传达，而非通过运行时链接。

### 2. 包含分页链接的响应包装

最常见的 “轻量级 HATEOAS” 模式 —— 在分页响应中包含下一页和上一页链接：

```json
{
  "data": [...],
  "links": {
    "next": "/api/users?cursor=abc123",
    "prev": "/api/users?cursor=xyz789"
  }
}
```

这种方式在无需完整 HATEOAS 的情况下仍然切实有用。

### 3. actions 数组

不提供URL链接，而是包含一个可用操作的数组：

```json
{
  "data": { "id": "order_123", "status": "pending" },
  "available_actions": ["cancel", "pay", "update"]
}
```

客户端知道该显示哪些按钮，而无需从链接构造URL。

### 4. GraphQL 内省

GraphQL 的模式内省（schema introspection）服务于与 HATEOAS 相同的可发现性目的 —— 客户端可以在运行时探索可用的类型、字段和操作。

## Roy Fielding 的论点

Roy Fielding 在 2000 年的博士论文中定义了包含六项约束的 REST。
HATEOAS ——“Hypermedia as the Engine of Application State”—— 是其中之一，Fielding 认为这是不可妥协的。
在 2008 年的一篇博客文章中，他明确写道：

> *“如果应用程序状态（以及由此而来的 API）的引擎不是由超文本驱动的，那么它就不是 RESTful 的，也不可能是 REST API。”*

按照这个定义，实际上没有任何生产级 API 是真正的 RESTful。
GitHub 的 API、Stripe 的 API、Twitter 的 API —— 它们都不满足 Fielding 的原始定义。

### Richardson 成熟度模型

Leonard Richardson 的成熟度模型为这个谱系提供了一个有用的框架：

- Level 0：单一 URI，单一方法（RPC 风格，如 SOAP）
- Level 1：多个 URI，单一方法（不含 HTTP 语义的资源）
- Level 2：多个 URI，正确的 HTTP 方法和状态码（大多数人所说的 REST）
- Level 3：超媒体控制（按 Fielding 定义真正的 REST）

<ins>大多数生产级 API 运行在 Level 2。
业界已经默默达成共识：对于典型用例，Level 3 增加的复杂性超过了其价值。</ins>
Fielding 对此仍然感到沮丧。

实际结论：当有人说他们的 API 是 “RESTful” 时，他们指的是 Level 2。
当 Fielding 说某个 API 是 RESTful 时，他指的是 Level 3。
这是两件不同的事情，承认这一差距有助于团队做出更清晰的架构决策，而不是为一个标签辩护。

## 真实世界中的 HATEOAS 实现

少数 API 已经承诺采用真正的超媒体。它们实际做了什么？

### HAL（超文本应用语言）

HAL 是采用最广泛的超媒体格式。
它使用 `_links` 表示相关资源，使用 `_embedded` 内联包含相关资源：

```json
{
  "id": "order_123",
  "status": "pending",
  "total": 4999,
  "_links": {
    "self": { "href": "/orders/order_123" },
    "cancel": { "href": "/orders/order_123/cancel" },
    "customer": { "href": "/customers/cust_456" }
  },
  "_embedded": {
    "items": [
      {
        "id": "item_1",
        "quantity": 2,
        "_links": {
          "product": { "href": "/products/prod_789" }
        }
      }
    ]
  }
}
```

HAL 简洁且可读性强。
它的缺点：无法描述每个链接应使用哪种 HTTP 方法，或应提供什么请求体。
这就是完整 HATEOAS 难以实现的原因 —— 仅有链接是不够的。

### 带链接的 JSON:API

JSON:API 包含一个 `links` 部分和用于相关资源的 `relationships`。
它还标准化了分页链接，这确实很有用：

```json
{
  "data": { "type": "orders", "id": "123", "attributes": { "status": "pending" } },
  "links": {
    "self": "/orders/123",
    "next": "/orders?page=2"
  },
  "relationships": {
    "customer": {
      "links": { "related": "/customers/456" }
    }
  }
}
```

### GitHub 和 PayPal 实际做了什么

GitHub 使用 `Link` 头部进行分页（标准的 HTTP 方式），但不在响应体中包含操作链接。
PayPal 的 HATEOAS 实现在支付响应中包含链接：

```json
{
  "id": "PAY-1AB23456CD789012EF34GHIJ",
  "state": "created",
  "links": [
    {
      "href": "https://api.paypal.com/v1/payments/payment/PAY-...",
      "rel": "self",
      "method": "GET"
    },
    {
      "href": "https://www.paypal.com/cgi-bin/webscr?cmd=...",
      "rel": "approval_url",
      "method": "REDIRECT"
    },
    {
      "href": "https://api.paypal.com/v1/payments/payment/PAY-.../execute",
      "rel": "execute",
      "method": "POST"
    }
  ]
}
```

PayPal 在每个链接中包含 HTTP 方法，这使其真正有用。
客户端可以跟随 `approval_url` 而无需硬编码 PayPal 的重定向 URL 结构。
对于一个公共支付 API 来说，这是一个合理的 HATEOAS 实现，因为 API URL 可能会发生变化。

## HATEOAS 与 API 版本管理

HATEOAS 的原始承诺：由于客户端跟随链接而非构造 URL，服务器可以自由更改 URL 结构而不会破坏客户端。
API 版本管理变得不再必要 —— 只需更新链接即可。

现实情况：客户端仍然硬编码入口 URL。
一旦它们跟随链接到达 `/api/v2/orders/123/cancel`，它们就已经获知（且很可能会缓存）该 URL。
硬编码已发现 URL 的客户端并不是真正的超媒体客户端。

### 通过内容协商与 URL 路径进行版本管理

HATEOAS 纯粹主义者在版本管理上的立场是使用内容协商：

```
Accept: application/vnd.myapi.v2+json
```

而不是更改 URL：

```
/api/v2/orders
```

其承诺是：同一个 URL 根据客户端请求的内容提供 v1 或 v2 版本。
结合 HATEOAS，客户端在理论上只需接受 v2 内容类型，即可迁移到 v2 而无需更改任何 URL 代码。

实践情况：开发者讨厌这种方式。
它无法通过将 URL 粘贴到浏览器中来测试。
它会破坏缓存（需要正确的 `Vary` 头部）。
它在日志中不够直观。
它在 API 网关中更难路由。

URL 路径版本管理在实践中胜出，因为它简单且易于调试。
有关版本管理策略以及如何在 [不破坏客户端的情况下](https://apiscout.dev/blog/api-breaking-changes-without-breaking-clients-2026) 管理 API 演变的完整分析，请参阅 [如何对 REST API 进行版本管理](https://apiscout.dev/blog/how-to-version-rest-apis-2026)。

## 在不使用完整 HATEOAS 的情况下构建状态机 API

在不承担完整开销的情况下获取 HATEOAS 收益的最实用方式是 `available_actions` 模式，结合状态特定的响应字段。

### available_actions 模式详解

```typescript
// 订单响应根据当前状态变化
interface OrderResponse {
  id: string;
  status: 'pending' | 'paid' | 'shipped' | 'delivered' | 'cancelled';
  total: number;
  available_actions: OrderAction[];
}

type OrderAction = 'pay' | 'cancel' | 'ship' | 'deliver' | 'refund' | 'dispute';

function getAvailableActions(order: Order): OrderAction[] {
  switch (order.status) {
    case 'pending':
      return ['pay', 'cancel'];
    case 'paid':
      return ['ship', 'refund', 'cancel'];
    case 'shipped':
      return ['deliver'];
    case 'delivered':
      return ['refund', 'dispute'];
    case 'cancelled':
      return [];
    default:
      return [];
  }
}
```

前端收到 `available_actions: ["pay", "cancel"]`，据此显示或隐藏相应的按钮。
当订单状态发生变化时（例如支付完成后），下一次 API 调用将返回不同的操作。
客户端不需要硬编码状态机逻辑。

### 使用 OpenAPI 鉴别器实现状态依赖响应

你可以使用 `discriminator` 在 OpenAPI 规范中形式化这一点：

```yaml
OrderResponse:
  oneOf:
    - $ref: '#/components/schemas/PendingOrder'
    - $ref: '#/components/schemas/PaidOrder'
    - $ref: '#/components/schemas/ShippedOrder'
  discriminator:
    propertyName: status
    mapping:
      pending: '#/components/schemas/PendingOrder'
      paid: '#/components/schemas/PaidOrder'
      shipped: '#/components/schemas/ShippedOrder'

PendingOrder:
  type: object
  properties:
    status:
      type: string
      enum: [pending]
    available_actions:
      type: array
      items:
        type: string
        enum: [pay, cancel]
```

这使得状态依赖的响应具有类型安全性，代码生成工具可以将其转换为 TypeScript 中的可辨识联合类型（discriminated union types）。

## 开发者体验的权衡取舍

### 可发现性 vs 可预测性

完整的 HATEOAS 优先考虑可发现性：客户端可以在不阅读文档的情况下探索 API，跟随链接前往任何地方。
但在构建集成时，开发者真正想要的是可预测性 —— 确切地知道调用某个操作应该使用哪个 URL。

在实践中，使用 OpenAPI 生成客户端的 TypeScript 开发者比 HATEOAS 客户端拥有更好的可发现性：他们在编辑器中获得自动补全、内联文档，并在编译时获得类型错误。
这比在运行时跟随链接提供了更好的开发者体验。

### 类型安全 vs 动态导航

HATEOAS 本质上是动态的：可用操作集是在运行时根据响应链接确定的。
TypeScript 的类型系统与此相悖 —— 你无法为动态发现的端点获得编译时安全性。
`available_actions` 模式是一种务实的折中方案：操作名称是带类型的（字符串枚举），但实现细节仍然保留在服务器端。

### 前端开发者的视角

作为 API 的主要使用者，大多数前端开发者发现 HATEOAS 在实践中令人困惑。
他们想知道：“取消订单我应该调用哪个 URL？”
答案是 “跟随响应中的 `cancel` 链接”，这比一个文档完善的 `POST /api/orders/{id}/cancel` 端点需要更多代码，且清晰度更低。

有关真正改善开发者体验的 API 设计原则，请参阅如何设计开发者喜爱的 REST API；关于 HATEOAS 风格链接始终能带来价值的唯一场景，请参阅 API 分页模式。

## 最终结论

HATEOAS 并未消亡 —— 它属于特定场景。
对于大多数构建供自己前端使用的 API 团队来说，它增加了复杂性而回报不成比例。
OpenAPI 规范、类型化客户端和良好的文档以更实用的方式解决了同样的问题。

**适合使用 HATEOAS 的情况：**

- 客户端无法随 API 一起更新
- 复杂状态机需要运行时操作发现
- 正在构建真正的通用 API 浏览器/探索器

**适合使用 available_actions 模式的情况：**

- 资源具有有意义的状态机（订单、订阅、工作流）
- 前端需要知道显示哪些按钮，而无需重复状态逻辑
- 你希望获得收益而不承担开销

**可以跳过 HATEOAS 的情况：**

- 你同时控制客户端和服务器
- 你使用类型化 API 客户端（TypeScript + 代码生成）
- 你正在构建标准的 CRUD API

`available_actions` 模式是 2026 年务实的平衡点：足够明确以实现类型安全，足够动态以传达有效的状态转换，同时客户端无需硬编码业务规则。
