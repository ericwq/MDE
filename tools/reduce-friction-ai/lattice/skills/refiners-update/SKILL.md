---
name: refiners-update
description: "在重大变更后更新现有的 Lattice 标准——lattice-init 的更新模式对应物。扫描 .lattice/standards/，询问变更了什么，并将每个受影响的标准路由到其精炼器的修订模式，记录 git 原生的变更说明。当用户说'更新精炼器'、'精炼器更新'、'我们的标准变了'、'更新我们的标准'、'架构变了，更新标准'、'我们切换了语言，更新标准'、'重大变更后修订标准'或'重新运行精炼器'时使用。"
---

# 精炼器更新

`lattice-init` 的更新模式对应物。`lattice-init` 检测*缺失*的标准并将你路由到精炼器来**创建**它们，而此分子检测*现有*标准并将你路由到每个精炼器的**修订**模式，在重大变更后更新它们——然后记录变更了什么以及为什么。

它编排精炼器；它从不重新实现它们的访谈。版本控制是 git 原生的：历史存在于提交中，每个修订的文档获得一行变更说明。

## 精炼器 → 标准文档映射

| 标准文档（在 `.lattice/standards/` 中） | 用于修订它的精炼器 | 配置键 |
|---|---|---|
| `knowledge-base.md` | `/knowledge-priming-refiner` | `paths.knowledge_base` |
| `language-idioms.md` | `/language-idioms-refiner` | `paths.language_idioms` |
| `architecture.md` | `/architecture-refiner` | `paths.architecture` |
| `ddd-principles.md` | `/ddd-refiner` | `paths.ddd_principles` |
| `clean-code.md` | `/clean-code-refiner` | `paths.clean_code` |
| `review-standards.md` | `/review-refiner` | `paths.review_standards` |
| `requirement-standards.md` | `/requirement-forge-refiner` | `paths.requirement_standards` |

## 工作流程

### 步骤 1：扫描现有标准

读取 `.lattice/config.yaml`。对上述映射中的每一行，解析路径并检查文档是否存在。呈现当前标准列表及状态。

**停止：如果没有 `.lattice/config.yaml` 存在，或没有找到标准文档**：没有可更新的内容。

### 步骤 2：捕获变更内容

用一两句话询问用户什么变更了以及为什么。这成为记录在每个修订文档上的变更说明，并驱动哪些标准受影响。

### 步骤 3：将变更映射到受影响的标准

从触发因素出发，提议哪些现有标准可能受影响及原因。

### 步骤 4：修订每个受影响的标准

对每个确认的标准：提供立即修订、跳过或跳过所有剩余的选择。修订时应用对应精炼器的修订路径，完成后追加变更说明。

### 步骤 5：摘要

报告更新结果，提醒用户历史是 git 原生的，并建议一起提交修订的标准。
