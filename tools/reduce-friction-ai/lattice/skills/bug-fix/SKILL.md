---
name: bug-fix
description: "调查、复现并安全修复带有回归保护的 bug。组合 context（上下文）、diagnosis（诊断）、architecture（架构）、code quality（代码质量）和 testing guardrails（测试护栏）为 reproduce-first（复现优先）的修复工作流。当用户说'fix this bug'、'debug this'、'investigate this failure'、'patch this regression'、'repair this issue'或'why is this broken'时使用。"
---

# Bug Fix（缺陷修复）

## 所需 Skills

基于 bug scope（缺陷范围）加载这些 skills（参见第 2 步和第 5 步中的何时使用）：

1. `framework:knowledge-priming` —— 加载项目上下文（技术栈、架构、约定）使 fix 匹配真实项目（始终）
2. `framework:context-anchoring` —— 在可用时加载现有 feature context doc（功能上下文文档），capture root cause + repair decisions（捕获根因和修复决策）用于 future（未来）（始终）
3. `framework:learning-harvest` —— 加载 prior operational learnings（先前的操作学习）inform diagnosis（告知诊断）；在 session 结束时 harvest new patterns（收获新模式）（始终）
4. `framework:collaborative-judgment` —— Surface meaningful repair trade-offs（呈现有意义的修复权衡）而非静默 patch choice（补丁选择）（始终）
5. `framework:clean-code` —— Keep fix focused（保持修复聚焦）、readable（可读）、minimal scope（最小范围）（始终）
6. `framework:test-quality` —— Create + validate failing regression test（创建并验证失败的回归测试）that proves bug exists + fix works（证明 bug 存在且修复有效）（始终）
7. `framework:architecture` —— Validate layer placement（验证层放置）、dependency direction（依赖方向）、correct repair location（正确的修复位置）（条件性）
8. `framework:domain-driven-design` —— Validate invariants（验证不变量）、aggregate boundaries（聚合边界）、domain behavior（领域行为）当 bug involves domain logic（缺陷涉及领域逻辑）时（条件性）
9. `framework:secure-coding` —— Validate trust boundaries（验证信任边界）、input handling（输入处理）、authorization（授权）、injection safety（注入安全性）当 bug touches security-sensitive code（缺陷触及安全敏感代码）时（条件性）

## 工作流（Workflow）

### 第 1 步：建立 Bug 上下文（Establish Bug Context）

从 failure 开始，而非 proposed fix。

- Gather **observed behavior**（观察到的行为）、**expected behavior**（期望的行为）、**reproduction path**（复现路径），任何 evidence（证据）：failing test（失败的测试）、error message（错误消息）、stack trace（堆栈跟踪）、log excerpt（日志摘录）、request payload（请求载荷）、recent change（最近的更改）。
- Use `framework:learning-harvest` Load Behavior（加载行为）。Focus hint（焦点提示）："bug investigation — focus：reliability、quality signals"。Prior learnings（先前的学习）关于 failure modes（故障模式）、fragile areas（脆弱区域）和 recurring defect classes（反复出现的缺陷类别）inform diagnosis（告知诊断）从一开始。
- Use `framework:context-anchoring` Document Discovery（文档发现）检查 affected feature/module（受影响的功能/模块）的现有 context doc（上下文文档）。
  - **如果找到** → Load it（加载它）（context-anchoring Load Behavior）。Honor logged decisions + constraints as active commitments while diagnosing（尊重已记录的决策和约束作为诊断期间的活跃承诺）。
  - **如果未找到** → Proceed from bug report + current code（从缺陷报告 + 当前代码继续）。Don't block diagnosis on missing context（不要因为缺少上下文而阻塞诊断）。

End this step，summarize bug one sentence：

> "Observed X（观察到 X）、expected Y（期望 Y）、reproducible via Z（可通过 Z 复现）。"

If can't state bug clearly yet（如果还无法清晰描述缺陷），keep gathering evidence before proposing code changes（在提出代码更改之前继续收集证据）。

