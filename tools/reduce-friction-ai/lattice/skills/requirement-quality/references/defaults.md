# Requirements：默认标准（Default Standards）

requirement-quality atom 的嵌入默认值。有观点的护栏——通过 refiner 覆盖或直接写入 `.lattice/standards/requirement-standards.md`。

---

## §1 Epic 定义（Epic Definition）

Epic 是一组相关 features 的命名组，形成连贯的产品区域或能力。一个 epic = 产品价值的有意义的增量——不是一个 feature，不是整个产品。

**导航单元**：epics group features。不直接实现。不能交给 design-blueprint。

**命名**：Title Case 名词短语——"User Authentication"、"Payment Processing"、"Order Management"。

**大小信号**：典型 epic 包含 3-7 features。少于 2→与 sibling epic merge。超过 10→挑战是否一个 epic 还是两个。

---

## §2 Feature 定义（Feature Definition）

Feature 是完整、自包含的产品行为单元。必须独立可设计和可实现——可以交给 design-blueprint 而不解决其范围外部的未知数。

**完整性测试**：developer 可以打开此文件，阅读它，并开始 design-blueprint 而不问一个澄清问题？如果不→feature 不完整。

**大小信号**：超过 5 scenarios→挑战是否一个 feature 还是两个。少于 2 scenarios→考虑与相关 feature merge。

**不是 story**：feature 是原子单元。Story decomposition 是团队执行关注点，在 feature spec 下游。

---

## §3 Feature 独立性规则（Feature Independence Rule）

当以下所有条件成立时，feature 是独立 specced：

1. **文件内要求完整**——无"TBD"、无"refer to Feature X for details"、无阻塞设计的未解决 open questions。
2. **无外部设计依赖**——design-blueprint 可以在等待另一个 feature 首先设计之前开始。
3. **连贯的用户价值**——独立交付用户可以做的东西。部分价值可以接受；没有另一个 feature 的零值不可以。

不能满足 (1) 的 feature→不完整，不写文件。
不能满足 (2) 的 feature→在 `depends_on` frontmatter 中记录依赖并注释约束。
不能满足 (3) 的 feature→挑战它是否是 feature 还是内部技术 slice。

**Cross-epic placement**：当 feature 的 scenarios 从两个不同 epics 中 drawing behaviors，它属于其大部分行为生活的 epic（主要 owner）。在 feature 文件中添加 cross-reference note 指向其他 epic，并在 `depends_on` 中记录 cross-epic 依赖。跨两个 epics 分割的 feature 没有记录关系是 orphaned dependency——它将在 design-blueprint 期间作为 blocker 浮现。

---

## §4 Scenario 定义（Scenario Definition）

Scenario 是 feature 必须处理的有界情况。不是 user story。不是 acceptance criterion。是一个 situation——一个命名的、scoped 的 context，有自己的可验证 outcomes 集。

**Nomenclature**：scenario

**结构**：
- Name：sentence case 的动词短语——"User submits valid form"、"Session expires during checkout"
- Description：一句话描述此 scenario 覆盖的情况
- Acceptance criteria：商定的格式中的 3-6 items

**排序**：chronological——自然实现序列。第一个 scenario 是 developer 首先构建的东西。最后一个 scenario 通常是 edge cases 和 error handling。

**每个 feature 的最大值**：5。达到 5→pause 并询问这是否仍然是一个 feature。
**每个 feature 的最小值**：2——一个 happy path，一个 failure 或 edge case。

---

## §5 Acceptance Criteria 格式（Acceptance Criteria Format）

**格式**：Given/When/Then

```
Given [context 或 precondition],
when [action 或 event],
then [expected outcome].
```

**可验证性规则**：每个 AC 必须有清晰的 pass/fail 条件。读者必须能够说"这发生了或没有发生。"模糊的 ACs 不是 ACs。

| 可接受 | 不可接受 |
|---|---|
| Given a logged-in user, when they submit a valid form, then a success message appears within 2s | The system should handle the form gracefully |
| Given an expired session, when a request is made, then a 401 response with code SESSION_EXPIRED is returned | Errors should be handled properly |
| Given a file over 5MB, when the user attempts upload, then an error message states the size limit | Large files should be rejected |

**每个 scenario 的最大值**：6。超过→scenario 太 broad；split。

---

## §6 优先级标记（Priority Notation）

| 值 | 含义 |
|---|---|
| P0 | Critical——必须交付以使 epic 可 ship。无 workaround。 |
| P1 | Important——应该在此 cycle 中交付；仅在 hard constraints 下可以 defer。 |
| P2 | Nice-to-have——当 capacity 允许时交付；product 没有它仍然 viable。 |

Priority 在 feature 级别（frontmatter）。不在 scenario 级别。

---

## §7 状态工作流（Status Workflow）

`draft` → `approved` → `in-design` → `implemented`

| Status | 含义 |
|---|---|
| draft | 正在 speccing——尚未由 stakeholders agreed |
| approved | Spec agreed；ready for design-blueprint |
| in-design | design-blueprint session in progress 或 complete |
| implemented | code-forge complete；feature shipped |

---

## §8 命名约定（Naming Conventions）

| Artifact | 约定 | 示例 |
|---|---|---|
| Epic name | Title Case 名词短语 | "Payment Processing" |
| Feature display name | Title Case | "User Login" |
| Feature file name | kebab-case | `user-login.md` |
| Scenario name | Verb phrase, sentence case | "User submits valid credentials" |
| Open question | Imperative sentence ending in `?` | "Should rate limiting apply per user or per IP?" |

---

## §9 实现注释（Slices）

Implementation notes 描述 feature 的自然 build 序列——什么将首先、第二、第三构建。排序"what"而非"how"。每个 feature 2-5 slices。

**详细级别**：behavioral，而非 technical。"Core form validation and submission"而非"wire the zod schema to the POST /api/forms endpoint"。

**目的**：design-blueprint 和 code-forge 的 sequencing hint。不是 story breakdown。不是 task list。developer 使用这些作为 chronological guide，而非 prescribed tickets。

**格式**：numbered list，每个是短短语。

```
1. Core [behavior] — [what it enables]
2. [Next behavior] — [what it adds]
3. [Error/edge handling] — [what it protects]
```

---

*Defaults informed by BABOK (Business Analysis Body of Knowledge), BDD (Behaviour-Driven Development) practice, Gojko Adzic Specification by Example (2011), and product management craft.*
*默认值受 BABOK（商业分析知识体系）、BDD（行为驱动开发）实践、Gojko Adzic 的 Specification by Example（2011）和产品管理工艺影响。*
