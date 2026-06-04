---
name: code-forge
description: "从批准的设计 blueprint 或口头需求生成实现代码。Compose context anchoring、architecture、clean code、DDD、security 和 test quality 为 inside-out implementation workflow。在从设计到代码时使用，实现 approved contracts，或当用户说'implement'、'code this'、'build it'、'forge the code'或'generate the code'时使用。"
---

# Code Forge（代码锻造）

## 所需 Skills

读取并应用：

1. `framework:knowledge-priming` —— 加载 proj context（stack、arch、conventions）使 impl 匹配真实 proj（始终）
2. `framework:context-anchoring` —— Load/find context anchor doc；enrich 作为 impl decisions made（始终）
3. `framework:learning-harvest` —— 加载 prior operational learnings inform impl；在 session 结束时 harvest new patterns（始终）
4. `framework:collaborative-judgment` —— Surface real judgment calls w/ structured opts vs silent assume（始终）
5. `framework:architecture` —— Layer place、dep direction、struct valid（始终）
6. `framework:clean-code` —— Craft rails：SRP、naming、complexity、err handle（始终）
7. `framework:domain-driven-design` —— Aggregates、entities、VOs、domain svcs（条件性：仅当 touch domain folder 时）
8. `framework:secure-coding` —— Trust bounds、injection prevent、secrets mgmt（条件性：仅 boundary-cross code 时）
9. `framework:test-quality` —— AAA struct、isolation、assert quality、naming（write tests 时始终）

## 工作流（Workflow）

### 第 1 步：建立实现上下文（Establish Implementation Context）

使用 `framework:learning-harvest` Load Behavior。Focus hint："implementation session — focus: implementation craft、quality signals、reliability"。Prior learnings 关于 coding patterns、recurring quality issues 和 failure modes inform implementation 从一开始——例如，learnings 说"anemic domain models keep appearing"，push behavior into entities。Learnings flag"missing input validation on VOs"，validate in constructors 从一开始。

使用 `framework:context-anchoring` Doc Discovery check existing context anchor doc for feature impl。

- **如果找到** → Load（context-anchor Load Behavior）。呈现 struct ack——feature name、decision count、open Qs、constraints。Honor all logged decisions/constraints as active commits。
- **如果未找到** → Nudge user："Have design doc/blueprint for feature？Or work from discussed？"Accept either graceful。
  - User provides doc → load、follow。
  - Proceed without → all atom rails still apply；just no struct blueprint ref。Work from verbal reqs in convo。

### 第 2 步：Plan Implementation Order（计划实现顺序）

**With blueprint**：Extract component list、layer assigns from context anchor doc。使用 L2（Components）decisions for layer place，L3（Interactions）for dep flow。

**Without blueprint**：Classify req components→arch layers using layer defs from `framework:architecture`。每个 component，确定：

- Primary responsibility？（biz rules、data access、coord、external I/O）
- Which layer in loaded arch doc matches responsibility？
- Dep constraints for that layer？

如果 `framework:architecture` no loaded layer defs（neither defaults nor custom doc resolved），warn："No arch rules avail。Run `/architecture-refiner` define arch standards。Proceed w/o arch guidance。"Continue w/ only remaining atom rails。

Present proposed layer assigns→user for approval before proceed。

Both cases，plan **inside-out impl order** following dep direction from loaded arch doc——start innermost layer（no outward deps），work outward。Each layer's deps should exist when built。

Classify each op per flow patterns in loaded arch doc（e.g., cmd vs query flows，或 equiv distinction your arch style）。

Present impl plan——ordered component list、layer assigns、flow classifs——confirm w/ user before write code。

After plan approved，ask user choose **review mode**：

> "How review impl？"
> 1. **Layer-by-layer**（rec）-- Impl each layer fully，pause for review before next。One review pt/layer。
> 2. **Full autonomy** -- Impl everything end-to-end，present complete result。One review pt at end。（If blueprint exists，still pause any deviation from approved design。）
> 3. **Component-by-component** -- Pause after each individual component for feedback。Max review pts。

Default **layer-by-layer** if user no preference。

