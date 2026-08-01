---
name: skill-tighten
description: "审计任何 Lattice SKILL.md 的语言合规性——移除说明性文字，将软性语言转换为祈使句，为硬规则添加 STOP: 门控，并删减冗余重复。与 skill-review（发现行为缺口）互补，修复导致 agent 在运行时跳过或低估指令的措辞。在编写或大幅修改任何 skill 之后使用，或当用户说 'tighten this skill'、'clean up the language'、'make this more effective'、'reduce the bloat'、'tighten the language' 或 'skill tighten' 时使用。独立运行——不调用其他 skill。"
---

# Skill Tighten

**核心职责：** 修复导致 agent 跳过或低估指令的语言。不是行为缺口——那是 `skill-review` 的工作。仅处理措辞：说明性文字、软性语言、缺失的执行信号、冗余重复。

**输入：** 以下一项或多项：
- 文件路径：`skills/atoms/clean-code/SKILL.md`
- Skill 名称：`clean-code`（自动解析为正确的路径）
- Tier：`atoms`（精简该 tier 中的所有 skill）
- 无参数：精简所有 tier 中的所有 skill

**输出：** 编辑后的文件 + 每个 skill 的报告：
```
## skill-tighten — {skill-name}
行数：{之前} → {之后}
更改：
1. {更改了什么} — {一行原因}
2. ...
结果：TIGHTENED（{N} 个更改）| CLEAN（无需更改）
```

**如何验证此 skill 已完成工作：**
- 每个被删节的节都只有说明性内容——零命令
- 每个被软化的短语都是“consider / think about / you may want to / it is recommended”
- 每个添加的 STOP: 都在之前没有执行信号的硬规则上
- 没有规则、门控、清单项或分支逻辑被移除
- 编辑后重新运行返回 CLEAN

---

## Step 1：读取 skill

读取完整的 SKILL.md。同时读取所有同级文件：`references/defaults.md`、`references/methodology-detail.md`、`assets/template.md`，或任何在 skill 正文中被 `Read` 指令引用的文件。

暂不编辑——先完成完整审计。

---

## Step 2：应用精简检查清单

对每个项目，扫描整个文件。标记找到的每个实例——不要在第一个就停止。

### T1 — 不可操作的节

解释 skill 为什么存在、描述问题或叙述上下文而不发出单个命令的节。删除整个节。保留指向引用文件的指针（如果存在）。

始终符合条件的模式：
- 标题为 "Core Principle"、"Purpose"、"Problem"、"Background"、"Why This Matters"、"How It Is Used"、"Integration with Other Skills" 的节
- 编号列表前的开头段落，重述编号列表将要做的事情
- 门控之后的结束段落，解释如果跳过门控会发生什么

**通过：** 节包含至少一个祈使指令（do, read, write, verify, check, apply, stop, flag）。
**失败：** 节仅包含陈述性语句、理由或范围评论。

### T2 — 软性语言

agent 视为可选的词汇和短语：

| 模式 | 替换为 |
|---|---|
| `consider X` | `X`（祈使句）或删除 |
| `think about X` | 删除 |
| `you may want to X` | 删除 |
| `it is recommended to X` | `X` |
| `should`（作为软建议） | `must` 或删除 |
| `try to X` | `X` |
| `where possible, X` | `X` 或删除 |

**通过：** 指令是祈使动词，没有模糊限定词。
**失败：** 指令包含上述任何模式。

### T3 — 缺失的 STOP: 门控

必须不能被跳过的硬规则——推进到下一步的门控、清单前言、不可协商的约束——以粗体文字陈述但没有 `**STOP:**` 前缀。

**通过：** 硬门控以 `**STOP:**` 作为其行或句子的第一个标记。
**失败：** 硬门控表述为 "Do NOT..."、"Never..."、"Always verify..."、"Must not..." 但没有 STOP: 前缀。

添加 `**STOP:**` 前缀。不要重写指令本身。

### T4 — 冗余重复

同一个观点用不同的话说了两次。保留更精炼的版本；删掉另一个。

模式：
- 节标题在该节开头句中被重述
- 指令后跟解释指令含义的句子
- 在明确门控之后的理由句子（"Without this, X would happen"）
- 相邻两个项目符号中的同一规则

**通过：** 每个陈述以最精炼的形式出现一次。
**失败：** 两个句子传达相同的指令或相同的约束。

### T5 — 表格列膨胀

列的内容解释为什么存在某个触发器，而不是触发器本身。

常见罪魁祸首：名称为 "Why"、"Reason"、"Rationale"、"Because"、"Notes" 的列，而表的其他列已经编码了可操作的信息。

**通过：** 表中的每一列都是可操作的——agent 可以用它做某事。
**失败：** 列仅包含 agent 阅读但无法作用于的解释性文字。

删除该列。不要删除行。

### T6 — 尾随理由句子

附加到明确指令的句子，解释为什么存在该指令。指令本身已经足够——解释增加了 token 但没有增加合规性。

模式：
- "Without this, agents will X"
- "This ensures that X"
- "This is because X"
- "X is important because Y"
- 任何以 "This" 开头的句子，跟在完整祈使句之后

**通过：** 指令以动作结束。没有尾随解释。
**失败：** 指令后跟以 "This"、"Without"、"Because" 开头的句子，或重述不遵循指令的后果。

---

## Step 3：执行编辑

应用所有发现。按检查清单项目的顺序编辑（T1 优先，T6 最后）——先做结构删减，再做措辞修复。

规则：
- 当 T1 适用时，完全删减节——不要改写它们
- 添加 `**STOP:**` 作为前缀——不要重写周围的指令
- 不要移除清单项、分支逻辑、门控或输出格式规范
- 不要重组节——仅做针对性删减和添加
- 如果 T1 节包含一条嵌入的祈使句，提取该行并丢弃节包装

---

## Step 4：报告

所有编辑后：

```
## skill-tighten — {skill-name}
行数：{之前} → {之后}（{差值}）
更改的文件：SKILL.md [, defaults.md, ...]

更改：
1. [T1] 删减 "{节名称}"——无命令，纯理由
2. [T3] 向 "{规则}" 添加 STOP:——原为粗体文字，现为执行门控
3. [T5] 从 {表名称} 删除 "Reason" 列——理由而非指令
...

结果：TIGHTENED（{N} 个更改）
```

如果无需更改：
```
结果：CLEAN——未发现语言合规性问题
```
