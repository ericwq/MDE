---
name: lattice-init
description: "为新 Lattice 项目提供引导式设置体验——扫描仓库、检测现有配置、按优先级建议运行 refiners，并创建 .lattice/ 配置。填补安装 skills 与获得首次价值之间的差距。当用户说'lattice init'、'set up lattice'、'initialize lattice'、'get started with lattice'或'configure lattice for this project'时使用。"
---

# Lattice Init（Lattice 初始化）

## 所需 Skills

按顺序读取并应用：

1. `framework:knowledge-priming` —— 加载项目上下文，了解项目是什么以及已存在什么

## 工作流

### 第 1 步：扫描项目

检测关于项目的信号，了解现有 Lattice 状态的全貌。

**语言/框架检测** —— 检查仓库根目录的文件：
- `package.json` → Node.js / TypeScript
- `go.mod` → Go
- `pom.xml` 或 `build.gradle` → Java
- `Cargo.toml` → Rust
- `requirements.txt` 或 `pyproject.toml` → Python
- `Gemfile` → Ruby
- `*.csproj` 或 `*.sln` → C# / .NET

如果在仓库根目录发现多种语言标记，记录所有标记并询问用户哪个是主要技术栈，然后再继续 refiner 建议。

**目录结构** —— 列出顶级目录。识别常见模式：
- `src/`、`lib/`、`app/` → 源代码
- `test/`、`tests/`、`spec/` → 测试套件
- `docs/` → 文档
- `cmd/`、`internal/`、`pkg/` → Go 项目结构
- `domain/`、`infrastructure/`、`application/` → 分层架构

**现有 `.lattice/` 状态** —— 检查已存在的 Lattice 工件：
- `.lattice/config.yaml` → 中心配置（检查 `language` 键）
- `.lattice/standards/language-idioms.md` → language idioms refiner 输出
- `.lattice/standards/knowledge-base.md` → knowledge priming 输出
- `.lattice/standards/architecture.md` → architecture refiner 输出（clean architecture、hexagonal、modular monolith 或自定义风格）
- `.lattice/standards/clean-code.md` → clean code refiner 输出
- `.lattice/standards/ddd-principles.md` → DDD refiner 输出
- `.lattice/standards/review-standards.md` → review refiner 输出
- `.lattice/context/` → 功能上下文文档（统计数量）
- `.lattice/learnings/operational-learnings.md` → 累积的运营经验（由 learning-harvest atom 管理）
- `.lattice/reviews/review-log.md` → 审查日志

### 第 2 步：呈现发现结果

报告发现的内容——简洁、结构化。向用户展示：

```
## 项目扫描结果

**项目**: [检测到的语言/框架]，位于 [仓库根目录]
**结构**: [找到的关键目录]

### Lattice 设置状态
- `.lattice/config.yaml`: [存在 / 未找到]
- 语言: [检测到的语言 / 配置中的语言键 / 未检测到]
- Language idioms: [在 .lattice/standards/language-idioms.md 找到 / 未找到]
- Knowledge base: [在 .lattice/standards/knowledge-base.md 找到 / 未找到]
- Architecture standards: [在 .lattice/standards/architecture.md 找到 / 未找到]
- Clean code standards: [存在 / 不存在]
- DDD standards: [存在 / 不存在]
- Review standards: [存在 / 不存在]
- Context documents: [找到 N 个 / 无]
- Review learnings: [存在 / 不存在]
- Review log: [存在 / 不存在]
```

**如果一切已设置完成**（配置文件存在且所有核心标准文档都存在）：确认"Lattice 已为此项目完全配置"，直接跳到第 4 步。

### 第 3 步：引导式设置

基于第 2 步发现的缺口，按优先级建议 refiners。逐一引导用户完成每个缺失的部分。

**优先级顺序**：

