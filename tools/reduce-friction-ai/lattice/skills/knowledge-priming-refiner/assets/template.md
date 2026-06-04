# Knowledge Priming Refiner 模板

此模板定义 `.lattice/standards/knowledge-base.md` 输出文档的结构。它包含 5 部分解剖结构，以及每个部分的访谈指导注释。

生成输出时，剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释。最终文档是参考文档，而非对话日志。

---

## Frontmatter（前置元数据）

<!-- INTERVIEW GUIDANCE:
默认使用 override，因为每个项目的知识基线都是独特的。
-->

```yaml
---
feature: "<项目名称> 知识基线"
mode: override
created: "<日期>"
---
```

---

## 序言（Preamble）

<!-- INTERVIEW GUIDANCE:
包含与所选模式匹配的序言。输出中只出现一个序言。
-->

**覆盖（override）序言：**

> 这是 [项目名称] 的知识基线。它为 AI 提供项目特定的上下文——技术栈、架构、可信来源和项目结构——以便生成的代码适合此代码库，而非默认使用通用模式。

**叠加（overlay）序言：**

> 此文档包含 [项目名称] 的选定知识基线部分。此处包含的部分是明确定义的；其他部分可以在未来修订中添加。

---

## 1. 架构概览（Architecture Overview）

<!-- INTERVIEW GUIDANCE:
目的：给 AI 大局观——这是什么类型的应用、主要组件是什么、它们如何交互。没有这个，AI 只能从文件名猜测架构。

询问："这是什么类型的应用？主要组件是什么，它们如何交互？"

追问问题：
- 这是单体、微服务、无服务器还是其他？
- 主要模块或服务是什么？
- 组件如何通信（HTTP、消息队列、事件、直接调用）？
- 是每个服务一个数据库还是共享数据库？
- 是否有任何塑造架构的外部集成？

良好内容的示例：

  这是一个基于微服务的电商平台。
  - **API Gateway**：处理路由、认证、限流
  - **User Service**：认证、用户档案、偏好设置
  - **Order Service**：购物车、结账、订单历史
  - **Notification Service**：邮件、短信、推送通知

  服务通过异步消息队列（RabbitMQ）通信。
  每个服务拥有自己的数据库（PostgreSQL）。

保持在 5-10 行。这是电梯演讲，而非架构文档。
-->

[此处填写架构概览内容]

---

## 2. 技术栈和版本（Tech Stack and Versions）

<!-- INTERVIEW GUIDANCE:
目的：告诉 AI 确切使用哪些技术和版本。版本号很重要，因为 API 在不同版本之间会变化——"Prisma 5.x"告诉 AI 使用哪个查询 API，而仅说"Prisma"可能会为任何版本生成代码。

包含"非 X"澄清——这些引导 AI 远离不适用的常见默认值。例如，"Fastify 4.x（非 Express）"防止 AI 默认使用 Express 模式。

此部分捕获工具**身份**（哪个框架、哪个版本）。语言级**习惯用法**（如何编写错误处理、如何组织测试、DI 如何工作）属于由 `language-idioms-refiner` 生成的 language-idioms 文档。

询问："你的项目使用哪些技术？尽可能包含版本号。是否有你明确不使用的常见替代方案？"

追问问题：
- 运行时和语言版本？（Node.js 20.x、Python 3.12、Go 1.22）
- 框架和版本？（Fastify 4.x、Django 5.x、Spring Boot 3.x）
- 数据库和 ORM？（PostgreSQL 15 + Prisma 5.x）
- 认证方式？（JWT + httpOnly cookies、OAuth2 + Clerk）
- 测试框架和运行器？（Vitest、pytest、Go testing）——仅工具名称和适用的"非 X"；测试习惯用法和模式由 language-idioms 文档覆盖
- 验证库？（Zod、Pydantic、Joi）
- 是否有任何"非 X"澄清？（Fastify 而非 Express、Vitest 而非 Jest）

良好内容的示例：

  - **运行时**：Node.js 20.x（LTS）
  - **框架**：Fastify 4.x（非 Express）
  - **数据库**：PostgreSQL 15 + Prisma ORM 5.x
  - **认证**：JWT + httpOnly cookies（非 localStorage）
  - **测试**：Vitest + Testing Library（非 Jest）
  - **验证**：Zod schemas（非 Joi）

