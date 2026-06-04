---
language: {语言}
version: "{版本}"
---

<!-- 这是 language-idioms-refiner 输出的模板。
     标记为 <!-- INTERVIEW GUIDANCE: --> 的部分包含 refiner 访谈的说明。
     从最终输出中剥离所有指导注释。

     重要：下面的 6 个核心部分标题是**稳定契约**。
     多个 atoms 按精确名称引用这些标题。不要重命名它们。
     可以在核心 6 个部分之后添加额外部分。 -->

# Language Idioms: {语言}

<!-- INTERVIEW GUIDANCE:
将 {Language} 替换为检测到的或确认的语言名称（例如 "Go"、"Rust"、"Python"）。
将 frontmatter 中的 {language} 和 {version} 替换为小写标识符和版本字符串。 -->

## Error Handling（错误处理）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- clean-code atom §8（错误处理）——将 throw/catch 伪代码适配到语言习惯用法
- secure-coding atom §1（信任边界识别）——适配边界的错误消息模式

基于检测到的语言提出（参见 SKILL.md 中的语言特定提案）。
展示："对于 [语言]，习惯性的错误处理方法是 [提案]。这匹配你们团队的使用方式吗？"

如果用户想要调整，追问问题：
- "你的团队使用特定的错误库吗？（例如 Go：pkg/errors vs fmt.Errorf，Rust：thiserror vs anyhow）"
- "你如何区分可恢复和不可恢复的错误？"
- "是否有任何项目特定的错误类型或错误包装约定？"

记录：
- 错误哲学（一句话：异常、错误返回、Result 类型等）
- 错误创建模式（如何构造错误）
- 错误传播模式（错误如何在调用栈中向上流动）
- 任何显著的习惯用法或库

目标：4-8 行。无代码示例——atoms 使用这些描述适配它们自己的伪代码。
-->

{语言} 使用 {错误哲学}。{错误创建模式}。{错误传播模式}。

{其他错误习惯用法或库选择（如果有）。}

## Type System & Object Model（类型系统与对象模型）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- clean-code atom §1（单一职责）——适配类/结构体内聚性指导
- domain-driven-design atom ——适配 entity、value object 和 aggregate 实现模式

基于检测到的语言提出。
展示："对于 [语言]，类型系统使用 [提案]。这匹配吗？"

如果用户想要调整，追问问题：
- "你使用语言的全类型系统吗？（例如 Go：使用泛型吗？TypeScript：严格模式？）"
- "Classes、structs、两者都用？你如何建模 domain 对象？"
- "继承还是组合？你的团队如何实现多态？"
- "你的语言如何处理 null/缺失？（null、Option、Maybe、nil）"

记录：
- 主要类型构造（classes、structs、data classes、records 等）
- 接口机制（名义的、结构化的、隐式的、traits、protocols）
- 组合模型（继承、嵌入、mixins、traits）
- Null 处理习惯用法

目标：4-8 行。
-->

{主要类型构造}。{接口机制}。{组合模型}。{Null 处理}。

## Naming Conventions（命名约定）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- clean-code atom §4（有意义的命名）——将命名模式表适配到语言约定

基于检测到的语言提出。
展示："对于 [语言]，标准命名约定是 [提案]。是否有任何团队特定的偏差？"

如果用户想要调整，追问问题：
- "除了语言标准之外，你的团队认为可接受的缩写有哪些？（例如 ctx、req、resp）"
- "包/模块命名约定？"
- "你严格遵循语言社区标准还是有团队变体？"

记录：
- 大小写约定（snake_case、camelCase、PascalCase——用于什么）
- 可见性标记（大小写、下划线、关键字）
- 缩写风格
- 包/模块命名

目标：4-6 行。简短——每个语言的命名约定都有很好的文档；仅捕获 atom 需要的内容。
-->

{大小写约定}。{可见性标记}。{缩写风格}。{包/模块命名}。

