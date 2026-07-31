# Lattice Skills 导航指南

本文档对所有 27 个 skill 进行分类梳理，每个 skill 附简要说明并链接到完整文档及附属内容，帮助快速定位所需工具。

---

## 一、原子（Atoms）— 代码质量与工艺守卫

原子是基础的代码质量守卫，专注于单一关注点。它们在代码生成、审查时自动激活，对特定类型的代码强制执行规则。

### 1. architecture（架构）
> 在生成或修改代码时强制执行架构规则。默认使用整洁架构（层级、依赖反转、命令/查询分离），支持任意架构风格。验证层级职责、依赖方向和结构约束。

- 📄 [SKILL.md](./architecture/SKILL.md) — 主文档（配置解析、自检清单、反模式扫描）
- 📎 [clean-architecture-defaults.md](./architecture/references/clean-architecture-defaults.md) — 整洁架构默认原则（层级职责表、依赖方向、各层规则、命令/查询流程、违规示例）
- 📎 [clean-architecture.md](./architecture/references/clean-architecture.md) — 整洁架构执行规则（自检清单、反模式扫描、歧义信号、依赖规则）
- 📎 [custom-architecture.md](./architecture/references/custom-architecture.md) — 自定义架构执行规则（团队文档读取、清单回退、架构应用）

### 2. clean-code（整洁代码）
> 强制执行函数专注性、命名清晰性、复杂度管理、错误处理和自文档化风格。管理编写单个代码单元的工艺——不是架构，不是安全，也不是测试结构。

- 📄 [SKILL.md](./clean-code/SKILL.md) — 主文档（配置解析、自检清单、反模式扫描、歧义信号）
- 📎 [defaults.md](./clean-code/references/defaults.md) — 默认原则（单一职责、函数大小、圈复杂度、命名、参数设计、DRY、注释、错误处理、可测试代码）

### 3. domain-driven-design（领域驱动设计）
> 应用 DDD 战术模式——强制执行聚合设计、值对象优于基本类型、实体标识规则。在创建或修改领域模型、设计聚合时激活。

- 📄 [SKILL.md](./domain-driven-design/SKILL.md) — 主文档（配置解析、自检清单、反模式扫描、歧义信号、范围声明）
- 📎 [defaults.md](./domain-driven-design/references/defaults.md) — 默认原则（聚合设计规则、实体模式、值对象模式、领域服务、领域事件、仓库模式、创建模式、分解指南、验证清单）

### 4. secure-coding（安全编码）
> 强制执行安全意识——信任边界识别、输入验证、注入防护、密钥管理和纵深防御授权。在生成处理用户输入、数据库查询、外部 API 的代码时激活。

- 📄 [SKILL.md](./secure-coding/SKILL.md) — 主文档（配置解析、自检清单、反模式扫描、歧义信号）
- 📎 [defaults.md](./secure-coding/references/defaults.md) — 默认原则（信任边界识别、输入验证、参数化查询、输出编码、授权检查、密钥管理、注入防护）

### 5. test-quality（测试质量）
> 强制执行 Arrange-Act-Assert 结构、每个测试一个行为、断言质量、测试隔离和有意义的命名。管理编写测试用例的工艺。

- 📄 [SKILL.md](./test-quality/SKILL.md) — 主文档（配置解析、自检清单、反模式扫描、类级审查、歧义信号）
- 📎 [defaults.md](./test-quality/references/defaults.md) — 默认原则（AAA 结构、单行为测试、断言模式、测试隔离、命名约定、测试数据构建器、测试金字塔、反模式目录）

---

## 二、精炼器（Refiners）— 标准定义工具

精炼器通过结构化对话为项目定义标准文档，供对应的原子消费。每个精炼器产出 `.lattice/standards/` 下的正式文档。

### 6. architecture-refiner（架构精炼器）
> 引导对话定义项目架构原则。支持多种风格：整洁架构（默认）、六边形/端口与适配器、模块化单体或自定义。产出 `architecture.md`。

- 📄 [SKILL.md](./architecture-refiner/SKILL.md) — 主文档（风格选择、模式选择、引导方法、输出组装、质量检查）
- 📎 [template-clean-arch.md](./architecture-refiner/assets/template-clean-arch.md) — 整洁架构模板（6 章节：层级职责、依赖方向、各层规则、流程、违规示例、验证清单）
- 📎 [template-generic.md](./architecture-refiner/assets/template-generic.md) — 通用架构模板（8 章节：层级定义、依赖规则、边界规则、各层规则、关键流程、验证清单、反模式、歧义信号）