### 第 2 步：复现和定位（Reproduce and Localize）

**Primary discipline**：不要呈现你尚未复现的 bug 的 fix。

Reproduce failure using strongest evidence available，this order：

1. **Existing failing automated test**（现有的失败自动化测试）-- best case（最佳情况）；use as regression guard（用作回归保护）
2. **New failing automated test**（新的失败自动化测试）-- preferred when no test exists yet（当尚无测试时的首选方案）
3. **Executable reproduction path**（可执行的复现路径）-- command（命令）、request sequence（请求序列）、deterministic manual flow（确定性手动流程）when automation not yet possible（当自动化尚不可行时）

Localize issue before editing：

- **Which layer likely source？**（哪一层可能是根源？）Use layer definitions from `framework:architecture`（使用 framework:architecture 的层定义）to identify which architectural layer defect originates in（以识别缺陷源自哪个架构层）
- **Production bug or test bug？**（生产环境缺陷还是测试缺陷？）Sometimes code correct、test/fixture wrong（有时代码正确，测试/夹具错误）
- **Failure symptom or root cause？**（故障症状还是根因？）Crashing line often downstream of real defect（崩溃行通常在真实缺陷的下游）
- **Bug cross trust boundary？**（缺陷是否跨越信任边界？）If yes、plan load `framework:secure-coding`（如果是，计划加载 framework:secure-coding）
- **Involve domain invariants or aggregate behavior？**（是否涉及领域不变量或聚合行为？）If yes、plan load `framework:domain-driven-design`（如果是，计划加载 framework:domain-driven-design）
- **Likely fix touch multiple layers or dependency flow？**（修复可能触及多个层或依赖流吗？）If yes、plan load `framework:architecture`（如果是，计划加载 framework:architecture）

If multiple plausible root causes remain（如果仍有多个合理的根因），use `framework:collaborative-judgment` to present leading hypotheses + what evidence would distinguish（使用 framework:collaborative-judgment 呈现主要假设 + 什么证据可以区分）。Don't guess and patch speculatively（不要猜测并推测性地打补丁）。

Before writing regression test（在编写回归测试之前），state root cause hypothesis explicitly（明确陈述根因假设），use `framework:collaborative-judgment` to surface（使用 framework:collaborative-judgment 呈现）：

> "Bug caused by [X]（缺陷由 X 引起）。When [C holds]（当 C 成立时）、correct outcome should be [P]（正确结果应为 P）。
> Confirm this by writing test that red before fix、green after（通过编写修复前为红色、修复后为绿色的测试来确认）。"

If user identifies flaw in hypothesis（如果用户识别出假设中的缺陷），revise before writing tests（在编写测试之前修订）。

End step with explicit bug contract：

> **C（bug condition/缺陷条件）：** [exact input/state triggering bug（触发缺陷的确切输入/状态）]
> **P（fix postcondition/修复后置条件）：** [what correct behavior looks like when C holds（当 C 成立时正确行为的样子）]
> **Preserved（保留）：** [what must remain identical for all inputs outside C（对于 C 之外的所有输入必须保持相同的内容）]

If can't state all three（如果无法陈述全部三项），keep localizing before writing tests（在编写测试之前继续定位）。

**Optional persistence check**：Now that bug reproduced + localized，decide whether persist investigation：

- If investigation complex（如果调查复杂）、involves multiple hypotheses（涉及多个假设）、likely span multiple sessions（可能跨越多个 session），ask if user wants persist diagnosis + repair decisions（询问用户是否希望持久化诊断和修复决策）
- If relevant context doc exists（如果存在相关上下文文档）→ plan enrich in Step 7（计划在第 7 步中丰富）
- If none exists + user wants persistence（如果不存在且用户希望持久化）→ propose creating one、confirm doc name per `framework:context-anchoring`（提议创建一个，根据 framework:context-anchoring 确认文档名称）、use as source of truth（用作权威来源）
- If user doesn't want persistence 或 bug narrow + local（如果用户不希望持久化或缺陷范围狭窄且局部）→ continue non-persistent mode（继续非持久模式）。Repair workflow still applies；decisions remain in-session（修复工作流仍然适用；决策保留在 session 内）

