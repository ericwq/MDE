# Dev Skills（开发技能）

用于创建和维护 Lattice 本身的辅助技能。

| 技能 | 用途 |
|-------|---------|
| **skill-align** | 审计并修复所有文档，确保与当前 skill 清单对齐 |
| **skill-forge** | 创建新的 Lattice skill（atom、molecule 或 refiner），遵循所有约定 |
| **skill-review** | 从多个角色视角对 skill 进行深度行为审计 |
| **skill-tighten** | 审计 SKILL.md 的语言合规性——移除啰嗦的说明文、软化语言，添加 STOP 门控 |
| **skill-validate** | 根据 tier 约定验证 SKILL.md，捕获结构错误 |

**典型流程**: `skill-forge` → `skill-validate` → `skill-review` → `skill-align`

规范源位于 `dev-skills/`。`.claude/skills/` 和 `.github/skills/` 都是指向此处的符号链接，确保 Claude Code 和 Copilot 共享单一事实来源。
