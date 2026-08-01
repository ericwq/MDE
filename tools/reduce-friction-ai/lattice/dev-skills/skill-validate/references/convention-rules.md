# Lattice 约定规则 — 验证器参考

skill-validate 的详细逐 tier 检查。与 PROJECT.md 一起阅读。

---

## Atoms

### 必需节（按此顺序）
1. YAML frontmatter
2. `## Config Resolution`
3. `## Self-Validation Checklist`
4. `## Active Anti-Pattern Scan`
5. `## Ambiguity Signals` *（仅限代码质量 atom——不包括 knowledge-priming、design-first、context-anchoring、collaborative-judgment）*
6. `## Core Principle`

### Config Resolution 检查
- [ ] 首先读取 `.lattice/config.yaml`
- [ ] 检查 `paths.{snake_case_key}` 以获取自定义文档路径
- [ ] 处理 `mode: overlay`（与默认值合并）和 `mode: override`（替换默认值）
- [ ] 未找到配置时回退到 `./references/defaults.md`
- [ ] 有 `references/defaults.md` 文件（代码质量 atom）——检查它是否存在

### Self-Validation Checklist 检查
- [ ] 每个项目都是编号的
- [ ] 每个项目有一个粗体标签（`**LABEL**：`）
- [ ] 使用祈使 STOP 语言："STOP after generating each component. Verify ALL..."
- [ ] 每个项目有明确的通过/失败条件——不是模糊的指导
- [ ] 检查失败时有修复指令（"If not → [action]"）

### Active Anti-Pattern Scan 检查
- [ ] 复选框格式：`- [ ] **模式名称**：描述 → 修复`
- [ ] 每个反模式有：名称、表现、修复方案
- [ ] 代码质量 atom 至少 5 个反模式
- [ ] 不与 Self-Validation Checklist 重叠（scan 捕获气味，checklist 捕获硬违规）

### Ambiguity Signals 检查
- [ ] 每个信号描述一个真正模糊的情况（两种有效方法）
- [ ] 每个信号有解决指导——不仅仅是 "it depends"
- [ ] 引用 `framework:collaborative-judgment` 或 `./references/defaults.md` 进行解决

### defaults.md 检查
- [ ] 存在于 `references/defaults.md`
- [ ] 用编号节（§1、§2...）结构化，与 refiner 产生的内容匹配
- [ ] 包含有主见的默认值——不是占位符或模糊指导
- [ ] 以归属/引用行结束

---

## Molecules

### 必需节
1. YAML frontmatter
2. `## Required Skills`（带有 `framework:{atom-name}` 引用和 always/conditional 标签）
3. `## Workflow`（编号步骤）
4. 可选：Mode Detection、Persona 节

### Required Skills 检查
- [ ] 每个 atom 引用使用 `framework:{name}` 格式
- [ ] 每个引用有 always/conditional 限定符
- [ ] 每个引用的 atom 存在于 `skills/atoms/{name}/SKILL.md`
- [ ] 没有 atom 内容被内联——只有引用

### 生成型 molecule 检查（code-forge、refactor-safely、bug-fix 模式）
- [ ] 步骤之间没有确认门控
- [ ] 仅在真正的判断调用上通过 `framework:collaborative-judgment` 暂停
- [ ] 线性编号步骤
- [ ] 不需要会话恢复检查（生成型 molecule 不在会话间维护活文档）

### 规划/交互型 molecule 检查（design-blueprint、architecture-compass、requirement-forge 模式）
- [ ] Step 1 检查是否有现有输出文档——如果找到，读取它并从最早未完成的步骤恢复
- [ ] 每个阶段有：（1）展示输出，（2）具体的针对性问题，（3）硬门控语言 "Do NOT advance to Step N until the user explicitly confirms"
- [ ] 可以提前退出，部分输出为有效结果
- [ ] 写入命名的 `.lattice/{subfolder}/`——绝不写入 `.lattice/` 根目录
- [ ] 子文件夹在 PROJECT.md 的已知子文件夹列表中

### 输出文档模板检查（规划型 molecules）
- [ ] SKILL.md 中定义了输出文档的模板或结构
- [ ] 模板包含 frontmatter 字段（如适用）
- [ ] 模板足够完整，新会话可以读取输出并恢复工作

---

## Refiners

### 必需节
1. YAML frontmatter
2. `## What This Produces`（输出路径、两种模式、config key、模板引用）
3. `## Scope Clarification`
4. `## Before You Begin`（检查现有文档、扫描仓库信号）
5. `## Choosing the Mode`（overlay vs override）
6. `## Facilitation Approach`（对话风格、overlay 流程、override 流程）
7. `## Section-by-Section Interview Guide`（引用模板）
8. `## Output Assembly`
9. `## Document Quality Checks`

### What This Produces 检查
- [ ] 输出路径是 `.lattice/standards/{name}.md`
- [ ] 两种模式（overlay 和 override）均已描述
- [ ] Config key 已记录（`paths.{snake_case_key}`）
- [ ] 引用 `./assets/template.md`
- [ ] 说明哪个 atom（或 molecule）消费生成的文档

### 引导方法检查
- [ ] Overlay 模式：简要展示默认值，询问是否匹配，仅记录更改
- [ ] Override 模式：逐节遍历，所有节出现在输出中
- [ ] 列出常见场景（例如 "I agree with everything" → 无需自定义文档）

### 模板检查（assets/template.md）
- [ ] 每个节有 `<!-- INTERVIEW GUIDANCE: -->` 注释
- [ ] 每个引导块包括：默认内容摘要、询问什么、探针问题、可定制 vs 固定的部分
- [ ] 存在默认内容（不仅仅是引导注释）
- [ ] Frontmatter 含 `mode:` 占位符
- [ ] 页脚有 project/date/mode 占位符

### Output Assembly 检查
- [ ] Overlay 模式输出说明（仅更改的节）
- [ ] Override 模式输出说明（所有节）
- [ ] Config 更新说明（如何写入/更新 `.lattice/config.yaml`）
- [ ] 从最终输出中剥离所有 `<!-- INTERVIEW GUIDANCE: -->` 注释

### Quality Checks 部分
- [ ] Overlay 模式检查列为复选框
- [ ] Override 模式检查列为复选框
- [ ] 两种模式共享：有效的 YAML frontmatter、格式良好的 markdown、config 已更新

---

## 所有 tiers — 命名和 frontmatter

- [ ] `name:` 是小写连字符格式（例如 `requirement-quality`，而非 `RequirementQuality`）
- [ ] 文件夹名称 = `name:` 字段（完全一致）
- [ ] `description:` 包含 skill 的功能和触发短语（用户会说什么）
- [ ] 描述足够具体以正确触发——不至于太窄而错过有效用例
- [ ] Frontmatter 中没有 YAML 语法错误
