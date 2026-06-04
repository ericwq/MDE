---
name: ddd-refiner
description: "通过结构化对话为仓库中的领域设计定义 DDD 护栏。生成正式的 ddd-principles.md 文档，供 domain-driven-design atom 作为覆盖使用。在设置领域设计原则、定义 aggregate 规则时使用，或当用户说'setup DDD'、'define domain rules'、'DDD principles'或'help me define my domain patterns'时使用。"
---

# DDD Refiner（DDD 定义器）

## 产出内容

- **输出**：`.lattice/standards/ddd-principles.md`（或来自 `.lattice/config.yaml` → `paths.ddd_principles` 的自定义路径）
- **两种模式**：
  - **叠加（Overlay）**（`mode: overlay`）：仅包含与默认值不同部分的精简文档。domain-driven-design atom 首先读取其内置的默认值，然后将此文档的部分叠加在上面。这是预期的常见情况。
  - **覆盖（Override）**（`mode: override`）：全面独立的文档，完全替换 atom 的内置默认值。适用于领域建模原则根本不同的团队。
- **默认模式**：叠加——仅生成用户想要更改的内容
- **配置键**：`.lattice/config.yaml` 中的 `paths.ddd_principles`
- **模板**：读取 `./assets/template.md` 获取完整的文档结构、默认内容和访谈指导注释

## 范围澄清

此 skill 定义*领域工艺的*规则，而非领域模型本身。领域模型通过功能演进；此文档定义护栏。它仅涵盖 DDD 战术模式——不涉及战略 DDD（无上下文映射、无微服务拓扑、无 bounded context 集成）。

## 开始之前

### 检查现有文档

在开始访谈之前，检查是否已存在自定义文档：

1. 读取 `.lattice/config.yaml` —— `paths.ddd_principles` 是否指向某个文件？
2. 如果是，读取该文件。询问用户：
   - "你已经有自定义 DDD 原则文档了。你想**修订**它（更新特定部分）、**重新开始**（新访谈）还是**补充**它（添加新部分）？"
   - 修订：加载现有文档，仅 walkthrough 用户想要更改的部分，就地更新。
   - 重新开始：继续下面的完整访谈流程。
   - 补充：跳到访谈的"新部分"部分。
3. 如果没有配置或没有现有文档，继续完整访谈流程。

### 扫描仓库

寻找影响对话的信号：

- **领域文件夹**：是否存在 `domain/`（或 `core/`、`model/`）文件夹？里面有什么？
- **现有 aggregates**：是否有 entities、value objects、aggregate roots？它们如何结构？
- **贫血模式**：entities 是数据持有者还是有行为？
- **身份模式**：Typed IDs、原始 UUIDs、数据库生成的 IDs？
- **事件模式**：是否使用 domain events？什么命名约定？
- **架构文档**：是否有任何现有的 DDD 文档、ADRs、领域词汇表？

在开始时与用户分享相关发现："我注意到你的项目已经有 [X 模式]。我将以此作为上下文。"

如果项目是新的且没有代码，以纯默认值作为起点继续。

## 选择模式

对话中的第一个决策。展示三个选项：

"你想如何定义你的 DDD 原则？

1. **自定义特定部分**（overlay）—— 保留默认值，仅更改与你的项目不同的部分。这会产生一个精简文档。大多数团队选择这个选项。
2. **从头定义所有内容**（override）—— walkthrough 所有部分并生成全面的独立文档。
3. **仅添加项目特定部分**（overlay with additions）—— 保留所有默认值不变，为你的团队特定规则添加新部分（例如，通用词汇表、bounded context 边界）。

默认值很好地覆盖了标准 DDD 战术模式。除非你的领域建模方法根本不同，否则推荐选项 1。"

映射选择：
- 选项 1 和 3 → `mode: overlay`
- 选项 2 → `mode: override`

## 引导方法

### 对话风格

- **一次一个部分**。不要一次性倾倒所有问题。按顺序 walkthrough 模板。
- **默认值优先**。对于每个部分，简要总结默认值，然后询问是否匹配。不要逐字读取整个默认值——总结关键点并询问。
- **记录决策，而非讨论**。输出文档应作为规范阅读，而非会议纪要。"我们讨论了 X 并决定 Y"是错误的。"Y"是正确的。
- **追问，而非审问**。使用模板指导注释中的追问问题作为用户答案模糊时的后续问题，而非检查清单。

### 对于 overlay 模式

这应该很快。许多部分将是"保持原样"。

1. 简要展示每个部分的默认值（2-3 句总结，而非完整内容）。
2. 询问："这匹配你的项目吗，还是你想更改它？"
3. 如果用户说匹配 → 跳过它（该部分**不会**出现在输出中）。
4. 如果用户想要更改 → 深入该部分的细节，讨论具体内容，记录更改。
5. 最后，询问："是否有任何你想添加的不在默认值中的部分？"（例如通用词汇表、bounded context 范围）。
6. 仅用户更改或添加的部分出现在输出文档中。

### 对于 override 模式

这是彻底的。每个部分都会得到关注并出现在输出中。

1. 完整 walkthrough 每个部分的细节。
2. 用户确认、修改或替换每个部分。
3. 所有部分出现在输出中——未更改的使用默认值，已更改的使用用户版本。

### 常见场景

