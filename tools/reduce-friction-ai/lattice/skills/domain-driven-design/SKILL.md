---
name: domain-driven-design
description: "在处理领域代码时应用 DDD 战术模式。强制执行 aggregate 设计、value objects over primitives、entity 身份规则和 bounded context 边界。在创建或修改领域模型、设计 aggregates、在 domain 层工作时使用，或当用户提到'domain'、'aggregate'、'value object'、'entity'、'bounded context'或'DDD'时使用。"
---

# Domain-Driven Design（领域驱动设计）

## 配置解析

Skill 支持项目自定义。解析：

1. 在仓库根目录查找 `.lattice/config.yaml`
2. 如果找到，检查 `paths.ddd_principles` 自定义文档路径
3. 如果自定义路径存在，读取文档，检查 YAML frontmatter `mode`：
   - **`mode: override`**（或无 mode）：自定义文档完全优先。
     替代嵌入默认值。必须全面——唯一参考。
   - **`mode: overlay`**：首先读取嵌入的 `./references/defaults.md`，然后将
     自定义文档部分叠加在上面。自定义部分替换匹配
     默认部分（按标题匹配）。新部分追加在默认值之后。
4. 如果没有配置、没有路径或路径未找到，读取 `./references/defaults.md`
5. **语言适配**：如果配置中存在 `paths.language_idioms`，读取 **"Type System & Object Model"** 部分并将 entity、value object 和 aggregate 实现模式适配到语言构造（例如，struct vs class、trait vs interface、data class vs record）。语言习惯用法优先于伪代码默认值。

Skill 附带的默认值代表有观点的最佳实践。
开箱即用于任何项目。仅当团队有与默认不同的特定标准时才覆盖。

## 自我验证清单（Self-Validation Checklist）

生成每个组件后 STOP。在继续之前验证所有以下内容。如果检查明显失败，在呈现之前修复代码。如果检查是判断调用且有多个有效方法（参见 Ambiguity Signal），标记——呈现选项和推理而非静默选择。

1. **ENTITY VS VALUE OBJECT**：每个领域对象——业务上跟踪独立实例随时间变化？是 → 有身份的 entity。否 → 不可变且自验证的 value object。
2. **AGGREGATE BOUNDARY**：事务不变性需要此对象在 aggregate 内部？如果否 → 通过 ID 分离 aggregate 引用。
3. **RICH BEHAVIOR**：Entity 有方法强制执行业务规则、guard 状态转换、触发 event？如果 entity 只是数据持有者 → 将逻辑从 service 移动到 entity。
4. **VALUE OBJECT COVERAGE**：扫描应该是 value object 的原始类型——string email、number amount、raw UUID as identifier → 包装 validate 的 value object。
5. **AGGREGATE COHESION**：列出 root 强制执行的业务规则。每个 internal entity 参与至少一个不变性？如果否 → 属于自己的 aggregate。
6. **DOMAIN EVENTS**：Domain event 为 state transition 触发其他 aggregate 响应、更改触发通知、审计/合规要求？不要为内部更改触发 event，没有其他部分响应。
7. **DOMAIN SERVICE**：Stateless 逻辑跨越多个 entity 放在 domain service 而非 application service？避免 I/O 和基础设施调用？
8. **FACTORY**：复杂 aggregate 创建封装工厂方法（`Order.create(...)`）或独立 factory class？初始创建和从持久化重建分开处理？

## 主动反模式扫描（Active Anti-Pattern Scan）

在验证上述清单后，扫描输出这些特定反模式。如果找到任何，在呈现代码之前修复。

- [ ] **贫血领域模型**：Entity 仅数据持有者 getter/setter；所有逻辑在 service 中 → 将业务规则移动到 entity 和 value object
- [ ] **原始痴迷**：Raw string for email、number for money、UUID for ID → 包装 validate 的 value object
- [ ] **上帝 Aggregate**：Aggregate 许多 entity、加载慢、高争用 → 分解仅保留共享事务不变性的内容
- [ ] **跨 Aggregate 事务**：Service 在一个事务中更新两个 aggregates → 使用 domain event 最终一致性
- [ ] **泄漏领域逻辑**：业务规则在 controller、application service 或 infrastructure 中 → 提取到 domain object 或 domain service
- [ ] **错误识别的 Entity/Value Object**：无生命周期的 entity，或有跟踪身份的 value object → 应用身份测试

## 模糊信号（Ambiguity Signals）

这些检查通常有多个有效结果。当遇到时，呈现选项而非静默选择。

- **Aggregate 边界大小**：Small aggregate（更多 event、最终一致性）vs large aggregate（简单事务、立即一致性）。两者都不 inherent 正确——取决于争用模式和不变性范围。
- **Entity vs Value Object**：某些概念（如 `Address` 或 `Money`）可能需要也可能不需要身份取决于领域复杂度。应用身份测试，但承认当 borderline 时。
- **Domain Service vs Entity Method**：逻辑跨越多个 entity 可以生活在 domain service 或作为 primary entity 上的方法。选择取决于哪个 entity"拥有"不变性。
- **对象创建模式**：Aggregate root 上的工厂方法、独立 factory class、builder pattern 或普通构造函数——取决于组装复杂度和团队约定。不规定模式；询问团队偏好哪种方法。

## 范围声明（Scope Statement）

Skill 在单个 repo、单个 bounded context（例如，一个 API——Order、User、Pricing）内操作。仅涵盖战术 DDD 模式——不涉及战略 DDD（无 context map、无微服务拓扑、无 bounded context 集成）。

如果任务似乎跨越多个 bounded context（例如，Order feature 调用 Shipping logic），在继续之前标记："此任务触及 [Context A] 和 [Context B]。跨上下文集成是战略 DDD——超出此 skill 的范围。你想限定到一个 context，还是继续知道跨上下文协调是你的责任？"

`framework:architecture` 提供结构外壳——代码在哪里、哪些层存在、依赖流向哪个方向。此 skill 定义如何在*内部*制作领域：rich model、invariant、aggregate boundary、ubiquitous language。

## 核心原则（Core Principle）

领域模型是业务规则的权威表达。Rich domain object 封装行为并强制执行不变性。代码使用业务的通用语言。

如果业务规则存在，应该通过领域模型表达——而非分散在 controller、application service 或 infrastructure 中。Entity 只是数据持有者，外部 service 做所有工作是贫血模型，此 skill 防止的主要反模式。

参见 `./references/defaults.md` 获取 aggregate 设计规则、entity/value object/domain service/domain event/repository/创建模式的代码示例、内联反模式警告和分解指南。
