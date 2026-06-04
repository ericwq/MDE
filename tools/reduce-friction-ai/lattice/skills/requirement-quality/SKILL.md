---
name: requirement-quality
description: "在生成或验证 feature 规范时应用需求质量原则。强制执行 feature 完整性、场景结构、AC 可验证性、feature 独立性和实现切片质量。在编写 feature specs、验证现有需求时使用，或当用户提到'validate this spec'、'check this feature'、'requirement quality'、'is this spec complete'或'requirement-quality'时使用。此 skill 管理编写单个 feature 规范的工艺——不是技术设计（参见 design-blueprint），不是实现（参见 code-forge）。"
---

# Requirement Quality（需求质量）

## 配置解析

Skill 支持项目特定标准。顺序：

1. 在仓库根目录查找 `.lattice/config.yaml`
2. 如果找到，检查 `paths.requirement_standards` 获取自定义文档路径
3. 如果自定义路径存在，读取文档并检查 YAML frontmatter `mode`：
   - **`mode: override`**（或无 mode）：自定义文档完全优先。替代嵌入默认值使用。必须全面——唯一参考。
   - **`mode: overlay`**：首先读取嵌入的 `./references/defaults.md`，然后将自定义文档部分叠加在上面。自定义部分替换默认值中匹配的部分（按标题匹配）。新部分追加在之后。
4. 如果没有配置/路径/文件，读取 `./references/defaults.md`

Skill 附带的默认值。有观点的最佳实践。开箱即用。仅当团队有不同标准时才覆盖。

自定义标准由 `requirement-forge-refiner` 产生→被此 atom 消费→由 `requirement-forge` molecule compose。每个项目运行一次 refiner。当标准演进时重新运行。

## 自我验证清单（Self-Validation Checklist）

在编写任何 feature 文件之前 STOP。验证所有检查。如果检查明显失败→在编写之前修复。如果是判断调用（参见 Ambiguity Signals）→标记并浮现选项。

**如果验证现有 spec**（非生成），相同检查适用——"在编写之前修复"意味着"在标记 approved 之前修复"。呈现发现作为带有严重性的质量报告。

**Draft vs approved enforcement**：对于 `status: draft`——项目 1、2、10 是必需的（核心框架必须从一开始就正确）。项目 3-9、11、12 是建议性的：标记发现但不阻止编写。对于 `status: approved`——所有项目必需，无例外。

1. **问题陈述**：命名特定的用户需求或痛苦——不是伪装的解决方案，不是模糊的改进？识别 WHO 有问题（特定用户类型或角色，而非"users"）？"我们需要一个 dashboard"是一个解决方案。"Users cannot track their order status after checkout"是一个需求——但哪些 users？Buyers？Admins？
2. **范围**：有明确的 out-of-scope 项目——不只是 in-scope？不完整的范围边界根本不是边界。
3. **边界条件**：Feature-wide edge cases、系统限制和约束记录？
4. **假设**：团队作为 true 继续的陈述是明确的——不是埋在 ACs 中或未声明？如果假设被证明是错误的，受影响的 scenarios 可识别？
5. **Scenario 名称**：每个 scenario 有动词短语名称（sentence case）描述情况——不是 feature 名称，不是 AC？
6. **AC 格式**：每个 AC 遵循商定的格式（默认：Given/When/Then）？每个有清晰的 pass/fail 条件？当 tester 可以编写自动化检查而不问澄清问题时，pass/fail 是清晰的。需要解释的结果（"responds"、"handles"、"works correctly"）必须用具体的、可观察的结果重写。
7. **失败覆盖**：至少一个 scenario 覆盖 failure、error 或 edge case——不是所有 success paths？
8. **Scenario 计数**：Feature 没有超过商定的最大值（默认：5）scenarios？如果在或超过→挑战这是否是一个 feature 还是两个。
9. **AC 计数**：每个 scenario 没有超过商定的最大值（默认：6）ACs？如果在或超过→挑战此 scenario 是否太 broad。
10. **独立性**：Feature 是自包含的——在 design-blueprint 开始之前没有未解决的外部未知数？检查 Open Questions 部分——任何影响范围、行为或 ACs 的未解决问题是 blocker。Feature 不能通过此检查与非空的 Open Questions，除非每个标记为非阻塞并陈述原因。
11. **实现注释**：Slices 按时间顺序排序，在"what"级别——无技术实现 specifics？
12. **连贯性**：所有 scenarios 是否解决 Problem Statement 中陈述的相同用户需求？如果 scenario 服务于不同的需求，它属于单独的 feature。

项目特定检查：如果加载的文档包含验证清单部分，在基本清单后应用这些。

当所有检查通过：输出"Spec passes requirement-quality — ready for write."（pre-write 模式）或"Spec passes requirement-quality — status: approved."（validation 模式）。

## 主动反模式扫描（Active Anti-Pattern Scan）

在清单后，扫描这些。如果找到→在编写之前修复或挑战。