### 第 3 步：首先添加回归保护（Add Regression Protection First）

**Phase A — Bug-Condition Tests（必须从 RED 开始）**

- Write smallest failing test that fires when C holds（编写当 C 成立时触发的最小失败测试）
- Prefer lowest-level test reproducing real failure without losing signal（优先选择最低级别的测试，复现真实故障而不丢失信号）
- Name test for broken behavior，not implementation detail（为损坏行为命名测试，而非实现细节）
- Assert correct expected outcome（postcondition P/后置条件 P），not just absence of failure（断言正确的期望结果，而不仅仅是没有失败）
- Apply `framework:test-quality` inline
- Run against unfixed code，confirm RED（在未修复的代码上运行，确认 RED/红色）
  - If green before fix、bug condition hypothesis wrong — stop、re-localize（如果修复前为绿色，缺陷条件假设错误——停止、重新定位）

**Stopping rule**：

- If can't create stable failing automated test，pause、explain why before making code changes
- Record closest executable reproduction you have（记录你拥有的最接近的可执行复现）
- Don't present speculative fix as "complete" without automated reproducer unless user explicitly accepts limitation（除非用户明确接受限制，否则不要在没有自动化复现器的情况下将推测性修复呈现为"完成"）
- If bug can't be tested directly due to tight coupling/deep integration（如果由于紧密耦合/深度集成而无法直接测试缺陷），introduce minimum structural seam needed to make testable（引入使可测试所需的最小结构接缝）（method extraction/方法提取、parameter injection/参数注入、interface boundary/接口边界）。Not refactor — prerequisite for regression protection（不是重构——回归保护的前提条件）。Apply `framework:clean-code` inline、keep seam minimal（内联应用 framework:clean-code，保持接缝最小）。

**Phase B — Preservation Baseline（必须保持 GREEN）**

- Identify existing tests covering behavior outside C（识别覆盖 C 之外行为的现有测试）
- If important adjacent behavior has no test coverage，add at most 2-3 targeted characterization tests（如果重要的相邻行为没有测试覆盖，最多添加 2-3 个有针对性的特征化测试）
- Confirm all preservation baseline tests green before applying fix（在应用修复之前确认所有保留基线测试为绿色）
- These tests must remain green through every change in Step 5 — any flip to red means fix has side effects；stop、narrow scope（这些测试必须在第 5 步的每次更改中保持绿色——任何翻转为红色意味着修复有副作用；停止、缩小范围）

### 第 4 步：选择最小安全修复（Choose the Minimal Safe Fix）

Separate **repair strategy** from code change itself。

Before editing，decide：

- What **root cause**（什么是**根因**）？
- What **smallest safe change**（什么是最小的安全更改）correcting it（纠正它）？
- What layer **right repair location**（哪一层是**正确的修复位置**）？
- Does issue require **local patch**（是否需要**本地补丁**）or **small structural correction**（或小范围结构修正）？

Default to smallest safe fix restoring correct behavior **without architectural backsliding**。

Guardrails：

- Apply `framework:architecture` layering rules when choosing repair location — don't patch in outer layer when rule belongs inward（在选择修复位置时应用 framework:architecture 分层规则——当规则属于内部时不要在外层打补丁）
- Don't widen task into unrelated cleanup（不要将任务扩大到无关的清理工作）
- Don't delete/weaken failing test just to make suite green（不要仅仅为了使测试套件通过而删除/削弱失败的测试）
- If real fix requires contract/design change beyond narrow repair，stop、discuss scope explicitly（如果真实修复需要超出狭窄修复范围的契约/设计更改，停止、明确讨论范围）
- Don't add guard clauses、null checks、defensive handling for inputs outside C — code path for correct inputs must be byte-for-byte identical before + after fix（不要为 C 之外的输入添加守卫子句、空值检查、防御性处理——正确输入的代码路径在修复前后必须逐字节相同）。

