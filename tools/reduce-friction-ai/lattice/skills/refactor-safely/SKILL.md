---
name: refactor-safely
description: "安全地重组现有代码而不改变外部可观察行为。Compose context、design、architecture、code quality 和 testing guardrails 为 characterization-first refactoring workflow。当用户说'refactor this'、'clean this up'、'untangle this module'、'move this to the right layer'、'simplify this code'或'improve this structure'时使用。"
---

# Refactor Safely（安全重构）

## 所需 Skills

基于 refactor scope 加载这些 skills（参见 Steps 3、5、6 中的条件使用）：

1. `framework:knowledge-priming` —— 加载项目上下文（技术栈、架构、约定）使 refactor 匹配真实项目而非通用模式（始终）
2. `framework:context-anchoring` —— 在可用时加载现有 feature context doc，capture approved refactor plan、preservation rules、structural decisions 用于 future sessions（始终）
3. `framework:learning-harvest` —— 加载 prior operational learnings inform refactor；在 session 结束时 harvest new patterns（始终）
4. `framework:collaborative-judgment` —— Surface meaningful trade-offs in structure、seams、migration sequence 而非静默选择 path（始终）
5. `framework:clean-code` —— Improve readability、responsibility boundaries、local code craft 同时防止 scope creep 和 wrong abstractions（始终）
6. `framework:test-quality` —— Lock current behavior with characterization tests、keep safety net reliable throughout refactor（始终）
7. `framework:design-first` —— 对重大结构更改选择性使用 progressive design 以便在编辑代码之前同意 target structure（条件性）
8. `framework:architecture` —— Validate layer placement、dependency direction、correct structural boundaries（条件性）
9. `framework:domain-driven-design` —— Validate domain behavior、aggregate boundaries、movement of business rules into correct domain objects（条件性）
10. `framework:secure-coding` —— Preserve validation、authorization、trust-boundary protections、safe data handling 当 refactor touches security-sensitive code 时（条件性）

## 工作流（Workflow）

### 第 1 步：建立重构上下文（Establish Refactor Context）

从**当前痛苦**开始，而非首选 abstraction。

- Identify target area：module、service、aggregate、endpoint path、subsystem
- Clarify **why** refactor needed：mixed responsibilities、duplication、wrong-layer logic、coupling、poor testability、unreadable control flow
- Clarify what user expects to improve：simpler structure、correct layer placement、smaller units、clearer domain behavior、easier testing、safer extension points
- Use `framework:learning-harvest` Load Behavior。Focus hint："refactoring session — focus：structural health、quality signals"。Prior learnings 关于 debt patterns、recurring structural issues 和 coupling problems inform which structural mistakes to prioritize correcting。
- Use `framework:context-anchoring` Document Discovery 检查 affected feature/module 的现有 context doc
  - **如果找到** → Load it（context-anchoring Load Behavior）。Honor existing decisions and constraints as active commitments while planning refactor
  - **如果未找到** → Proceed from conversation and current code。Don't block planning on missing context

End step，summarize intent one sentence：

> "Refactor X to improve Y while preserving Z。"

If can't state improvement target and preservation target that clearly yet，continue clarifying before planning changes。

**Optional persistence check**：

- If refactor substantial、risky，或 likely span multiple sessions，ask whether user wants persist approved plan
- If relevant context doc already exists and user wants persistence → load and update it
- If no relevant doc exists and user wants persistence → propose creating one，confirm doc name per `framework:context-anchoring`，then use as source of truth for approved plan
- If user doesn't want persistence 或 refactor small and local → continue in non-persistent mode。Approval gates still apply；plan simply remains in-session

### 第 2 步：Define Preservation Boundaries（定义保留边界）

Refactoring changes structure，**not behavior**。在提议 structural edits 之前使 preservation contract explicit。

List behaviors that must remain unchanged：

- Public API contracts and response shapes
- Domain invariants and state transitions
- Persistence semantics and side effects
- Event emission and integration behavior
- Authorization and security posture
- Error behavior where externally visible
- Performance or operational characteristics if part of current contract

Also list explicit **out-of-scope changes**：

- New features
- Schema changes
- Contract changes
- Intentional behavior changes
- Unrelated cleanup outside approved area

This step defines refactor's safety boundary。If desired outcome requires changing preserved behavior，stop and discuss whether task actually bug fix、feature，或 broader redesign。

### 第 3 步：Propose High-Level Structural Plan（提议高层结构计划）

**Zero Refactor Rule**：在用户批准 target structure 和 transition plan 之前无 structural code changes。

对于 small refactors，plan may be brief。For larger ones，use `framework:design-first` selectively：

- Start at **Level 2（Components）** to define target responsibilities and boundaries
- Use **Level 3（Interactions）** when data flow or dependency direction will change
- Use **Level 4（Contracts）** when internal interfaces or seams need formalized
- Don't use Level 1（Capabilities）unless user-facing scope actually changing

Present：

- **Current structural problems** -- what wrong with current shape
- **Target structure** -- what components、classes、functions should exist after refactor
- **Movement plan** -- what logic moves where
- **Preservation boundaries** -- what will stay behaviorally unchanged
- **Out-of-scope items** -- what will not be changed this pass

End step with explicit gate：

> "Does this refactor plan look correct？Should I proceed to Step 4：characterization tests？"

Don't write refactor code until user explicitly approves。

