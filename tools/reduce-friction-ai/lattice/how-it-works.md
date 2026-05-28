# Lattice 的工作原理
本文档解释了 Lattice 背后的可组合性模型 —— 为什么分为三个层级、每个层级如何工作，以及它们如何协同运作。

## 可组合性模型

Lattice 解决三个不同的问题，每个问题对应一个层级：

1. **原子 (Atoms)** 解决护栏问题：如何确保生成的代码遵循某个具体原则（整洁代码、DDD、安全性），确保 AI 不会在半路忘记？

2. **分子 (Molecules)** 解决编排问题：如何在正确的时机运行一个应用了合适护栏的多步骤工作流（设计 → 实现 → 审查）？

3. **精炼 (Refiners)** 解决定制化问题（可选）：如何根据你项目的特定标准，来调整原子技能的默认设置或分子技能的行为，而无需修改技能本身的源码？

每个层级都构建在下一级之上。
分子技能组合原子技能。
精炼技能可选择性地配置原子技能或分子技能的行为 —— 即使没有精炼技能，开箱即用的技能也能正常工作。
这种分层意味着原子技能保持通用性和可复用性，分子技能专注于工作流，而项目特定的决策则存放在配置文件中 —— 而不是硬编码在技能定义里。

## 两个层级

上述三个层级只是 Lattice 的一半 —— 即 **基础框架 (base framework)** 。
原子技能、分子技能和精炼技能是静态的、可组合的工程技能。
它们随框架一起提供，编码了各种原则和工作流，在每个项目上都以相同的方式运行。
基础框架是骨架 —— 结构正确、可移植且稳定。

<ins>另一半是 **动态上下文层 (living context layer)** ：即 `.lattice/` 文件夹。
由精炼技能产生的标准、功能上下文文档、积累的审查见解以及健康日志 —— 所有这些都与具体项目相关，并在每个功能周期中不断增长</ins>。
动态上下文层是肌肉 —— 它随着使用而增强，适应你所做的工作，并使基础框架的能力越来越强大。

<ins>这两个层级通过一个读/写循环进行交互。
基础框架从上下文层 *读取*</ins>：原子技能加载项目特定的标准；
code-forge、refactor-safely 和 bug-fix 加载过去的经验教训；
knowledge-priming 加载项目的标识。
<ins>流水线 (pipline) 向上下文层 *写入*</ins>：精炼技能生成标准文档；
design-blueprint 和 code-forge 创建并丰富上下文文档；
refactor-safely 记录已批准的结构性决策；
bug-fix 记录根本原因及修复决策；
review 捕获见解并记录健康摘要。
每个周期 (cycle) 都会为下一个周期积累更多价值。

这种收益会随着时间的推移不断累积。
经过几个功能周期后，原子技能应用的不再是通用规则，而是 *你的* 规则，并结合了 *你的* 审查历史所提供的信息。
code-forge 不会重复哪些审查已经发现过的错误。
随着精炼技能的重新运行，标准会变得更加精准。
健康日志展示的是各个功能之间的趋势，而不仅仅是快照。
基础框架从未改变，但上下文层让它在每一次使用中变得更加智能。

## 技能清单

所有技能及其调用命令。
在 AI 工具的聊天界面中输入命令即可调用任意技能。

### 原子技能 —— 直接调用或由分子技能组合调用

| 技能 | 命令 | 所强化的内容 |
|------|------|----------|
| clean-code | `/clean-code` | 函数聚焦、命名清晰度、复杂度管理、错误处理、自文档化风格 |
| architecture | `/architecture` | 层职责、依赖方向、结构规则。默认为整洁架构；通过 architecture-refiner 支持任意风格 |
| domain-driven-design | `/domain-driven-design` | 聚合设计、用值对象替代基本类型、实体标识规则、限界上下文边界 |
| secure-coding | `/secure-coding` | 信任边界意识、输入验证、注入防护、密钥管理 |
| test-quality | `/test-quality` | AAA (Arrange-Act-Assert) 结构、每个测试只测一个行为、断言质量、测试隔离、命名有意义 |
| knowledge-priming | `/knowledge-priming` | 加载项目特定的上下文（技术栈、架构、约定），使所有技能具备项目感知能力 |
| design-first | `/design-first` | 在编写任何代码之前，通过 5 个渐进层级进行结构化设计 |
| context-anchoring | `/context-anchoring` | 每个功能一个的动态文档，跨会话捕获决策与推理过程 |
| collaborative-judgment | `/collaborative-judgment` | 暴露真正的判断性决策或缺失/冲突的知识，而不是默默假设 |
| learning-harvest | `/learning-harvest` | 管理操作性经验的完整生命周期 —— 加载既有模式、收获新的实践见解、保持文档精简 |
| requirement-quality | `/requirement-quality` | 功能规格说明质量 —— 完整性、场景结构、验收条件可验证性、独立性、实现切片质量 |