### 7. clean-code-refiner（整洁代码精炼器）
> 引导对话定义整洁代码原则——函数大小阈值、复杂度限制、命名约定、错误处理策略等。产出 `clean-code.md`。

- 📄 [SKILL.md](./clean-code-refiner/SKILL.md) — 主文档（范围澄清、模式选择、引导方法、输出组装、质量检查）
- 📎 [template.md](./clean-code-refiner/assets/template.md) — 整洁代码模板（10 章节：单一职责、小而专注的函数、圈复杂度、有意义的命名、参数设计、DRY、注释、错误处理、可测试代码、验证清单）

### 8. ddd-refiner（DDD 精炼器）
> 引导对话定义 DDD 战术模式护栏——聚合设计规则、实体模式、值对象目录、领域事件约定。产出 `ddd-principles.md`。

- 📄 [SKILL.md](./ddd-refiner/SKILL.md) — 主文档（范围澄清、模式选择、引导方法、输出组装、质量检查）
- 📎 [template.md](./ddd-refiner/assets/template.md) — DDD 模板（9 章节：聚合设计规则、实体模式、值对象模式、领域服务规则、领域事件、仓库模式、创建模式、分解指南、验证清单）

### 9. knowledge-priming-refiner（知识预载精炼器）
> 引导对话创建项目知识库——技术栈、架构概述、目录布局、受信任来源和项目约定。产出 `knowledge-base.md`。回答"AI 需要了解这个项目的什么？"

- 📄 [SKILL.md](./knowledge-priming-refiner/SKILL.md) — 主文档（目的、范围边界、引导方法、5 章节介绍、质量检查）
- 📎 [template.md](./knowledge-priming-refiner/assets/template.md) — 知识库模板（5 章节：架构概述、技术栈与版本、精选知识来源、项目结构、项目约定）

### 10. language-idioms-refiner（语言惯用法精炼器）
> 引导对话定义语言特定的惯用法和模式——错误处理、类型系统、命名约定、测试模式、依赖管理。产出 `language-idioms.md`，被多个原子消费以将伪代码默认值适配到项目语言。

- 📄 [SKILL.md](./language-idioms-refiner/SKILL.md) — 主文档（6 核心章节、语言特定方案含 Go/Rust/Python/Java/TypeScript/C#）
- 📎 [template.md](./language-idioms-refiner/assets/template.md) — 语言惯用法模板（6 章节：错误处理、类型系统与对象模型、命名约定、测试模式、参数与函数设计、依赖管理）

### 11. requirement-forge-refiner（需求锻造精炼器）
> 引导对话定义需求标准——史诗和功能定义、场景结构、验收标准格式、优先级和状态工作流。产出 `requirement-standards.md`。

- 📄 [SKILL.md](./requirement-forge-refiner/SKILL.md) — 主文档（范围澄清、模式选择、引导方法、9 章节介绍、常见场景）
- 📎 [template.md](./requirement-forge-refiner/assets/template.md) — 需求标准模板（9 章节：史诗定义、功能定义、独立性规则、场景定义、验收标准格式、优先级、状态工作流、命名约定、实现切片）

### 12. review-refiner（审查精炼器）
> 引导对话自定义审查流程——原子加载规则、严重性分类、报告格式、范围规则和健康日志。产出 `review-standards.md`。

- 📄 [SKILL.md](./review-refiner/SKILL.md) — 主文档（范围澄清、7 章节介绍）
- 📎 [template.md](./review-refiner/assets/template.md) — 审查标准模板（7 章节：原子加载策略、严重性分类、报告偏好、范围规则、洞察捕获、健康日志、自定义审查维度）

---

## 三、分子（Molecules）— 工作流编排

分子将多个原子和精炼器组合成端到端工作流，覆盖完整的开发活动——从设计到实现到审查。

### 13. design-blueprint（设计蓝图）
> 运行完整的设计工作流——从建立上下文，经过 5 个递进设计层级（能力→组件→交互→契约→实现），到已批准的蓝图。处理新功能和恢复现有工作。

- 📄 [SKILL.md](./design-blueprint/SKILL.md) — 主文档（所需技能、建立上下文、遍历设计层级、最终确定蓝图）

### 14. code-forge（代码锻造）
> 从已批准的设计蓝图或口头需求生成实现代码。将上下文锚定、架构、整洁代码、DDD、安全性和测试质量组合成由内而外的实现工作流。

