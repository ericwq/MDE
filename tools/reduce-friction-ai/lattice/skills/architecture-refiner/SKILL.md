---
name: architecture-refiner
description: "通过结构化对话为仓库定义架构原则。支持多种架构风格：clean architecture（默认）、hexagonal / ports & adapters、modular monolith 或自定义。生成正式的架构文档，供对应的 atom 使用。在设置新项目、定义架构标准时使用，或当用户说'setup architecture'、'define layers'、'architecture principles'、'help me define my architecture'、'hexagonal architecture'、'modular monolith'、'ports and adapters'或'define my architecture style'时使用。"
---

# Architecture Refiner（架构定义器）

## 第 0 步：风格选择

首先，询问用户团队使用哪种架构风格：

"你的团队使用哪种架构风格？

1. **Clean Architecture**（默认）—— 层（Domain、Application、Interface、Infrastructure）、依赖倒置、命令/查询分离
2. **Hexagonal / Ports & Adapters** —— 核心领域被 ports 包围，adapters 在外层
3. **Modular Monolith** —— 垂直切片，每个模块拥有自己的层
4. **自定义/从头定义** —— 你描述层和规则"

**分支：**

- **选项 1** → 继续下面的 clean architecture 流程（现有访谈）。模板：`./assets/template-clean-arch.md`。输出：`.lattice/standards/architecture.md`。配置键：`paths.architecture`。不需要 `architecture_mode` 键（默认为 `clean`）。
- **选项 2–4** → 继续通用架构流程。模板：`./assets/template-generic.md`。输出：`.lattice/standards/architecture.md`。配置键：`paths.architecture`。此外，在 `.lattice/config.yaml` 中设置 `architecture_mode: custom`。

本文档的其余部分描述 **clean architecture 流程**（选项 1）。对于 **通用流程**（选项 2–4），读取 `./assets/template-generic.md` 并按照其 `<!-- INTERVIEW GUIDANCE: -->` 注释操作。下面的引导方法、对话风格、输出组装和文档质量检查适用于两种流程——替换适当的模板、输出路径和配置键。

## 产出内容

**对于 clean architecture（选项 1）：**

- **输出**：`.lattice/standards/architecture.md`（或来自 `.lattice/config.yaml` → `paths.architecture` 的自定义路径）
- **两种模式**：
  - **叠加（Overlay）**（`mode: overlay`）：仅包含与默认值不同部分的精简文档。architecture atom 首先读取其内置的 clean-architecture 默认值，然后将此文档的部分叠加在上面。这是预期的常见情况。
  - **覆盖（Override）**（`mode: override`）：全面独立的文档，完全替换 atom 的内置默认值。适用于想要从头定义 clean architecture 的团队。
- **默认模式**：叠加——仅生成用户想要更改的内容
- **配置键**：`.lattice/config.yaml` 中的 `paths.architecture`
- **模板**：读取 `./assets/template-clean-arch.md` 获取完整的文档结构、默认内容和访谈指导注释

**对于其他风格（选项 2–4）：**

- **输出**：`.lattice/standards/architecture.md`（或来自 `.lattice/config.yaml` → `paths.architecture` 的自定义路径）
- **模式**：始终为 `override`——对于非 clean-architecture 风格没有内置默认值可以叠加
- **配置键**：`.lattice/config.yaml` 中的 `paths.architecture`
- **额外配置**：在 `.lattice/config.yaml` 中设置 `architecture_mode: custom`
- **模板**：读取 `./assets/template-generic.md` 获取文档结构和访谈指导注释

## 开始之前

### 检查现有文档

在开始访谈之前，检查是否已存在自定义文档：

1. 读取 `.lattice/config.yaml` —— 检查 `paths.architecture`。
2. 如果相关路径存在（基于第 0 步选择的风格），读取该文件。询问用户：
   - "你已经有自定义架构文档了。你想**修订**它（更新特定部分）、**重新开始**（新访谈）还是**补充**它（添加新部分）？"
   - 修订：加载现有文档，仅 walkthrough 用户想要更改的部分，就地更新。
   - 重新开始：继续下面的完整访谈流程。
   - 补充：跳到访谈的"新部分"部分。
3. 如果没有配置或没有现有文档，继续完整访谈流程。

### 扫描仓库

寻找影响对话的信号：

