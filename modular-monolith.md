## 模块化单体系列

---
## 模块化单体：集成风格
2019-07-26 📂 架构和设计 📂 模块化单体
![integration](./mmonolith/imgs/modular_monolith_integration_styles_logo-825x510.jpg)

在这篇文章中，我只想讨论缺失的部分 —— 模块化单体架构中模块的集成风格。

[更多内容](./mmonolith/integration.md)

---

## 模块化单体：架构实施
2020-03-22 📂 模块化单体
![enforcement](./mmonolith/imgs/Monolith_arch_enforcement-825x510.jpg)

在之前的文章中，我们讨论了什么是模块化单体架构，以及可能影响其选择的架构驱动因素。
在这篇文章中，我想重点讨论强制实施所选架构的方法。

[更多内容](./mmonolith/enforcement.md)

---

## 模块化单体：架构驱动因素

2019-12-26 📂 架构和设计 📂 模块化单体
![driver](./mmonolith/imgs/Modular_Monolith_Architectural_Drivers_Promo-825x510.jpg)

在第一篇关于模块化单体架构的文章中，我重点介绍了该架构的定义和模块化的描述。
在这篇文章中，我想讨论一些在我看来最为常见的 *架构驱动因素 (Architectural Drivers)* ，
它们可能导向 *模块化单体 (Modular Monolith)* 或 *微服务 (Microservices)* 架构。

[更多内容](./mmonolith/drivers.md)

---

## 模块化单体：入门指南

2019-12-02 📂 架构和设计 📂 模块化单体
![primer](./mmonolith/imgs/Modular_Monolith_a_Primer-825x510.jpg)

自微服务架构兴起以来，许多年已经过去，它仍然是系统架构背景下讨论的主要话题之一。
云解决方案、容器化以及支持分布式系统开发和维护的先进工具（如 Kubernetes）的普及，进一步促进了这一现象。

观察社区、公司中正在发生的事情，以及与程序员的对话，可以得出结论：大多数新项目都在使用微服务架构来实现。
此外，一些遗留系统也在向这种方法迁移。

[更多内容](./mmonolith/primer.md)

---

# Kamil Grzybek 权限/授权相关文章完整链接
官网主页：https://www.kamilgrzybek.com/
博客总入口：https://www.kamilgrzybek.com/blog

## 一、三篇核心授权专题（DDD+Clean Architecture 权限必读）
### 1. Authorization in Domain-Driven Design（区分应用层鉴权 / 领域业务规则，PermissionChecker 标准实现）
https://www.kamilgrzybek.com/blog/posts/authorization-in-domain-driven-design

### 2. Identity as a Bounded Context（身份认证授权独立限界上下文、ACL防腐层、模块化单体权限拆分）
https://www.kamilgrzybek.com/blog/posts/identity-as-a-bounded-context

### 3. Resource-Based Authorization with DDD Aggregates（聚合根行级数据权限、资源归属授权 ReBAC 落地）
https://www.kamilgrzybek.com/blog/posts/resource-based-authorization-with-ddd-aggregates

## 二、配套相关延伸文章（架构基础，支撑权限分层理解）
1. Modular Monolith: Domain-Centric Design（模块化单体+Clean分层，鉴权分层位置）
https://www.kamilgrzybek.com/blog/posts/modular-monolith-domain-centric-design

2. Domain Model Validation（区分**领域内业务校验**和**应用层权限校验**，避坑核心）
https://www.kamilgrzybek.com/blog/posts/domain-model-validation

3. Modular Monolith: Integration Styles（业务模块与Identity权限上下文通信方式）
https://www.kamilgrzybek.com/blog/posts/modular-monolith-integration-styles

## 三、配套开源代码仓库（完整权限实现源码）
ModularMonolithWithDDD（包含完整Identity限界上下文、Policy鉴权、权限校验用例）
https://github.com/kgrzybek/modular-monolith-with-ddd

## 四、补充说明
1. 所有文章均为英文，无官方中文翻译；
2. 三篇授权专题是行业公认 DDD+Clean 权限落地标杆，明确规范：
   - 领域层**不碰**用户、角色、权限；
   - 应用用例层统一执行授权校验；
   - 身份/权限单独作为通用支撑限界上下文；
3. 访问如打不开可尝试浏览器无痕模式，网站无墙。
