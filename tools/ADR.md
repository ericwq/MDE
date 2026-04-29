# 架构决策记录

| [Martin Fowler](https://martinfowler.com/)| |
|:---|:---|
|<img src="img/mf.jpg" width="20%" />| |
|[原文](https://martinfowler.com/bliki/ArchitectureDecisionRecord.html)| 2026/3/26|

---
架构决策记录（Architecture Decision Record, ADR）是一种简短文档，用于记录并说明与产品或生态系统相关的单项决策。
这类文档应精简，篇幅控制在数页以内，包含决策内容、制定背景以及重要影响。
若后续决策发生变更，不应修改原有记录，而是关联至新的替代决策文档。

与大多数书面文档一样，编写 ADR 具备两大作用。
<ins>其一，留存决策记录，便于相关人员在数月乃至数年后，理解系统现有架构的设计缘由。
其二，或许价值更高，撰写过程能够梳理思路，在团队协作场景中效果尤为显著</ins>。
撰写关键文档时，往往会暴露出各方不同观点，促使团队展开讨论，并有望化解分歧。

<ins>一般原则是遵循新闻报道常用的 *倒金字塔 (inverted pyramid)* 写作风格。
核心要点置于文档开头，细节内容后置补充</ins>。

<ins>业内普遍建议，将决策记录存放于对应代码库的源码仓库中，常规存放路径为 `doc/adr`</ins>。
这种方式便于研发人员随时查阅。
同理，文档应采用 Markdown 等轻量标记语言编写，可像代码一样直接阅读、比对差异。
同时可通过构建任务，将文档自动发布至产品团队官网。

对于覆盖范围超出单一代码库、涉及更广泛生态系统的架构决策记录，将其存放于单个产品代码仓库并不合适。
部分人员也认为，把 ADR 放在 git 工具中，会让非开发人员难以查阅与编辑。

<ins>每份 ADR 需独立成文件，文件名采用单调递增序号命名，并附带能够概括决策内容的名称，便于在目录列表中快速识别查阅。
（示例：“0001-动态网页采用 HTMX 方案” ）</ins>

<ins>每份 ADR 都设有状态标识：讨论阶段为 **提议**；
经团队采纳并正式生效后为 **已接受**；
当决策发生重大修改或被替代时标记为 **已废止**，并关联新的替代 ADR。
一份 ADR 一旦通过审批，便不再重新审议或修改，如需调整，只能以新记录废止旧版本。
借此可完整留存决策日志，清晰追溯各项规范的生效周期</ins>。

<ins>ADR 不仅记录最终方案，还需附上简要决策依据。
内容需总结促使决策产生的核心问题，以及权衡过的各项利弊取舍。
编写时可参考设计模式中的 "制约因素 (force)" 思路。
同时，应当清晰列出所有认真考量过的备选方案，并逐一说明其优劣，会很有价值</ins>。

任何决策都会产生相应后果。
部分后果可间接推断，但有时仍需单独增设章节进行明确说明。
决策通常存在一定不确定性，因此记录决策的置信程度十分必要。
同时，可在此处注明，一旦产品环境发生哪些变动，团队便需要重新评估该项决策。

ADR 在 [建议流程 (Advice Process)](https://martinfowler.com/articles/scaling-architecture-conversationally.html) 中占据核心地位，其作用不局限于留存决策内容，撰写过程还能汇集专业意见、统一团队共识。
在这种情况下，它们还应包括在形成 ADR 过程中收集的建议，
尽管为了保持简洁，可能更好的方式是在 ADR 中对建议进行总结，并单独保留完整的建议记录。

需要牢记的核心原则是简洁性。
ADR 应简明扼要、直击重点，通常控制在单页篇幅。
如有补充佐证材料，仅保留外部链接即可。

<ins>ADR 虽多用于软件架构领域的决策留存，但撰写精简决策记录的理念，同样适用于各类其他业务场景。
这类决策日志可形成完整的历史存档，清晰追溯各项方案的制定缘由，为现状提供完整溯源依据</ins>。

### 延伸阅读
Michael Nygard 于 2011 年发表一篇 [ADR 格式的文章](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions)，首次提出了 “ADR” 这一术语。
决策日志的概念并非由他首创，但他大力倡导采用轻量型文档，聚焦决策本身。
其创作理念主要借鉴了 Phillipe Kruchten 关于决策登记簿与决策日志的论述，以及 [软件设计模式](https://martinfowler.com/articles/writingPatterns.html) 的行文风格。
他的这篇文章在同类题材中质量上乘，本文撰写的初衷，仅为补充介绍此后该领域的相关发展。

<ins>在本网站上，有 [Harmel-Law](https://martinfowler.com/articles/scaling-architecture-conversationally.html#adr) 以及 [Rowse 和 Shepherd](https://martinfowler.com/articles/building-infrastructure-platform.html#ArchitecturalDecisionRecords) 的文章中一些关于 ADR 格式的简短示例</ins>。

adr-tools 是一个简单的命令行工具，用于管理 ADR。
它包括一组供自身使用的 ADR，这是该形式的一个很好的示例。

### 致谢
Andrew Harmel-Law, Brandon Cook, David Lucas, Francisco Dias, Giuseppe Matheus Pereira, John King, Kief Morris, Michael Joyce, Neil Price, Shane Gibson, Steven Peh, and Vijay Raghavan Aravamudhan，在内部通讯平台参与了本文初稿的讨论。Michael Nygard 为本文提供了其相关著作的背景溯源信息。
