# Skills 索引

本目录包含 Lattice 框架的所有技能（Skills），按功能分类为三类：**原子（Atoms）**、**分子（Molecules）** 和 **精炼器（Refiners）**。

---

## 原子（Atoms）

原子是基础检查/验证技能，直接应用于代码或规范的检查。它们是构成分子的基本单元。

### 1. architecture — 架构规则检查

在生成或修改代码时强制执行架构规则。默认为 clean architecture；通过 architecture-refiner 支持任何架构风格。使用加载的架构规则验证层职责、依赖方向和结构约束。

- [SKILL.md](architecture/SKILL.md) — 主技能文件
- [references/clean-architecture.md](architecture/references/clean-architecture.md) — Clean Architecture 默认规则
- [references/clean-architecture-defaults.md](architecture/references/clean-architecture-defaults.md) — Clean Architecture 默认配置
- [references/custom-architecture.md](architecture/references/custom-architecture.md) — 自定义架构规则

### 2. clean-code — 代码质量标准

在生成或修改实现代码时应用整洁代码原则。强制执行函数职责单一、命名清晰、复杂度管理、错误处理和自文档化风格。

- [SKILL.md](clean-code/SKILL.md) — 主技能文件
- [references/defaults.md](clean-code/references/defaults.md) — 整洁代码默认原则

### 3. collaborative-judgment — 协作式判断协议

处理代码生成、设计和审查过程中模糊决策和缺失/冲突知识的协议。确保 AI 以结构化选项呈现真正的判断调用，并在幻觉风险时停止而非静默假设。

- [SKILL.md](collaborative-judgment/SKILL.md) — 主技能文件

### 4. context-anchoring — 上下文锚定

管理每个功能的活文档，在活跃开发期间跨 AI 会话捕获决策、约束和推理。范围限定在功能级别——设计、实现、缺陷修复、重构。

- [SKILL.md](context-anchoring/SKILL.md) — 主技能文件
- [assets/feature-doc-template.md](context-anchoring/assets/feature-doc-template.md) — 功能文档模板

### 5. design-first — 优先设计

在编写任何代码之前，引导结构化设计思维，通过 5 个渐进式层级。层级：能力（Capabilities）、组件（Components）、交互（Interactions）、契约（Contracts）、实现（Implementation）。

- [SKILL.md](design-first/SKILL.md) — 主技能文件
- [references/methodology-detail.md](design-first/references/methodology-detail.md) — 方法论细节

### 6. domain-driven-design — 领域驱动设计

在处理领域代码时应用 DDD 战术模式。强制执行 aggregate 设计、value objects over primitives、entity 身份规则和 bounded context 边界。

- [SKILL.md](domain-driven-design/SKILL.md) — 主技能文件
- [references/defaults.md](domain-driven-design/references/defaults.md) — DDD 默认原则

### 7. knowledge-priming — 知识预热

加载项目特定的上下文——技术栈、架构概览、目录布局、可信来源和约定——以便所有 skills 在了解这个项目实际是什么的情况下运行。

- [SKILL.md](knowledge-priming/SKILL.md) — 主技能文件

### 8. learning-harvest — 学习收获

管理 operational learnings 生命周期——加载 prior learnings 以 inform current work、harvest new patterns worth preserving，并保持文档随时间推移保持精简。

- [SKILL.md](learning-harvest/SKILL.md) — 主技能文件

### 9. requirement-quality — 需求质量检查

在生成或验证 feature 规范时应用需求质量原则。强制执行 feature 完整性、场景结构、AC 可验证性、feature 独立性和实现切片质量。

- [SKILL.md](requirement-quality/SKILL.md) — 主技能文件
- [references/defaults.md](requirement-quality/references/defaults.md) — 需求质量默认原则

### 10. secure-coding — 安全编码

在生成或修改代码时应用安全意识的思考。强制执行信任边界意识、输入验证、注入防护、密钥管理和纵深授权。

- [SKILL.md](secure-coding/SKILL.md) — 主技能文件
- [references/defaults.md](secure-coding/references/defaults.md) — 安全编码默认原则

### 11. test-quality — 测试质量标准

在生成或审查测试代码时应用测试质量原则。强制执行 Arrange-Act-Assert 结构、每个测试一个行为、断言质量、测试隔离、有意义的命名和测试数据管理。

- [SKILL.md](test-quality/SKILL.md) — 主技能文件
- [references/defaults.md](test-quality/references/defaults.md) — 测试质量默认原则

---

## 分子（Molecules）

分子是组合多个原子/技能形成完整工作流的技能。它们编排原子的输出，提供端到端的解决方案。

### 1. architecture-compass — 架构指南针

现有代码库的架构思维伙伴——扫描代码库、进行结构化访谈、就当前架构状态和推荐方向达成一致，并生成可分享的洞察文档。范围限定在一个仓库、模块或文件夹内。不执行转换——它只进行定位。

- [SKILL.md](architecture-compass/SKILL.md) — 主技能文件
- [references/interview-guide.md](architecture-compass/references/interview-guide.md) — 访谈指南

### 2. bug-fix — Bug 修复

调查、复现并安全修复带有回归保护的 bug。组合 context（上下文）、diagnosis（诊断）、architecture（架构）、code quality（代码质量）和 testing guardrails（测试护栏）为 reproduce-first（复现优先）的修复工作流。

