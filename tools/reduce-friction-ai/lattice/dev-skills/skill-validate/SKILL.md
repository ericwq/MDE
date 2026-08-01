---
name: skill-validate
description: "根据所有 tier 约定验证任何 Lattice SKILL.md——atoms、molecules 和 refiners。在进入仓库之前捕获结构错误、破损的交叉引用和约定违规。如果你刚编写或修改了 Lattice skill 文件但尚未运行此工具，请立即运行——手动审查始终会遗漏此 skill 专门设计用于捕获的同类错误。当用户说 'validate this skill'、'check this skill'、'does this follow conventions'、'review this skill file'、'check my SKILL.md' 或 'skill validate' 时使用。报告 PASS/FAIL，附带具体的文件和节发现及可操作的修复方案。独立运行——不调用其他 skill。"
---

# Skill Validator

**核心职责：** 验证 SKILL.md 在结构上正确，遵循所有 Lattice tier 约定，并与框架其余部分正确组合。

**输入：** 以下一项或多项：
- 文件路径：`skills/atoms/clean-code/SKILL.md`
- Skill 名称：`clean-code`（自动解析为正确的路径）
- Tier：`atoms`（验证该 tier 中的所有 skill）
- 无参数：验证所有三个 tier 的所有 skill（计数在运行时从 `skills/` 推导——绝不硬编码）

**输出：** 每个 skill 的发现报告：
```
## Skill Validator — {skill-name}
Tier：{atom | molecule | refiner}

### Structural          PASS / FAIL — 具体发现
### Tier conventions    PASS / FAIL — 具体发现
### Cross-references    PASS / FAIL — 具体发现
### Three-angle review  PASS / WARN / FAIL 每个视角

结果：PASS | FAIL（N 个错误，M 个警告）
```

**如何验证此 skill 已完成工作：**
- 每个发现引用具体的文件和节（没有模糊的“缺少内容”——确切说明缺少什么以及在哪里缺少）
- FAIL 发现具有可操作的修复方案，而不仅仅是对问题的描述
- 报告区分错误（必须修复）和警告（需要判断）
- “修复模式”应用后，重新运行验证器返回 PASS

## Step 1：加载约定

读取 `PROJECT.md`——Skill Conventions 部分。这是事实来源。**STOP：不要依赖记忆。**

读取 `references/convention-rules.md` 获取详细的逐 tier 清单。

## Step 2：运行结构检查（所有 tiers）

对每个正在验证的 SKILL.md：

```
[ ] Frontmatter：name 字段存在，小写连字符
[ ] Frontmatter：description 字段存在且非空
[ ] Frontmatter：description 包含触发短语（用户会输入什么）
[ ] 文件夹名称与 name 字段完全匹配
[ ] Molecule 中没有内联的 atom 内容（不重复 framework:{atom} 已提供的内容）
```

## Step 3：运行 tier 特定检查

读取 `references/convention-rules.md` 获取完整的逐 tier 清单。根据 tier 应用相关部分（从文件路径确定：atoms/、molecules/、refiners/）。

## Step 4：运行交叉引用检查

对 molecule 中的每个 `framework:{atom-name}` 引用：
```bash
ls skills/atoms/{atom-name}/SKILL.md 2>/dev/null || echo "BROKEN REF: framework:{atom-name}"
```

对 refiner 或 atom 中引用的每个 `paths.{key}` config key：
- 检查它是否出现在 `docs/configuration.md` 的 paths 表中

对 molecule 中引用的每个 `.lattice/{subfolder}/` 路径：
- 检查该子文件夹是否在 `PROJECT.md` 的已知子文件夹列表中

## Step 5：三角结构审查

这三个视角是固定的——它们匹配依赖 Lattice skill 正常工作的三种利益相关者类型。这是结构审查（skill 是否遵循约定？），而非行为审查（它在实践中是否有效？）——后者使用 `skill-review`。

每个视角提出不同的问题：

**Product Owner 视角**——"这能为用户产生正确的输出吗？"
- 如果是 molecule：输出文档结构（来自 SKILL.md 模板）是否让 PO 清晰地了解产生了什么？
- 如果是 atom：质量检查是否基于真实的用户需求，而不仅仅是技术形式？
- 如果是 refiner：产生的标准文档是否解决了用户的配置问题？

**Business Analyst / Practitioner 视角**——"规则完整且可执行吗？"
- 清单项是否足够具体，每项都有明确的通过/失败条件？
- 反模式是否被命名并有修复方案，而不仅仅是症状描述？
- 模糊信号是否真正模糊，还是仅仅是规则的缺口？
- 对于 molecules：是否所有实用场景都被处理（全新开始、已有材料、中断会话、单项请求）？

**Technical Lead 视角**——"这与框架其余部分正确组合吗？"
- Atom 引用（`framework:{name}`）是否解析为真实的 skill？
- Config key 是否遵循 snake_case 并与 configuration.md 中的匹配？
- Molecule 是否写入命名的 `.lattice/` 子文件夹（绝不写入根目录）？
- 对于规划型 molecules：会话恢复检查是否在 Step 1 存在？
- 对于生成型 molecules：是否没有确认门控？

## Step 6：报告

**STOP：通过的类别仅显示单词 `PASS`**——没有重述的评论；仅在 FAIL/WARN 时显示发现文本。格式化发现如下：

```
## Skill Validator — {skill-name}
Tier：{atom | molecule | refiner}

### Structural
PASS

### Tier conventions
FAIL — [Atom] Self-Validation Checklist 第 3 项缺少 STOP 语言
FAIL — [Molecule] 规划型 molecule Step 2 没有确认门控

### Cross-references
PASS

### Three-angle review
[PO]   PASS
[BA]   WARN — 没有处理中断会话的指导（恢复行为）
[Tech] FAIL — .lattice/myoutput/ 不在已知子文件夹列表中

---
结果：FAIL（2 个错误，1 个警告）
```

区分错误（必须修复）和警告（应考虑）。

## Step 7：修复模式（可选）

如果用户说 "fix it" 或 "apply fixes"——将所有错误级别的发现直接应用到文件。修复后重新运行验证。**STOP：在询问用户应用哪些警告之前，不要修复警告。**