"非 X"澄清特别有价值——它们是其他 skills 无法知道的特定于项目的反模式。
-->

[此处填写技术栈内容]

---

## 3. 精选知识来源（Curated Knowledge Sources）

<!-- INTERVIEW GUIDANCE:
目的：将 AI 指向团队真正信任的来源，而非让它从通用互联网中获取。每个团队都有他们阅读的官方文档、影响其架构的博客文章和捕获宝贵经验的内部参考。

询问："你的团队最信任的 5-10 个来源是什么？官方文档、博客文章、内部参考？"

追问问题：
- 你实际引用哪些官方文档？（不是所有文档——是你使用的那些）
- 是否有塑造你的架构或模式的博客文章或文章？
- 你是否有内部文档（ADRs、错误约定、API 设计指南）？
- 是否有任何特别不信任或已过时的来源？

良好内容的示例：

  ### 官方文档
  | 主题 | 来源 | 我们信任它的原因 |
  |------|------|------------------|
  | Fastify 路由 | https://fastify.dev/docs/latest/Guides/Getting-Started | 官方，匹配我们的 v4.x |
  | Prisma 关系 | https://www.prisma.io/docs/orm/prisma-schema/data-model/relations | 模式模式的权威来源 |

  ### 内部参考
  | 主题 | 路径 | 捕获内容 |
  |------|------|----------|
  | 错误约定 | docs/error-handling.md | 我们的特定模式 |
  | API 设计决策 | docs/adr/003-api-versioning.md | 决策依据 |

保持精选——5-10 个高价值来源，而非全面的书目。
-->

[此处填写精选知识来源内容]

---

## 4. 项目结构（Project Structure）

<!-- INTERVIEW GUIDANCE:
目的：告诉 AI 各部分的位置。当 AI 知道目录布局时，它会正确放置新文件并使用正确的导入路径。没有这个，AI 会猜测——而且经常猜错。

询问："你的目录结构是什么样的？展示前 2-3 层。"

追问问题：
- 业务逻辑在哪里？
- 路由处理器/控制器放在哪里？
- 类型或 schema 在哪里定义？
- 是否有用于横切关注点的共享/公共目录？
- 测试如何组织（内联、单独的 test/ 目录）？
- Monorepo？如果是，包/工作区结构是什么？

良好内容的示例：

  src/
  +-- modules/           # 功能模块（users/、products/、orders/）
  |   +-- [module]/
  |       +-- service.ts    # 业务逻辑
  |       +-- routes.ts     # HTTP 处理器
  |       +-- schema.ts     # Zod schemas
  |       +-- types.ts      # TypeScript 类型
  +-- shared/            # 横切关注点（db、auth、queue）
  +-- config/            # 环境配置

使用 ASCII 树格式。包含简要注释解释每个目录的内容。
-->

[此处填写项目结构内容]

---

## 5. 项目约定（Project Conventions）

<!-- INTERVIEW GUIDANCE:
目的：捕获其他 skills 无法推断的项目特定约定。此部分故意精简——编码原则（命名、函数设计、反模式）由 clean-code atom 处理。仅包含此项目独有的约定。

询问："是否有任何从技术栈和结构中无法明显看出来的项目特定约定？例如，文件命名模式、模块组织规则或团队特定实践？"

属于此部分的示例：
- "文件使用 kebab-case（user-service.ts，而非 UserService.ts）"
- "每个模块通过 index.ts barrel 文件导出"
- "功能标志通过 LaunchDarkly 管理；绝不硬编码开关"
- "所有 API 响应遵循 { data, error, meta } 信封格式"

不属于此部分的示例（由其他 skills 覆盖）：
- 函数命名原则（clean-code）
- SRP、复杂度、错误处理模式（clean-code）
- 层职责、依赖方向（architecture）
- Aggregate 设计、value objects（domain-driven-design）
- 语言级习惯用法：命名大小写约定、错误处理哲学、测试模式、DI 方法（language-idioms 文档）

此部分是可选的。如果项目除了技术栈暗示之外没有约定，可以跳过它。
-->

[此处填写项目约定内容——如果没有则省略]

---

## 页脚（Footer）

<!-- INTERVIEW GUIDANCE:
在输出中包含项目名称、生成日期和模式指示器。
-->

---
*为 [项目名称] 生成于 [日期]。模式：[override|overlay]*
*由 knowledge-priming-refiner skill 生成。*
