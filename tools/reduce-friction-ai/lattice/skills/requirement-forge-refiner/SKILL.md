---
name: requirement-forge-refiner
description: "通过结构化对话为项目定义需求标准——epic 和 feature 定义、场景结构、AC 格式、优先级标记、状态工作流和命名约定。生成正式的 requirement-standards.md，供 requirement-quality atom 通过配置解析读取，为其内置默认值定制团队的流程。在设置新项目、定义产品标准时使用，或当用户说'set up requirement standards'、'define feature standards'、'configure requirement forge'、'define how features should be structured'或'requirement forge refiner'时使用。"
---

# Requirement Forge Refiner（需求定义器）

## 产出内容

- **输出**：`.lattice/standards/requirement-standards.md`（或来自 `.lattice/config.yaml` → `paths.requirement_standards` 的自定义路径）
- **两种模式**：
  - **叠加（Overlay）**（`mode: overlay`）：仅包含与内置默认值不同部分的精简文档。`requirement-quality` atom 首先读取其嵌入的 `defaults.md`，然后将此文档的部分叠加在上面。这是预期的常见情况。
  - **覆盖（Override）**（`mode: override`）：全面独立的文档，完全替换 atom 的内置默认值。适用于产品流程与默认值根本不同的团队。
- **默认模式**：叠加——仅生成团队想要更改的内容
- **配置键**：`.lattice/config.yaml` 中的 `paths.requirement_standards`
- **消费者**：`requirement-quality` atom（通过配置解析）→ `requirement-forge` molecule（compose 该 atom）
- **模板**：读取 `./assets/template.md` 获取完整的文档结构、默认内容和访谈指导注释

## 范围澄清

此 refiner 定义需求如何*结构和表达*用于此项目。它不定义：

- 构建什么（那是 requirement-forge molecule 的工作）
- 架构或技术设计（那是 architecture-refiner 的工作）
- 领域建模模式（那是 ddd-refiner 的工作）

此处产生的标准回答：什么是 epic，什么是 feature，什么是 scenario，ACs 如何编写，features 如何命名和优先级排序。这些是 `requirement-quality` atom 强制执行规则——molecule compose 该 atom 并自动继承这些规则。

## 开始之前

### 检查现有标准文档

1. 读取 `.lattice/config.yaml` —— 检查 `paths.requirement_standards`。
2. 如果路径存在，读取该文件。询问用户：
   - "你已经有需求标准文档了。你想**修订**它（更新特定部分）、**重新开始**（新访谈）还是**补充**它（添加新部分）？"
   - 修订：加载现有文档，仅 walkthrough 用户想要更改的部分，就地更新。
   - 重新开始：继续下面的完整访谈流程。
   - 补充：跳到访谈的"新部分"部分。
3. 如果没有配置或没有现有文档，继续完整访谈。

### 首先问两个定向问题

在正式访谈之前，询问：

1. "你的团队是否已经有编写需求的方式——任何现有的 PRDs、Confluence 模板或 Jira 约定我应该知道的？"
2. "在定义标准之前，是否有任何特定的产品领域或术语我应该知道的？"

这两个问题是结构化访谈开始之前的唯一自由形式倾听。综合你听到的并继续传递——在此阶段不要问后续问题。

## 选择模式

展示三个选项：

"你想如何定义你的需求标准？

1. **自定义特定部分**（overlay）—— 保留内置默认值，仅更改与你的项目不同的部分。这会产生一个精简文档。大多数团队选择这个选项。
2. **从头定义所有内容**（override）—— walkthrough 所有部分并生成全面的独立文档。
3. **仅添加项目特定部分**（overlay with additions）—— 保留所有默认值不变，添加新部分，例如领域术语或自定义状态工作流。

内置默认值很好地覆盖了标准产品规范实践。除非团队的约定根本不同，否则推荐选项 1。"

映射选择：
- 选项 1 和 3 → `mode: overlay`
- 选项 2 → `mode: override`

## 引导方法

### 对话风格

- **一次一个部分**。不要一次性呈现所有问题。按顺序 walkthrough 模板。
- **默认值优先**。对于每个部分，简要总结默认值，然后询问是否匹配。不要逐字读取默认值——总结关键点并询问。
- **提出，而非仅仅询问**。当用户的答案模糊时，提出最合理的解释并询问他们确认或纠正。"听起来你想要 MoSCoW 优先级——所以'Must'、'Should'、'Could'、'Won't'。对吗？"
- **记录决策，而非讨论**。输出文档作为规范阅读。"我们讨论了 X 并决定 Y"是错误的。"Y"是正确的。
- **挑战弱定义**。如果用户如此广泛地定义"feature"以至于它包含整个 epic，推回："那个范围听起来像 epic——一个 feature 应该在一次 design-blueprint 会话中独立设计。我们能收紧定义吗？"

### 对于 overlay 模式

这应该很快。许多部分将是"保持原样"。

1. 用 2-3 句话展示每个部分的默认值。
2. 询问："这匹配你的项目吗，还是你想更改它？"
3. 如果匹配 → 跳过它（该部分**不会**出现在输出中）。
4. 如果需要更改 → 讨论具体内容，记录更改。
5. 所有部分之后，询问："有什么要添加的不被覆盖——领域特定术语、自定义字段、团队约定？"
6. 仅更改或添加的部分出现在输出文档中。

### 对于 override 模式

每个部分都会得到关注并出现在输出中。

1. 完整 walkthrough 每个部分的细节。
2. 用户确认、修改或替换每个部分。
3. 所有部分出现在输出中。

