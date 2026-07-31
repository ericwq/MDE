---
name: architecture
description: "在生成或修改代码时强制执行架构规则。默认使用整洁架构；通过架构精炼器支持任意架构风格。使用已加载的架构规则验证层级职责、依赖方向和结构约束。在生成代码、审查架构、创建新文件，或用户提及'架构'、'层级'、'结构'、'依赖规则'、'六边形架构'、'端口与适配器'、'模块化单体'或'洋葱架构'时使用。也可在审查生成代码的结构合规性时使用。"
---

# 架构

## 配置解析

**步骤 1 — 确定模式：**

1. 读取仓库根目录下的 `.lattice/config.yaml`
2. 检查 `architecture_mode` 键
   - 如果 `architecture_mode: custom` → **自定义模式**
   - 如果缺失，或为其他值 → **整洁架构模式**（默认）

**步骤 2 — 加载执行规则：**

- **整洁架构模式** → 读取 `./references/clean-architecture.md` 获取执行指令（自检清单、反模式扫描、歧义信号、结构原则）
- **自定义模式** → 读取 `./references/custom-architecture.md` 获取执行指令

**步骤 3 — 加载架构内容：**

- **整洁架构模式：**
  1. 检查 `.lattice/config.yaml` 中的 `paths.architecture` 是否存在自定义文档
  2. 如果找到，读取文档并检查 YAML 前置元数据中的 `mode`：
     - **`mode: overlay`**：先读取 `./references/clean-architecture-defaults.md`，然后将自定义文档按章节叠加。章节按标题匹配——自定义章节替换匹配的默认章节，新章节追加。
     - **`mode: override`**：自定义文档完全优先。必须内容全面。
  3. 如果没有自定义文档 → 读取 `./references/clean-architecture-defaults.md`

- **自定义模式：**
  1. 检查 `.lattice/config.yaml` 中的 `paths.architecture` 是否存在团队架构文档
  2. 如果找到 → 读取它。作为唯一参考——没有默认值。
  3. 如果未找到 → 提示："未找到架构文档。请运行 `/architecture-refiner` 并选择您的架构风格以定义团队标准。"

**步骤 4 — 语言适配：**

如果配置中存在 `paths.language_idioms`，读取 **"依赖管理"** 章节，并将依赖方向执行适配到语言惯用法（例如，Go 的接口在消费者端定义、Java DI 容器、Rust trait 约束）。语言惯用法优先于伪代码默认值。

## 执行

在生成每个组件后停止。从已加载的执行规则（clean-architecture.md 或 custom-architecture.md）中读取**自检清单**和**反模式扫描**并应用。

**项目特定检查：** 如果架构内容文档（在步骤 3 中加载）包含**验证清单**章节（§6），则在执行规则清单之后，将这些检查作为额外的项目特定验证来应用。
