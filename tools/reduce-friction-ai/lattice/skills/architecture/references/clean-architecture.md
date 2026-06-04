# Clean Architecture 执行规则

这些是 clean architecture 模式的执行指令。定义自我验证清单、反模式扫描、模糊信号，当 `architecture_mode` 为 `clean`（默认）时架构 atom 应用的结构原则。

详细内容（层责任表、每层规则、命令/查询流程示例、违规/修复对）在 `./clean-architecture-defaults.md` 或团队的 overlay/override 文档中。

## 自我验证清单

生成每个组件后 STOP。在继续之前验证所有项目。如果检查失败，在呈现之前修复代码。如果有多个有效方法的判断调用（参见下面的模糊信号），标记它——呈现选项和推理而非静默选择。

1. **操作类型**：状态更改（命令）还是读取（查询）？首先确定——决定整个流程。
2. **命令流程**：状态更改操作——数据通过 domain 流向 Repository？在持久化之前强制执行 domain 不变性？
3. **查询流程**：读取操作——使用 Provider（而非 Repository）？在不需要强制执行不变性的地方避免 domain 对象？
4. **依赖方向**：所有源代码依赖指向内部？Domain 层没有来自外层的导入？
5. **层放置**：每个类在正确的层中？Controllers 仅翻译，application services 编排，domain 强制执行规则，infrastructure 实现接口。
6. **边界数据**：跨越层边界的数据使用简单结构（DTOs、plain objects）？没有框架特定类型或 entities 泄漏到外部。
7. **接口所有权**：Repository 接口在 domain 层定义？Provider 契约不在 domain 层中？
8. **单层**：每个类恰好属于一个架构层？没有类跨越 HTTP 解析、业务逻辑和数据库访问。

## 主动反模式扫描

在验证上述清单后，扫描输出以查找这些反模式。如果找到任何，在呈现代码之前修复。

- [ ] **控制器中的业务逻辑**：Controller 做出超出翻译的业务决策 → 提取到 domain 或用例
- [ ] **Domain 依赖 Infrastructure**：Domain 导入数据库客户端、HTTP 库或外部服务 → 在 domain 中定义接口，在 infrastructure 中实现
- [ ] **God Classes（上帝类）**：单个类更改每种类型的要求 → 按层分解为专注的类
- [ ] **贫血架构**：层存在于文件夹中但未强制执行依赖规则 → 验证导入，添加接口
- [ ] **泄漏数据格式**：数据库模式更改破坏 API 契约 → 在每层边界映射 DAO、domain 对象和响应 DTO
- [ ] **循环依赖**：两个层相互导入（例如，application 导入 infrastructure 类型，infrastructure 导入 application 类型）→ 在内层引入接口
- [ ] **胖 Application Service**：业务规则或 domain 逻辑累积在编排层 → 将决策移动到 domain entities 或 domain services
- [ ] **泄漏 Entity**：Domain 对象直接从 controller 返回而非映射到响应 DTO → 添加边界映射步骤

## 模糊信号

这些检查通常有多个有效结果。当遇到时，呈现选项而非静默选择。

- **层放置**：协调 domain 对象的逻辑但也包含业务规则，可以是 domain service 或 application service。区别：逻辑是业务规则还是编排业务规则？
- **查询复杂性**：读取操作需要在返回数据之前强制执行业务规则，模糊 Provider vs Repository 边界。
- **DTO 粒度**：每个端点一个 DTO vs 相关端点间共享 DTOs——类型安全和重复之间的权衡。

## 核心原则

Clean Architecture 关于**结构**——代码在哪里，哪些层存在，依赖流向哪个方向。与 DDD 不同，DDD 关于在 domain 层内*制作*领域逻辑。此 skill 处理结构外壳；DDD 处理内部领域工艺。

结构约束：业务规则独立可测试，不耦合到框架、UI、数据库或外部机构。任何外层组件交换而不触碰 domain 逻辑。

## 依赖规则

使架构工作的单一规则：**源代码依赖只指向内部。**

内层中的任何东西都不知道外层。外层中声明的任何名称——函数、类、变量、数据格式——不被内层中的代码提及。

原因：隔离。当内层不知道外层时，可以交换、重写或删除任何外层而不向内级联更改。

当控制流必须向外时（例如，用例需要调用 repository），使用**依赖倒置**：内层定义接口，外层实现它。即使运行时调用向外，源代码依赖也指向内部。跨越边界的数据应该是简单结构——DTOs、plain objects、primitives——永远不要框架特定类型。

参见 `./clean-architecture-defaults.md` 获取层责任表、每层规则、命令/查询流程示例和违规/修复对。