### 常见场景

- **"我同意所有内容"** → 不需要自定义文档。"内置默认值已经激活。不需要自定义文档——requirement-forge 将自动使用默认值。"
- **"我们使用 MoSCoW 优先级"** → Overlay §6 only。
- **"我们称它们为'use cases'而非'scenarios'"** → Overlay §4（重命名 + 更新任何引用"scenario"的描述）。
- **"我们有更长的状态工作流"** → Overlay §7。
- **"我们有应该一致的领域特定术语"** → Overlay with additions——添加§10 领域术语。
- **"我们的 features 倾向于更大"** → Overlay §2 和§4——调整大小信号和最大场景计数。

## 分部分访谈指南

读取 `./assets/template.md`，并按照每个部分的 `<!-- INTERVIEW GUIDANCE: -->` 注释操作。这些注释包含具体问题、追问问题以及可自定义 vs 固定的内容。

### 跨部分依赖表

| 决策于 | 影响 | 如何影响 |
|---|---|---|
| §2 — Feature 大小定义 | §4 场景计数 | 更大的 features 容忍更多 scenarios；更 tight 的 features 需要较低的 cap |
| §4 — Scenario nomenclature | §8 命名约定 | 如果"scenario"被重命名，命名约定必须使用新术语 |
| §4 — 每个 feature 的最大 scenarios | §2 feature 定义 | 这两个必须一致——§2 中的 split 信号应该与§4 中的 cap 对齐 |
| §5 — AC 格式 | §4 场景结构 | AC 格式决定每个 scenario 的 criteria 外观 |
| §6 — 优先级标记 | feature 文件 frontmatter | 每个生成的 feature 文件中使用的 priority 字段格式 |
| §7 — 状态工作流 | feature 文件 frontmatter | 每个生成的 feature 文件中使用的 status 字段 |
| §8 — 命名约定 | all file generation | Molecule 生成的 feature 文件名称和 display names |

当触发依赖时，通知用户："因为你更改了 [X]，我们也应该审查 [Y]——它受那个决策影响。"

### Overlay 特定部分流程

对于 9 个默认部分中的每一个：

1. 用 2-3 句话总结部分的关键点。
2. 询问："这匹配你的项目吗？"
3. **是** → 移动到下一部分。该部分不会出现在输出中。
4. **否** → 使用模板指导深入细节。生成用户版本。
5. 所有 9 个部分之后，询问添加内容。

### Override 特定部分流程

对于 9 个默认部分中的每一个：

1. 展示部分的完整内容。
2. 询问："这可以直接使用，还是你想修改它？"
3. **原样** → 不变地包含默认内容。
4. **修改** → 讨论更改，生成修改版本。
5. 所有部分都进入输出。

## 输出组装

### 对于 overlay 模式

1. YAML frontmatter：`mode: overlay`
2. Overlay 序言（来自模板）
3. 仅列出包含部分的目录
4. 仅用户更改或添加的部分——每个必须自包含且完整
5. 部分标题必须与 `template.md` 完全匹配（molecule 按标题匹配部分）
6. 新部分（§10+）在默认部分之后包含
7. 页脚包含项目名称、日期、模式

### 对于 override 模式

1. YAML frontmatter：`mode: override`
2. Override 序言（来自模板）
3. 完整目录（所有 9+ 部分）
4. 所有部分：未更改的使用默认值，已更改的使用用户版本，新部分在末尾
5. 页脚包含项目名称、日期、模式

### 对于两种模式

从输出中剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释。最终文档是干净的规范。

**确定输出路径：**
1. 如果 `.lattice/config.yaml` 存在且有 `paths.requirement_standards`，使用该路径。
2. 否则，默认为 `.lattice/standards/requirement-standards.md`。

**写入文档：**
1. 如果 `.lattice/standards/` 目录（和 `.lattice/` 父目录）不存在，创建它。
2. 将文档写入确定的路径。

**更新配置：**
1. 如果 `.lattice/config.yaml` 不存在，创建它：
   ```yaml
   paths:
     requirement_standards: .lattice/standards/requirement-standards.md
   ```
2. 如果 `.lattice/config.yaml` 存在但没有 `paths.requirement_standards`，添加该键。保留所有现有内容。
3. 如果键已经存在，不需要配置更改。

**向用户确认：**
"你的需求标准已写入 `[PATH]`，使用 **[overlay|override]** 模式。Requirement-forge molecule 现在将使用这些标准，不会重新询问此处覆盖的结构问题。"

## 文档质量检查

在编写最终文档之前，验证：

### Overlay 模式检查

- [ ] 每个包含的部分是自包含且完整的（不是 diff 或部分部分）
- [ ] 部分标题与 `template.md` 完全匹配
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: overlay`
- [ ] 仅包含更改或添加的部分

### Override 模式检查

- [ ] 所有 9 个默认部分存在（加上任何添加）
- [ ] §2 feature 大小信号与§4 最大场景计数一致
- [ ] §4 scenario nomenclature 与§8 命名约定一致
- [ ] §5 AC 格式与§4 描述的场景 criteria 一致
- [ ] §6 priority 值和§7 status 值匹配§8 中的 frontmatter 字段描述
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: override`
- [ ] 文档可读为独立规范

### 两种模式

- [ ] Frontmatter 是有效的 YAML，具有正确的模式值
- [ ] 文档是格式良好的 markdown
- [ ] 配置文件（`.lattice/config.yaml`）正确更新
- [ ] 输出路径存在且可写
