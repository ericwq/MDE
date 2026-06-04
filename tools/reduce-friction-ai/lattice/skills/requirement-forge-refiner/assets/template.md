---
mode: overlay
project: "[项目名称]"
date: "[日期]"
---

<!-- INTERVIEW GUIDANCE:
OVERLAY PREAMBLE——在 overlay 模式输出中逐字包含此文本，替换项目名称和日期。
OVERRIDE PREAMBLE——用下面的 override 序言替换。
-->

此文档为 **[项目名称]** 定制内置需求标准。此处呈现的部分覆盖 `requirement-quality` atom 中的相应默认值。此处未出现的部分使用 atom 的嵌入默认值不变。

<!-- OVERRIDE PREAMBLE（对于 mode: override 使用此替代 overlay 序言）：
此文档为 **[项目名称]** 定义需求标准。`requirement-quality` atom 使用此文档作为其唯一的结构决策来源——无内置默认值适用。
-->

---

## §1 Epic 定义（Epic Definition）

<!-- INTERVIEW GUIDANCE:
默认：Epic 是一组相关 features 的命名组，形成连贯的产品区域或能力。一个 epic 代表产品价值的有意义的可交付增量——不是单个 feature，也不是整个产品。命名：Title Case 名词短语（例如，"User Authentication"、"Payment Processing"）。

询问："这个 epic 的定义匹配你们团队思考 epics 的方式吗？是否有任何你想要设置的大小或范围约束？"

追问问题：
- "你期望典型的 epic 包含多少 features——3-5？10+？"
- "'epic'是你团队已经使用的术语，还是你使用不同的术语？"
- "Epics 应该映射到团队拥有权、产品区域或交付里程碑吗？"

可自定义：命名约定、范围描述、大小期望。
固定：epics 是导航性的——它们 group features，不直接实现。
-->

Epic 是一组相关 features 的命名组，形成连贯的产品区域或能力。一个 epic 代表产品价值的有意义的增量——不是单个 feature，也不是整个产品。

**命名：** Title Case 名词短语（例如，"User Authentication"、"Payment Processing"）。

---

## §2 Feature 定义（Feature Definition）

<!-- INTERVIEW GUIDANCE:
默认：Feature 是完整、自包含的产品行为单元。它是独立可设计和可实现的——它可以交给 design-blueprint 而不解决其范围外部的未知数。无法完全 specced 的 feature，不知道其边界之外的东西的答案，还不够独立。

大小信号：超过 5 个 scenarios 表明 feature 应该 split。少于 2 个 scenarios 的 feature 可能太小，应该与相关 feature merge。

询问："这个 feature 的定义匹配你所说的'feature'吗？大小信号对你的上下文正确吗？"

追问问题：
- "你的 features 通常在多大之前感觉太大？"
- "Features 应该映射到单独的 design-blueprint 会话，还是更大的范围可以？"
- "在你的领域中是否有 cases 一个 feature  legitimately 有很多 scenarios？"

可自定义：大小信号（最大 scenarios）、独立性的描述。
固定：features 必须独立可设计和可实现。
-->

Feature 是完整、自包含的产品行为单元。它是独立可设计和可实现的——它可以交给 design-blueprint 然后 code-forge，而不解决其范围外部的未知数。

**大小信号：** 超过 5 个 scenarios 表明 feature 应该 split。少于 2 个 scenarios 的 feature 表明它应该与相关 feature merge。

---

## §3 Feature 独立性规则（Feature Independence Rule）

<!-- INTERVIEW GUIDANCE:
默认：如果以下情况，feature 被认为是独立的：(a) 所有要求和 acceptance criteria 在 feature 文件中完全指定，(b) 在 design-blueprint 开始之前不需要外部决策或设计，(c) 它独立交付连贯的用户价值——用户可以使用此 feature alone 完成有意义的操作。

询问："这个独立性规则匹配你期望的吗？在你的领域中是否有 exceptions——features 故意依赖另一个 in progress？"

追问问题：
- "是否有任何 cases 两个 features 必须一起设计？"
- "'独立交付连贯的用户价值'是正确的测试，还是部分价值可以接受？"

可自定义：具体标准 (a, b, c) 可以调整。
固定：独立性对于 pipeline（forge → design → code）在没有 constant re-briefing 的情况下工作是必需的。
-->