- 📄 [SKILL.md](./code-forge/SKILL.md) — 主文档（所需技能、建立实现上下文、规划实现顺序、逐组件实现、跨组件验证、丰富上下文）

### 15. bug-fix（Bug 修复）
> 调查、重现并安全地修复 bug，具备回归保护。采用先重现再修复的工作流——定位、添加回归测试、选择最小安全修复、验证无回归。

- 📄 [SKILL.md](./bug-fix/SKILL.md) — 主文档（所需技能、建立上下文、重现和定位、添加回归保护、选择最小修复、实施修复、验证、捕获根本原因）

### 16. refactor-safely（安全重构）
> 在不改变外部可观察行为的前提下安全地重组现有代码。以特征测试为先——定义保留边界、提出结构计划、添加特征保护、小步绿改重构。

- 📄 [SKILL.md](./refactor-safely/SKILL.md) — 主文档（所需技能、建立上下文、定义保留边界、提出结构计划、特征保护、选择策略、小步重构、验证、捕获决策）

### 17. review（代码审查）
> 执行结构化代码审查——根据变更内容有条件地加载相关原子，运行其自检清单和反模式扫描，生成按严重性排序的报告（critical/warning/suggestion）。

- 📄 [SKILL.md](./review/SKILL.md) — 主文档（所需技能、配置解析、识别差异、加载原子、运行验证、生成报告、收获经验）

### 18. requirement-forge（需求锻造）
> 通过协作式产品访谈生成结构化功能规范。扮演高级 PM 和业务分析师角色——挑战范围、在每个决策点提出选项。生成 `.lattice/requirements/` 中的史诗/功能层级结构。

- 📄 [SKILL.md](./requirement-forge/SKILL.md) — 主文档（所需技能、PM/BA 角色、标准和会话检查、接收、史诗定义、功能发现、功能规范、刷新生成视图、自主模式）
- 📎 [output-templates.md](./requirement-forge/references/output-templates.md) — 输出模板（顶点文件 index.md、史诗文件、功能文件的精确模板）

### 19. architecture-compass（架构指南针）
> 现有仓库的架构思考伙伴——扫描代码库，进行结构化访谈，就当前架构状态和推荐方向达成一致。产出可共享的洞察文档。不执行转换——它负责定向。

- 📄 [SKILL.md](./architecture-compass/SKILL.md) — 主文档（所需技能、加载上下文、静默扫描、四幕访谈、当前架构共识、推荐方向、差距评估、编写洞察文档）
- 📎 [interview-guide.md](./architecture-compass/references/interview-guide.md) — 访谈指南（四幕弧线、问题库、答案解读表、对话原则、红旗信号）

### 20. lattice-init（Lattice 初始化）
> Lattice 项目的引导式设置和升级检查体验——扫描仓库，检测现有配置，按优先级顺序建议精炼器和可用升级，创建或协调 `.lattice/` 配置。

- 📄 [SKILL.md](./lattice-init/SKILL.md) — 主文档（扫描项目、呈现发现、引导式设置、后续步骤）
- 📎 [requirements-migration.md](./lattice-init/references/requirements-migration.md) — 需求布局迁移指南（旧版 index.md → 分片布局的逐步迁移过程）

### 21. refiners-update（精炼器更新）
> 在重大变更后更新现有 Lattice 标准——`lattice-init` 的更新模式对应物。扫描标准文档，询问变更了什么，将每个受影响的标准路由到其精炼器的修订模式。

- 📄 [SKILL.md](./refiners-update/SKILL.md) — 主文档（所需技能、精炼器→标准映射表、扫描标准、捕获变更、映射影响、修订标准、摘要）

---

## 四、上下文与知识（Context & Knowledge）

这些 skill 管理跨会话的知识持久化、设计方法论和决策协议。

### 22. knowledge-priming（知识预载）
> 加载项目特定上下文——技术栈、架构概述、目录布局和约定——使所有技能在了解项目实际情况的基础上运行。

- 📄 [SKILL.md](./knowledge-priming/SKILL.md) — 主文档（配置解析、文档内容结构、范围边界）

### 23. context-anchoring（上下文锚定）
> 管理每个功能的活文档，跨 AI 会话捕获决策、约束和推理。处理创建新的上下文文档、加载已有文档以及用新决策丰富它们。

