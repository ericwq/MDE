---
name: lattice-init
description: "Lattice 项目的引导式设置和升级检查体验——扫描仓库，检测现有配置和过时的约定，按优先级顺序建议精炼器和可用升级，创建或协调 .lattice/ 配置。弥合安装技能与获得首次价值之间的鸿沟，以及升级 Lattice 与采用其最新约定之间的鸿沟。当用户说'lattice init'、'设置 lattice'、'初始化 lattice'、'开始使用 lattice'、'为此项目配置 lattice'、'检查 lattice 升级'或'升级 lattice 约定'时使用。"
---

# Lattice 初始化

## 工作流程

### 步骤 1：扫描项目

检测项目信号以了解项目形态和现有 Lattice 状态。

**语言/框架检测**——检查仓库根目录文件：`package.json` → Node.js/TypeScript、`go.mod` → Go、`pom.xml`/`build.gradle` → Java、`Cargo.toml` → Rust 等。

**目录结构**——列出顶级目录并识别常见模式。

**现有 `.lattice/` 状态**——检查已存在的 Lattice 工件：config.yaml、各标准文档、上下文文档、经验文档等。

### 步骤 2：呈现发现

以结构化格式呈现扫描结果，包括项目信息、关键目录和 Lattice 设置状态。

### 步骤 3：引导式设置

**优先级顺序**：

1. **需求布局升级**（如果检测到旧布局）——一次性迁移，不触碰功能文件内容
2. **knowledge-priming-refiner**——捕获项目身份
3. **language-idioms-refiner**——定义语言工程模式
4. **architecture-refiner**——定义项目架构标准
5. **ddd-refiner**——捕获领域建模规则
6. **clean-code-refiner**——定制编码标准
7. **review-refiner**——自定义审查流程

对每个缺口，呈现三个选择：**立即运行**、**稍后跳过**或**跳过所有剩余**。

### 步骤 4：后续步骤

呈现 Lattice 工作流概览：设计功能 (`/design-blueprint`)、实现 (`/code-forge`)、安全重构 (`/refactor-safely`)、修复 bug (`/bug-fix`)、审查 (`/review`)。

如果步骤 3 中有跳过的精炼器，添加提醒。
