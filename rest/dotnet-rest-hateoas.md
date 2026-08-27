# HATEOAS — REST API 中的超媒体链接（何时值得使用）

<i>什么是 HATEOAS，它能带来什么好处，如何在 ASP.NET Core 中实现超媒体链接，
以及诚实的权衡取舍：何时该添加它，何时对你的使用场景来说属于过度设计。</i>

[Learnixo](https://learnixo.io/) 📅2026年4月15日 🕰️阅读时长6分钟 [原文](https://learnixo.io/blog/dotnet-rest-hateoas)

.NET C# REST ASP.NET Core HATEOAS Hypermedia API Design

---

HATEOAS（Hypermedia as the Engine of Application State）是大多数 API 从未达到的 REST 成熟度级别 —— 而且对大多数 API 来说，这也没问题。
本文说明了什么是 HATEOAS，它实际能给你带来什么，如何在 ASP.NET Core 中实现它，以及诚实的权衡取舍在哪里。

## HATEOAS 的含义

在普通的 REST API 中，客户端需要提前知道 URL 结构。
如果你想取消订单 42，你需要知道端点是 `DELETE /api/orders/42` —— 这些知识是硬编码在客户端中的。

在 HATEOAS API 中，每个响应都包含描述客户端接下来可以做什么的链接，以及这些操作的 URL。
客户端在运行时发现 API，而不是将 URL 结构内置在代码中。

```json
{
  "id": 42,
  "status": "processing",
  "total": 299.99,
  "_links": {
    "self":   { "href": "/api/orders/42",        "method": "GET"    },
    "cancel": { "href": "/api/orders/42/cancel", "method": "POST"   },
    "items":  { "href": "/api/orders/42/items",  "method": "GET"    },
    "customer": { "href": "/api/customers/7",    "method": "GET"    }
  }
}
```

cancel 链接仅当订单确实可以取消时才会出现。
如果订单已经发货，该链接就不存在 —— 客户端不需要知道业务规则，它只需要检查链接是否存在即可。

## Richardson 成熟度模型

REST 有不同的成熟度级别。
大多数 “REST API” 处于 Level 2；HATEOAS 属于 Level 3。

- Level 0 —— 单个端点，动作在请求体中（XML-RPC、SOAP）
- Level 1 —— 资源（每个资源有独立的 URL）
- Level 2 —— HTTP 动词 + 正确的状态码
- Level 3 —— 超媒体控制（HATEOAS）

大多数团队应该达到 Level 2并停留在此，除非他们有特定的理由需要升级到 Level 3。

## HATEOAS 实际能给你带来什么

它适用于：

- 被许多不同的未知客户端调用的公共 API
- 工作流状态很重要的 API（订单、审批、工作流），且客户端不应了解状态机
- 希望在不破坏客户端的情况下更改 URL 的 API

它不能解决的问题：

- 客户端仍然需要理解每个链接的语义（cancel vs refund）
- 它并不能消除 API 文档 —— 客户端仍然需要知道链接的含义
- 大多数移动端/SPA 客户端仍然会硬编码 URL 模式

诚实总结：HATEOAS 对于公共 API 和工作流密集型领域确实有用。
对于内部微服务 API 或简单的 CRUD 服务来说，则属于过度设计。

## 在 ASP.NET Core 中实现 HATEOAS

### 步骤 1：链接模型

```csharp
public record Link(string Href, string Rel, string Method);

public class LinkedResource<T>
{
    public T Data { get; init; }
    public IReadOnlyList<Link> Links { get; init; }

    public LinkedResource(T data, IReadOnlyList<Link> links)
    {
        Data  = data;
        Links = links;
    }
}
```

### 步骤 2：包含链接的控制器

```csharp
[ApiController]
[Route("api/orders")]
public class OrdersController : ControllerBase
{
    [HttpGet("{id}")]
    public async Task<ActionResult<LinkedResource<OrderDto>>> GetById(
        int id, CancellationToken ct)
    {
        var order = await _service.GetAsync(id, ct);
        if (order is null) return NotFound();

        var dto   = OrderDto.From(order);
        var links = BuildLinks(order);

        return Ok(new LinkedResource<OrderDto>(dto, links));
    }

    private List<Link> BuildLinks(Order order)
    {
        var links = new List<Link>
        {
            new(Url.Action(nameof(GetById), new { id = order.Id })!, "self", "GET"),
            new(Url.Action(nameof(GetItems), new { orderId = order.Id })!, "items", "GET"),
            new($"/api/customers/{order.CustomerId}", "customer", "GET"),
        };

        // 基于状态的条件链接
        if (order.Status == OrderStatus.Processing)
        {
            links.Add(new(
                Url.Action(nameof(Cancel), new { id = order.Id })!,
                "cancel", "POST"));
        }

        if (order.Status == OrderStatus.Shipped)
        {
            links.Add(new(
                Url.Action(nameof(TrackShipment), new { id = order.Id })!,
                "track", "GET"));
        }

        if (order.Status == OrderStatus.Delivered)
        {
            links.Add(new(
                Url.Action(nameof(Refund), new { id = order.Id })!,
                "refund", "POST"));
        }

        return links;
    }
}
```

状态为 Processing 的订单响应示例：

```json
{
  "data": {
    "id": 42,
    "status": "processing",
    "total": 299.99,
    "customerName": "Acme Corp"
  },
  "links": [
    { "href": "/api/orders/42",        "rel": "self",     "method": "GET"  },
    { "href": "/api/orders/42/items",  "rel": "items",    "method": "GET"  },
    { "href": "/api/customers/7",      "rel": "customer", "method": "GET"  },
    { "href": "/api/orders/42/cancel", "rel": "cancel",   "method": "POST" }
  ]
}
```

对于状态为 Delivered 的订单，cancel 链接不存在，取而代之的是 refund 链接。
客户端通过 `links.find(l => l.rel === 'cancel')` 检查 —— 如果返回 null，则隐藏取消按钮。
客户端无需了解状态机逻辑。

## HAL — 一种超媒体格式标准

与其发明自己的链接 (link) 结构，不如使用 HAL（Hypertext Application Languag），这是一种广泛使用的超媒体标准：

```json
{
  "id": 42,
  "status": "processing",
  "total": 299.99,
  "_links": {
    "self":   { "href": "/api/orders/42" },
    "cancel": { "href": "/api/orders/42/cancel" },
    "items":  { "href": "/api/orders/42/items" }
  },
  "_embedded": {
    "items": [
      { "productId": 5, "quantity": 2, "price": 149.99 }
    ]
  }
}
```

HAL 使用 `_links` 表示链接，使用 `_embedded` 表示嵌套资源。它有一个已注册的媒体类型：`application/hal+json`。

```bash
dotnet add package Halcyon.Net
```

```csharp
[HttpGet("{id}")]
public async Task<IActionResult> GetById(int id, CancellationToken ct)
{
    var order = await _service.GetAsync(id, ct);
    if (order is null) return NotFound();

    var response = HALResponse.Create(OrderDto.From(order))
        .AddSelfLink(Url.Action(nameof(GetById), new { id })!)
        .AddLink("items", Url.Action(nameof(GetItems), new { orderId = id })!);

    if (order.Status == OrderStatus.Processing)
        response.AddLink("cancel", Url.Action(nameof(Cancel), new { id })!);

    return Ok(response);
}
```

## 集合链接与分页

HATEOAS 在分页集合 (pagination collections) 中表现出色：

```json
{
  "data": [ { "id": 1, ... }, { "id": 2, ... } ],
  "totalCount": 87,
  "page": 2,
  "pageSize": 10,
  "_links": {
    "self":  { "href": "/api/orders?page=2&pageSize=10", "method": "GET" },
    "first": { "href": "/api/orders?page=1&pageSize=10", "method": "GET" },
    "prev":  { "href": "/api/orders?page=1&pageSize=10", "method": "GET" },
    "next":  { "href": "/api/orders?page=3&pageSize=10", "method": "GET" },
    "last":  { "href": "/api/orders?page=9&pageSize=10", "method": "GET" }
  }
}
```

```csharp
private List<Link> BuildPaginationLinks(PagedQuery query, int total)
{
    var totalPages = (int)Math.Ceiling((double)total / query.PageSize);
    var links = new List<Link>
    {
        new(BuildUrl(query.Page, query.PageSize), "self", "GET"),
        new(BuildUrl(1, query.PageSize),          "first", "GET"),
        new(BuildUrl(totalPages, query.PageSize), "last",  "GET"),
    };

    if (query.Page > 1)
        links.Add(new(BuildUrl(query.Page - 1, query.PageSize), "prev", "GET"));

    if (query.Page < totalPages)
        links.Add(new(BuildUrl(query.Page + 1, query.PageSize), "next", "GET"));

    return links;
}
```

这些 next/prev/first/last 链接正是 GitHub API 分页的工作方式 —— 客户端从不硬编码页码。

## 何时添加 HATEOAS（诚实建议）

**适合添加的情况：**

- 你正在构建一个被你无法控制的客户端调用的公共 API
- 你的领域包含复杂的状态机（订单、审批、订阅）
- URL 稳定性很重要 —— 你希望在不破坏客户端的情况下更改 URL
- 你将 Richardson 成熟度 Level 3 作为一个明确的设计目标

**可以跳过的情况：**

- 仅供自己前端/移动应用调用的内部 API
- 没有有意义的工作流状态的简单 CRUD
- 团队内部未达成共识 —— 半成品式的 HATEOAS 比完全没有更糟

**从简单开始：** 先添加分页链接 (pagination links) —— 它们立即可用且工作量小。
当领域有需求时，再添加状态相关的链接（cancel、refund、approve）。
不要仅仅为了宣称有 HATEOAS 而在各处添加 self 链接。