- [ ] **Solution as problem**：问题陈述说"我们需要 X"而不是"users cannot do Y"→询问 X 解决什么用户需求；围绕需求重写
- [ ] **Vague problem**："improve the experience"、"make it faster"、"better UX"→无可验证的结果；推动具体的、可观察的用户影响
- [ ] **Persona-less problem**：问题陈述说"users"而不识别哪个用户类型或角色→推动具体性；不同的 personas 产生不同的 ACs
- [ ] **Hidden assumption**：AC 或 scenario 依赖未声明的假设（"assumes user is logged in"但没有 Assumptions 部分记录这个）→使假设明确或添加覆盖它不成立的 cases 的 scenario
- [ ] **Boundaryless scope**：scope 部分仅列出 in-scope，nothing explicitly out→强制至少 3 个明确的 exclusions；未定义的范围=无限范围
- [ ] **Happy-path-only spec**：每个 scenario 是 success path，无 failure 或 error scenario→在 feature 完成之前添加至少一个 failure scenario
- [ ] **AC sprawl**：单个 scenario 积累 7+ ACs→scenario 太 broad；提议 split 为两个命名 scenarios
- [ ] **Scenario sprawl**：feature 有 6+ scenarios→feature 可能是两个；在添加更多之前 pause 并挑战范围
- [ ] **Vague AC**："the system should handle errors gracefully"、"response should be fast"、"it should work correctly"→无 pass/fail 条件；重写为具体的 Given/When/Then
- [ ] **Implementation AC**：AC 指定技术方法（"system shall use Redis"、"shall call the /api/v2 endpoint"）→requirements 指定行为，而非实现；重写为可观察的结果
- [ ] **Orphaned feature**：`depends_on` 为空但 scenario 引用另一个 feature 的数据或行为→标记缺失的依赖
- [ ] **Cross-epic feature undocumented**：feature scenarios 引用不同 epic 的行为但没有记录 cross-epic 依赖→将 feature 放在其主要 epic 中，添加其他 epic 作为 cross-reference，在 `depends_on` frontmatter 中记录依赖
- [ ] **Technical task as feature**：feature name 或问题陈述描述基础设施、tooling 或 engineering 工作（"Set up database schema"、"Configure CI/CD"、"Write unit tests"）→不是 product feature；重定向到实现层，挑战它服务的用户需求
- [ ] **Wrong granularity — too fine**：feature 实际上是单个 acceptance criterion 或 micro-behavior（"Show error on wrong password"）→merge 到代表完整用户 facing 行为的更大 feature
- [ ] **Wrong granularity — too coarse**：feature 包含整个产品区域有 10+ implicit behaviors（"User management"）→挑战并 decompose 为离散的 independently-implementable features
- [ ] **Generic slices**：Implementation notes 使用适用于任何 feature 的 placeholder labels（"Core functionality"、"Error handling"、"Edge cases"）→每个 slice 必须命名 THIS feature 正在构建的特定行为
- [ ] **Undefined domain terms**：Scenarios 使用未在 spec 中任何地方定义的专业/领域术语→添加 inline definitions 或 Glossary 部分用于对新的团队成员不明显的术语

## 模糊信号（Ambiguity Signals）

多个有效结果。标记它——呈现选项和推理。如果加载了 `framework:collaborative-judgment`，使用它结构化呈现。

- **Feature boundary**：两个相关 behaviors——一个 feature 还是两个？取决于每个是否可以独立设计。如果 Behavior A 需要知道 Behavior B 的设计来 spec 自己的 ACs，它们是一个 feature。
- **Scenario granularity**：两个相关 situations——一个 scenario 有更多 ACs，还是两个单独的 scenarios？如果两者共享相同的 precondition 和 trigger，group。如果它们在任一上不同→separate scenarios。
- **Priority**：feature 服务于多个用户类型或 epics 有不同的 urgency→为 product decision 浮现；不要静默分配。
- **Independence borderline**：feature 依赖另一个 feature 但该依赖是 well-understood 和 stable→判断调用是否标记为 dependent 或作为 independent 继续。
- **Assumption vs. requirement**：陈述读起来既是 assumption 又是 requirement——"users will have verified email"可以是记录 precondition 或构建 verification feature→为 product decision 浮现。

## 核心原则（Core Principle）

Requirement-quality atom 管理**feature 规范的质量和完整性**——什么使 spec well-formed、verifiable 和 independently implementable。

区别于：
- **design-blueprint**：管理 feature 如何技术设计（components、interactions、contracts）
- **code-forge**：管理 feature 如何实现（layer order、atom enforcement）
- **test-quality**：管理 tests 如何结构和编写

Feature spec 存在以明确回答三个问题：
1. **此解决什么用户需求？**（Problem Statement）
2. **系统必须处理什么情况？**（Scenarios）
3. **我们如何知道它完成了？**（Acceptance Criteria）

不能回答所有三个的 spec 不为 design-blueprint 准备。

参见 `./references/defaults.md` 获取 epic/feature/scenario 定义、AC 格式示例、优先级标记、状态工作流、命名约定和实现切片指南。
