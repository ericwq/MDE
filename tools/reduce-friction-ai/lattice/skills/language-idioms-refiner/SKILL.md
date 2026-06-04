---
name: language-idioms-refiner
description: "通过结构化对话为仓库定义语言特定的习惯用法和模式。生成 language-idioms.md 文档，被多个 atoms 消费以将伪代码默认值适配到项目的语言。在设置新项目、切换语言时使用，或当用户说'setup language'、'define language idioms'、'configure language'、'language patterns'或'adapt for Go/Rust/Python'时使用。"
---

# Language Idioms Refiner（语言习惯用法定义器）

## 产出内容

- **输出**：`.lattice/standards/language-idioms.md`（或来自 `.lattice/config.yaml` -> `paths.language_idioms` 的自定义路径）
- **模式**：始终独立——atoms 中没有内置的语言默认值可以叠加。对于修订，加载现有文档并就地更新部分。
- **配置键**：
  - `language`（顶层）——语言标识符（例如 `go`、`rust`、`python`、`java`、`typescript`）
  - `paths.language_idioms`——产出文档的路径
- **模板**：读取 `./assets/template.md` 获取完整的文档结构、预填充示例和访谈指导注释
- **消费者**：多个 atoms 按标题名称引用特定部分：

| 部分 | 消费者 |
|---------|----------|
| **错误处理** | `clean-code` atom（§8）、`secure-coding` atom（§1 信任边界消息） |
| **类型系统与对象模型** | `clean-code` atom（§1 SRP/内聚性）、`domain-driven-design` atom（entities、VOs、aggregates） |
| **命名约定** | `clean-code` atom（§4） |
| **测试模式** | `test-quality` atom（§5 命名、§4 隔离、§6 builders） |
| **参数与函数设计** | `clean-code` atom（§2 函数大小、§5 参数） |
| **依赖管理** | `clean-code` atom（§9 可测试性/DI）、`architecture` atom（依赖方向） |

这六个部分标题是**稳定契约**。Atoms 按名称引用它们。消费者可以添加额外部分，但这六个必须存在。

## 范围澄清

此文档捕获**项目的语言如何表达工程模式**——atoms 需要适配其伪代码默认值的语言级习惯用法。清晰的边界：

| 关注点 | 归属位置 | 不在此处 |
|---------|----------|----------|
| 项目身份、技术栈、目录布局 | `knowledge-priming` atom | 无项目结构或框架文档 |
| 代码工艺规则（阈值、启发式） | `clean-code` atom / overlay | 无函数大小限制或 DRY 规则 |
| 架构层、依赖方向 | `architecture` atom / overlay | 无层定义 |
| 领域建模护栏 | `domain-driven-design` atom / overlay | 无 aggregate 规则 |
| 语言内的团队特定偏好 | Atom-specific overlays | 无团队决策（见下文） |

**与 atom overlays 的关键区别**：此文档描述*语言如何工作*。Atom overlays 描述*团队如何在语言内工作*。

- **语言习惯用法文档**："Go 使用显式错误返回（`if err != nil`），而非异常"
- **Clean-code overlay**："我们使用 `fmt.Errorf('context: %w', err)` 进行包装，自定义错误类型用于领域错误"

语言习惯用法是关于语言的**事实**。Atom overlays 是团队**选择**。

## 开始之前

### 检查现有文档

1. 读取 `.lattice/config.yaml` —— `paths.language_idioms` 是否指向某个文件？
2. 如果是，读取该文件。询问用户：
   - "你已经有 **[语言]** 的语言习惯用法文档了。你想**修订**它（更新特定部分）、**重新开始**（新访谈）还是**切换语言**？"
   - 修订：加载现有文档，仅 walkthrough 用户想要更改的部分。
   - 重新开始：继续下面的完整访谈流程。
   - 切换语言：为新语言进行完整访谈；现有文档将被替换。
3. 如果没有配置或没有现有文档，继续完整访谈流程。

### 检测语言

在开始访谈之前确定项目语言：

