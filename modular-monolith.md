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
