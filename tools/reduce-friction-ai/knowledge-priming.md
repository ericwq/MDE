# 知识预置 (Knowledge Priming)
*AI 编码助手会默认采用其训练数据中的通用模式。
我建议将项目上下文视为基础设施 ——即在每次会话前预置到模型中的版本化文件—— 而非依赖临时的复制粘贴。
这本质上属于手动检索增强生成（RAG），并且我相信它能从根本上提升 AI 生成代码的质量。*

|[Rahul Garg](https://www.linkedin.com/in/techygarg/)| |
|:---|---:|
|<img src="../img/rahul-garg.jpg" width="30%" /> | Rahul 是Thoughtworks 的首席工程师，常驻印度 Gurgaon 。他热衷于通过 DDD 与 Clean Architecture 打造可维护的软件，并探索 AI 如何助力团队实现工程卓越。|
| [原文](https://martinfowler.com/articles/reduce-friction-ai/knowledge-priming.html) |2026/2/24|
| | 本文是 [降低 AI 辅助开发挫折的模式](../reduce-friction-ai-main.md) 系列的一部分 |

---
内容
- [默认行为问题](#默认行为问题)
  - [知识层级](#知识层级)
- [知识预置是什么](#知识预置是什么)
  - [效果对比](#效果对比)
- [预置文档的结构剖析](#预置文档的结构剖析)
  - [1. 架构概览](#1-架构概览)
  - [2. 技术栈与版本](#2-技术栈与版本)
  - [3. 精选知识来源](#3-精选知识来源)
  - [4. 项目结构](#4-项目结构)
  - [5. 命名规范](#5-命名规范)
  - [6. 代码示例](#6-代码示例)
  - [7. 需避免的反模式](#7-需避免的反模式)
- [预置作为基础设施，而非个人习惯](#预置作为基础设施而非个人习惯)
- [常见误区](#常见误区)
  - [信息过载陷阱](#信息过载陷阱)
- [保持预置文档持续更新](#保持预置文档持续更新)
- [真实案例](#真实案例)
- [取舍与局限](#取舍与局限)
- [总结](#总结)

---
当我引导一位新开发者入职时，我不会只是把他们指向代码库，然后说 “去做吧”。
我会带他们熟悉我们的开发规范，向他们展示我们认可的优质代码示例，
并解释我们做出某些架构决策的原因 —— 比如我们为何选用 Fastify 而非 Express，为何服务采用函数式而非面向类的设计，为何校验逻辑放在路由层。
只有在完成这些上下文铺垫之后，我才会期望他们编写出贴合项目要求的代码。

AI 编码助手同样需要这样的入职引导。

许多开发者在使用 AI 助手时，都会经历一种可称之为 “挫折循环 (Frustration Loop)” 的过程：
生成代码 → 发现与代码库不兼容 → 修正后重新生成 → 不断重复，直至放弃或勉强接受大幅修改后的结果。
我逐渐意识到，这种协作挫折并非源于 AI 能力不足，而是因为缺少了关键一步 —— 我们在尚未向 AI 提供所需上下文的情况下，就要求它产出成果。

<ins>本文探讨的是我所提出的 ***知识预置 (Knowledge Priming)*** 理念 —— 即在要求 AI 生成代码之前，先向其提供经过精心整理的项目上下文</ins>。

其核心思路十分直白：AI 助手就像能力极强、却完全不具备上下文的协作者。
它们的工作效率远超人类，但对特定项目的开发规范、约束条件与历史一无所知。
缺乏上下文时，它们会默认采用通用模式，而这些模式未必适配项目需求。

## 默认行为问题
如果不进行知识预置就让 AI 生成代码，通常会出现以下情况：

**请求**：“创建一个处理认证的 UserService”

AI 会生成 200 行代码，却采用了这些不符合项目的方案：
- Express.js（项目实际使用 Fastify）
- JWT 存储在 localStorage（项目使用 httpOnly cookie）
- `utils/auth.js` 辅助文件（项目规范是 `lib/services/`）
- 面向类的语法（代码库为函数式风格）
- 过时的 bcrypt API（项目使用最新版本）

这段代码可以运行，语法无误，甚至能通过基础测试，但对当前代码库而言完全不合适。

<ins>原因何在？
因为 AI 会默认使用它的训练数据 —— 融合了数百万个代码仓库、教程和 Stack Overflow 回答的综合信息。
它生成的是来自互联网的 “通用方案”，而非适配特定团队的正确方案</ins>。

这就好比我让一名新员工入职第一天，在没有任何引导的情况下直接写代码。
他只能依靠过往经验，而这些经验未必符合我们的开发规范。

### 知识层级
我发现将 AI 的知识分为三个层级会很有帮助，按优先级排序：

- 训练数据（最低优先级）：数百万个代码库、教程、通用模式 —— 往往已经过时。这是 “互联网通用方案”。
- 对话上下文（中等优先级）：当前会话中讨论过的内容、AI 近期查看过的文件。在较长会话中会逐渐淡化。
- 预置文档（最高优先级）：明确的项目上下文 —— 架构决策、命名规范、具体版本与设计模式。一旦提供，这些内容会覆盖通用默认值。

这个层级划分至关重要。
当提供预置文档时，本质上是在下达这样的指令：“忽略通用的互联网模式，这是本项目的执行规范。”
而根据我的经验，AI 确实会遵循这一指令。

从技术上讲，这属于手动检索增强生成（RAG）—— 在上下文窗口中填入高价值、面向特定项目的 token，以此覆盖优先级更低的训练数据。
就像新员工的过往习惯在明确的团队规范面前会被修正一样，AI 基于训练数据形成的默认行为也会让位于显式的预置信息。

这一做法之所以有效，存在其内在机制原理。
<ins>Transformer 模型通过注意力机制处理上下文，而这种机制本质上拥有有限的资源配额：
上下文窗口中的每个 token 都会争夺对模型输出的影响</ins>。
当窗口充满通用训练数据模式时，模型会借助所见过的内容的平均水平进行生成。
而当窗口被具体、高信息价值的项目上下文占据时，这些 token 会获得更高的注意力权重，引导模型生成符合项目要求的内容。
<ins>这也是精选比数量更重要的原因：一份聚焦的预置文档不只是补充上下文，更会改变模型注意力的分配重心。
*「译注：上下文窗口：LLM 单次能一次性读取、理解并参与计算的最大文本长度，单位是 Token。」</ins>*

## 知识预置是什么
<ins>知识预置，是指在要求 AI 生成代码之前，与其共享精选的文档、架构模式和版本信息的做法</ins>。

你可以把它理解为给新员工的入职资料包：
- “这是我们的技术栈和对应版本”
- “这是代码的组织结构”
- “这是我们的命名规范”
- “这是本代码库中的优质代码示例”

### 效果对比
不进行知识预置时，让 AI 生成 UserService 相关代码，可能会得到基于Express.js、类风格、路径错误且 API 过时的实现 —— 需要花费 45 分钟修复或完全重写。

进行知识预置后，同样的需求会生成基于 Fastify、函数式风格、路径正确且使用最新 API 的代码 —— 仅需 5 分钟评审与小幅调整即可。

<ins>我无法断言这是经过验证的结论，但其逻辑十分合理：明确的上下文会覆盖通用默认行为。我个人的实验结果也令人振奋</ins>。

## 预置文档的结构剖析
一份优质的预置文档并非信息的简单堆砌，而是经过精心梳理、结构清晰的指南，不多不少，恰好为 AI 提供所需内容。

我建议分为七个部分，每一部分都对应我向团队新成员做入职引导时会讲解的内容：

### 1. 架构概览
*我会对新员工说：“先让我介绍一下项目全貌 (big picture)。”*

<ins>即项目全貌：这是何种类型的应用？主要组件有哪些？它们之间如何交互</ins>？

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
## Architecture Overview
This is a microservices-based e-commerce platform.
- API Gateway: Handles routing, auth, rate limiting
- User Service: Authentication, profiles, preferences
- Order Service: Cart, checkout, order history
- Notification Service: Email, SMS, push notifications

Services communicate via async message queues (RabbitMQ).
Each service owns its database (PostgreSQL).
```

\## 架构概览
<br/>
本项目为基于微服务架构的电商平台。

- API 网关：负责路由、身份认证、限流
- 用户服务：身份认证、个人资料、偏好设置
- 订单服务：购物车、结账、订单历史
- 通知服务：邮件、短信、推送通知

各服务通过异步消息队列（RabbitMQ）进行通信，
<br/>
每个服务独立管理自身的数据库（PostgreSQL）。
</div><br/>

### 2. 技术栈与版本
*我会对新员工说：“这是我们的技术栈 —— 注意区分有版本差异的 API。”*

<ins>明确具体内容至关重要。版本号尤为关键 —— API 会随版本迭代发生变化</ins>。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
## Tech Stack
- **Runtime**: Node.js 20.x (LTS)
- **Framework**: Fastify 4.x (not Express)
- **Database**: PostgreSQL 15 with Prisma ORM 5.x
- **Auth**: JWT with httpOnly cookies (not localStorage)
- **Testing**: Vitest + Testing Library (not Jest)
- **Validation**: Zod schemas (not Joi)
```

\## 技术栈
<br/>
- **运行环境**：Node.js 20.x（LTS）
- **框架**：Fastify 4.x（非 Express）
- **数据库**：PostgreSQL 15 搭配 Prisma ORM 5.x
- **认证**：使用 httpOnly Cookie 存储 JWT（非 localStorage）
- **测试**：Vitest + Testing Library（非 Jest）
- **数据校验**：Zod schema（非 Joi）
</div><br/>

### 3. 精选知识来源
*我会对新员工说：“在你上网搜索之前，先看看这些塑造了团队思路的文档和博客。从这里开始。”*

每个团队都有自己信赖的资料来源：
他们实际参考的官方文档，还有影响了架构设计的博客文章、讲解清晰的教程，以及承载了文档从未提及的经验总结的文章。
这些内容共同构成了团队的共享心智模型。

<ins>当 AI 优先参考这些精选来源，而非其庞大而通用的训练数据时，输出结果会更快贴合需求。
团队的思考方式已经提前融入其中</ins>。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
## Curated Knowledge

### Official Documentation
| Topic | Source | Why We Trust It |
|-------|--------|-----------------|
| Fastify routing | https://fastify.dev/docs/latest/Guides/Getting-Started | Official, matches our v4.x |
| Prisma relations | https://www.prisma.io/docs/orm/prisma-schema/data-model/relations | Authoritative for schema patterns |

### Blogs and Articles We Follow
| Concept | Source | Why It Shaped Our Thinking |
|---------|--------|---------------------------|
| Error handling patterns | [team-vetted blog URL] | Clearer than official docs, practical examples |
| Testing strategies | [team-vetted blog URL] | Influenced our test architecture |

### Internal References
| Topic | Path | What It Captures |
|-------|------|------------------|
| Error conventions | docs/error-handling.md | Our specific patterns |
| API design decisions | docs/adr/003-api-versioning.md | Decision rationale |
```

\## 精选知识

\### 官方文档
| 主题 | 来源 | 选用原因 |
|------|------|----------|
| Fastify routing | https://fastify.dev/docs/latest/Guides/Getting-Started | 官方文档，与我们使用的 v4.x 版本匹配 |
| Prisma relations | https://www.prisma.io/docs/orm/prisma-schema/data-model/relations | schemas 模式方面的权威参考 |

\### 团队认可的博客与文章
| 设计理念 | 来源 | 对我们思路的影响 |
|----------|------|------------------|
| 异常处理模式 | [团队审核通过的博客地址] | 比官方文档更清晰，包含实用示例 |
| 测试策略 | [团队审核通过的博客地址] | 影响了我们的测试架构设计 |

\### 内部参考资料
| 主题 | 文件路径 | 核心内容 |
|------|----------|----------|
| 异常规范 | docs/error-handling.md | 团队专属的模式 |
| API 设计决策 | docs/adr/003-api-versioning.md | 决策的依据与思路 |
</div><br/>

请保持内容精选而非全面。选取 5 到 10 个真正塑造团队工作方式的来源。

### 4. 项目结构
*我会对新员工说：“这是各类文件的存放位置。文件路径很重要。”*

<ins>东西都在哪里。文件路径至关重要</ins>。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
src/
├── lib/
│   ├── services/      # Business logic (UserService, OrderService)
│   ├── repositories/  # Database access layer
│   ├── schemas/       # Zod validation schemas
│   └── utils/         # Pure utility functions
├── routes/            # Fastify route handlers
├── middleware/        # Auth, logging, error handling
├── types/             # TypeScript type definitions
└── config/            # Environment-specific config
```

```
src/
├── lib/
│   ├── services/      # 业务逻辑（用户服务、订单服务）
│   ├── repositories/  # 数据访问层
│   ├── schemas/       # Zod 校验规则
│   └── utils/         # 纯工具函数
├── routes/            # Fastify 路由处理
├── middleware/        # 认证、日志、异常处理
├── types/             # TypeScript 类型定义
└── config/            # 环境相关配置
```

</div></br>

### 5. 命名规范
*我会对新员工说：“这是我们的命名规范。一致性比个人偏好更重要。”*

<ins>明确的规范可以避免代码风格混乱</ins>。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
## Naming Conventions
- **Files**: kebab-case (`user-service.ts`, not `UserService.ts`)
- **Functions**: camelCase, verb-first (`createUser`, `validateToken`)
- **Types/Interfaces**: PascalCase with descriptive suffixes (`UserCreateInput`, `AuthResponse`)
- **Constants**: SCREAMING_SNAKE_CASE (`MAX_RETRY_COUNT`)
- **Boolean variables**: is/has/can prefix (`isActive`, `hasPermission`)
```

\## 命名规范
- **文件**：短横线命名法（kebab-case）示例：`user-service.ts`，而非 `UserService.ts`
- **函数**：小驼峰命名法（camelCase），动词开头 示例：`createUser`、`validateToken`
- **类型/接口**：大驼峰命名法（PascalCase），搭配描述性后缀 示例：`UserCreateInput`、`AuthResponse`
- **常量**：大写下划线命名法（SCREAMING_SNAKE_CASE） 示例：`MAX_RETRY_COUNT`
- **布尔型变量**：以 is/has/can 开头 示例：`isActive`、`hasPermission`

</div></br>

### 6. 代码示例
*我会对新员工说：“这是我们认可的优质代码示例。请遵循这种写法。”*

用示例展示，而不只是口头说明。从代码库中选取 2–3 个 “优质代码” 案例。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```TypeScript
// lib/services/user-service.ts
import { prisma } from '../db/client'
import { UserCreateInput, UserResponse } from '../types/user'
import { hashPassword } from '../utils/crypto'

export async function createUser(input: UserCreateInput): Promise<UserResponse> {
  const hashedPassword = await hashPassword(input.password)
  
  const user = await prisma.user.create({
    data: {
      ...input,
      password: hashedPassword,
    },
    select: {
      id: true,
      email: true,
      createdAt: true,
      // Never return password
    },
  })
  
  return user
}
```
</div></br>

注意：服务采用纯函数实现，而非类。需要时通过参数传入依赖项。

### 7. 需避免的反模式
*我会对新员工说：“这些是 *不要做* 的事。我们都是付出过代价才总结出这些经验的。”*

<ins>明确告诉 AI 哪些做法不可取，以此避免常见错误</ins>。

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
## Anti-patterns (Do NOT use)
- Class-based services (use functional approach)
- Express.js patterns (this project uses Fastify)
- Storing JWT in localStorage (use httpOnly cookies)
- Using any type (always define proper types)
- Putting business logic in route handlers (use services)
- Raw SQL queries (use Prisma ORM)
```

\## 反模式（严禁使用）
- 基于类的服务（采用函数式写法）
- Express.js 相关写法（本项目使用 Fastify）
- 将 JWT 存储在 localStorage 中（使用 httpOnly 类型 Cookie）
- 使用 any 类型（务必定义合理的类型）
- 将业务逻辑写在路由处理器中（使用服务层）
- 原生 SQL 查询（使用 Prisma ORM）
</div><br/>

## 预置作为基础设施，而非个人习惯
我认为最有效的方式，是将知识预置视为基础设施，而非依赖个人习惯。

<ins>与其在每次会话开始时手动粘贴上下文（这种习惯很容易被遗忘），不如将预置文档存放在代码仓库中，使其自动生效</ins>：
*「译注：可惜缺少其他开源智能体（如：OpenCode，CLine）的参考」*

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
# Cursor
.cursor/
├── rules                    # Always-on project context (auto-loaded)
└── commands/
└── priming.md          # Referenceable with @priming

# GitHub Copilot
.github/
└── copilot-instructions.md  # Workspace-level instructions

# Claude Projects
Upload priming doc to Project Knowledge
```

```
# Cursor
.cursor/
├── rules                    # 常驻项目上下文（自动加载）
└── commands/
└── priming.md               # 可通过 @priming 直接引用

# GitHub Copilot
.github/
└── copilot-instructions.md  # 工作区级指令

# Claude 项目
将预置文档上传至 Project Knowledge
```

</div></br>

为何基础设施优于手动复制粘贴
- 支持版本控制：变更可审计、可评审
- 自动生效：无需每次对话手动复制粘贴
- 团队全局一致：所有人使用相同上下文
- 可在 PR 中评审变更：治理流程融入现有工作流

这将知识预置从 “个人效率技巧” 转变为 “团队基础设施”，
区别在于：一个是容易被遗忘的个人习惯，一个是能长期落地的团队实践。

正如新员工入职材料会作为组织资产长期维护，而非每次临时拼凑，预置文档也应被视为一等重要的项目产出物。

## 常见误区
在我的实践中，我观察到了几种失败模式 (failure mode)：

| 误区 | 优化方案 |
|------|----------|
| 信息过多：长达 20 页以上的文档会让 AI 过载、分散重点 | 精简为 1 至 3 页必要上下文 |
| 描述笼统：“现代最佳实践” 对 AI，等于什么也没说| 要具体："Fastify 4.x、Prisma 5.x、函数式服务" |
| 缺少示例：只文字描述模式 (pattern)，无代码参照 | 包含代码仓库中 2-3 真实代码片段 |
| 过时上下文：6 个月前的预置文档 | 每个月或在重大变更时，定期审查更新 |
| 缺失反模式：只告诉 AI 什么能做，未明确禁止项 | 清确列出需要禁止的编码模式与设计方案 |

### 信息过载陷阱
<ins>常见误区之一，是将预置文档写成面面俱到的完整文档。
不应该是这样的。
它是给 AI 的速查手册 (cheat sheet)，用于生成规范代码的最小上下文</ins>。

若预置文档篇幅超过 3 页，请自查以下问题：
- AI 生成服务（代码），真的需要所有内容吗？
- 详细文档能否存放在其他地方，此处仅做引用？
- 是否包括了极少用到的边缘场景？

AI 随时可以追问补充信息。先聚焦，仅在需要时再做扩充。

## 保持预置文档持续更新
文档总会逐渐失效。
每个团队都有过时的维基和陈旧的 README 的墓地。
如何避免预置文档沦为其中一员？

将其视作代码，而非普通文档：
- 放入代码仓库：`docs/ai-priming.md`
- 修改需 PR 评审（与代码变更流程一致）
- 技术负责人每季度审查（适配依赖更新）

引用，不要复制：
- 认证相关设计决定："详见 ADR-007"
- API 接口契约："参考 `/api/schema.yaml` 中的 OpenAPI spec"
- 部署模式："查阅运维操作手册"

更新触发：

| 触发场景 | 对应操作 |
| ---- | ---- |
| 框架 (framework) 版本更新 | 更新技术栈板块 (stack section) |
| 新增架构模式 | 补充示例代码 |
| AI 重复性错误 | 增加反模式 |
| 大规模重构 | 审查结构板块 (structure section) |

一份过期的预置文档，反而不如没有。
它会引导 AI 产出过时的代码模式。
而存放于代码仓库、像代码一样评审的预置文档，从机制上就能保证内容持续更新。

## 真实案例
以下是我过往项目中一份精简版的知识预置文档：

<div style="background-color: darkblue; padding: 8px; border-left: 4px solid lightblue;">

```
# Acme API - Priming Context

## Quick Overview
B2B SaaS API for inventory management. Multi-tenant, event-driven.

## Stack
- Node.js 20, Fastify 4, TypeScript 5
- PostgreSQL 15 + Prisma 5 (multi-tenant via tenantId)
- Auth: Clerk (external), JWT validation middleware
- Queue: BullMQ + Redis for async jobs
- Testing: Vitest

## Trusted Sources
### Docs
- Fastify: https://fastify.dev/docs/latest
- Prisma multi-tenancy: https://www.prisma.io/docs/orm/prisma-client/queries/multi-tenancy

### Blogs We Follow
- BullMQ patterns: [team-vetted blog on queue handling]

### Internal
- ADRs: docs/adr/ (architecture decisions)
- Error handling: docs/error-conventions.md

## Structure
src/
├── modules/           # Feature modules (users/, products/, orders/)
│   └── [module]/
│       ├── service.ts    # Business logic
│       ├── routes.ts     # HTTP handlers
│       ├── schema.ts     # Zod schemas
│       └── types.ts      # TypeScript types
├── shared/            # Cross-cutting (db, auth, queue)
└── config/            # Env config

## Patterns
- Functional services (no classes)
- All queries include `where: { tenantId }` (multi-tenant)
- Validation at route level with Zod
- Errors thrown as `AppError` with status codes

## Anti-patterns
- No classes for services
- No raw SQL (use Prisma)
- No business logic in routes
- No hardcoded tenantId

## Example Service
[Include one short example from the codebase]
```

```
# Acme API - 预置上下文

## 简要概述
用于库存管理 B2B SaaS API，多租户、事件驱动。

## 技术栈
- Node.js 20、Fastify 4、TypeScript 5
- PostgreSQL 15 + Prisma 5（通过租户 ID 实现多租户隔离）
- 认证：Clerk (外部)、JWT 校验中间件
- 消息队列：BullMQ + Redis 处理异步任务
- 测试：Vitest

## 权威参考资料
### 官方文档
- Fastify：https://fastify.dev/docs/latest
- Prisma 多租户：https://www.prisma.io/docs/orm/prisma-client/queries/multi-tenancy

### 团队参考博文
- BullMQ 设计模式：[团队审定的队列处理技术博客]

### 内部文档
- 架构决策记录 (ADRs) ：docs/adr/
- 错误处理：docs/error-conventions.md

## 目录结构
src/
├── modules/           # 业务模块（用户、商品、订单等）
│   └── [模块名]/
│       ├── service.ts    # 业务逻辑
│       ├── routes.ts     # HTTP 处理器
│       ├── schema.ts     # Zod schemas
│       └── types.ts      # TypeScript 类型
├── shared/            # Cross-cutting（数据库、认证、队列）
└── config/            # 环境配置

## 模式
- 函数式服务 (禁止用类)
- 所有数据库查询必须包括 `where: { tenantId }` (多租户)
- 在路由层使用 Zod 进行参数校验
- 错误统一抛出带状态码的 `AppError`

## 反模式
- 禁止使用类来编写服务
- 禁止编写原生 SQL，(使用 Prisma)
- 禁止在路由层编写业务逻辑
- 禁止硬编码 tenantId

## 服务代码示例
[包含代码库中的一段简短的代码示例]
```

</div></br>

<ins>注意：全文控制在 50 行以内。这是目标原则：聚焦、具体、可操作</ins>。

## 取舍与局限
该方案并非毫无代价：
- 前期成本：编写并维护预置文档需要投入时间。
- 收益递减：处理极简任务时，这类额外开销得不偿失。
- 过期上下文风险：过期的预置文档，不如不用。
- 不能保证：即便预置内容良好，AI 有时也会产生错误的输出。

<ins>我推断，这套方式在复杂工作中收益最高，尤其适合跨多轮会话、需要团队协作的场景。
若是快速的工具函数开发，手动修改调整，比维护上下文基础设施更加高效</ins>。

## 总结
<ins>知识预置本质上就是手动 RAG ：在生成代码前，向 AI 的上下文窗口注入高价值、项目专属的信息。
其假设很直接 —— 明确上下文应该优先于通用默认配置，从而产生符合代码库的输出，而非 "网络通用方案“ </ins>。

<ins>我目前的观点是，关键转变在于：将上下文视作基础设施（代码仓库内受控的版本化文件），而非个人习惯（每次会话手动复制粘贴）</ins>。
基础设施能够长期存续，而个人习惯终将松懈流失。

预置是所有协作优化的底层根基。
当 AI 预先理解整体架构，设计先行的沟通会更高效；
当 AI 熟知团队规范，自定义指令的执行效果会更佳。
在知识预置上的投入，会产生长期复利价值。

## 结尾
### 重要修订记录
2026/2/24：首次发布