### 分子技能 —— 调用以运行完整工作流

| 技能 | 命令 | 功能描述 |
|------|------|----------|
| lattice-init | `/lattice-init` | 引导式设置 —— 扫描项目、检测配置、建议精炼技能、创建 `.lattice/config.yaml` |
| requirement-forge | `/requirement-forge` | 作为资深产品经理+业务分析师进行协作式功能规格定义 —— 在 `.lattice/requirements/` 中生成 epic/功能层级结构，直接作为 design-blueprint 的输入 |
| design-blueprint | `/design-blueprint` | 通过 5 个层级的完整设计工作流，在编写任何代码前生成经批准的设计蓝图 |
| code-forge | `/code-forge` | 根据经批准的设计蓝图或口头需求，采用由内而外的分层顺序进行实现 |
| refactor-safely | `/refactor-safely` | 在不改变可观测行为的前提下重构现有代码；使用特征测试作为安全网 |
| bug-fix | `/bug-fix` | 调查问题、通过失败测试复现、然后进行最小化的安全修复 |
| review | `/review` | 结构化的增量代码审查，按严重程度排序输出发现结果；捕获经验教训供后续会话使用 |
| architecture-compass | `/architecture-compass` | 针对现有代码库的架构思考伙伴 —— 扫描代码库、进行引导式访谈、就当前状态和推荐方向达成共识、生成见解文档 |

### 精炼技能 —— 调用以生成项目特定标准

| 技能 | 命令 | 产出 |
|------|------|----------|
| knowledge-priming-refiner | `/knowledge-priming-refiner` | `.lattice/standards/knowledge-base.md` |
| language-idioms-refiner | `/language-idioms-refiner` | `.lattice/standards/language-idioms.md` |
| architecture-refiner | `/architecture-refiner` | `.lattice/standards/architecture.md` |
| ddd-refiner | `/ddd-refiner` | `.lattice/standards/ddd-principles.md` |
| clean-code-refiner | `/clean-code-refiner` | `.lattice/standards/clean-code.md` |
| review-refiner | `/review-refiner` | `.lattice/standards/review-standards.md` |
| requirement-forge-refiner | `/requirement-forge-refiner` | `.lattice/standards/requirement-standards.md` |

---
## 原子技能详解

### 它们是什么

<ins>每个原子技能都是一个单一关注点的技能文件，用于传授一项工程原则</ins>。
它包含该原则的规则、一个自我验证清单（采用命令式的 “停-并-验证 (STOP-and-verify)” 语言）、一个主动反模式扫描（复选框格式）以及一个配置解析机制。
<ins>原子技能不产生产出物 —— 它们在生成后验证阶段应用其检查项，就像一位有经验的开发者在提交代码之前会自行审查自己的代码一样</ins>。

### 它们如何工作

当一个原子技能被激活时，它提供两种验证工具：一个 **自我验证清单 (Self-Validation Checklist)**（带有编号、标签化检查项，采用命令式的 “停” 语言）和一个 **主动反模式扫描 (Active Anti-Pattern Scan)**（复选框格式，用于扫描输出结果）。
这些工具被分子技能（如 code-forge、refactor-safely 和 bug-fix）在其验证环节中使用 —— 在生成、重构或修复代码之后，AI 会针对其输出运行相关的原子技能清单，并在呈现结果之前修复违反项。
这种两遍 (two-pass) 模型（先生成，再验证）比同时进行生成与验证更加可靠。

### 始终应用 vs 条件应用原子技能