- 📄 [SKILL.md](./context-anchoring/SKILL.md) — 主文档（范围、配置解析、文档生命周期、创建/加载/丰富行为、文档发现、输出格式）
- 📎 [feature-doc-template.md](./context-anchoring/assets/feature-doc-template.md) — 功能文档模板（前置元数据、决策日志、未决问题、约束、关键文件）

### 24. design-first（设计优先）
> 在编写任何代码之前，通过 5 个递进层级（能力→组件→交互→契约→实现）引导结构化设计思维。零实现规则——设计未达成一致前无代码。

- 📄 [SKILL.md](./design-first/SKILL.md) — 主文档（5 层级详解、零实现规则、复杂度校准、入口评估、层级完成协议、简单性检查、反模式）
- 📎 [methodology-detail.md](./design-first/references/methodology-detail.md) — 方法论详解（各层级好/差输出示例、顺序图符号、接口定义模式）

### 25. learning-harvest（经验收获）
> 管理操作经验的生命周期——加载先前经验指导当前工作，收获值得保留的新模式，随时间保持文档精简。从实践中积累可操作模式，补充标准和默认值。

- 📄 [SKILL.md](./learning-harvest/SKILL.md) — 主文档（范围边界、文档结构、加载/收获/精简行为、自检清单、独立调用）

### 26. collaborative-judgment（协作判断）
> 处理模糊决策和缺失/冲突知识的协议。确保 AI 用结构化选项呈现真正的判断决策，并在存在幻觉风险时停止——而非默默假设。

- 📄 [SKILL.md](./collaborative-judgment/SKILL.md) — 主文档（何时决策 vs 何时询问、呈现格式、批处理、解决方案、递减规则）

### 27. requirement-quality（需求质量）
> 在生成或验证功能规范时应用需求质量原则。强制执行功能完整性、场景结构、验收标准可验证性、功能独立性和实现切片质量。

- 📄 [SKILL.md](./requirement-quality/SKILL.md) — 主文档（配置解析、自检清单、主动反模式扫描、歧义信号）
- 📎 [defaults.md](./requirement-quality/references/defaults.md) — 默认标准（史诗定义、功能定义、独立性规则、场景定义、验收标准格式、优先级、状态工作流、命名约定、实现切片）

---

## 快速查找表

| 场景 | 使用 Skill | 直达链接 |
|------|-----------|----------|
| 新项目初始化 | `lattice-init` | [SKILL.md](./lattice-init/SKILL.md) |
| 定义架构标准 | `architecture-refiner` | [SKILL.md](./architecture-refiner/SKILL.md) |
| 定义编码标准 | `clean-code-refiner` | [SKILL.md](./clean-code-refiner/SKILL.md) |
| 定义领域规则 | `ddd-refiner` | [SKILL.md](./ddd-refiner/SKILL.md) |
| 定义语言惯用法 | `language-idioms-refiner` | [SKILL.md](./language-idioms-refiner/SKILL.md) |
| 创建项目知识库 | `knowledge-priming-refiner` | [SKILL.md](./knowledge-priming-refiner/SKILL.md) |
| 定义需求规范标准 | `requirement-forge-refiner` | [SKILL.md](./requirement-forge-refiner/SKILL.md) |
| 自定义审查流程 | `review-refiner` | [SKILL.md](./review-refiner/SKILL.md) |
| 评审现有代码库架构 | `architecture-compass` | [SKILL.md](./architecture-compass/SKILL.md) |
| 编写功能需求 | `requirement-forge` | [SKILL.md](./requirement-forge/SKILL.md) |
| 设计功能架构 | `design-blueprint` | [SKILL.md](./design-blueprint/SKILL.md) |
| 从设计生成代码 | `code-forge` | [SKILL.md](./code-forge/SKILL.md) |
| 修复 Bug | `bug-fix` | [SKILL.md](./bug-fix/SKILL.md) |
| 重构代码 | `refactor-safely` | [SKILL.md](./refactor-safely/SKILL.md) |
| 审查代码 | `review` | [SKILL.md](./review/SKILL.md) |
| 更新已有标准 | `refiners-update` | [SKILL.md](./refiners-update/SKILL.md) |

---

> **架构说明**：Lattice 的三层设计——**原子**（单一关注点的质量守卫）、**精炼器**（生成标准文档的结构化访谈）、**分子**（编排多个原子完成完整工作流）。上下文 skill（`knowledge-priming`、`context-anchoring`、`design-first`、`learning-harvest`、`collaborative-judgment`）贯穿各层提供支撑。