1. **从配置中**：检查 `.lattice/config.yaml` 中的 `language` 键。
2. **从项目文件**（如果没有配置键）：
   - `package.json` → TypeScript / JavaScript
   - `tsconfig.json` → TypeScript（确认优于 JavaScript）
   - `go.mod` → Go
   - `pom.xml` 或 `build.gradle` 或 `build.gradle.kts` → Java 或 Kotlin
   - `Cargo.toml` → Rust
   - `requirements.txt` 或 `pyproject.toml` 或 `setup.py` → Python
   - `Gemfile` → Ruby
   - `*.csproj` 或 `*.sln` → C# / .NET
   - `Package.swift` → Swift
3. **检测到多种语言**：询问用户哪个是主要语言。每个项目一个 language-idioms 文档（覆盖主要语言）。
4. **未检测到**：直接询问用户。

展示检测到的语言："我检测到这是一个 **Go** 项目（找到 `go.mod`）。我将为每个部分提出 Go 习惯用法模式。你可以确认或调整。"

## 访谈流程

此 refiner 与其他 refiners 的工作方式不同。不是展示默认值并询问"更改还是保留？"，而是**提出语言特定的内容**并询问"这匹配你们团队的使用方式吗？"

### 第 1 步：确认语言和版本

"我检测到 **[语言] [版本]**。这是正确的吗？"

记录语言和版本。这些放入文档 frontmatter。

### 第 2 步：通过提案 walkthrough 部分

对于 6 个核心部分中的每一个：

1. **提出**基于检测到的语言的预填充内容（参见下面的语言特定提案）。
2. **展示**提案："这是我推荐用于 [语言] 的内容。这匹配你们团队使用 [语言] 的方式吗？"
3. **用户确认** → 原样记录。
4. **用户调整** → 讨论具体内容，记录他们的版本。

### 第 3 步：额外部分

在 6 个核心部分之后，询问："我应该添加任何语言特定的模式吗？例如：并发模式、内存管理、async/await 习惯用法或框架特定约定。"

记录用户想要的任何额外部分。

### 第 4 步：生成文档

组装并写入文档。

## 分部分访谈指南

读取 `./assets/template.md`，并按照每个部分的 `<!-- INTERVIEW GUIDANCE: -->` 注释操作。

### 6 个核心部分

| # | 部分 | 捕获内容 |
|---|---------|----------|
| 1 | **错误处理** | 语言错误哲学（异常、错误返回、Result 类型）、错误传播模式、错误创建习惯用法 |
| 2 | **类型系统与对象模型** | 类 vs 结构体、接口（名义 vs 结构化）、继承 vs 组合、泛型、类型安全习惯用法 |
| 3 | **命名约定** | 大小写约定、可见性修饰符、缩写风格、包/模块命名、习惯用法模式 |
| 4 | **测试模式** | 测试框架习惯用法、测试组织、断言模式、mocking 方法、测试命名风格 |
| 5 | **参数与函数设计** | 参数传递习惯用法、options/config 模式、多返回值、命名参数、函数签名 |
| 6 | **依赖管理** | DI 方法（容器 vs 手动）、接口放置、wiring 模式、导入/模块约定 |

### 跨部分意识

| 决策于 | 影响 | 如何影响 |
|---------|------|----------|
| §1 错误处理 | §4 测试 | 错误模式决定如何测试错误路径 |
| §2 类型系统 | §5 参数、§6 依赖 | 对象模型塑造函数签名和 DI 方法 |
| §3 命名 | §4 测试 | 命名约定同样适用于测试名称 |

## 语言特定提案

对于知名语言，为每个部分预填充习惯用法默认值。访谈确认或调整这些值。对于未识别的语言，提出开放式问题。

