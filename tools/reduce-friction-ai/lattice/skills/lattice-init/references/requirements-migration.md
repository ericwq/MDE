# 需求布局迁移

仅在 `lattice-init` 步骤 3 确认用户想要将旧版 `.lattice/requirements/index.md` 迁移到分片布局时读取此文件。

## 1. 检测计划

读取 `.lattice/requirements/index.md`。记录每个 `### [史诗名称]` 章节、史诗名称、描述和功能表行。检查每个功能文件自身的 `epic:` 前置元数据与旧索引中的位置之间的不一致。

## 2. 呈现计划

向用户展示：检测到的史诗及其新文件、`index.md` 将被重写为精简顶点形式、每个功能文件的链接将重新指向、`.lattice/config.yaml` 将获得 `requirements_layout: sharded`。呈现任何史诗位置不一致并让用户解决。

## 3. 创建史诗文件

对每个检测到的史诗，使用需求锻造输出模板中的史诗文件模板写入 `.lattice/requirements/epics/{epic-slug}.md`。

## 4. 重写顶点索引

将 `.lattice/requirements/index.md` 重写为精简顶点形式。

## 5. 重新指向功能文件

对每个功能文件，将 `## Links` → `Epic index:` 行从 `../index.md` 更新为 `../epics/{epic-slug}.md`。

## 6. 更新配置

确保 `.lattice/config.yaml` 有 `requirements_layout: sharded`。
