# Lattice Sync — 逐文档审计清单

对每个文档，应用所列的检查项。每个 `[ ]` 项都是潜在的发现。

---

## 1. `docs/how-it-works.md`

### Atoms 表（§ Skill Inventory → Atoms）

对实时清单中的每个 atom：
- [ ] 行存在，包含正确的 `name` 和 `/name` 命令
- [ ] `What it enforces` 描述准确且为最新
- [ ] 不存在已移除 atom 的行

### Molecules 表（§ Skill Inventory → Molecules）

对实时清单中的每个 molecule：
- [ ] 行存在，包含正确的 `name` 和 `/name` 命令
- [ ] `What it does` 描述反映当前行为（特别是在工作流变更后）
- [ ] 不存在已移除 molecule 的行

### Refiners 表（§ Skill Inventory → Refiners）

对实时清单中的每个 refiner：
- [ ] 行存在，包含正确的 `name` 和 `/name` 命令
- [ ] `Produces` 路径正确，与实际写入内容匹配
- [ ] 不存在已移除 refiner 的行

### Always vs Conditional atoms（§ Atoms in Depth）

- [ ] 每个 conditional atom 均列出，条件正确陈述
- [ ] 不存在已移除的 atom
- [ ] `requirement-quality` 列为 conditional（在 requirement-forge / spec validation 期间适用）

### Molecules in Depth

对实时清单中的每个 molecule：
- [ ] 有专用的 `### <molecule-name>` 节
- [ ] `Composes:` 列表与 molecule 的 `Required Skills` 节准确匹配
- [ ] `How it works` 步骤反映当前工作流（而非过时版本）
- [ ] 不存在已移除 molecule 的节

### Refiners in Depth 表

对每个 refiner：
- [ ] 行存在
- [ ] `Consumed by` 正确命名消费该 refiner 的 **atom**（而非 molecule），针对目标为 atom 的 refiner
- [ ] 例外：`review-refiner` → 直接被 `review` molecule 消费（正确）
- [ ] 例外：（当前无——所有其他 refiner 都走 atom → molecule 路径）

### `.lattice/` 文件夹结构部分

- [ ] `standards/` 列出所有当前的 refiner 输出文件
- [ ] 所有已知子文件夹均已列出（`requirements/`、`insights/`、`context/`、`learnings/`、`reviews/`、`standards/`）
- [ ] 子文件夹生命周期表中每个子文件夹对应一行

### Pipeline 部分

- [ ] 所有管道路径使用当前 molecule 名称
- [ ] 完整管道包含 `requirement-forge` 作为可选的上游步骤

---

## 2. `docs/configuration.md`

### 文件结构 YAML 示例

- [ ] 任何 atom 或 refiner 使用的每个 `paths.*` key 均出现在 YAML 块中
- [ ] YAML 块中没有 key 引用已移除的 skill

### `paths` Keys 表

对实时清单中的每个 refiner：
- [ ] 行存在，`Key` 正确（snake_case）
- [ ] `Purpose` 描述准确
- [ ] `Produced by` 命名正确的 refiner
- [ ] `Default path` 正确（`.lattice/standards/<name>.md`）
- [ ] `Consumed by` 正确命名 atom（而非 molecule）——除 `review_standards` 命名 `review` molecule 外
- [ ] `Mode` 正确（`overlay` vs `override` vs `standalone`）

特殊情况 — `requirement_standards`：
- [ ] `Consumed by`：`requirement-quality` atom（而非 "requirement-forge molecule"）
- [ ] `Mode`：`overlay (recommended)`

---

## 3. `docs/practical-guide.md`

### 目录列表

- [ ] 文档中的每个 `## Section` 标题出现在目录列表中
- [ ] 没有目录条目指向不存在的节

### Requirements 部分

- [ ] 涵盖：何时使用 requirement-forge vs design-blueprint
- [ ] 涵盖：输出结构（index.md + features/）
- [ ] 涵盖：什么是 scenario
- [ ] 涵盖：refiner 可选 / 默认值传达
- [ ] 涵盖：处理已有的 PRD / 非结构化材料
- [ ] 涵盖：会话中断 / 恢复行为
- [ ] 涵盖：单功能快速路径

### Workflow 部分

- [ ] `/design-blueprint` Q&A 提及 requirement-forge 作为上游选项

### Getting Started 部分

- [ ] `.lattice/` 文件夹描述提及 `requirements/` 子文件夹

### Troubleshooting 部分

- [ ] 没有引用已移除的 skill 或旧的 skill 名称

---

## 4. `README.md`

### 管道描述

- [ ] 管道文本中命名 `requirement-forge` 作为上游步骤
- [ ] 管道使用当前的 molecule 名称并按正确顺序排列

