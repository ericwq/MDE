# 需求锻造——输出模板

在写入 `.lattice/requirements/` 文档时读取此文件。严格使用这些模板。

## 顶点文件：`.lattice/requirements/index.md`

精简且很少手动触及。边界注释以下的所有内容在步骤 6 通过扫描 `epics/*.md` 头部生成——绝不手动追加行。

```markdown
---
project: [项目名称]
last_updated: [日期]
---

# 需求索引——[项目名称]

## 定义
**史诗：** [来自已加载标准或内置默认值]
**功能：** [来自已加载标准或内置默认值]

## 词汇表（如标准包含 §10 领域术语）

## 史诗（自动生成——从 epics/*.md 头部重新生成，不手动编辑）
```

## 史诗文件：`.lattice/requirements/epics/{epic-slug}.md`

包含手写头部（名称、描述）和自动生成的功能表。

## 功能文件：`.lattice/requirements/features/{feature-name}.md`

完整功能规范模板，包含前置元数据（feature、epic、priority、status、depends_on）、问题陈述、范围、边界条件、假设、场景及验收标准和实现切片。