- **目录结构**：`src/`（或等效）是否已经有层？它们叫什么名字？
- **现有模式**：是否有现有的 controllers、services、repositories、providers？使用什么命名约定？
- **DI 模式**：是否有 DI 容器、手动注入或框架提供的注入？
- **架构文档**：是否有任何现有的架构文档（ADRs、README 部分）？
- **框架**：使用什么框架？（NestJS、Spring、Django 等）这影响命名约定和常见模式。

在开始时与用户分享相关发现："我注意到你的项目已经有 [X 结构]。我将以此作为上下文。"

如果项目是新的且没有代码，以纯默认值作为起点继续。

## 选择模式

对话中的第一个决策。展示三个选项：

"你想如何定义你的架构原则？

1. **自定义特定部分**（overlay）—— 保留默认值，仅更改与你的项目不同的部分。这会产生一个精简文档。大多数团队选择这个选项。
2. **从头定义所有内容**（override）—— walkthrough 所有部分并生成全面的独立文档。
3. **仅添加项目特定部分**（overlay with additions）—— 保留所有默认值不变，为你的团队特定规则添加新部分。

默认值很好地覆盖了标准 clean architecture。除非你的架构根本不同，否则推荐选项 1。"

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
5. 最后，询问："是否有任何你想添加的不在默认值中的部分？"（例如命名约定、框架特定规则）。
6. 仅用户更改或添加的部分出现在输出文档中。

### 对于 override 模式

这是彻底的。每个部分都会得到关注并出现在输出中。

1. 完整 walkthrough 每个部分的细节。
2. 用户确认、修改或替换每个部分。
3. 所有部分出现在输出中——未更改的使用默认值，已更改的使用用户版本。

### 常见场景

- **"我同意所有内容"** → 不需要自定义文档。告诉用户："内置默认值已经激活并匹配你的偏好。不需要自定义文档——architecture atom 将自动使用 clean-architecture 默认值。"
- **"我同意，除了一个部分"** → Overlay 模式，仅访谈那一个部分。
- **"我们使用 CQRS"** → Overlay §3.2 + §4（它们是耦合的——CQRS 更改服务模式，从而同时更改两种流程）。
- **"我们不使用 Providers"** → Overlay §3.4 + §4.2 + §4.3 + §6（Provider 移除通过查询流程和验证清单传播）。
- **"我们有额外的层"** → Overlay §1 + §2 + §3（新层需要职责、依赖放置和每层规则）。

## 分部分访谈指南

读取 `./assets/template-clean-arch.md`（对于 clean architecture）或 `./assets/template-generic.md`（对于其他风格），并按照每个部分的 `<!-- INTERVIEW GUIDANCE: -->` 注释操作。这些注释包含要问的具体问题、追问问题以及可自定义 vs 固定的内容。

### 跨部分依赖表

早期部分的决策影响后期部分。当用户更改早期部分时，标记依赖部分：

| 决策于 | 影响 | 如何影响 |
|---------|------|----------|
| §1 — 层名称 | 所有部分 | 名称必须在所有地方保持一致 |
| §1 — 额外层 | §2（图表）、§3（每层规则） | 新层需要依赖放置和规则 |
| §3.2 — 服务模式（统一 vs CQRS） | §4.1、§4.2 | CQRS 使用单独的处理程序而非统一服务 |
| §3.4 — Provider 模式（是/否） | §4.2、§4.3、§6 | 无 Provider → 读取通过 Repository；比较表和清单更改 |

当触发依赖时，通知用户："因为你更改了 [X]，我们也应该审查 [Y]——它受那个决策影响。"

### Overlay 特定部分流程

对于 6 个默认部分中的每一个：

1. 用 2-3 句话总结部分的关键点。
2. 询问："这匹配你的项目吗？"
3. **是** → 移动到下一部分。该部分不会出现在输出中。
4. **否** → 使用模板指导深入部分细节。生成用户版本。
5. 所有 6 个部分之后，询问新部分。

### Override 特定部分流程

对于 6 个默认部分中的每一个：

1. 展示部分的完整内容。
2. 询问："这可以直接使用，还是你想修改它？"
3. **原样** → 在输出中不变地包含默认内容。
4. **修改** → 讨论更改，生成修改版本。
5. 所有 6 个部分之后，询问新部分。
6. 所有部分都进入输出。

## 输出组装

### 对于 overlay 模式