### Go

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | 显式错误返回（`value, err := ...`）、`if err != nil`、使用 `fmt.Errorf("context: %w", err)` 包装错误、对于预期情况使用哨兵错误、无异常 |
| 类型系统 | 带方法的结构体（接收者函数）、隐式接口（结构化类型）、通过嵌入组合、无继承、无类 |
| 命名 | 导出 = 大写，未导出 = 小写、小作用域中使用短名称、缩写完全大写（`HTTP`、`ID`）、包名是标识符的一部分（`http.Client` 而非 `http.HTTPClient`） |
| 测试 | 表格驱动测试、`t.Run` 用于子测试、`testing.T` 参数、测试文件 `_test.go` 内联、无需断言库（stdlib 比较） |
| 参数 | 接受接口返回结构体、functional options 模式用于配置（`WithTimeout(5*time.Second)`）、多返回值、无方法重载 |
| 依赖 | 传递接口参数（而非构造函数 DI）、在消费者处定义接口而非提供者、无 DI 容器、在 `main()` 或 `cmd/` 中显式 wiring |

### Rust

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | `Result<T, E>` 用于可恢复、`panic!` 用于不可恢复、`?` 运算符用于传播、`thiserror` 用于库错误、`anyhow` 用于应用错误 |
| 类型系统 | Structs + impl blocks、traits（显式实现）、带数据的 enums（代数类型）、所有权/借用、无继承、无 null（使用 `Option<T>`） |
| 命名 | `snake_case` 函数/变量、`PascalCase` 类型/traits、`SCREAMING_SNAKE` 常量、生命周期名称短（`'a`、`'b`） |
| 测试 | `#[test]` 属性、`#[cfg(test)] mod tests` 在同一文件中、集成测试在 `tests/` 目录中、`assert_eq!`/`assert!` 宏 |
| 参数 | 所有权：借用（`&T`）vs move、泛型边界（`impl Trait`）、builder 模式用于复杂配置、无默认参数 |
| 依赖 | Trait objects（`dyn Trait`）或泛型（`impl Trait`）用于抽象、无 DI 容器、显式构造 |

### Python

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | EAFP 优于 LBYL（try/except，而非 if-checks）、上下文管理器（`with`）用于清理、自定义异常继承自基类、raise/except |
| 类型系统 | Classes、dataclasses（`@dataclass`）、protocols 用于结构化类型（PEP 544）、duck typing、鼓励 type hints 但在运行时可选 |
| 命名 | `snake_case` 函数/变量、`PascalCase` classes、`SCREAMING_SNAKE` 常量、`_private` 约定（单下划线）、`__dunder__` 用于魔术方法 |
| 测试 | 首选 pytest、fixtures 用于 setup/teardown、`@pytest.mark.parametrize` 用于数据驱动测试、plain `assert`（pytest 重写）、测试文件 `test_*.py` |
| 参数 | `**kwargs` 用于 options、命名/关键字参数、默认值、dataclass 或 TypedDict 用于配置对象 |
| 依赖 | 使用 protocols/ABCs 的构造函数注入，或函数参数、无重量级 DI 容器（或需要时使用 `dependency-injector`） |

### Java / Kotlin

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | **Java**：unchecked 异常优于 checked（现代风格），自定义异常扩展 `RuntimeException`。**Kotlin**：sealed class Result 模式、`runCatching`、无 checked 异常 |
| 类型系统 | **Java**：classes、interfaces、records（16+）、sealed classes（17+）。**Kotlin**：data classes、sealed hierarchies、null 安全（`?`）、extension functions |
| 命名 | `camelCase` variables/methods、`PascalCase` classes/interfaces、`SCREAMING_SNAKE` constants、packages `lowercase.dotted` |
| 测试 | JUnit 5、`@Test`、`@ParameterizedTest`、`@Nested` 用于分组、Mockito（Java）/ MockK（Kotlin）、AssertJ 用于流畅断言 |
| 参数 | **Java**：builder 模式用于 >3 个参数、方法重载。**Kotlin**：命名参数、默认值、data class config |
| 依赖 | 构造函数注入（Spring、Guice 或手动）、DI 容器是习惯用法、面向接口编程 |

### TypeScript

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | try/catch + 自定义 Error 子类、可选的 typed error handling（通过库使用 Result 模式）、无 checked 异常 |
| 类型系统 | Interfaces、type aliases、union/intersection types、generics、`unknown` 优于 `any`、discriminated unions 用于状态 |
| 命名 | `camelCase` variables/functions、`PascalCase` types/classes/enums、`SCREAMING_SNAKE` constants、接口无 `I` 前缀 |
| 测试 | Jest 或 Vitest、`describe`/`it` blocks、mock functions（`jest.fn()`）、`expect().toBe()` 断言、`.test.ts` 或 `.spec.ts` 内联 |
| 参数 | 带解构的 options objects、默认值、rest parameters、overloaded signatures 用于类型缩小 |
| 依赖 | 构造函数注入、DI 可选（tsyringe、inversify）、或模块级 factory functions |

