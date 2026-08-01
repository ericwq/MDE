---
name: skill-align
description: "审计并修复所有 Lattice 文档——README、docs/、PROJECT.md、GitHub issue 模板和 CLAUDE.md——确保它们与当前 skill 清单完全对齐。文档漂移是 Lattice 中用户困惑的最常见来源——某个 skill 存在于代码库中但未出现在文档中，或者重命名的 skill 在 bug 报告模板中留下了过时的引用。如果你对 skills/ 做了任何更改且尚未运行此工具，请立即运行。当用户说 'align docs'、'audit docs'、'update documentation'、'skill align'、'check docs are in sync'、'audit skill inventory'、'ensure docs are aligned'、'are the docs up to date' 或 'what needs updating' 时使用。独立运行——不调用其他 skill。"
---

# Lattice Sync

**核心职责：** 保持每一份面向公众的文档与实际 skill 清单同步。`skills/` 目录是事实来源——所有文档均由此派生。

**输入：** 无需输入。每次运行时读取 `skills/` 的实时状态和所有文档。可选：指定某个 skill 名称或 tier 以缩小审计范围。

**输出：**
- 一份发现报告，以 `[GAP]`、`[STALE]` 或 `[WRONG]` 列出每个缺口，附带文件名和描述
- 所有发现的缺口在以下文件中就地修复：`README.md`、`docs/how-it-works.md`、`docs/configuration.md`、`docs/practical-guide.md`、`PROJECT.md`、`CLAUDE.md`、`.github/ISSUE_TEMPLATE/bug_report.yml`
- 最终清洁确认："No gaps found" 或列出所做的更改

**如何验证此 skill 已完成工作：**
```bash
grep -rn "requirements atom\|framework:requirements\b" \
  docs/ README.md PROJECT.md CLAUDE.md .github/ skills/ \
  --include="*.md" --include="*.yml"
```
此 grep 返回任何结果都意味着同步不完整。清洁运行应不产生输出。

同时验证：`skills/` 中的每个 skill 都出现在 bug_report.yml 下拉菜单和 how-it-works.md 清单表中。

## 何时运行

- 创建任何新的 atom、molecule 或 refiner 之后
- 重命名或移除某个 skill 之后
- 更改了某个 skill 的 config key、consumed-by 关系或输出路径之后
- 当用户怀疑文档过期时

## 第一阶段 — 构建实时清单

读取 skills 目录树。**STOP：不要假设你已经知道清单内容**——始终从文件读取。

```bash
find skills/ -name "SKILL.md" | sort
```

对找到的每个 SKILL.md，提取：
- `name:` 字段（frontmatter）
- `tier:` 从路径推断（`atoms/`、`molecules/`、`refiners/`）
- Config key：在文件中 grep `paths\.\w+`
- Consumes：grep `framework:` 引用（仅 molecules）
- Produces：grep `.lattice/standards/` 路径（仅 refiners）
- Output subfolder：grep `.lattice/\w+/` 路径（仅 molecules）

构建结构化清单：
```
atoms:    [name, config_key, has_refiner, has_defaults_md]
molecules: [name, composes[], output_subfolder]
refiners:  [name, config_key, produces_path, consumed_by_atom]
```

在开始第二阶段之前，阅读 `references/audit-checklist.md` 以获取完整的逐文档审计规则。

---

## 第二阶段 — 逐文档审计

按以下顺序检查每个文档。对每个文档，应用 `references/audit-checklist.md` 中的检查项。**STOP：零发现的文档在报告中不出现条目**——不要为其打印标题或“无缺口”行。按如下格式记录每个发现：

```
[GAP]    文件:行号 — 描述缺少或错误的内容
[STALE]  文件:行号 — 描述引用了已不存在的内容
[WRONG]  文件:行号 — 描述某个关系（consumed-by、produces）不正确
```

需要审计的文档：
1. `docs/how-it-works.md`
2. `docs/configuration.md`
3. `docs/practical-guide.md`
4. `README.md`
5. `PROJECT.md` — Skill 约定、已知子文件夹列表和仓库结构计数都在此处
6. `.github/ISSUE_TEMPLATE/bug_report.yml`
7. `.github/ISSUE_TEMPLATE/skill_request.yml` *（通用模板——仅在存在 skill 特定示例时检查）*
8. `.github/ISSUE_TEMPLATE/documentation.yml` *（通用模板——仅在出现 skill 名称时检查）*
9. `knowledge-base/requirement-forge-requirements.md` *（如果存在——检查是否与当前 skill 名称匹配）*

