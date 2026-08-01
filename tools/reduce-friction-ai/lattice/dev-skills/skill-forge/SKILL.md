---
name: skill-forge
description: "创建新的 Lattice skill——atom、molecule 或 refiner——遵循所有框架约定。手动编写 skill 文件几乎必然会产生约定违规：错误的部分顺序、遗漏的确认门控、缺少正确结构的 defaults.md。此 skill 了解所有约定并引导你完成创建过程。在向 Lattice 添加任何新的 atom、molecule 或 refiner 时使用，或当用户说 'create a new skill'、'add an atom'、'add a molecule'、'add a refiner'、'build X for Lattice'、'new lattice skill' 或 'skill forge' 时使用。不进行验证、文档对齐或部署——这些是你之后运行的独立 skill。"
---

# Lattice Skill Forge

**核心职责：** 为新的 Lattice skill 创建正确的文件结构。

**输入：** 关于该 skill 应该做什么以及何时触发的描述。

**输出：** 写入正确路径的一个或多个 skill 文件：
- Atom → `skills/atoms/{name}/SKILL.md` + `skills/atoms/{name}/references/defaults.md`
- Molecule → `skills/molecules/{name}/SKILL.md`
- Refiner → `skills/refiners/{name}/SKILL.md` + `skills/refiners/{name}/assets/template.md`

**如何验证此 skill 已完成工作：**
- 所有必需文件存在于正确的路径
- 文件夹名称与 `name:` frontmatter 完全匹配
- 所有 tier 必需的节按正确顺序存在
- 无占位内容——所有节包含真实、具体的内容

---

## Step 1：理解意图并选择 tier

询问用户：*"这个 skill 应该做什么？什么时候触发？简要描述一下。"*

根据描述确定 tier：

| 该 skill... | Tier |
|---|---|
| 通过检查清单和反模式扫描来执行**一项**原则 | **Atom** |
| 将多个 atom 编排为多步骤工作流 | **Molecule** |
| 运行引导式访谈以生成 `.lattice/standards/*.md` 文件 | **Refiner** |

陈述你的判断并与用户确认。在继续之前获得明确的 tier 同意。

---

## Step 2：需求对齐（仅 molecules 和 refiners）

在编写任何 SKILL.md 行之前，先就设计达成一致。

检查 `knowledge-base/` 中是否有现有的需求文档：
```bash
ls knowledge-base/ | grep -i {name}
```

**如果找到** → 读取它，总结关键设计决策，确认它们仍然反映意图。

**如果未找到** → 在编写之前通过对话解决以下问题：

对于 **molecule：**
- 它组合哪些 atom？（读取 `skills/atoms/` 查看现有内容。）
- 它是**生成型**（产生代码/工件，线性流程）还是**规划/交互型**（产生活文档，每个阶段有确认门控）？
- 它向 `.lattice/` 写入什么？哪个子文件夹？（必须使用命名子文件夹，绝不能是根目录。）
- 会话恢复行为是什么——它如何处理中断的会话？

对于 **refiner：**
- 它配置哪个 atom？（refiner 必须有一个读取其输出的 atom。）
- 它向 `.lattice/config.yaml` 添加什么 `paths.{snake_case_key}` config key？
- 访谈模板涵盖哪些部分？
- Overlay/override——默认模式是哪种？

写一段设计摘要并与用户确认。

**STOP：在设计确认之前不要编写 SKILL.md。**

---

## Step 3：读取当前约定

读取 `PROJECT.md`——Skill Conventions 部分。始终重新读取；绝不要依赖记忆。

注意当前的 skill 计数（atoms/molecules/refiners）——创建后需要在 PROJECT.md 中更新，但这是 `skill-align` 的工作。

---

## Step 4：编写 skill 文件

### 编写 Atom

**`skills/atoms/{name}/SKILL.md`**——节按以下确切顺序排列：

1. **YAML frontmatter**——`name`（小写连字符）、`description`（包含触发短语）
2. **Config Resolution**——始终使用此模式：
   - 检查 `.lattice/config.yaml` 中的 `paths.{config_key}`
   - 如果找到：读取文档，检查 `mode: overlay`（与默认值合并）或 `mode: override`（替换）
   - 如果未找到：读取 `./references/defaults.md`
