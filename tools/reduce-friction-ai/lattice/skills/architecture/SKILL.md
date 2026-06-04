---
name: architecture
description: "在生成或修改代码时强制执行架构规则。默认为 clean architecture；通过 architecture-refiner 支持任何架构风格。使用加载的架构规则验证层职责、依赖方向和结构约束。在生成代码、审查架构、创建新文件时使用，或当用户提到'architecture'、'layers'、'structure'、'dependency rules'、'hexagonal architecture'、'ports and adapters'、'modular monolith'或'onion architecture'时使用。审查生成代码的结构合规性时也使用。"
---

# Architecture（架构）

## 配置解析

Skill 支持两种模式：**clean architecture**（默认）和 **custom architecture**（团队定义）。模式决定加载哪些规则。

**第 1 步——确定模式：**

1. 读取仓库根目录的 `.lattice/config.yaml`
2. 检查 `architecture_mode` 键
   - 如果 `architecture_mode: custom` → **custom 模式**
   - 如果缺失或其他值 → **clean architecture 模式**（默认）

**第 2 步——加载执行规则：**

- **Clean architecture 模式** → 读取 `./references/clean-architecture.md` 获取执行指令（自我验证清单、反模式扫描、模糊信号、结构原则）
- **Custom 模式** → 读取 `./references/custom-architecture.md` 获取执行指令

**第 3 步——加载架构内容：**

- **Clean architecture 模式：**
  1. 检查 `.lattice/config.yaml` 中的 `paths.architecture` 获取自定义文档
  2. 如果找到，读取文档并检查 YAML frontmatter 中的 `mode`：
     - **`mode: overlay`**：首先读取 `./references/clean-architecture-defaults.md`，然后将自定义文档部分叠加在上面。按标题匹配部分——自定义部分替换匹配的默认值，新部分追加。
     - **`mode: override`**：自定义文档完全优先。必须是全面的。
  3. 如果没有自定义文档 → 读取 `./references/clean-architecture-defaults.md`

- **Custom 模式：**
  1. 检查 `.lattice/config.yaml` 中的 `paths.architecture` 获取团队架构文档
  2. 如果找到 → 读取它。唯一参考——无默认值。
  3. 如果未找到 → 浮现："未找到架构文档。运行 `/architecture-refiner` 并选择你的架构风格以定义团队标准。"

**第 4 步——语言适配：**

如果配置中存在 `paths.language_idioms`，读取 **"Dependency Management"** 部分并将依赖方向执行适配到语言习惯用法（例如，Go interface-at-consumer、Java DI 容器、Rust trait bounds）。语言习惯用法优先于伪代码默认值。

## 执行

生成每个组件后 STOP。从加载的执行规则（clean-architecture.md 或 custom-architecture.md）读取 **自我验证清单** 和 **反模式扫描** 并应用。

**项目特定检查：** 如果架构内容文档（在第 3 步加载）包含 **Validation Checklist** 部分（§6），在应用执行规则清单后，将这些检查作为额外的项目特定验证。这些是架构-refiner 生成的团队定制检查。