并非每个原子技能都适用于每一段代码。
这种区分对于独立使用和分子技能组合都至关重要：

**始终应用：**

- **clean-code** —— 每段代码都能从单一职责原则 (Single Responsibility Principle)、清晰的命名、可控的复杂度以及恰当的错误处理中受益。
- **architecture** —— 默认为整洁架构（分层、依赖方向），但也支持你文档化的任意架构风格。结构规则普遍适用。
- **knowledge-priming** —— 项目上下文（技术栈、架构、约定）始终相关。没有它，AI 会退回到通用的默认假设。
- **learning-harvest** —— 来自过往会话的操作性经验为当前工作提供信息；新的实践模式在会话结束时被提出，供用户挑选。
- **collaborative-judgment** —— 真正的判断性决策和缺乏依据的不确定性应该被暴露出来，而不是默默解决。由分子技能与其他原子技能组合使用。

**条件应用：**

- **domain-driven-design** —— 仅在触及领域层代码时应用。控制器或基础设施适配器不需要聚合边界检查。
- **secure-coding** —— 仅在代码跨越信任边界时应用：HTTP 处理器、数据库查询、外部 API 调用、文件 I/O、用户输入处理。
- **test-quality** —— 仅在编写测试代码时应用。AAA (Arrange-Act-Assert) 结构和测试隔离不适用于生产代码。
- **requirement-quality** —— 仅当编写或验证功能规格说明时应用。由 `requirement-forge` 组合使用；也可以独立调用来验证手写的规格说明。

### 特殊的原子技能

有 5 个原子技能的用途与 code-quality 类原子不同：

- **knowledge-priming** 是一个上下文原子。
它加载项目的标识 ——技术栈、架构概览、目录布局、可信来源以及约定—— 使所有其他技能都能在了解项目实际状况的前提下运行。
没有它，AI 会退回到 “互联网的平均水平”。
与质量类原子技能不同，它没有内置的默认值 —— 每个项目的标识都是独一无二的。
知识库文档由 `knowledge-priming-refiner` 创建或手动编写。

- **design-first** 是一个方法论原子技能，而不是代码质量原子技能。
它通过 5 个渐进层级（能力 → 组件 → 交互 → 契约 → 实现）在编写任何代码之前引导结构化思考。
它防止 AI 直接跳入实现阶段。

- **context-anchoring** 是一个持久化机制。
它管理每个功能的动态文档，跨会话捕获决策、约束和推理过程。
它解决了 AI 上下文衰减的问题 —— 到了第 30 多条消息之后，早期决策会自相矛盾，除非它们被写下来。

- **learning-harvest** 是一个经验性知识机制。
它管理操作性经验的完整生命周期 —— 一份横切的文档，记录在工作过程中发现的模式。
与标准（预先定义的规则）不同，操作性经验捕获的是团队在实践中反复付出代价才学到的东西，或者那些不断被证明有效的方法。
AI 提出建议；用户确认哪些内容进入文档。
它用项目级的经验模式来补充 context-anchoring（每个功能的决策）。

- **collaborative-judgment** 是一个歧义处理协议。
它确保 AI 以结构化选项的形式暴露真正的判断性决策，并在遇到缺失或冲突的依据时停下来，而不是默默假设。
每个 code-quality 原子都定义了自己的歧义信号（特定领域的灰色地带）；
该原子技能定义了如何呈现、批处理、澄清和解决这些信号。
随着项目标准变得越来越具体，它的活跃度会降低。
完整的设计原理请参见 [协同判断](collaborative-judgment.md) 。


### 配置解析

每个 code-quality 原子技能都通过相同的解析机制支持项目特定的定制化：

1. 在代码库根目录下查找 `.lattice/config.yaml`

2. 检查该原子的配置键（例如 `paths.clean_code`、`paths.architecture`）

3. 如果该路径下存在自定义文档，检查其 YAML 前置元数据中的 `mode`：
   - **`mode: overlay`（默认值，推荐）**：首先读取原子技能的内置默认值，然后在之上应用自定义文档的各个章节。
   章节通过标题进行匹配 —— 自定义章节替换匹配的默认章节，新增章节会被追加。
   你还可以添加默认值中完全不存在的全新章节（例如特定语言的惯用法、团队特定规则）。
   - **`mode: override`**：自定义文档完全替换原子技能的默认值。
   当你的标准与默认值根本不同并且你希望完全控制时使用此模式。