### 第 3 步：Implement Per Component（每个组件实现）

Each component in planned order，gen **code+tests together**——tests not afterthought。

Every component：

- **Place correct arch layer** per `framework:architecture`。Valid dep direction follows loaded arch rules。
- **Apply `framework:clean-code` self-valid** during gen。Run inline checks：SRP comply、meaningful naming、low cyclomatic complexity、proper err handle、no magic vals、clean func sigs、no dead code、appropriate abstract level、clear control flow、minimal comments（code self-doc）。
- **Write tests** using `framework:test-quality` self-valid。

Conditional checks per component：

- **If domain layer** → Apply `framework:domain-driven-design` self-valid。
- **If trust boundary**（HTTP handler、external API call、user input process、file I/O）→ Apply `framework:secure-coding` self-valid。
- **If blueprint exists** → Verify component fulfills L4（Contracts）spec。Flag any deviation from agreed contract。

**Post-Gen Verification**（applies every component，all review modes）：

After gen each component，before present→user：

1. Run **Self-Valid Checklist** from each applicable atom against every func/class this component。Atoms use imperative STOP-verify lang——follow literally。
2. Run **Active Anti-Pattern Scan** from each applicable atom。Check every box scan list。
3. Violations found → fix before present。Don't present code you know violates atom checklist。
4. Judgment calls flagged（see each atom's Ambiguity Signals）→ collect。Present using `framework:collaborative-judgment` protocol before show code。Don't silent resolve。
5. All checks pass，no flagged judgment calls → present w/ brief comply note（e.g., "All clean-code、DDD checks pass"）。Keep one line when clean——only verbose when report violations，fixes。

**Pacing -- follow user's chosen review mode**：

- **Layer-by-layer**：Impl all components within layer，present full layer（code+tests）for review before next layer。
- **Full autonomy**：Impl all layers continuous。Present complete impl（all code+tests）at end。Skip→Step 4（Cross-Component Verif）after all components done。
- **Component-by-component**：Present each component w/ tests individually。Wait approval before next。
- **Exception（all modes）**：Component needs significant deviation from plan（new dep、changed contract、unexpected complexity），pause immediately，discuss before continue——regardless chosen review mode。

### 第 4 步：Cross-Component Verification（跨组件验证）

Step checks **arch coherence**——not code quality（verified per-component Step 3）。After all components impl：

- **With blueprint**：Verify interaction flows match L3（Interactions）design。Every designed interaction traceable in code。
- **Dep direction**：Apply `framework:architecture` verif across all components——verify inter-component dep direction follows loaded arch rules。No layer import from layer not permitted depend。
- **Zero Impl Rule**：Check no new components、interactions、contracts intro beyond planned Step 2。Something added，flag——may be necessary，but should be conscious decision，not scope creep。
- **Final security scan**：Apply `framework:secure-coding` across component boundaries。Check data flowing between components crosses trust bounds safely。
- **Learnings check**：If operational learnings loaded Step 1，verify previously-flagged patterns not recur this impl。Past insight said"anemic domain models keep appearing"——check entities this impl have behavior。

### 第 5 步：Enrich Context（丰富上下文）

Throughout Steps 3-4，use `framework:context-anchoring` Enrich behavior keep living doc current：

- **Add key files** as created——path、purpose、layer assign。
- **Capture impl decisions**——lib choices、pattern selects、deviations from blueprint、tradeoffs made。
- **Resolve open Qs**——Qs from design phase answered during impl，log resolution。
- **If no context doc exists**，significant impl decisions made → suggest create。Decisions worth preserve future sessions。

Use `framework:learning-harvest` Harvest Behavior。Session context："implementation session — code generation from design contracts"。Synthesize 和 propose cross-cutting patterns from this session——implementation gotchas、design-to-reality gaps、library/framework lessons that could inform future implementations。User confirms what enters the document。

After enrich context doc，recommend review：

> "Impl complete。Recommend run `/review` on gen code before consider feature done——provides independent quality assess against same atom standards，catches issues generator may blind to，captures learnings future sessions。"
