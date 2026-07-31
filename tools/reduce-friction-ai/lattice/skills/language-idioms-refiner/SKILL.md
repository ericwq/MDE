---
name: language-idioms-refiner
description: "引导结构化对话，为仓库定义语言特定的惯用法和模式。生成 language-idioms.md 文档，被多个原子消费以将伪代码默认值适配到项目语言。在设置新项目、切换语言，或用户说'设置语言'、'定义语言惯用法'、'配置语言'、'语言模式'或'为 Go/Rust/Python 适配'时使用。"
---

# 语言惯用法精炼器

## 这将产出什么

- **输出**：`.lattice/standards/language-idioms.md`（或来自 `.lattice/config.yaml` → `paths.language_idioms` 的自定义路径）
- **模式**：始终独立——原子中没有嵌入的语言默认值可供叠加。对于修订，加载现有文档并原地更新章节。
- **配置键**：`language`（顶层）和 `paths.language_idioms`
- **模板**：读取 `./assets/template.md` 获取完整文档结构、预填充示例和访谈指导注释
- **消费者**：多个原子按标题名称引用特定章节：

| 章节 | 消费者 |
|---------|------------|
| **错误处理** | `clean-code` 原子（§8）、`secure-coding` 原子（§1 信任边界消息） |
| **类型系统与对象模型** | `clean-code` 原子（§1 单一职责/内聚）、`domain-driven-design` 原子（实体、值对象、聚合） |
| **命名约定** | `clean-code` 原子（§4） |
| **测试模式** | `test-quality` 原子（§5 命名、§4 隔离、§6 构建器） |
| **参数与函数设计** | `clean-code` 原子（§2 函数大小、§5 参数） |
| **依赖管理** | `clean-code` 原子（§9 可测试性/DI）、`architecture` 原子（依赖方向） |

这六个章节标题是**稳定契约**。原子按名称引用它们。

## 语言特定方案

对于熟知的语言，用惯用法默认值预填充每个章节。访谈确认或调整这些。对于未识别的语言，使用开放式问题。

### Go

| 章节 | 方案摘要 |
|---------|-----------------|
| 错误处理 | 显式错误返回（`value, err := ...`），`if err != nil`，用 `fmt.Errorf("context: %w", err)` 包装错误，预期情况的哨兵错误，无异常 |
| 类型系统 | 带方法的结构体（接收器函数），隐式接口（结构类型），通过嵌入实现组合，无继承，无类 |
| 命名 | 导出 = 大写，未导出 = 小写，小作用域短名称，首字母缩写全大写（`HTTP`、`ID`），包名是标识符的一部分 |
| 测试 | 表驱动测试，`t.Run` 子测试，`testing.T` 参数，测试文件 `_test.go` 同目录，无需断言库 |
| 参数 | 接受接口返回结构体，函数选项模式（`WithTimeout(5*time.Second)`），多返回值，无方法重载 |
| 依赖 | 传递接口参数（非构造函数 DI），在消费者端定义接口，无 DI 容器，在 `main()` 或 `cmd/` 中显式装配 |

### Rust

| 章节 | 方案摘要 |
|---------|-----------------|
| 错误处理 | `Result<T, E>` 用于可恢复，`panic!` 用于不可恢复，`?` 运算符传播，库用 `thiserror`，应用用 `anyhow` |
| 类型系统 | 结构体 + impl 块，trait（显式实现），带数据的枚举（代数类型），所有权/借用，无继承，无 null（用 `Option<T>`） |
| 命名 | `snake_case` 函数/变量，`PascalCase` 类型/trait，`SCREAMING_SNAKE` 常量，生命周期名短（`'a`、`'b`） |
| 测试 | `#[test]` 属性，同文件 `#[cfg(test)] mod tests`，`tests/` 目录集成测试，`assert_eq!`/`assert!` 宏 |
| 参数 | 所有权：借用（`&T`）vs 移动，泛型约束（`impl Trait`），复杂配置用构建器模式，无默认参数 |
| 依赖 | trait 对象（`dyn Trait`）或泛型（`impl Trait`）用于抽象，无 DI 容器，显式构造 |

### Python

| 章节 | 方案摘要 |
|---------|-----------------|
| 错误处理 | EAFP 优于 LBYL（try/except 而非 if-check），上下文管理器（`with`）用于清理，自定义异常继承基类 |
| 类型系统 | 类、dataclass（`@dataclass`）、protocol 用于结构类型（PEP 544）、鸭子类型、类型提示鼓励但运行时可选 |
| 命名 | `snake_case` 函数/变量，`PascalCase` 类，`SCREAMING_SNAKE` 常量，`_private` 约定，`__dunder__` 魔法方法 |
| 测试 | 偏好 pytest，fixture 用于设置/清理，`@pytest.mark.parametrize` 数据驱动测试，普通 `assert`，测试文件 `test_*.py` |
| 参数 | `**kwargs` 用于选项，命名/关键字参数，默认值，dataclass 或 TypedDict 用于配置对象 |
| 依赖 | 构造函数注入配合 protocol/ABC，或函数参数，无重量级 DI 容器 |

### Java / Kotlin、TypeScript、C# / .NET 及其他语言

（完整方案见 `./assets/template.md` 原文及 SKILL.md）