4. 如果不存在配置，则使用原子技能内置的 `./references/defaults.md`

5. **语言适配**：如果 `paths.language_idioms` 存在，原子会从语言惯用法文档中读取它所需的特定章节，并将其伪代码默认值适配到项目的语言。
每个原子声明它引用了哪些章节（例如，整洁代码读取 “错误处理”、“命名约定” 等）。
当语言惯用法与伪代码模式冲突时，语言惯用法优先。
有关 `language_idioms` 键的详细信息，请参见 [配置参考](configuration.md) 。

完整的解析顺序为：**默认值 → 语言惯用法（如果存在）→ 自定义覆盖（如果存在）**

原子开箱即用，带有预设的默认值。
定制化是可选的，不是必需的。
大多数团队使用覆盖模式 —— 默认值是很好的起点，通常只需要调整少数几个章节。

**两种定制化路径** ：运行精炼技能（通过引导式访谈生成标准文档）或直接编辑 `.lattice/standards/` 中的标准文档。
两者产生相同的结果：一个原子通过配置解析能够读取的文件。
当你的标准演进时，重新运行精炼技能或直接编辑文件即可。

完整的有效配置键列表及其功能说明，请参见 [配置参考](configuration.md) 。

## 分子技能详解

### 它们是什么

分子技能是编排好的多步骤工作流。
每个分子技能组合多个原子技能，在工作流的适当阶段使用它们。分子技能引用原子技能 —— 而不是重复原子技能的内容。

### 共享模式：生命周期分子技能

五个分子技能共享一个通用的会话基础设施：**design-blueprint**、**code-forge**、**bug-fix**、**refactor-safely** 和 **review**。

**始终组合**（每个会话）：knowledge-priming、context-anchoring、learning-harvest、collaborative-judgment

**会话生命周期**：
1. 会话开始时加载操作性经验（learning-harvest 加载）
2. 加载或创建功能的上下文（context-anchoring）
3. 执行分子技能特定的工作，同时按相关性应用质量原子技能
4. 用决策丰富上下文（context-anchoring 丰富）
5. 提议跨切面的经验教训 (learning)，供用户确认（learning-harvest 收获）

**质量原子技能**（根据会话所触及的内容有条件地加载）：architecture、clean-code、domain-driven-design、secure-coding、test-quality

以下各个分子技能的描述将只聚焦于它们各自区别于其他的特性。

---

### lattice-init

引导式设置 —— 弥合安装 Lattice 与获得首次价值之间的差距。
每个项目运行一次。

**组合使用的原子技能**：knowledge-priming

扫描项目，呈现设置状态，按优先级顺序建议精炼技能（knowledge-priming 优先），然后展示从设计到审查的工作流。

### requirement-forge

作为资深产品经理+业务分析师的双重角色，进行协作式功能规格定义。
流水线中的上游环节 —— 产出的规格说明供 `design-blueprint` 使用。

**组合使用的原子技能**：requirement-quality（始终）、knowledge-priming（条件应用）、collaborative-judgment（始终）

**两种模式**：collaborative（每个阶段设有确认关口）或 autonomous（静默起草全部内容，然后呈现）。

**工作流程**：标准检查 → 会话恢复 → 需求收集（现有材料或口头描述）→ epic 定义 → 每个 epic 的功能发现 → 每个功能的功能规格（框架在场景之前确认）→ 写入索引文件 (apex index)。
产出：`.lattice/requirements/` 文件夹，直接供 design-blueprint 使用。

### design-blueprint

在编写任何代码之前，产出经过批准的设计蓝图。

**独有原子技能**：design-first、architecture、domain-driven-design

**区别特征**：依次驱动 design-first 的 5 个渐进层级。
在第 2 至第 4 层级，应用 architecture（层职责分配、依赖方向）和 DDD（聚合识别、实体/值对象分类）。
每个层级在用户批准后持久化到上下文文档中 —— 该上下文文档 *即为* 设计蓝图。
止步于第 4 层（契约），交由 code-forge 处理。

