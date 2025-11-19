## 术语表

#### ▶[上一节](appendix.md)

以下是对本书中选定术语、模式名称及其他概念的简要定义。

### AGGREGATE
一组关联对象，在数据变更时被视为整体处理。外部引用仅限于 `AGGREGATE` 中指定为 'root' 的成员。 `AGGREGATE` 边界内适用一组一致性规则。

### analysis pattern
代表业务建模中公共构造 (common construction) 的一组概念。可能仅适用于单一领域，也可能横跨多个领域（ [Fowler 1997](references.md#fowler-1997) , p. 8）。

### ASSERTION
对程序在某个时刻正确状态的陈述，与实现方式无关。通常，`ASSERTION` 规定操作的结果或设计元素的不变性。

### BOUNDED CONTEXT
特定模型的适用范围界定。`BOUNDING CONTEXTS` 使团队成员清晰且共同理解哪些要素必须保持一致，哪些要素可独立发展。

### client
调用被设计元素并利用其功能的程序元素。

### cohesion
逻辑一致性与依赖性。

### command (a.k.a. modifier)
对系统产生某种改变的操作（例如设置变量）。一种有意产生副作用的操作。

### CONCEPTUAL CONTOUR
领域本身内在的一致性，若能在模型中体现，可帮助设计更自然地适应变化。

### context
决定词语或陈述含义的出现环境。
另见 [BOUNDED CONTEXT](#bounded-context)

### CONTEXT MAP
展示项目中涉及的 `BOUNDED CONTEXTS` 及其模型间的实际关联关系。

### CORE DOMAIN
模型的独特部分，是用户目标的核心所在，使应用程序与众不同并具有价值。

### declarative design
一种编程形式，其中对属性的精确描述实际控制着软件运行。可执行的规范。

### deep model
对领域专家核心关切及其最相关知识的精辟表达。深度模型摒弃领域表象与浅薄解读。

### design pattern
描述为解决特定情境中普遍设计问题而定制的通信对象与类。（ [Gamma 等，1995](references.md#gamma-1995)，第3页）

### distillation
分离混合物成分以提取精华的过程，使其以更具价值和实用性的形式呈现。在软件设计中，指对模型关键要素的抽象化处理，或将大型系统拆分以突出 `CORE DOMAIN`。

### domain
知识、影响或活动的范畴。

### domain expert
软件项目成员中精通应用领域而非软件开发的人员。区别于普通用户，领域专家对专业领域具有深厚造诣。

### domain layer
`LAYERED ARCHITECTURE` 中负责领域逻辑的设计与实现部分。领域层承载着领域模型的软件表达形式。

### ENTITY
本质上并非由其属性定义，而是由连续性与身份认同的纽带所界定的对象。

### FACTORY
一种封装复杂创建逻辑的机制，旨在为客户端抽象化所创建对象的类型。

### function
一种计算并返回结果的操作，不产生可观察的副作用。

### immutable
指创建后可观察状态永不改变的特性。

### implicit concept
理解模型或设计含义所必需，但从未被明示的概念。

### INTENTION-REVEALING INTERFACE
一种设计，其中类名、方法名及其他元素既传达原始开发者的创建初衷，也体现其对客户端开发者的价值。

### invariant
对某个设计元素的 `ASSERTION`，该 `ASSERTION` 在除特定瞬态情况（如方法执行中途或未提交的数据库事务过程中）之外的所有时刻都必须成立。

### iteration
指程序通过微小步骤反复优化的过程，亦指其中单次优化步骤。

### large-scale structure
一套高层次概念、规则或二者的集合，用于确立整个系统的设计模式。这种语言能让系统在宏观层面被讨论和理解。

### LAYERED ARCHITECTURE
一种分离软件系统关注点、隔离领域层等的技术。

### life cycle
对象从创建到删除期间可能经历的一系列状态序列，通常包含确保状态转换完整性的约束条件。可能包含 `ENTITY` 在系统间及不同 `BOUNDED CONTEXTS` 间的迁移过程。

### model
描述领域特定方面的抽象体系，可用于解决该领域相关问题。

### MODEL-DRIVEN DESIGN
一种设计模式，其中软件元素的子集与模型元素紧密对应。同时也是模型与实现协同开发的过程，确保二者始终保持一致。

### modeling paradigm
一种特定的领域概念划分方式，结合工具将这些概念转化为软件对应物（例如面向对象编程和逻辑编程）。

### REPOSITORY
一种封装存储、检索和搜索行为的机制，模拟对象集合的行为。

### responsibility
执行任务或掌握信息的义务（ [Wirfs-Brock 等，2003](references.md#wirfs-brock-2003)，第 3 页）。

### SERVICE
作为接口提供的操作，在模型中独立存在，不包含封装状态。

### side effect
操作导致的任何可观察状态变化，无论是否有意为之，甚至包括蓄意更新。

### SIDE-EFFECT-FREE FUNCTION 
参见 [function](#function)

### STANDALONE CLASS
无需参照其他类即可理解和测试的类，仅依赖系统基本操作和基础库。

### stateless
设计元素的特性，允许客户端使用其任何操作而不考虑该元素的历史状态。无状态元素可能使用全局可访问的信息，甚至可能改变该全局信息（即可能产生副作用），但不持有影响其行为的私有状态。

### strategic design
适用于系统大部分模块的建模与设计决策。此类决策影响整个项目，需在团队层面作出。

### supple design
一种将深度模型内在能力赋予客户端开发者的设计方案，使其能够创建清晰灵活的表达式，以稳健方式实现预期结果。同样重要的是，它利用相同的深度模型使设计本身易于实施者塑形重构，从而适应新见解。

### UBIQUITOUS LANGUAGE
一种围绕领域模型构建的语言，由所有团队成员使用，用于将团队的所有活动与软件相连接。

### unification
模型的内部一致性，确保每个术语都具有唯一性且规则之间不存在矛盾。

### VALUE OBJECT
描述某些特征或属性但不携带身份概念的对象。

### WHOLE VALUE
表示单一完整概念的对象。

#### ▶[下一节](references.md)