当以下情况时，feature 被认为是独立 specced：
- 所有要求和 acceptance criteria 在 feature 文件中完全陈述。
- 在 design-blueprint 开始之前不需要外部决策。
- 它独立交付连贯的用户价值——用户可以使用此 feature alone 完成有意义的操作。

---

## §4 Scenario 定义（Scenario Definition）

<!-- INTERVIEW GUIDANCE:
默认：Scenario 是 feature 必须处理的有界情况。Scenarios 有名称，按时间顺序排序（自然实现序列），包含 3-6 acceptance criteria。默认 nomenclature 是"scenario"。

每个 feature 的最大值：~5。如果 feature 积累更多，挑战是否应该 split。
每个 feature 的最小值：2——至少一个 happy path 和一个 failure 或 edge case。单个 scenario 的 feature 可能太小，无法 standalone。

每个 scenario 的最大 ACs：~6。如果 scenario 积累更多，挑战 scenario 是否太 broad。

询问："'scenario'感觉像你们团队的正确词汇吗？一些团队使用'use case'、'story'、'case'或'flow'。每个 scenario 的 3-6 AC 范围对你的上下文正确吗？"

追问问题：
- "你的团队是否已经有这个概念的现有名称？"
- "每个 scenario 6 ACs 对你的典型 feature 复杂度太 tight 还是正好？"

可自定义：nomenclature、最大 scenario 计数、最大 AC 计数。
固定：scenarios 按时间顺序排序（自然实现序列）——这是不可协商的，因为它使 feature 文件可直接用作 design-blueprint 和 code-forge 的输入。不要提供替代排序。
-->

Scenario 是 feature 必须处理的有界情况。Scenarios 是：
- 命名用描述情况的动词短语（例如，"User submits valid form"）。
- 按时间顺序排序——自然实现序列。
- 有界：每个 scenario 3-6 acceptance criteria。如果需要更多，scenario 太 broad。

**Nomenclature：** scenario
**排序：** chronological——自然实现序列。不可协商。
**每个 feature 的最大值：** 5。超过 5 表明 feature 应该 split。
**每个 feature 的最小值：** 2。至少一个 happy path 和一个 failure 或 edge case。
**每个 scenario 的最大 ACs：** 6。超过 6 表明 scenario 太 broad。

---

## §5 Acceptance Criteria 格式（Acceptance Criteria Format）

<!-- INTERVIEW GUIDANCE:
默认：Given/When/Then。每个 criterion："Given [context], when [action], then [outcome]."

询问："你的团队用 Given/When/Then 格式写 ACs，还是你更喜欢不同的风格？"

如果用户不确定要展示的选项：
1. Given/When/Then——结构化，映射到 test cases，feature specs 中最常见
2. Bullet statements——"The system must [do X] when [condition Y]."更快写，较少结构化。
3. Numbered——与 bullets 相同但编号用于 traceability。
4. Hybrid——行为 ACs 用 Given/When/Then，非功能或 constraint ACs 用 bullets。

追问问题：
- "你的 ACs 需要直接映射到 test cases 吗？"
- "你需要 ACs 上的 traceability IDs 用于 compliance 吗？"

可自定义：整个格式。
固定：ACs 必须可验证——每个有一个清晰的 pass/fail 条件。
-->

**格式：** Given/When/Then

每个 acceptance criterion：
```
Given [context 或 precondition],
when [action 或 event],
then [expected outcome].
```

ACs 必须可验证——每个有一个清晰的 pass/fail 条件。模糊的 ACs（"系统应该优雅地处理错误"）不可接受。

---

## §6 优先级标记（Priority Notation）

<!-- INTERVIEW GUIDANCE:
默认：P0 / P1 / P2。P0 = critical / must-have。P1 = important / should-have。P2 = nice-to-have。

询问："你的团队如何表达 feature 优先级？P0/P1/P2、MoSCoW、High/Medium/Low，还是其他？"

要展示的选项：
1. P0/P1/P2——compact，engineering-friendly
2. MoSCoW——Must/Should/Could/Won't——product-friendly，BA 实践中常见
3. High/Medium/Low——simple，广泛理解
4. Numbered（1, 2, 3）——simple ordinal
5. Custom——团队定义自己的值

追问问题：
- "Priority 应该仅在 feature 级别，还是在 feature 内的 scenario 级别？"
- "谁设置 priority——product、engineering，或 jointly？"

可自定义：notation 和 labels。
固定：priority 必须出现在 feature 文件的 frontmatter 中，以便 molecule 可以使用它。
-->

