# 模块化单体：架构驱动因素

[原文](https://www.kamilgrzybek.com/blog/posts/modular-monolith-architectural-drivers) 📂 架构和设计 📂 模块化单体  2019-12-26

![top](./imgs/Modular_Monolith_Architectural_Drivers_Promo-825x510.jpg)

## 引言

在 [第一篇](./primer.md) 关于模块化单体架构的文章中，我重点介绍了该架构的定义和模块化的描述。
提醒一下，*模块化单体 (Modular Monolith)*：

- 是一个恰好只有一个部署单元的系统
- 是对以模块化方式设计的单体系统的明确命名
- 模块化意味着模块：
  - 必须独立、自治
  - 拥有提供所需功能的一切（按业务领域分离）
  - 被封装并具有定义良好的接口/契约

在这篇文章中，我想讨论一些在我看来最为常见的 *架构驱动因素 (Architectural Drivers)* ，它们可能导向 *模块化单体 (Modular Monolith)* 或 *微服务 (Microservices)* 架构。

但 *架构驱动因素 (Architectural Drivers)* 到底是什么？

## 架构驱动因素

一般来说，你不能说 X 架构比另一种更好。
你不能说单体比微服务更好，[整洁架构 (Clean Architecture)](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html) 比 [分层架构](https://www.oreilly.com/library/view/software-architecture-patterns/9781491971437/ch01.html) 更好，
3 层比 4 层更好或更差，等等。

同样的规则也适用于其他考量，例如 ORM vs 原生 SQL、
“*当前状态 (Current State)*” 持久化 vs [事件溯源 (Event Sourcing)](https://martinfowler.com/eaaDev/EventSourcing.html)、
[贫血领域模型 (Anemic Domain Model)](https://www.martinfowler.com/bliki/AnemicDomainModel.html) vs 富领域模型、
面向对象设计 vs 函数式编程……以及更多。

那么，如果（不幸的是）没有 “最好” 的存在，我们该如何选择架构/方法/范式/工具/库呢？

### 上下文为王

我们的 **每一个决策都是在特定的上下文** 中做出的。
每个项目都是不同的（这源于项目的定义），因此每个上下文也是不同的。
这意味着<ins>在一种上下文中做出的相同决策可能带来极好的结果，而在另一种上下文中则可能导致毁灭性的失败</ins>。
因此，不加批判地使用他人/其他公司的方法，可能会导致大量痛苦、金钱浪费，并最终导致项目的终结。

![Every Project is different and has different Context](./imgs/Modular-Monolith_Contexts-1-1024x321.jpg)
*每个项目都是不同的，并具有不同的上下文*

