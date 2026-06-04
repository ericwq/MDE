---
name: design-blueprint
description: "运行完整的设计工作流——从建立上下文通过 5 个渐进式设计层级到批准的 blueprint。Compose context anchoring、design-first methodology、architecture 和 DDD 为统一流程。处理新 features（创建 context doc）和恢复现有工作（加载 context doc）。在开始设计、规划架构时使用，或当用户说'design a feature'、'blueprint'、'start designing'、'plan the architecture'或'let's design before coding'时使用。"
---

# Design Blueprint（设计蓝图）

## 所需 Skills

按顺序读取并应用 skills：

1. `framework:knowledge-priming` —— 加载项目上下文（技术栈、架构、约定）ground decisions 真实项目
2. `framework:context-anchoring` —— 创建或加载 feature context anchor doc
3. `framework:learning-harvest` —— 加载 prior operational learnings inform design；在 session 结束时 harvest new patterns（始终）
4. `framework:collaborative-judgment` —— 浮现真正的 design judgment calls structured options 而非静默假设（始终）
5. `framework:design-first` —— Walk through 5 progressive design levels
6. `framework:architecture` —— 在 Component 和 Interaction 级别应用结构规则
7. `framework:domain-driven-design` —— 在 Component、Interaction、Contract 级别应用领域建模

## 工作流（Workflow）

### 第 1 步：建立上下文（Establish Context）

使用 `framework:learning-harvest` Load Behavior。Focus hint："design session — focus: design patterns, reliability, structural health"。Prior learnings 关于 decomposition choices、failure-prone patterns 和 structural debt inform design decisions 从一开始。

使用 `framework:context-anchoring` set up feature living doc。

- **Document Discovery**：检查现有 context anchor doc feature（扫描 context base directory，match by feature name 或 frontmatter）。
- **如果存在** → Load（context-anchoring Load Behavior）。呈现 structured acknowledgment——feature name、decision count、open questions、constraints。Resume last design checkpoint recorded doc。
- **如果不存在** → Create（context-anchoring Create Behavior）。从 template 创建新 feature doc。在创建之前与用户确认 feature name、summary、requirement doc link。

### 第 2 步：Walk the Design Levels（走过设计层级）

如果 key use cases 或 success criteria 现在不清楚，使用 `framework:collaborative-judgment` surface what needs answering 在开始 Level 1 之前。

通过 `framework:design-first` 5 levels sequentially drive。每个 level，呈现 design output，获取用户 approval，然后**在 advance 之前将 approved output persist into context anchor doc**。Context doc 是 blueprint——如果没有写下来，不存在。

**Enrichment rule**：在用户批准每个 level 之后，使用 `framework:context-anchoring` Enrich behavior write following into context doc：

1. **Approved level output** 本身（capabilities list、component diagram、interaction flows，或 contracts）——捕获为**clean、structured summary**在 dedicated section 下该 level。使用与 level presentation 相同的格式：Level 1 的 numbered list、Level 2 的 component table + diagram、Level 3 的 sequence/flow、Level 4 的 typed interfaces。
2. **Decisions made** 在 level discussion 期间——choices、reasoning、alternatives rejected。
3. **Constraints identified** —— emerged 的 non-negotiable boundaries。
4. **Open questions** surfaced 但 remain unresolved。

NOT advance next level until current level output persisted。Context doc 必须是 single source truth every stage。

在应用 architectural atoms 每个 level 时，使用 `framework:collaborative-judgment` surface real design judgment calls immediately——不在 batch 期间设计，每个 level constrains next。

Apply architectural atoms levels where add value：

**Level 1（Capabilities）**：
- 呈现 capabilities list per `framework:design-first`。
- On approval → Enrich context doc with approved capabilities under `## Design: Level 1 -- Capabilities` section。

**Level 2（Components）**：
- Apply `framework:architecture`——validate each component maps defined architectural layer、dependencies follow loaded architecture rules、component boundaries clear。
- Apply `framework:domain-driven-design`——identify aggregates、entities、value objects。Determine which components live domain layer which infrastructure。
- On approval → Enrich context doc with approved component list、layer assignments、diagram under `## Design: Level 2 -- Components` section。Log architectural decisions（layer choices、DDD classifications）Decisions Log。

**Level 3（Interactions）**：
- Apply `framework:architecture`——validate data flows follow patterns defined loaded architecture doc 和 boundary crossing rules respected。
- Apply `framework:domain-driven-design`——define aggregate interactions、domain events。Cross-aggregate communication should use domain events eventual consistency。
- On approval → Enrich context doc with approved interaction flows（sequence diagrams、data flow descriptions）under `## Design: Level 3 -- Interactions` section。Log flow decisions Decisions Log。

**Level 4（Contracts）**：
- Apply `framework:domain-driven-design`——define repository interfaces、value object types、aggregate root boundaries。Contracts should reflect tactical patterns agreed earlier levels。
- Apply `framework:architecture`——validate contracts respect boundary-data rules 和 interface ownership per loaded architecture doc。
- On approval → Enrich context doc with approved interfaces 和 type definitions under `## Design: Level 4 -- Contracts` section。Log contract decisions Decisions Log。

### 第 3 步：Finalize Blueprint（最终化蓝图）

在 Level 4（Contracts）approved 和 persisted 之后：

- **Verify completeness**：Context doc 现在必须包含所有四个 design level sections（Capabilities、Components、Interactions、Contracts）加上 design process 期间做出的每个 decision。如果任何 level output missing from doc，在 proceeding 之前 enrich now。
- **Write design summary**：使用 `framework:context-anchoring` Enrich add `## Design Summary` section 到 context doc containing：
  - Components 和 layer assignments
  - Key contracts 和 interfaces
  - Architectural constraints
  - Domain model decisions（如果 applicable）
  - Open questions resolved during design
  - Design status：**Approved -- ready for implementation**
- **Log completion decision**：Add decision entry Decisions Log："Design approved at Level 4。Blueprint complete ready for implementation。"
- Present summary user as confirmation。
- Design complete。**NOT proceed Level 5（Implementation）**——那是 separate concern handled by `framework:code-forge` molecule 或 equivalent implementation skill。
- Use `framework:learning-harvest` Harvest Behavior。Session context："design session — architectural decomposition and contract definition"。Synthesize 和 propose cross-cutting patterns from this session——decomposition approaches、architectural trade-offs、scope decisions that could inform future designs。User confirms what enters the document。
- Suggest user invoke `/code-forge` when ready begin coding against approved blueprint。
