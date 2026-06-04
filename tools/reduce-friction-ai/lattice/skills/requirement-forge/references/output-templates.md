# Requirement Forge — 输出模板

在写入 `.lattice/requirements/` 文档时阅读此文件。请严格使用这些模板。

---

## Apex File：`.lattice/requirements/index.md`

```markdown
---
project: [项目名称]
last_updated: [日期]
---

# Requirements Index —— [项目名称]

## Definitions

**Epic：** [来自加载的标准或内置默认值]
**Feature：** [来自加载的标准或内置默认值]

---

## Epics

### [Epic Name]
[一段描述。]

| Feature | Summary | Status | Priority | Depends On |
|---|---|---|---|---|
| [Feature A](features/feature-a.md) | 一行 summary | draft | P0 | — |
| [Feature B](features/feature-b.md) | 一行 summary | draft | P1 | Feature A |

<!-- 如果加载的标准包括§10 领域术语，添加：-->
## Glossary（词汇表）

| Term | Definition |
|---|---|
| [Term] | [来自标准§10 的项目特定定义] |

<!-- 如果在 intake 期间提供源 documents，添加：-->
## Source Materials（源材料）

| Document | Type | Features Derived |
|---|---|---|
| [document name or path] | [PRD / stakeholder notes / Jira export / etc.] | [Feature A, Feature B] |

## Deferred Items（推迟条目）
从源材料中故意不映射到此 cycle 中任何 feature 的内容。

- [Item] — 推迟的原因
```

---

## Feature File：`.lattice/requirements/features/{feature-name}.md`（Feature 文件）

```markdown
---
feature: [Feature Name]
epic: [Epic Name]
status: draft
priority: [来自加载的标准]
depends_on: []
personas: []
source_docs: []
---

# [Feature Name]

## Problem Statement（问题陈述）

## User / Personas（用户/角色）
谁会遇到这个问题？请列出具体的用户类型或角色 —— 而不是泛指的 “用户”。

## Scope（范围）
**In scope：**
**Out of scope：**

## Boundary Conditions（边界条件）

## Assumptions（假设）
团队在推进时暂时视为真实的前提陈述。如果任何假设被证明是错误的，请重新审视受影响的场景。

## Scenarios（场景）

### Scenario 1: [Verb phrase]
[一句话描述情况。]

**Acceptance Criteria：**
- Given [context], when [action], then [outcome]

### Scenario 2: [Verb phrase]
...

*（场景按时间顺序排列——即自然的实现顺序。）*

## Implementation Notes（实现注释）
1. ...
2. ...

## Open Questions（开放问题）
- [ ] ...

## Links（链接）
- Design: *（当 design-blueprint 为此 feature 创建上下文锚点文档时更新）*
- Epic index：[index.md](../index.md)
```