If multiple valid repair strategies with meaningful trade-offs（如果有多个具有有意义权衡的有效修复策略），present using `framework:collaborative-judgment` before proceeding（在继续之前使用 framework:collaborative-judgment 呈现）。

### 第 5 步：实现修复（Implement the Fix）

Always apply：

- `framework:clean-code` -- keep delta focused（保持 delta 聚焦）、readable（可读）、easy to reason about（易于推理）
- `framework:test-quality` -- maintain regression test（维护回归测试）+ any nearby supporting tests（以及任何附近的辅助测试）

Conditionally apply based on localized root cause：

- **If fix changes layer responsibilities、dependency direction、architectural flow**（如果修复更改层职责、依赖方向、架构流）→ Apply `framework:architecture`
- **If fix changes domain behavior、invariants、aggregate boundaries、value objects**（如果修复更改领域行为、不变量、聚合边界、值对象）→ Apply `framework:domain-driven-design`
- **If fix touches input validation、authorization、queries、external boundaries、sensitive data**（如果修复触及输入验证、授权、查询、外部边界、敏感数据）→ Apply `framework:secure-coding`

After implementing fix（实现修复后），before presenting（在呈现之前）：

1. Re-run regression test、confirm now green
2. Run applicable atom self-validation checklists against changed code
3. Run applicable anti-pattern scans
4. Fix any violations before presenting result

### 第 6 步：验证非回归（Verify Non-Regression）

Verify repair three levels：

1. **Fix proof** -- regression test that was red before fix now green。Asserts correct outcome，not just absence of original failure。
2. **Preservation proof** -- tests covering behavior adjacent to bug still pass。If preservation baseline tests added in Step 3，must remain green。Any flip from green to red means fix has side effects — stop、narrow scope before continuing。
3. **Structural confidence** -- fix didn't introduce wrong-layer workaround、dependency violation、weakened security posture

When reporting completion（报告完成时），explicit about verification scope（明确验证范围）：

- What was re-run
- What now passes
- What not verified + why

If fix narrow + confidence high，say so briefly。If verification partial，say so clearly。

### 第 7 步：捕获根因并闭环（Capture Root Cause and Close the Loop）

Use `framework:context-anchoring` Enrich behavior to preserve important parts of repair：

- Bug summary（缺陷摘要）：observed vs expected behavior（观察到的行为与期望的行为）
- Root cause（根因）：what actually failed + where（什么实际失败 + 在哪里）
- Repair decision（修复决策）：why this fix chosen over alternatives（为什么选择此修复而非其他方案）
- Protection added（添加的保护）：regression test or executable reproducer now guarding behavior（回归测试或可执行复现器现在保护行为）
- Key files changed（更改的关键文件）：path + purpose（路径 + 用途）

If no context doc exists + fix exposed non-trivial design/domain lesson，suggest creating one。

Use `framework:learning-harvest` Harvest Behavior（收获行为）。Session context（会话上下文）："bug investigation — root cause diagnosis and repair"。Synthesize 和 propose cross-cutting patterns from this session（综合并提出本次 session 中的跨领域模式）——root cause categories（根因类别）、failure modes likely to recur elsewhere（可能在其他地方反复出现的故障模式）、boundary condition gaps（边界条件差距）。User confirms what enters the document（用户确认哪些内容进入文档）。

After fix complete（修复完成后），recommend `/review` when change（当更改时推荐 /review）：

- touches multiple layers
- changes security-sensitive code
- changes domain behavior
- introduces non-trivial structural correction

`/review` provides independent pass on repair（/review 提供对修复的独立检查），can capture broader learnings for future work（可以捕获更广泛的学习结果以供未来工作）。