### code-forge

根据批准的设计蓝图或口头需求生成实现代码。

**独有原子技能**：architecture（始终）、clean-code（始终）、domain-driven-design（条件应用：领域层）、secure-coding（条件应用：信任边界）、test-quality（编写测试时始终应用）

**区别特征**：
- 规划由内而外的构建顺序（领域层 → 基础设施层 → 应用层 → 接口层）
- 按组件同时生成代码和测试
- 生成后验证：运行原子技能自我验证清单和反模式扫描，在呈现之前修复违反项
- 跨组件验证：架构一致性、依赖方向、无计划外范围、过去的经验教训不重复出现
- 用户选择审查模式：逐层审查（推荐）、完全自主、或逐组件审查
- 完成后建议运行 `/review`

### bug-fix

调查、复现并安全修复缺陷，提供回归防护。从错误行为出发，而不是从新需求开始。

**独有原子技能**：clean-code（始终）、test-quality（始终）、architecture/DDD/secure-coding（根据根因有条件应用）

**区别特征**：
- 修复前需要一个可复现的失败（最好使用自动化测试）
- 在编辑前定位根因 —— 分类源层，仅加载相关的原子技能
- 捕获该缺陷的最小化失败测试 —— 这是本工作流的主要差异化特征
- 最小化安全修复，不造成架构倒退
- 验证：回归测试通过、保留基线完整、无副作用
- 对于较大的修复，建议运行 `/review`

### refactor-safely

在不改变外部可观测行为的前提下重构现有代码。
从结构痛点出发，要求在对目标结构达成一致之后才能进行任何重构编辑。

**独有原子技能**：clean-code（始终）、test-quality（始终）、design-first（条件应用：重大结构变更）、architecture/DDD/secure-coding（条件应用）

**区别特征**：
- 零重构规则：在用户批准目标结构之前，不进行任何结构性代码更改
- 首先用特征测试锁定当前行为 —— 这是本工作流的主要差异化特征
- 对于重大重构，在 design-first 的第 2-4 层级有选择地使用
- 以小规模、保持绿灯的步骤进行重构：每个切片都保持特征测试通过
- 偏离规则：如果已批准的计划变得不安全，立即暂停
- 对于大规模重构，建议运行 `/review`

### architecture-compass

面向现有代码库的架构思考伙伴。
通过就当前状态和推荐方向达成一致来提供定位 —— 不执行转换。

**组合使用的原子技能**：knowledge-priming（始终）、architecture（始终）、domain-driven-design（条件应用：仅战略层面）、collaborative-judgment（始终）

**范围限定于一个代码库、模块或文件夹。**

**工作流程**：
1. 从现有的 `.lattice/insights/architecture.md` 加载或恢复
2. 静默代码库扫描（15-25 次有针对性的读取，考古学扫描）
3. 四幕访谈：燃烧平台、历史、愿景、护栏
4. 当前架构确认（Mermaid 图表，明确确认关口）
5. 推荐方向（最小可行，明确确认关口，有效的停止点）
6. 差距评估和 2-3 个带成功标准的第一步行动
7. 写入 `.lattice/insights/architecture.md`

### review

结构化的、基于增量范围的代码审查。
根据变更内容有条件地加载质量原子技能。

**独有原子技能**：clean-code（始终）、architecture/DDD/secure-coding/test-quality（根据增量有条件应用）

**配置**：可选择读取 `.lattice/standards/review-standards.md`（来自 review-refiner），以定制原子技能加载规则、严重程度分类、报告格式、范围规则。
边界划分：*原子技能检查什么* → 该原子技能对应的精炼技能；*审查流程如何工作* → review-refiner。

**区别特征**：
- 增量范围：仅审查变更的内容，而非整个代码库
- 条件原子加载：分析增量所触及的层/领域/边界
- 每个原子技能的两遍验证：清单（硬性规则）然后反模式扫描（代码坏味）
- 按严重程度排序的报告（严重 → 警告 → 建议），附带具体修复建议
- 每次审查都以 “做得好的方面” 作为结尾
- 将结构化摘要记录到 `.lattice/reviews/review-log.md`，用于项目健康度趋势分析