**Notation：** P0 / P1 / P2

| 值 | 含义 |
|---|---|
| P0 | Critical——必须交付以使 epic 可 ship |
| P1 | Important——应该交付；仅在约束下可以 defer |
| P2 | Nice-to-have——如果 capacity 允许则交付 |

Priority 在 feature 级别（frontmatter 字段）。

---

## §7 状态工作流（Status Workflow）

<!-- INTERVIEW GUIDANCE:
默认：draft → approved → in-design → implemented

询问："这个状态工作流匹配 features 如何通过你团队流程移动吗？你需要额外的 statuses 吗？"

追问问题：
- "是否有'sready for design'status 不同于'approved'？"
- "你跟踪 feature 是'in development'vs'designed'吗？"
- "你是否 ever mark features 为 deprecated 或 cancelled？"

常见添加：
- ready-for-design（在 approval 之后，design 开始之前）
- in-development（在 design approved 之后，code in progress）
- deprecated / cancelled

可自定义：整个 workflow 和 status labels。
固定：status 字段必须存在于 feature 文件的 frontmatter 中。
-->

Feature status 按此顺序进展：

`draft` → `approved` → `in-design` → `implemented`

| Status | 含义 |
|---|---|
| draft | 正在 speccing——尚未 agreed |
| approved | Spec agreed；ready for design-blueprint |
| in-design | design-blueprint session in progress 或 complete |
| implemented | code-forge complete；feature shipped |

---

## §8 命名约定（Naming Conventions）

<!-- INTERVIEW GUIDANCE:
默认：
- Epic names：Title Case 名词短语
- Feature file names：kebab-case（例如，user-login.md）
- Feature display names：Title Case
- Scenario names：sentence case 的动词短语（例如，"User submits valid form"）

询问："你的团队是否已经有应该 carry over 的命名约定？"

追问问题：
- "Feature file names 应该包括 epic name 作为 prefix 吗？"
- "你是否有命名跨越多个 epics 的 things 的约定？"
- "Scenario names 应该包括 ID 用于 traceability（例如，SC-001）吗？"

可自定义：所有命名约定。
固定：file names 必须是 filesystem-safe（无 spaces，preferred lowercase）。
-->

| Artifact | 约定 | 示例 |
|---|---|---|
| Epic name | Title Case 名词短语 | "Payment Processing" |
| Feature file name | kebab-case | `user-login.md` |
| Feature display name | Title Case | "User Login" |
| Scenario name | Verb phrase, sentence case | "User submits valid credentials" |

---

## §9 实现切片（Implementation Slices）

<!-- INTERVIEW GUIDANCE:
默认：高级"what"only，按时间顺序排序。每个 slice 描述将在该步骤中构建什么——不是如何实现。每个 feature 2-5 slices。

目的：给 developers 和 designers 一个自然的 build order，而不 prescribe implementation decisions。这些是 ordering hints，不是 story breakdowns。

询问："你想要 feature 文件的实现切片部分中的多少细节？高级 ordering hints，还是更详细的 breakdown？"

追问问题：
- "Slices 应该引用特定 components 或保持纯 behavioral？"
- "2-5 slice 范围合适，还是你的 features 倾向于需要更多？"

可自定义：详细级别、计数范围。
固定：slices 按时间顺序排序并描述"what"，而非"how"。
-->

Implementation slices 是 feature 文件内的 ordering hints——它们描述自然 build 序列而不 prescribe 技术实现。

**详细级别：** 高级"what"only。无实现 specifics。
**计数：** 每个 feature 2-5 slices。超过 5 表明 feature 可能太大。
**格式：** numbered list，每个是描述在该步骤中构建什么的短短语。

示例：
```
1. Core form render and validation
2. Submission and success state
3. Error states and retry logic
```

---

<!-- INTERVIEW GUIDANCE：§10+ 是 additions，不是 defaults。仅当用户定义领域特定术语或其他未被§1-§9 覆盖的项目特定标准时包含。

在结束时询问："是否有任何领域特定术语或项目特定约定我们应该记录在这里，以便 molecule 使用正确的语言？"

常见添加：
- 领域术语（molecule 应该一致使用的术语词汇表）
- 自定义 frontmatter fields（除了 feature、epic、status、priority）
- Cross-epic dependency notation
- Stakeholder 或 ownership fields
-->

---

*由 requirement-forge-refiner 生成 | 项目：[项目名称] | 日期：[日期] | 模式：overlay*
