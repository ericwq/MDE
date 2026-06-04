---
name: learning-harvest
description: "管理 operational learnings 生命周期——加载 prior learnings 以 inform current work、harvest new patterns worth preserving，并保持文档随时间推移保持精简。提供从实践中积累可操作模式的协议，补充标准和默认值。当 workflow session 完成并产生值得持久化的洞察、开始应该从 prior patterns 受益的 session，或当用户说'harvest learnings'、'what have we learned'、'capture this pattern'、'tighten learnings'或'operational learnings'时使用。"
---

# Learning Harvest（学习收获）

## 目的（Purpose）

Standards define how a team intends to work。Operational learnings capture what the team discovers while doing the work——experiential knowledge that only emerges from practice。

This atom helps the user curate a single living document of cross-cutting project patterns。The AI synthesizes and proposes; the user decides what enters。Every entry is user-confirmed。Not a log。Not a report。Not auto-generated。A user-curated collection of experiential patterns。

## 范围边界（Scope Boundary）

Operational learnings are NOT rules。They are what you learn while applying rules。

| Standards (refiner output, atom defaults) | Operational Learnings (this document) |
|---|---|
| "Domain layer must not import from infrastructure" | "When adding a new aggregate, we keep forgetting to define the repository interface first — design interface before implementation" |
| "Functions should have single responsibility" | "Service classes that start small grow past 500 lines within 3 features — split by command type proactively at ~200 lines" |
| "Value objects must validate in constructor" | "Date range VOs without explicit inclusive/exclusive documentation cause boundary bugs every time — document semantics alongside validation" |

**The standard is the rule。The operational learning is what we discovered while applying the rule on this project。**

If an entry reads like a rule that should always be followed, it belongs in a standards document (run the relevant refiner)。If it reads like "here's what we keep learning the hard way" or "here's an approach that keeps working for us" — it belongs here。

Patterns that recur frequently may graduate to standards via a refiner。That promotion path is part of the Tighten behavior。

## 配置解析（Config Resolution）

1. Check `.lattice/config.yaml` for `paths.operational_learnings`
2. If found, use that file path
3. If not, use default `.lattice/learnings/operational-learnings.md`

**Backward compatibility**: If default path not found but `.lattice/learnings/review-insights.md` exists, suggest migration to the new unified format。If user declines, read legacy file as flat input but do not write to it。

## 文档结构（Document Structure）

```markdown
# Operational Learnings

Experiential patterns from practice。Complements standards (what should be) with experience (what we keep learning)。

## Design Patterns
<!-- Decomposition, architecture choices, scope decisions that proved good or bad -->

## Implementation Craft
<!-- Coding approaches, library gotchas, design-to-reality gaps -->

## Quality Signals
<!-- Recurring quality issues that keep appearing despite rules -->

## Reliability
<!-- Bug root causes, failure modes, fragile areas, boundary condition gaps -->

## Structural Health
<!-- Architectural drift, debt accumulation, coupling issues, migration lessons -->
```

**Entry format**: `- YYYY-MM-DD [context] Pattern — actionable takeaway`

- `context`: type of session (e.g.、"design"、"implementation"、"review"、"bug fix"、"refactoring")。Not a feature name——learnings are cross-cutting。
- Each entry ONE bullet, max 2 lines, scannable in under 10 seconds。

## Load Behavior（加载行为）

Invoked at session start。Composing workflow passes a **focus hint** (relevant categories)。

1. Resolve file path per Config Resolution。
2. If file not found——"No operational learnings yet。" Continue。Non-blocking。
3. If found——surface relevant entries (3-5 most recent from matching categories) as brief context。Treat as soft guidance, not hard constraints。

## Harvest Behavior（收获行为）

Invoked at session end。Composing workflow passes a **session context** (what kind of work happened)。

**Governing principle:** The atom NEVER writes autonomously。It recommends。The user decides。Prefer omission over speculation。Most sessions will not produce learnings——empty harvest is normal and expected。

**Steps**：

1. **Synthesize。** Review session decisions, trade-offs, outcomes。Identify candidates that would help a *different* future session on a *different* feature。

2. **Filter — hard evidence gate。** Each candidate must pass ALL five。If any fails, drop silently。

   | Filter | Drop if... |
   |--------|------------|
   | **Evidence** | No concrete session event produced this——just prior knowledge |
   | **Cross-cutting** | Only relevant to this feature's specific context |
   | **Actionable** | Requires this conversation's context to understand |
   | **Recurrence** | No structural reason it will happen again |
   | **Confidence** | Below 80%——mere possibility, not grounded insight |

   No candidates pass? Say so and stop。Do NOT force output。Do NOT lower the bar。

3. **Propose to user。** Present filtered candidates with category and wording。Frame as conversation：

   > I noticed these cross-cutting patterns that might help future sessions:
   > 1. [Category] Pattern — takeaway
   >
   > Worth capturing? Accept, edit, add your own, or skip entirely。

4. **User decides。** Accept, edit wording, reject some, add their own, or skip all。Do NOT argue for rejected entries。User judgment is final。

5. **Write confirmed entries only。** Dedup against existing entries (update with recurrence note if same pattern exists)。Create file/dir if needed。

6. **Assess health。** If document growing dense or entries overlap: suggest Tighten。If pattern recurred 4+ times: suggest promoting to standards。User decides——never auto-prune。

## Tighten Behavior（精简行为）

Triggered during Harvest (when bloat detected) or invoked standalone。

1. Read full document。
2. Identify: consolidation opportunities (same pattern, different words), noise (one-off, never recurred), promotion candidates (recurred 4+ times——suggest refiner), stale entries (project has changed)。
3. Present proposals to user。Apply only what user confirms。

## 自我验证清单（Self-Validation Checklist）

Before writing any entry, verify ALL。If any fails, do not write。

1. **User confirmed**——STOP: Explicit user approval for every entry。No exceptions。
2. **Evidence grounded**——STOP: Produced by a specific session event, not prior knowledge。
3. **Experiential, not prescriptive**——STOP: Reads like "what we learned" not "what the rule should be。" If it's a rule, it belongs in standards via a refiner。
4. **Cross-cutting**——STOP: Applies beyond this feature。Feature-specific decisions belong in context anchor doc。
5. **Confident and actionable**——STOP: 80%+ confident, future session can act on it without this conversation's context。
6. **Not redundant**——STOP: Not already in standards, atom defaults, or existing learnings。At most, add recurrence note。
7. **Concise**——STOP: Scannable in 10 seconds。Two lines max。

## 与其他 Skills 的集成（Integration with Other Skills）

Workflows invoke **Load** at session start (with focus hint) and **Harvest** at session end (with session context)。The atom never references specific workflows——it applies the protocol uniformly to whatever composes it。

Relationship to other infrastructure atoms:
- `context-anchoring`——per-feature decisions (specific)。This atom——cross-feature patterns (general)。
- `knowledge-priming`——project identity (static)。This atom——accumulated experience (growing)。
- `collaborative-judgment`——surfaces decisions in-session。This atom——preserves patterns across sessions。