有关每个精炼技能产出什么、以及它针对哪个原子技能或分子技能，请参见下方的 “精炼技能详解”。

## 精炼技能详解

精炼技能是可选的。
原子技能开箱即用，带有预设的默认值。
当你希望对你的项目调整默认值时，运行精炼技能。
每个精炼技能运行一个引导式访谈，并将标准文档写入 `.lattice/standards/` —— 原子技能在后续每次调用时通过配置解析读取该文档。

| 精炼技能 | 产出文件 | 使用者 |
|---------|----------|-------------|
| **knowledge-priming-refiner** | `.lattice/standards/knowledge-base.md` — 项目标识、技术栈、目录布局、可信来源、约定 | 所有原子技能和分子技能（通过 knowledge-priming 原子技能） |
| **language-idioms-refiner** | `.lattice/standards/language-idioms.md` — 特定语言的错误处理、类型系统、命名、测试、依赖注入模式 | clean-code、architecture、domain-driven-design、test-quality、secure-coding |
| **architecture-refiner** | `.lattice/standards/architecture.md` — 层结构与依赖规则。支持整洁架构（默认）、六边形架构、模块化单体、或任意自定义风格 | architecture 原子技能 |
| **ddd-refiner** | `.lattice/standards/ddd-principles.md` — 聚合设计、值对象规则、针对你领域定制的限界上下文约束 | domain-driven-design 原子技能 |
| **clean-code-refiner** | `.lattice/standards/clean-code.md` — 团队特定的编码标准、阈值和约定 | clean-code 原子技能 |
| **review-refiner** | `.lattice/standards/review-standards.md` — review 分子技能的原子加载规则、严重程度分类、报告格式、范围规则 | review 分子技能 |
| **requirement-forge-refiner** | `.lattice/standards/requirement-standards.md` — 针对团队产品流程定制的 epic/功能定义、场景结构、验收条件格式、优先级标记、状态工作流、命名约定 | `requirement-quality` 原子技能（通过配置解析）；该原子技能由 `requirement-forge` 分子技能组合使用 |

> **test-quality 和 secure-coding 没有对应的精炼技能** —— 这两个原子技能拥有强大的内置默认值，
适用于大多数团队。
如需定制，请手动编写 `.lattice/standards/test-quality.md` 或 `.lattice/standards/secure-coding.md`，并通过 `.lattice/config.yaml` 中的 `paths.test_quality` / `paths.secure_coding` 指向它们。

**生成标准文档的两条路径**：运行精炼技能（引导式访谈 → 自动为你创建文件）或直接在 `.lattice/standards/` 中编写文件。
两者产生的结果相同 —— 原子技能只关心文档是否存在，不关心它是如何创建的。
每当你的标准演进时，可以重新运行精炼技能或直接编辑文件。

关于将这些文档连接到对应原子技能的 `.lattice/config.yaml` 配置项的完整列表，请参见 [配置参考](configuration.md) 。

## 设计到代码的流水线

有两种常见的入口路径：

```
计划性功能开发（完整流水线）：
  lattice-init → requirement-forge → design-blueprint → code-forge → review

计划性功能开发（设计已经明确）：
  lattice-init → design-blueprint → code-forge → review

重构驱动的工作：
  refactor-safely → review

缺陷驱动的工作：
  bug-fix → review

架构方向（现有代码库）：
  architecture-compass → refactor-safely / design-blueprint / code-forge（按第一步行动选择）
```

功能开发从需求开始，在实现之前产出经过批准的设计蓝图。`
requirement-forge` 是可选的，但当功能范围或问题尚未完全明确时建议使用 —— 它产出的结构化功能规格说明可直接被 `design-blueprint` 消费。
重构工作从结构痛点开始，在代码重塑开始之前，产出批准的目标结构以及特征测试。
缺陷工作从失败行为开始，在修复之前产出可复现的失败用例。
所有路径最终汇聚到 `review`，进行独立的质量检查。

每个阶段既消费也产出 `.lattice/` 中的产物 —— 流水线正是驱动动态上下文层不断增长的引擎。
上下文锚定将这些阶段串联在一起：设计阶段创建的上下文文档，将批准的设计蓝图带入实现阶段，捕获批准的重构计划和缺陷根因，为审查提供信息，并在任何未来的会话中恢复完整的上下文。