3. **Self-Validation Checklist**——编号项目，粗体标签，祈使 STOP 语言，每个项目有明确的通过/失败条件和修复方案
4. **Active Anti-Pattern Scan**——复选框格式（`- [ ] **名称**：表现 → 修复`），最少 5 项
5. **Ambiguity Signals**——两种有效方法同时存在的真正灰色地带；每个信号都有解决指导
6. **Core Principle**——atom 管控什么，不管控什么（与其他 atom 的边界）

**`skills/atoms/{name}/references/defaults.md`**——嵌入式默认值：
- 使用编号节（§1、§2...），与未来 refiner 产生的内容匹配
- 有主见、具体的内容——不是占位符
- 以归属/引用行结束

### 编写 Molecule

**`skills/molecules/{name}/SKILL.md`**：

1. **YAML frontmatter**——name，description 带触发短语
2. **Required Skills**——将每个 atom 列为 `framework:{name}`，带有 always/conditional 限定符
3. **Mode Detection**（如果 molecule 有模式）——模式如何被调用，各自改变什么
4. **Workflow**——编号步骤，每步有明确的输入和输出

**生成型 molecule 约定**（`code-forge`、`bug-fix`、`refactor-safely` 模式）：
- 线性编号步骤，无确认门控
- 仅在真正需要判断的地方通过 `framework:collaborative-judgment` 暂停
- 不需要会话恢复检查

**规划/交互型 molecule 约定**（`design-blueprint`、`requirement-forge` 模式）：
- Step 1 必须检查是否有现有输出文档。如果找到：读取它，确定最早未完成的步骤，从那里恢复
- 每个阶段必须有三个要素：（1）展示输出，（2）提出具体的针对性问题，（3）硬门控：*"Do NOT advance to Step N until the user explicitly confirms."*
- 可以提前退出——部分输出是有效结果
- 始终写入命名 `.lattice/{subfolder}/`——绝不写入 `.lattice/` 根目录

### 编写 Refiner

**`skills/refiners/{name}/SKILL.md`**——涵盖以下所有内容：
- 它产生什么（输出路径、两种模式、config key、哪个 atom 读取它）
- 范围澄清（此 refiner 不配置什么）
- 开始访谈前检查现有文档
- 模式选择对话（overlay vs override，何时使用每种）
- 引导方法（每次一节，默认优先，记录决策而非讨论）
- 逐节引导（引用 `./assets/template.md` 及其 Interview Guidance 注释）
- 输出组装规则（overlay：仅更改的节；override：所有节）
- Config 更新说明（将 `paths.{key}` 写入 `.lattice/config.yaml`）
- 文档质量检查（每种模式一个清单）

**`skills/refiners/{name}/assets/template.md`**：
- 完整文档结构，每个节有 `<!-- INTERVIEW GUIDANCE: -->` 注释
- 每个引导块包含：默认内容摘要、询问什么、探针问题、可定制 vs 固定的部分
- Overlay 前言和 override 前言（独立块，清晰标注）
- 带有 project/date/mode 占位符的页脚
- INTERVIEW GUIDANCE 注释在最终生成文档中被剥离

---

## Step 5：确认文件完整

完成前验证：
- [ ] 所有必需文件存在于正确的路径
- [ ] 文件夹名称 = `name:` frontmatter 字段（逐字符完全一致）
- [ ] `description:` 包含触发短语——用户实际会输入什么来调用此 skill
- [ ] 所有 tier 特定的节按正确顺序存在
- [ ] 对于 atoms：`defaults.md` 存在，有 §-编号节，包含真实内容
- [ ] 对于 refiners：`template.md` 存在，每个节有 Interview Guidance 注释
- [ ] 对于规划型 molecules：每个阶段有三部分确认门控模式
- [ ] 没有节包含占位文本（"TBD"、"TODO"、"add content here"）

报告创建了什么以及创建位置。不要运行验证、同步或部署——这些是独立的步骤。