- **"我同意所有内容"** → 不需要自定义文档。告诉用户："内置默认值已经激活并匹配你的偏好。不需要自定义文档——domain-driven-design atom 将自动使用默认值。"
- **"我同意，除了一个部分"** → Overlay 模式，仅访谈那一个部分。
- **"我们有贫血 entities 并想修复它"** → Overlay §2（entity 模式，贫血反模式是内联的）+ §9（entity 检查）。
- **"我们尚未使用 domain events"** → Overlay §5（domain events）带有简化方法或移除说明。还要检查§1（跨 aggregate 协调，反模式是内联的）。
- **"我们的 aggregates 太大了"** → Overlay §1（aggregate 设计，god aggregate 反模式是内联的）+ §8（分解指南）。
- **"我们想添加通用词汇表"** → Overlay 仅新§10。

## 分部分访谈指南

读取 `./assets/template.md`，并按照每个部分的 `<!-- INTERVIEW GUIDANCE: -->` 注释操作。这些注释包含要问的具体问题、追问问题以及可自定义 vs 固定的内容。

### 跨部分依赖表

早期部分的决策影响后期部分。当用户更改早期部分时，标记依赖部分：

| 决策于 | 影响 | 如何影响 |
|---------|------|----------|
| §1 — Aggregate 边界 | §6（repositories）、§5（events）、§8（分解） | 每个 aggregate root 一个 repository；跨 aggregate 协调的事件 |
| §1 — 大小阈值 | §8（分解触发器） | 自定义阈值更改分解警告信号 |
| §2 — Entity 身份策略 | §3（typed ID value objects）、§6（repository 签名） | Typed IDs 必须是 value objects；repository findById 使用 typed IDs |
| §3 — Value object 目录 | §2（entity 字段） | 新 value objects 出现在 entity 定义中 |
| §5 — Event 模式 | §1（跨 aggregate 协调） | Events 是跨 aggregate 一致性的机制 |
| §6 — Repository 模式 | §1（aggregate root 识别） | 仅 roots 获得 repositories |

当触发依赖时，通知用户："因为你更改了 [X]，我们也应该审查 [Y]——它受那个决策影响。"

### Overlay 特定部分流程

对于 9 个部分中的每一个：

1. 用 2-3 句话总结部分的关键点。
2. 询问："这匹配你的项目吗？"
3. **是** → 移动到下一部分。该部分不会出现在输出中。
4. **否** → 使用模板指导深入部分细节。生成用户版本。
5. 所有 9 个部分之后，询问新部分。

### Override 特定部分流程

对于 9 个部分中的每一个：

1. 展示部分的完整内容。
2. 询问："这可以直接使用，还是你想修改它？"
3. **原样** → 在输出中不变地包含默认内容。
4. **修改** → 讨论更改，生成修改版本。
5. 所有 9 个部分之后，询问新部分。
6. 所有部分都进入输出。

## 输出组装

### 对于 overlay 模式

1. YAML frontmatter：`mode: overlay`
2. Overlay 序言文本（来自模板）
3. 仅列出包含部分的目录
4. 仅用户更改或添加的部分
5. 每个部分必须自包含——它是默认值中该部分的完整替换。不要编写 diffs 或部分部分。
6. 部分标题必须与 `defaults.md` 完全匹配（atom 按标题匹配部分）
7. 新部分（§10+）在默认部分之后包含
8. 页脚包含项目名称、日期、模式

### 对于 override 模式

1. YAML frontmatter：`mode: override`
2. Override 序言文本（来自模板）
3. 完整目录（所有 10+ 部分）
4. 所有部分：未更改的使用默认值，已更改的使用用户版本，新部分在末尾
5. 页脚包含项目名称、日期、模式

### 对于两种模式

从输出中剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释。最终文档是干净的规范。

**确定输出路径：**
1. 如果 `.lattice/config.yaml` 存在且有 `paths.ddd_principles`，使用该路径。
2. 否则，默认为 `.lattice/standards/ddd-principles.md`。

**写入文档：**
1. 如果 `.lattice/standards/` 目录（和 `.lattice/` 父目录）不存在，创建它。
2. 将文档写入确定的路径。

**更新配置：**
1. 如果 `.lattice/config.yaml` 不存在，创建它：
   ```yaml
   paths:
     ddd_principles: .lattice/standards/ddd-principles.md
   ```
2. 如果 `.lattice/config.yaml` 存在但没有 `paths.ddd_principles`，添加该键。保留所有现有内容。
3. 如果 `.lattice/config.yaml` 存在且已经有该键，不需要配置更改。

**向用户确认：**
"你的 DDD 原则文档已写入 `[PATH]`，使用 **[overlay|override]** 模式。Domain-driven-design atom 现在将 [在默认值之上 | 替代默认值] 使用它。"

## 文档质量检查

在编写最终文档之前，验证：

### Overlay 模式检查

- [ ] 每个包含的部分是自包含且完整的（不是 diff 或部分部分）
- [ ] 部分标题与 `defaults.md` 完全匹配（用于 atom 的部分匹配）
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: overlay`
- [ ] 仅包含更改/添加的部分——未更改的部分被省略

### Override 模式检查

- [ ] 模板中的每个部分都存在（§1 到 §9，加上任何新部分）
- [ ] 术语在所有部分中一致
- [ ] 代码示例使用伪代码（语言无关，与 defaults.md 相同风格）
- [ ] 验证清单（§9）与 §1 到 §7 中定义的规则一致
- [ ] 内联反模式警告与其各自部分中定义的模式对齐
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: override`
- [ ] 文档可读为独立规范

### 两种模式

- [ ] Frontmatter 是有效的 YAML，具有正确的模式值
- [ ] 文档是格式良好的 markdown
- [ ] 配置文件（`.lattice/config.yaml`）正确更新
- [ ] 输出路径存在且可写