上下文文档的生命周期是：**创建**（新功能）→ **加载**（恢复工作）→ **丰富**（捕获决策）。
所有三种行为都需要明确的用户确认 —— AI 提议，用户决断。

## `.lattice/` 文件夹

`.lattice/` 文件夹是前面所述的动态上下文层 —— 项目特有的 AI 记忆，随着每个功能周期不断增长。
框架产生的所有持久化产物都存放在这里，按不同的生命周期组织到子文件夹中。

### 目录结构

```
.lattice/
├── config.yaml              # 中心配置文件（根目录下唯一的文件）
├── standards/               # 精炼技能产出的定制化文档
│   ├── knowledge-base.md
│   ├── clean-code.md
│   ├── architecture.md
│   ├── ddd-principles.md
│   ├── review-standards.md
│   └── requirement-standards.md
├── requirements/            # requirement-forge 产出的功能规格说明
│   ├── index.md             # Epic/功能索引总表
│   └── features/
│       └── <feature>.md
├── context/                 # 每个功能的动态文档
│   └── <feature>.md
├── learnings/               # 操作性经验由 learning-harvest 原子技能管理
│   └── operational-learnings.md
├── reviews/                 # 项目健康度的审查日志
│   └── review-log.md
└── insights/                # architecture-compass 产出的架构见解
    └── architecture.md
```

### 子文件夹生命周期

| 子文件夹 | 用途 | 生命周期 |
|---------|------|----------|
| `standards/` | 精炼技能产出的定制化文档，由原子技能通过配置解析读取 | 稳定 —— 项目设置时一次性设定，极少更改 |
| `requirements/` | requirement-forge 产出的 epic/功能规格说明。`index.md` 为索引总表；`features/` 存放各功能文件 | 每周期 (cycle) —— 功能规格编写时创建，规格演进时更新。供 design-blueprint 使用 |
| `context/` | 由 context-anchoring 管理的每个功能的动态文档 | 每功能 —— 功能开始时创建，在设计和实现过程中丰富 |
| `learnings/` | 由 `learning-harvest` 原子技能管理的操作性经验 —— 从设计、实现、审查和修复会话中积累的模式。在会话开始时加载，在会话结束时收割。 | 仅追加，带自我调节的收紧机制 —— 当文档变得密集时，原子技能会建议合并整理 |
| `reviews/` | 审查日志条目，用于项目健康度可见性 | 滚动窗口 —— 上限约 20 条，旧条目会被汇总 |
| `insights/` | architecture-compass 产出的架构见解文档 | 每项目一份 —— 随方向演进更新 |

### 约定

**规则**：所有持久化产物均放入子文件夹。
除 `config.yaml` 外，不得将任何文件直接放在 `.lattice/` 根目录下。

这一约定确保随着框架增加新能力，文件夹保持井然有序。
每种新的输出类型都会获得自己的子文件夹，并拥有清晰的生命周期定义。

## 原子技能、分子技能与精炼技能的区别

| 维度 | 原子技能 | 分子技能 | 精炼技能 |
|------|---------|----------|----------|
| **用途** | 传授一项原则 | 编排一个工作流 | 可选地定制原子技能的默认值 |
| **调用方式** | 根据上下文自动激活，或被分子技能调用 | 用户显式调用（如 `/design-blueprint`） | 用户在有定制需求时调用（如 `/architecture-refiner`） |
| **产出的产物** | 无（内联检查） | 设计蓝图、审查报告、上下文文档 | `.lattice/` 配置文件 |
| **是否组合其他技能？** | 否 | 是（组合原子技能） | 否 |
| **是否可由精炼技能配置？** | 是（通过 `.lattice/` 配置文件） | review 分子技能支持通过 review-refiner 配置 | 不适用 |
| **使用频率** | 每次生成（自动） | 每个功能、缺陷或审查 | 根据需要 —— 当标准首次设定或演进时 |
| **是否必需？** | 是（核心护栏） | 否（但对于结构化工作流推荐使用） | 否（原子技能使用内置默认值即可工作） |
| **能否独立使用？** | 是 | 是 | 是 |