If persistence enabled，use `framework:context-anchoring` Enrich behavior to capture approved preservation boundaries、target structure、movement plan、out-of-scope items。Don't proceed to Step 4 until plan written。

### 第 4 步：Add Characterization Protection First（首先添加特征化保护）

在更改结构之前，用 tests lock current behavior。

- Identify existing tests that already protect preserved behavior
- Strengthen weak tests if too implementation-coupled or too vague to serve as guardrails
- Add **characterization tests** for important behaviors currently implicit
- Prefer **lowest-level test** that faithfully captures preserved behavior without missing important integration effects
- Characterization tests must describe **current observable behavior**，not intended refactored shape
- Apply `framework:test-quality` inline

**Stopping rule**：

- If important preserved behavior not protected by tests，pause and make that gap explicit before refactoring
- Don't start structural edits without believable safety net unless user explicitly accepts risk
- Green characterization tests are baseline for refactor；if red before first structural change，resolve that first or re-scope task

This step workflow's differentiator：refactor not considered safe until current behavior executable and guarded。

End step with explicit gate：

> "Characterization tests in place and passing。Ready to discuss refactor strategy and pacing？"

Don't proceed to strategy selection until safety net verified green。

### 第 5 步：Choose Refactor Strategy and Pacing（选择重构策略和节奏）

在用户批准高层 plan 和 safety net in place 之后，choose implementation approach。

Preferred strategies：

- **Extract and redirect** -- extract focused units、route callers gradually
- **Introduce seam，then migrate** -- add interface or boundary、then move behavior behind it
- **Move behavior inward** -- shift business rules from outer layers into appropriate inner layer per `framework:architecture`
- **Split and collapse** -- separate unrelated responsibilities、then remove old mixed path

Preferred pacing：

> "How would you like review refactor？"
> 1. **Slice-by-slice**（recommended）-- Refactor one safe slice at time、pause after each slice。Best for risky legacy code。
> 2. **Layer-by-layer** -- Complete refactor for one structural layer or concern、then pause for review。Best for broader architectural cleanup。
> 3. **Full autonomy** -- Execute approved refactor end-to-end、present complete result at end。Best for tightly scoped、low-risk refactors。（Still pause if slice reveals approved plan unsafe or invalid — see Step 6 Deviation Rule。）

Default to **slice-by-slice** if user doesn't express preference。

### 第 6 步：Refactor in Small Green Steps（在小绿步骤中重构）

仅在 approved preservation boundaries 和 target structure 内实现。

For each slice：

1. Make one structural improvement from approved plan
2. Re-run relevant characterization tests
   - If any characterization test goes red，**stop immediately**。Don't proceed to next slice。Fix regression or revert slice before continuing。
3. Apply applicable atom self-validation checklists
4. Run applicable anti-pattern scans
5. Fix violations before presenting slice
6. Collect judgment calls for slice using `framework:collaborative-judgment`，surface them before presenting slice's code。Don't interrupt mid-slice unless approved plan becomes unsafe or invalid。

Always apply：

- `framework:clean-code` -- better boundaries、simpler control flow、smaller focused units、clearer naming
- `framework:test-quality` -- maintain strong characterization tests and nearby supporting tests

Conditionally apply：

- **If responsibilities move across layers or dependency direction changes** → Apply `framework:architecture`
- **If business rules、aggregates、value objects，或 domain behavior move or sharpen** → Apply `framework:domain-driven-design`
- **If trust boundaries、authz、validation、queries，或 sensitive data handling touched** → Apply `framework:secure-coding`

**Deviation rule**：

- If implementation reveals approved refactor plan incomplete、unsafe，或 would require changing preserved behavior，pause immediately and discuss before continuing

### 第 7 步：Verify Preservation and Structural Improvement（验证保留和结构改进）

Refactor succeeds only if **both** true：

1. **Behavior preserved**
2. **Structure measurably better**

Verify preservation：

- Characterization tests still pass
- No intended outward behavior changed
- Preserved contracts remain intact
- Security posture not weakened

Verify structural improvement：

- Responsibilities clearer
- Dependency direction improved or at least no worse
- Duplication or entanglement reduced
- Testability and readability improved
- Old paths or temporary scaffolding removed when migration complete

When reporting completion，be explicit about both：

- What behavior preserved and how verified
- What structural improvement achieved
- What intentionally deferred for later refactor

### 第 8 步：Capture Decisions and Remaining Debt（捕获决策和剩余债务）

Use `framework:context-anchoring` Enrich behavior to preserve important parts of refactor：

- Refactor scope：what area changed
- Preservation boundaries：what explicitly kept stable
- Target structure：what shape approved
- Strategy chosen：why this migration path selected over alternatives
- Key files changed：path and purpose
- Deferred debt：what remains and why intentionally left for later

If no context doc exists and refactor involved non-trivial structural reasoning，suggest creating one so decisions not lost across sessions。

Use `framework:learning-harvest` Harvest Behavior。Session context："refactoring session — structural restructuring and debt resolution"。Synthesize 和 propose cross-cutting patterns from this session——structural debt that accumulated、migration strategies that worked、characterization test gaps discovered。User confirms what enters the document。

After refactor complete，recommend `/review` when change：

- touches multiple layers
- changes domain boundaries
- changes security-sensitive code
- leaves temporary migration scaffolding
- large enough that independent quality pass would add confidence

`/review` provides independent pass on refactor，can capture broader structural learnings for future work。