### Getting Started 步骤

- [ ] `/requirement-forge` 步骤存在（标记为可选但推荐）
- [ ] 步骤编号在添加后连续且正确

### The Three Tiers 表

- [ ] tier 描述中没有出现过时的 skill 名称
- [ ] Atoms 行描述准确反映当前 atom 清单

---

## 5. `PROJECT.md`

### 已知子文件夹列表（Key Patterns 部分）

- [ ] `requirements/` 列出，描述为 "epic/feature specs produced by requirement-forge"
- [ ] `insights/` 列出
- [ ] 所有其他已知子文件夹均存在

### Skill Conventions 部分

- [ ] 如果有新的 tier 约定建立，已记录
- [ ] 没有引用已不存在的 skill

### Repository Structure 部分

- [ ] Molecule 计数准确
- [ ] Refiner 计数准确
- [ ] Atom 计数准确

---

## 6. `.github/ISSUE_TEMPLATE/bug_report.yml`

### Skill 下拉选项

下拉菜单必须包含实时清单中的每个 skill，按 tier 分组，格式如下：
- `<name> (atom)` — 用于 atoms
- `<name> (molecule)` — 用于 molecules
- `<name> (refiner)` — 用于 refiners

检查——从第一阶段构建的实时清单推导预期条目，而非从任何硬编码列表：
- [ ] 对实时清单中的每个 atom 名称：下拉菜单中存在 `<name> (atom)` 条目
- [ ] 对实时清单中的每个 molecule 名称：下拉菜单中存在 `<name> (molecule)` 条目
- [ ] 对实时清单中的每个 refiner 名称：下拉菜单中存在 `<name> (refiner)` 条目
- [ ] 下拉菜单中不存在实时清单中没有的 skill 条目（过时条目）
- [ ] `Other / unsure` 选项作为第一个选项存在

不要与硬编码列表比较。与你在第一阶段通过读取 `skills/` 构建的清单比较。该清单始终是最新的。

---

## 7. `.github/ISSUE_TEMPLATE/skill_request.yml`

这是一个通用表单。仅在出现 skill 特定名称时检查：
- [ ] 如果有任何特定 skill 名称被引用为示例，验证它们仍然存在

---

## 8. `.github/ISSUE_TEMPLATE/documentation.yml`

这是一个通用表单。仅检查：
- [ ] 如果下拉菜单中列出了特定文档文件名，验证这些文件仍然存在

---

## 9. `knowledge-base/` 文件

对 `knowledge-base/` 中的任何需求文档：
- [ ] 引用的 skill 名称与当前清单名称匹配（例如 `requirement-quality` 而非 `requirements`）
- [ ] 状态字段为最新

---

## 跨文档检查（在所有独立文档检查之后运行）

### 名称一致性

对每个 skill：
- [ ] 文件夹名称 == `name:` frontmatter 字段（例如文件夹 `requirement-quality/` → `name: requirement-quality`）
- [ ] 所有文档使用相同的规范名称（不会混用 `requirement-quality` 和 `requirements`）

### Config key 一致性

对每个 refiner：
- [ ] refiner SKILL.md 中使用的 config key 与 `docs/configuration.md` 中的 key 匹配
- [ ] key 遵循 snake_case 约定

### Required Skills 一致性（molecules）

对 molecule 中的每个 `Required Skills` 条目：
- [ ] 引用的 atom（`framework:<name>`）存在于 `skills/atoms/<name>/`
- [ ] 对 atom 的描述准确（always/conditional）

### 硬编码 refiner 列表一致性（molecules）

某些 molecule 维护自己的 refiner 列表或表，而非动态推导。目前：`refiners-update` 的 "Refiner → standards document map" 表，以及 `lattice-init` 的 Step 3 优先级列表。对每个此类列表：
- [ ] 实时清单中的每个 refiner（来自第一阶段）都有对应的行/条目
- [ ] 不存在 `skills/refiners/` 中已不存在的 refiner 条目
- [ ] 条目中引用的任何 config key 与该 refiner 实际的 `paths.*` key 匹配

这不同于上述的 "Required Skills 一致性"——Required Skills 列出 molecule 始终/条件性组合的 atoms；此项检查的是 molecule 自己的硬编码路由表是否与实时 refiner 目录一致。

### 过时引用扫描

```bash
# 在所有修复后运行此命令以发现残留的过时引用
grep -rn "requirements atom\|framework:requirements\b\|consumed.*requirement-forge molecule" \
  docs/ README.md PROJECT.md CLAUDE.md .github/ skills/ \
  --include="*.md" --include="*.yml" 2>/dev/null
```

此 grep 的任何结果都是 `[STALE]` 发现。