- [SKILL.md](bug-fix/SKILL.md) — 主技能文件

### 3. code-forge — 代码锻造

从批准的设计 blueprint 或口头需求生成实现代码。Compose context anchoring、architecture、clean code、DDD、security 和 test quality 为 inside-out implementation workflow。

- [SKILL.md](code-forge/SKILL.md) — 主技能文件

### 4. design-blueprint — 设计蓝图

运行完整的设计工作流——从建立上下文通过 5 个渐进式设计层级到批准的 blueprint。Compose context anchoring、design-first methodology、architecture 和 DDD 为统一流程。

- [SKILL.md](design-blueprint/SKILL.md) — 主技能文件

### 5. lattice-init — Lattice 初始化

为新 Lattice 项目提供引导式设置体验——扫描仓库、检测现有配置、按优先级建议运行 refiners，并创建 .lattice/ 配置。填补安装 skills 与获得首次价值之间的差距。

- [SKILL.md](lattice-init/SKILL.md) — 主技能文件

### 6. refactor-safely — 安全重构

安全地重组现有代码而不改变外部可观察行为。Compose context、design、architecture、code quality 和 testing guardrails 为 characterization-first refactoring workflow。

- [SKILL.md](refactor-safely/SKILL.md) — 主技能文件

### 7. requirement-forge — 需求锻造

通过协作式产品访谈生成结构化 feature 规范。作为高级 PM 和 business analyst pair——带着观点到达，挑战范围，在每个决策时提出选项。

- [SKILL.md](requirement-forge/SKILL.md) — 主技能文件
- [references/output-templates.md](requirement-forge/references/output-templates.md) — 输出模板

### 8. review — 代码审查

通过对相关原子（atoms）组合验证清单来执行结构化的代码审查。根据代码变更范围有条件地加载原子——始终加载 clean-code，仅在变更触及架构/领域驱动设计/安全/测试领域时加载 architecture/DDD/security/tests。

- [SKILL.md](review/SKILL.md) — 主技能文件

---

## 精炼器（Refiners）

精炼器通过结构化对话定义/配置技能。它们生成正式的文档，供对应的 atom 或 molecule 使用作为其流程配置。

### 1. architecture-refiner — 架构精炼器

通过结构化对话为仓库定义架构原则。支持多种架构风格：clean architecture（默认）、hexagonal / ports & adapters、modular monolith 或自定义。生成正式的架构文档，供对应的 atom 使用。

- [SKILL.md](architecture-refiner/SKILL.md) — 主技能文件
- [assets/template-generic.md](architecture-refiner/assets/template-generic.md) — 通用模板
- [assets/template-clean-arch.md](architecture-refiner/assets/template-clean-arch.md) — Clean Architecture 模板

### 2. clean-code-refiner — 代码质量精炼器

通过结构化对话为仓库定义代码质量原则。生成正式的 clean-code.md 文档，供 clean-code atom 用作其覆盖。

- [SKILL.md](clean-code-refiner/SKILL.md) — 主技能文件

### 3. ddd-refiner — DDD 精炼器

通过结构化对话为仓库中的领域设计定义 DDD 护栏。生成正式的 ddd-principles.md 文档，供 domain-driven-design atom 作为覆盖使用。

- [SKILL.md](ddd-refiner/SKILL.md) — 主技能文件
- [assets/template.md](ddd-refiner/assets/template.md) — DDD 模板

### 4. knowledge-priming-refiner — 知识预热精炼器

通过结构化对话创建项目特定的知识基线文档。生成 knowledge-base.md，为 AI 提供项目的技术栈、架构、可信来源和项目结构信息。

- [SKILL.md](knowledge-priming-refiner/SKILL.md) — 主技能文件
- [assets/template.md](knowledge-priming-refiner/assets/template.md) — 知识基线模板

### 5. language-idioms-refiner — 语言习惯精炼器

通过结构化对话为仓库定义语言特定的习惯用法和模式。生成 language-idioms.md 文档，被多个 atoms 消费以将伪代码默认值适配到项目的语言。

- [SKILL.md](language-idioms-refiner/SKILL.md) — 主技能文件
- [assets/template.md](language-idioms-refiner/assets/template.md) — 语言习惯模板

### 6. requirement-forge-refiner — 需求锻造精炼器

通过结构化对话为项目定义需求标准——epic 和 feature 定义、场景结构、AC 格式、优先级标记、状态工作流和命名约定。生成正式的 requirement-standards.md，供 requirement-quality atom 通过配置解析读取。

- [SKILL.md](requirement-forge-refiner/SKILL.md) — 主技能文件
- [assets/template.md](requirement-forge-refiner/assets/template.md) — 需求标准模板

### 7. review-refiner — 审查精炼器

促进结构化对话以自定义审查分子的工作方式——包括原子加载规则、严重程度分类、报告格式、范围规则、洞察捕获和健康日志。生成正式的 review-standards.md 文档，供审查分子用作其流程配置。

- [SKILL.md](review-refiner/SKILL.md) — 主技能文件
- [assets/template.md](review-refiner/assets/template.md) — 审查标准模板

---

## 统计

| 分类 | 数量 |
|------|------|
| 原子（Atoms） | 11 |
| 分子（Molecules） | 8 |
| 精炼器（Refiners） | 7 |
| **总计** | **26** |