在进行任何更改之前，先呈现汇总的发现报告：

```
## Lattice Sync — 发现

### docs/how-it-works.md
[GAP] Atoms 表缺少：requirement-quality
[STALE] Refiners in Depth：requirement-forge-refiner 的 consumed-by 写的是 "molecule"，但应写 "requirement-quality atom"

### .github/ISSUE_TEMPLATE/bug_report.yml
[GAP] Skill 下拉菜单缺少：requirement-quality (atom)

总计：N 个缺口，M 个过时引用，P 个错误关系
```

询问：*"准备应用所有修复吗？还是有哪些发现想跳过？"*

---

## 第三阶段 — 修复

应用每个已确认的修复。对每个文档，在单次编辑中完成所有更改。**STOP：不要对同一文件进行多次编辑。**

所有修复应用后，运行最终验证 grep 以确认不再有过时的引用：

```bash
# 检查过时的 atom 名称或错误的 consumed-by 文本
grep -rn "<old-name>\|consumed.*molecule\|consumed.*wrong" \
  docs/ README.md PROJECT.md CLAUDE.md .github/ --include="*.md" --include="*.yml"
```

如果 grep 返回结果，在宣称完成之前修复它们。

---

## 第四阶段 — 部署（可选）

如果用户想将更新后的 skill 推送到其 AI 工具的 skills 目录：

```bash
./tools/install.sh /path/to/your/skills/folder
```

如果未提供目标路径，询问用户。Claude Code 的默认路径：`~/.claude/skills/`。

---

## 始终需要验证的关键关系

Lattice 文档中最容易出错的关系是 refiner → atom → molecule 链。文档经常偏离事实，说某个 refiner “被 molecule 消费”，而实际上它被 molecule 所组合的某个 atom 消费。

对实时清单中的每个 refiner，动态推导正确的关系：
1. 读取 refiner 的 SKILL.md——找到它说哪个 atom 消费它（查找 "consumed by" 或 "reads this document"）
2. 读取该 atom 的 Config Resolution——确认它读取相同的 `paths.{key}`
3. 读取组合该 atom 的 molecule——确认链条完整
4. 如果任何文档说 refiner 直接被 molecule 消费（而不是 atom），标记为 `[WRONG]`

例外：`review-refiner` 直接被 `review` molecule 消费——它配置的是 molecule 的工作流，而非某个 atom。这是当前框架中唯一正确的 molecule 直接消费场景。

---

## 已知的 `.lattice/` 子文件夹

每个写入活文档的 molecule 必须使用命名子文件夹。如果发现写入 `.lattice/` 的新 molecule 且其子文件夹不在此列表中，标记为需要添加到 `PROJECT.md`：

- `standards/` — refiner 输出
- `context/` — 功能锚定文档（context-anchoring atom）
- `learnings/` — 运维经验（learning-harvest atom）
- `reviews/` — 审查日志
- `insights/` — architecture-compass 输出
- `requirements/` — epic/feature 规格（requirement-forge）

---

## 好的标准

同步完成的条件：
- `skills/` 中的每个 skill 都出现在 `docs/how-it-works.md` 的清单表中
- 每个 refiner 在 `docs/configuration.md` 的 paths 表中有正确的条目，包括正确的 consumed-by atom
- 每个 molecule 出现在 `bug_report.yml` 的 skill 下拉菜单中
- 每个 atom 出现在 `bug_report.yml` 的 skill 下拉菜单中
- 每个 refiner 出现在 `bug_report.yml` 的 skill 下拉菜单中
- `PROJECT.md` 的已知子文件夹列表覆盖每个 `.lattice/` 输出目录
- 没有文档包含 `skills/` 中已不存在的 skill 名称
- `README.md` 和 `docs/how-it-works.md` 中的管道描述使用当前的 molecule 名称并按正确顺序排列
- 每个硬编码了自己 refiner 列表的 molecule（`refiners-update` 的映射表、`lattice-init` 的 Step 3 优先级列表），每个条目与 `skills/refiners/` 中的实时 refiner 一一对应——无缺失、无过时

参见 `references/audit-checklist.md` 获取详尽的逐文档规则。
