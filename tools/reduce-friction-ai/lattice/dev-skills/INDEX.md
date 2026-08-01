# Dev Skills（开发技能）— 索引

用于创建和维护 Lattice 本身的辅助技能集合。

---

## 技能列表

### [skill-align](./skill-align/SKILL.md)

审计并修复所有 Lattice 文档（`README.md`、`docs/`、`PROJECT.md`、GitHub issue 模板、`CLAUDE.md`），确保与当前 skill 清单完全对齐。文档漂移是用户困惑的最常见来源——skill-align 是最终的“真相核对”工具，在所有文档修改后都应运行。

**附属文件：**
- [audit-checklist.md](./skill-align/references/audit-checklist.md) — 逐文档审计清单，列出每个文档需要检查的具体项目

---

### [skill-forge](./skill-forge/SKILL.md)

创建新的 Lattice skill（atom、molecule 或 refiner），遵循所有框架约定。引导用户完成 tier 选择、需求对齐、约定读取和文件编写，确保创建的文件结构正确、内容完整。

**典型调用：** `skill-forge → skill-validate → skill-review → skill-align`

---

### [skill-review](./skill-review/SKILL.md)

从三个独立角色视角对 Lattice skill 进行深度行为审计。提出与 skill 相关的 3 个审查角色，运行独立场景分析，合并高置信度、实用的发现为按严重程度排序的缺口报告，附修复建议。与 `skill-validate`（结构）互补——此 skill 关注实际使用中的行为缺口。

---

### [skill-tighten](./skill-tighten/SKILL.md)

审计任何 Lattice SKILL.md 的语言合规性——移除说明性文字、将软性语言转换为祈使句、为硬规则添加 `STOP:` 门控、删减冗余重复。修复导致 agent 在运行时跳过或低估指令的措辞问题。

---

### [skill-validate](./skill-validate/SKILL.md)

根据所有 tier 约定验证 SKILL.md——atoms、molecules 和 refiners。在进入仓库之前捕获结构错误、破损的交叉引用和约定违规。报告 PASS/FAIL，附带具体文件和节的发现及可操作修复方案。

**附属文件：**
- [convention-rules.md](./skill-validate/references/convention-rules.md) — 详细的逐 tier 检查清单，覆盖 atom、molecule、refiner 的每个必需节和字段

---

## 典型工作流

```
skill-forge       创建新 skill
    ↓
skill-validate    验证结构正确性
    ↓
skill-review      深度行为审计
    ↓
skill-tighten     精简语言，移除冗余
    ↓
skill-align       同步所有文档
```

---

## 目录结构

```
dev-skills/
├── README.md
├── INDEX.md                          ← 当前文件
├── skill-align/
│   ├── SKILL.md
│   └── references/
│       └── audit-checklist.md
├── skill-forge/
│   └── SKILL.md
├── skill-review/
│   └── SKILL.md
├── skill-tighten/
│   └── SKILL.md
└── skill-validate/
    ├── SKILL.md
    └── references/
        └── convention-rules.md
```