## Testing Patterns（测试模式）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- test-quality atom §5（测试命名约定）——将命名示例适配到语言测试框架
- test-quality atom §4（测试隔离技术）——将隔离模式适配到语言习惯用法
- test-quality atom §6（测试数据 Builders 和 Factories）——将 builder 模式适配到语言构造

基于检测到的语言提出。
展示："对于 [语言]，测试方法通常是 [提案]。这是你们团队使用的吗？"

如果用户想要调整，追问问题：
- "哪个测试框架？（例如 Go：stdlib、Rust：内置、Python：pytest vs unittest、Java：JUnit 5 vs TestNG）"
- "测试文件组织？内联还是单独的 test 目录？"
- "Mocking 方法？（例如 Go：interfaces、Python：unittest.mock、Java：Mockito）"
- "断言库？（例如 AssertJ、FluentAssertions、testify）"
- "数据驱动/参数化测试？表格驱动？"

记录：
- 测试框架和断言方法
- 测试文件组织和命名
- Mocking/stubbing 习惯用法
- 数据驱动测试模式

目标：4-8 行。
-->

{测试框架和断言}。{测试文件组织}。{Mocking 习惯用法}。{数据驱动模式}。

## Parameter & Function Design（参数与函数设计）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- clean-code atom §2（小而专注的函数）——将函数设计适配到语言功能
- clean-code atom §5（参数设计）——将分组和 options 模式适配到语言习惯用法

基于检测到的语言提出。
展示："对于 [语言]，函数和参数设计通常遵循 [提案]。这匹配吗？"

如果用户想要调整，追问问题：
- "你的语言支持命名/关键字参数吗？你使用它们吗？"
- "多返回值？你如何处理它们？"
- "Options/config 模式？（例如 Go：functional options、Python：**kwargs、Java：builders）"
- "方法重载可用且被使用？"

记录：
- 参数传递习惯用法（位置、命名、解构等）
- 复杂输入的 options/config 模式
- 多返回值或输出参数
- 方法重载/默认参数

目标：4-6 行。
-->

{参数传递习惯用法}。{Options/config 模式}。{多返回值}。{重载/默认值}。

## Dependency Management（依赖管理）

<!-- INTERVIEW GUIDANCE:
此部分被以下模块消费：
- clean-code atom §9（测试友好代码）——适配 DI 和可测试性模式
- architecture atom ——将依赖方向执行适配到语言习惯用法

基于检测到的语言提出。
展示："对于 [语言]，依赖注入通常通过 [提案] 处理。这匹配你们团队的使用方式吗？"

如果用户想要调整，追问问题：
- "DI 容器还是手动 wiring？哪一个？"
- "你在哪里定义接口——在提供者处还是在消费者处？"
- "你如何在入口点 wiring 依赖？（main 函数、composition root、框架配置）"
- "影响依赖结构的导入/模块约定？"

记录：
- DI 方法（容器、构造函数注入、函数参数等）
- 接口放置（提供者端 vs 消费者端）
- Wiring 位置（main、composition root、框架配置）
- 显著约定

目标：4-6 行。
-->

{DI 方法}。{接口放置}。{Wiring 位置}。{显著约定}。

<!-- INTERVIEW GUIDANCE:
在所有 6 个核心部分之后，询问：
"我应该添加任何上面未涵盖的语言特定模式吗？例如：
- 并发模式（goroutines、async/await、threads）
- 内存管理（所有权、GC 调优、pool 模式）
- 模块/包组织
- 框架特定习惯用法"

如果用户添加部分，使用 ## 标题格式并保持简洁（每个部分 4-8 行）。
这些额外部分默认不被 atoms 引用，但在与核心部分一起加载时提供有用的上下文。

记住：从最终输出中剥离所有 <!-- INTERVIEW GUIDANCE: --> 注释。
产出的文档应该是干净、精简的规范（~40-60 行）。 -->