1. **Knowledge-priming-refiner**（如果缺少 `.lattice/standards/knowledge-base.md`）—— "捕获项目身份——技术栈、架构、目录布局、约定。其他所有 skills 都使用此上下文来做出更好的决策。"
2. **Language-idioms-refiner**（如果缺少 `.lattice/standards/language-idioms.md`）—— "定义你的语言如何表达工程模式——错误处理、类型系统、命名、测试、DI。多个 atoms 使用此文件将伪代码默认值适配到你的语言。快速访谈：提出语言习惯的默认值，你确认或调整。"
3. **Architecture-refiner**（如果缺少 `.lattice/standards/architecture.md` 且项目有源代码目录）—— "定义项目架构标准——层结构、依赖规则、验证清单。支持多种风格：clean architecture（默认）、hexagonal / ports & adapters、modular monolith 或自定义。"
4. **DDD-refiner**（如果缺少 `.lattice/standards/ddd-principles.md` 且项目有 domain 文件夹或类似 domain 的结构）—— "捕获 aggregate 设计规则、entity 模式、domain event 约定，以便 DDD atom 强制执行领域建模风格。"
5. **Clean-code-refiner**（如果缺少 `.lattice/standards/clean-code.md`）—— "定制编码标准——函数大小限制、复杂度阈值、命名约定。默认值对大多数项目效果良好，因此是可选的。"
6. **Review-refiner**（如果缺少 `.lattice/standards/review-standards.md`）—— "自定义 review molecule 的工作方式——atom 加载规则、严重程度级别、报告格式、范围规则。默认值对大多数项目效果良好，因此是可选的。"

**对于每个缺口**，向用户展示：
- refiner 的作用（一句话，来自上述描述）
- 三个选择：**立即运行**、**稍后跳过**或**跳过所有剩余**

**如果用户说"run"** → 告诉用户调用 refiner："运行 `/[refiner-name]` 现在启动引导式访谈。"如果 refiner 在完成之前退出，用户可以重新运行——`.lattice/standards/` 中已存在的部分输出不会阻止访谈重新启动。

**如果用户说"skip"** → 移动到下一个 refiner 优先级顺序。

**如果用户说"skip all"** → 跳到第 4 步。

**配置文件创建**：如果 `.lattice/config.yaml` 不存在且用户未运行任何 refiner（全部跳过），创建最小配置文件：

```yaml
# .lattice/config.yaml -- Lattice 框架配置
# 所有路径相对于仓库根目录。
# 运行 refiners 来填充：/knowledge-priming-refiner, /language-idioms-refiner, /architecture-refiner, /ddd-refiner, /clean-code-refiner, /review-refiner

version: 1
language: {检测到的语言}
paths: {}
```

如果用户运行了至少一个 refiner，refiner 本身会创建或更新配置文件——无需在此处创建。即使没有运行任何 refiner，也从检测到的语言设置 `language` 键——当缺少 language-idioms 文档时，atoms 会将其作为回退使用。

### 第 4 步：下一步

呈现工作流，让用户知道接下来做什么。

```
## 你已准备好

Lattice 已设置完成。以下是工作流：

1. **设计功能**：`/design-blueprint` —— 逐步走过 5 个渐进式设计层级
2. **实现**：`/code-forge` —— 从 blueprint 生成代码，内置质量检查
3. **安全重构**：`/refactor-safely` —— 先同意目标结构，添加 characterization 保护，在不改变行为的情况下改进代码
4. **修复 bug**：`/bug-fix` —— 复现失败，添加回归测试，应用最小安全修复
5. **审查**：`/review` —— 根据 atom 标准审计生成的代码

Atoms（architecture、clean-code、DDD、secure-coding 等）在这些工作流中自动激活。
你也可以单独使用 atoms——它们根据你正在处理的内容应用检查。
```

如果任何 refiner 在第 3 步被跳过，添加提醒：

```
### 跳过的 refiners
你可以随时运行这些来进一步自定义 Lattice 以适应你的项目：
- [列出跳过的 refiners 及其斜杠命令]
```
