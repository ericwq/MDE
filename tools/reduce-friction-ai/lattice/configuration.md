# 配置参考

`.lattice/config.yaml` 是启用 Lattice 的项目的中心配置文件。
它将逻辑键 (logical keys) 映射到原子技能和分子技能在运行时加载的项目特定文档。
该文件是可选的 —— 所有技能开箱即用，带有内置默认值。
仅当你想定制某个技能的行为时才添加键 (keys)。
解析算法的工作原理请参见 [how-it-works.md](how-it-works.md#配置解析) 。

## 文件结构

```yaml
version: 1
language: go
paths:
  language_idioms: .lattice/standards/language-idioms.md
  knowledge_base: .lattice/standards/knowledge-base.md
  clean_code: .lattice/standards/clean-code.md
  architecture: .lattice/standards/architecture.md
  ddd_principles: .lattice/standards/ddd-principles.md
  test_quality: .lattice/standards/test-quality.md
  secure_coding: .lattice/standards/secure-coding.md
  review_standards: .lattice/standards/review-standards.md
  requirement_standards: .lattice/standards/requirement-standards.md
  context_base: .lattice/context/
  operational_learnings: .lattice/learnings/operational-learnings.md

architecture_mode: clean
```

## 顶层字段

**`version`**
- 类型：整数
- 描述：Schema 版本。当前为 `1`。

**`language`**
- 类型：字符串
- 描述：项目的主要语言标识符（例如 `go`、`rust`、`python`、`java`、`typescript`、`csharp`）。
由 `lattice-init` 或 `language-idioms-refiner` 设置。
当 `paths.language_idioms` 文档不存在时，原子技能将其作为后备使用。

**`paths`**
- 类型：映射
- 描述：逻辑键 (logical key) → 文件路径映射。所有键均为可选。

**`architecture_mode`**
- 类型：字符串
- 描述：架构强制执行模式。`clean`（默认值）或 `custom`。参见下文。

## `paths` 键

**`language_idioms`**
- 用途：语言特定模式 —— 错误处理理念、类型系统、命名约定、测试惯用法、参数设计、依赖管理。横切关注点：被多个原子技能消费。
- 生成方式：`language-idioms-refiner`
- 默认路径：`.lattice/standards/language-idioms.md`
- 使用者：`clean-code`、`test-quality`、`secure-coding`、`domain-driven-design`、`architecture` 原子技能
- 模式：standalone（无 overlay/override —— 始终完整）

**`knowledge_base`**
- 用途：项目标识 —— 技术栈、架构、约定、可信来源。无内置默认值；每个项目都独一无二。
- 生成方式：`knowledge-priming-refiner`
- 默认路径：`.lattice/standards/knowledge-base.md`
- 使用者：`knowledge-priming` 原子技能
- 模式：`override`（标准）

**`clean_code`**
- 用途：代码工艺规则 —— 函数大小、命名、复杂度、错误处理。
- 生成方式：`clean-code-refiner`
- 默认路径：`.lattice/standards/clean-code.md`
- 使用者：`clean-code` 原子技能
- 模式：`overlay`（推荐）

**`architecture`**
- 用途：架构标准 —— 层结构、依赖规则、结构验证。在整洁架构模式和自定义架构模式下均使用。
- 生成方式：`architecture-refiner`
- 默认路径：`.lattice/standards/architecture.md`
- 使用者：`architecture` 原子技能
- 模式：`overlay`（整洁模式）或 `override`（自定义模式）

**`ddd_principles`**
- 用途：战术 DDD 模式 —— 聚合设计、实体/值对象规则、领域服务、领域事件。
- 生成方式：`ddd-refiner`
- 默认路径：`.lattice/standards/ddd-principles.md`
- 使用者：`domain-driven-design` 原子技能
- 模式：`overlay`（推荐）

**`test_quality`**
- 用途：测试结构与质量规则 —— AAA 结构、隔离、断言模式、命名约定。
- 生成方式：无精炼技能——手动编写或通过 `/knowledge-priming-refiner` 编写通用约定
- 默认路径：`.lattice/standards/test-quality.md`
- 使用者：`test-quality` 原子技能
- 模式：`overlay`（推荐）

**`secure_coding`**
- 用途：信任边界与注入防护 —— 输入验证、密钥管理、授权、错误消息策略。
- 生成方式：无精炼技能——手动编写或通过 `/knowledge-priming-refiner` 编写通用约定
- 默认路径：`.lattice/standards/secure-coding.md`
- 使用者：`secure-coding` 原子技能
- 模式：`overlay`（推荐）

**`review_standards`**
- 用途：审查流程配置 —— 原子技能加载策略、严重程度分类、报告格式、见解捕获。分子技能级配置，非原子技能级。
- 生成方式：`review-refiner`
- 默认路径：`.lattice/standards/review-standards.md`
- 使用者：`review` 分子技能
- 模式：`overlay`（推荐）

**`requirement_standards`**
- 用途：需求标准 —— epic/功能定义、场景结构、验收条件格式、优先级标记、状态工作流、命名约定。由 `requirement-quality` 原子技能通过配置解析消费；`requirement-forge` 分子技能组合该原子技能。
- 生成方式：`requirement-forge-refiner`
- 默认路径：`.lattice/standards/requirement-standards.md`
- 使用者：`requirement-quality` 原子技能
- 模式：`overlay`（推荐）

**`context_base`**
- 用途：每个功能动态文档的**目录**路径。与所有其他键不同，这是一个目录，而非文件。
- 生成方式：无（由 `context-anchoring` 原子技能管理）
- 默认路径：`.lattice/context/`
- 使用者：`context-anchoring` 原子技能
- 模式：不适用

`operational_learnings`
- 用途：操作性经验文件 —— 从实践中积累的模式（设计、实现、审查、修复）。由 `learning-harvest` 原子技能管理的动态文档，非标准文档。
- 生成方式：（无——由 `learning-harvest` 原子技能管理）
- 默认路径：`.lattice/learnings/operational-learnings.md`
- 使用者：`learning-harvest` 原子技能
- 模式：不适用（仅追加的动态文档，无 overlay/override）

## `architecture_mode` 键

控制 `architecture` 原子技能内部加载哪些强制执行规则。
该键决定原子技能的行为 —— 它不影响其他原子技能或分子技能的行为。

| 值 | 行为 |
|-----|------|
| `clean`（若缺失则为默认值） | architecture 原子技能加载整洁架构强制执行规则（`references/clean-architecture.md`），并使用 `references/clean-architecture-defaults.md` 作为基础内容。如果设置了 `paths.architecture`，则自定义文档将作为覆盖层或替换层应用于整洁架构默认值之上。 |
| `custom` | architecture 原子技能加载自定义架构强制执行规则（`references/custom-architecture.md`），并将团队在 `paths.architecture` 指向的文档作为唯一内容进行读取。没有嵌入默认值 —— 该文档本身就是标准。 |

**何时使用每种模式：**

- **团队使用整洁架构（默认，无需配置）：** 原子技能加载内置的整洁架构规则。无需任何设置。

- **团队使用整洁架构并带有定制：** 运行 `/architecture-refiner`，选择 “Clean Architecture”，定制各个部分。
生成一个带有 `mode: overlay` 或 `mode: override` 的文档。
配置：设置 `paths.architecture`，`architecture_mode` 可省略（默认为 `clean`）。

- **团队使用六边形架构、模块化单体或自定义风格：** 运行 `/architecture-refiner`，选择相应的风格。
生成一个带有 `mode: override` 的文档。
配置：设置 `paths.architecture`，`architecture_mode: custom`。

`architecture-refiner` 会根据用户选择的风格自动设置 `architecture_mode`。

## 自定义文档 Frontmatter

标准文档（由 `paths` 键指向的文件）在 YAML frontmatter 中声明其合并模式：

```yaml
---
mode: overlay
---
```

| 模式 | 行为 |
|------|------|
| `overlay`（默认值） | 自定义文档的章节应用于原子技能内置默认值之上。章节通过标题进行匹配 —— 自定义章节替换匹配的默认章节；新章节被追加。 |
| `override` | 自定义文档完全替换原子技能的内置默认值。当你的标准与默认值根本不同且你希望完全控制时使用。 |

`knowledge_base` 始终为 `override` —— 项目标识是独一无二的，完全替换通用默认值。
自定义架构文档（`architecture_mode: custom`）也始终为 `override` —— 没有可叠加的默认值。
`language_idioms` 始终为 standalone —— 原子技能中没有内置的语言默认值；
该文档提供了原子技能按章节标题引用的完整语言上下文。

## 创建和更新配置

**通过精炼技能（推荐）：** 运行相应的精炼技能（例如 `/architecture-refiner`）。
引导式访谈会生成标准文档并自动写入配置键。

**手动操作：** 在仓库根目录下创建 `.lattice/config.yaml`，并添加指向已编写或将要编写的文档的键。
每当你的标准演进时，重新运行精炼技能或直接编辑标准文档即可。