1. YAML frontmatter：`mode: overlay`
2. Overlay 序言文本（来自模板）
3. 仅列出包含部分的目录
4. 仅用户更改或添加的部分
5. 每个部分必须自包含——它是默认值中该部分的完整替换。不要编写 diffs 或部分部分。
6. 部分标题必须与 `clean-architecture-defaults.md` 完全匹配（atom 按标题匹配部分）
7. 新部分（§7+）在默认部分之后包含
8. 页脚包含项目名称、日期、模式

### 对于 override 模式

1. YAML frontmatter：`mode: override`
2. Override 序言文本（来自模板）
3. 完整目录（所有 6+ 部分）
4. 所有部分：未更改的使用默认值，已更改的使用用户版本，新部分在末尾
5. 页脚包含项目名称、日期、模式

### 对于两种模式

从输出中剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释。最终文档是干净的规范。

**确定输出路径：**

1. 如果 `.lattice/config.yaml` 存在且有 `paths.architecture`，使用该路径。
2. 否则，默认为 `.lattice/standards/architecture.md`。

这对于所有风格都是相同的——both clean architecture customizations and other styles write to `paths.architecture`。

**写入文档：**
1. 如果 `.lattice/standards/` 目录（和 `.lattice/` 父目录）不存在，创建它。
2. 将文档写入确定的路径。

**更新配置：**

对于 clean architecture（选项 1）：
1. 如果 `.lattice/config.yaml` 不存在，创建它：
   ```yaml
   paths:
     architecture: .lattice/standards/architecture.md
   ```
2. 如果 `.lattice/config.yaml` 存在但没有 `paths.architecture`，添加该键。保留所有现有内容。
3. 如果 `.lattice/config.yaml` 存在且已经有该键，不需要配置更改。

对于其他风格（选项 2–4）：
1. 如果 `.lattice/config.yaml` 不存在，创建它：
   ```yaml
   paths:
     architecture: .lattice/standards/architecture.md
   architecture_mode: custom
   ```
2. 如果 `.lattice/config.yaml` 存在，添加或更新：
   - `paths.architecture` 指向输出路径
   - `architecture_mode: custom`
   - 保留所有现有内容。

**向用户确认：**

对于 clean architecture：
"你的架构文档已写入 `[PATH]`，使用 **[overlay|override]** 模式。Architecture atom 现在将 [在 clean-architecture 默认值之上 | 替代 clean-architecture 默认值] 使用它。"

对于其他风格：
"你的架构文档已写入 `[PATH]`，使用 `architecture_mode: custom`。Architecture atom 将把它作为你项目的唯一架构标准。"

## 文档质量检查

在编写最终文档之前，验证：

### Overlay 模式检查

- [ ] 每个包含的部分是自包含且完整的（不是 diff 或部分部分）
- [ ] 部分标题与 `defaults.md` 完全匹配（用于 atom 的部分匹配）
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: overlay`
- [ ] 仅包含更改/添加的部分——未更改的部分被省略

### Override 模式检查

- [ ] 模板中的每个部分都存在（§1 到 §6，加上任何新部分）
- [ ] 层名称在所有部分中一致
- [ ] 依赖图（§2）匹配层表（§1）
- [ ] 代码示例使用伪代码（语言无关，与 defaults.md 相同风格）
- [ ] 验证清单（§6）与 §3 和 §4 中定义的规则一致
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] Frontmatter 有 `mode: override`
- [ ] 文档可读为独立规范

### 通用流程检查（选项 2–4）

- [ ] 文档 frontmatter 有 `mode: override`
- [ ] §1 到 §7 部分存在（§8 Ambiguity Signals 是可选的，加上任何新部分）
- [ ] 层名称在所有部分中一致
- [ ] 依赖图（§2）匹配层表（§1）
- [ ] §6（验证清单）包含至少 3 个具体、可验证的检查
- [ ] §7（反模式）包含至少 3 个带有症状和修复的反模式
- [ ] 无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] 文档可读为独立规范
- [ ] 配置有 `architecture_mode: custom` 设置

### 两种模式（所有流程）

- [ ] Frontmatter 是有效的 YAML，具有正确的模式值
- [ ] 文档是格式良好的 markdown
- [ ] 配置文件（`.lattice/config.yaml`）正确更新
- [ ] 输出路径存在且可写