### C# / .NET

| 部分 | 提案摘要 |
|---------|----------|
| 错误处理 | Exceptions 用于异常情况、自定义异常从 `Exception` 基类派生、`try/catch/finally`、业务逻辑无错误码、`Result` 模式日益流行 |
| 类型系统 | Classes、interfaces（显式）、records（C# 9+）、structs（值类型）、nullable reference types（C# 8+）、generics |
| 命名 | `PascalCase` methods/properties/classes、`camelCase` local variables/parameters、`_camelCase` private fields、接口 `I` 前缀（`IService`） |
| 测试 | xUnit 或 NUnit、`[Fact]`/`[Theory]`（xUnit）、`[Test]`/`[TestCase]`（NUnit）、FluentAssertions、Moq 用于 mocking |
| 参数 | Named parameters、optional parameters with defaults、builder pattern 或 options pattern（`IOptions<T>`）用于配置 |
| 依赖 | 通过内置 DI（`IServiceCollection`）的构造函数注入、DI 容器是习惯用法、interface-first |

### 其他语言

对于上面未列出的语言，对每个部分使用开放式问题：

1. "[语言] 如何处理错误？异常、错误返回、代数类型还是其他？"
2. "对象模型是什么？Classes、structs、traits、protocols？继承还是组合？"
3. "命名约定是什么？大小写风格、可见性标记、模块命名？"
4. "测试生态系统是什么？框架、组织、断言风格？"
5. "你如何向函数传递配置和 options？Named params、objects、builders？"
6. "依赖注入如何处理？Containers、手动 wiring、interface parameters？"

## 输出组装

1. YAML frontmatter：`language` 和 `version`
2. 标题行：`# Language Idioms: {Language}`
3. 包含已确认/调整内容的全部 6 个核心部分
4. 用户添加的任何额外部分
5. 剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释

**目标大小**：40-60 行聚焦内容。每个部分应为 4-8 行：简短的哲学陈述加上关键习惯用法模式的简洁列表。无代码示例——atoms 有自己的伪代码示例，它们使用此文档的指导进行适配。

**确定输出路径：**
1. 如果 `.lattice/config.yaml` 存在且有 `paths.language_idioms`，使用该路径。
2. 否则，默认为 `.lattice/standards/language-idioms.md`。

**更新配置：**
1. 在顶层设置 `language: {语言}`（创建或更新）。
2. 设置 `paths.language_idioms` 指向输出文件。
3. 如果 `.lattice/config.yaml` 不存在，创建它。保留所有现有内容。

**向用户确认：**
"你的语言习惯用法文档已写入 `[PATH]`，用于 **[语言] [版本]**。以下 atoms 现在将适配它们的模式：clean-code、test-quality、secure-coding、domain-driven-design 和 architecture。"

## 文档质量检查

在编写最终文档之前，验证：

- [ ] 所有 6 个核心部分标题存在且完全匹配：`Error Handling`、`Type System & Object Model`、`Naming Conventions`、`Testing Patterns`、`Parameter & Function Design`、`Dependency Management`
- [ ] 内容特定于语言，而非通用建议（"使用错误返回"而非"正确处理错误"）
- [ ] 无代码工艺规则（阈值、复杂度限制）——这些属于 atom overlays
- [ ] 无项目身份信息（技术栈、目录布局）——这属于 knowledge-priming
- [ ] 内容简洁——每个部分 4-8 行，总文档少于 60 行
- [ ] Frontmatter 有正确的 `language` 和 `version` 值
- [ ] 配置文件正确更新，包含 `language` 键和 `paths.language_idioms`
- [ ] 输出中无残留的 `<!-- INTERVIEW GUIDANCE: -->` 注释
