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

Skill 附带有默认值。有观点的最佳实践。开箱即用。仅当团队有不同标准时才覆盖。

自定义标准由 `requirement-forge-refiner` 产生 → 被本原子仅能消费 → 由 `requirement-forge` 分子技能组合。
每个项目运行一次 refiner。当标准演进时重新运行。

## 自我验证清单（Self-Validation Checklist）

## 自检清单

在编写任何特性文件之前，请先停下来。验证所有检查项。如果某项检查明确未通过 → 在编写之前修复。如果属于主观判断（见“歧义信号”部分）→ 标记并列出可选方案。

**如果是验证已有的规格说明**（而非生成新的规格说明），同样适用上述检查项 —— “在编写之前修复”应理解为“在标记为已批准之前修复”。将发现的问题以包含严重程度的质量报告形式呈现。

**草稿状态 vs 批准状态的执行要求**：对于状态为 `status: draft` 的文档 —— 第1、2、10项为必须项（核心框架必须从一开始就正确）。第3–9、11、12项为建议项：标记出发现的问题，但不阻止编写。对于状态为 `status: approved` 的文档 —— 所有项均为必须项，无一例外。

1. **问题陈述**：明确指出具体的用户需求或痛点——不是伪装成问题的解决方案，也不是模糊的改进描述。要说明谁（具体的用户类型或角色，而非泛指的“用户”）遇到了这个问题？“我们需要一个仪表盘”是一个解决方案。“用户在下单后无法追踪订单状态”是一个需求——但具体是哪些用户？买家？管理员？
2. **范围**：包含明确的超出范围的事项——而不仅仅是范围内的事项？不完整的范围边界根本算不上边界。
3. **边界条件**：记录了功能级别的边缘情况、系统限制和约束条件？
4. **假设**：团队暂定为真的陈述是明确写出的——而不是隐藏在验收条件中或未被说明？如果某个假设被证明是错误的，受影响的场景是否可被识别？
5. **场景名称**：每个场景都有一个动词短语形式的名称（句子大小写）来描述该情境——不是功能名称，也不是验收条件？
6. **验收条件格式**：每个验收条件都遵循约定的格式（默认：Given/When/Then）？每个条件都有明确的通过/失败判定标准？当测试人员无需提出澄清性问题就能编写自动化检查时，通过/失败的判定才是清晰的。需要解释的结果（如“响应”、“处理”、“正确工作”）必须重写为具体的、可观察的结果。
7. **失败场景覆盖**：至少有一个场景覆盖了失败、错误或边缘情况——并非全都是成功路径？
8. **场景数量**：特性的场景数量不超过约定的上限（默认：5个）？如果达到或超过上限 → 质疑这是否应该作为一个特性还是拆分为两个。
9. **验收条件数量**：每个场景的验收条件数量不超过约定的上限（默认：6个）？如果达到或超过上限 → 质疑该场景是否过于宽泛。
10. **独立性**：特性是自包含的——在开始设计蓝图之前没有未解决的外部未知因素？检查“待解决问题”部分——任何影响范围、行为或验收条件的未解决问题都是阻塞项。如果“待解决问题”非空，特性不能通过此项检查，除非每个问题都被标记为非阻塞并附有理由。
11. **实现说明**：切片按时间顺序排列，处于“做什么”的层面——不包含具体的技术实现细节？
12. **一致性**：所有场景是否都解决了“问题陈述”中所述的同一个用户需求？如果某个场景服务于不同的需求，它应该属于另一个独立的特性。

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
