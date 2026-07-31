# 设计优先方法论详解

5 级递进式设计方法论的扩展参考。用于符号指导、接口模式、理解每个层级好与差的输出。

## 各层级好与差的输出

### Level 1：能力

**好**——有范围的、面向用户的、无实现细节：
1. 用户在其订单发货时收到邮件通知
2. 用户可在其账户中查看通知历史
3. 失败的投递自动重试

**差**——泄露实现细节或范围蔓延：命名技术（属于 Level 2+）、描述内部机制（属于 Level 3+）、添加未请求的功能。

### Level 2：组件

**好**——具有单一职责的命名构建块：
- **NotificationHandler**：接收通知请求，验证载荷，排队等待投递
- **EmailDeliveryWorker**：处理排队的通知，通过配置的提供商发送
- **DeliveryTracker**：记录投递状态，为用户查询提供历史

**差**——包括交互模式或实现：描述组件如何对话（Level 3）及如何存储数据（Level 5）。

### Level 3：交互

**好**——组件之间传递什么，而非如何传递：
1. Controller → NotificationHandler：`NotificationRequest`（收件人、模板、变量）
2. NotificationHandler → Queue：`DeliveryJob`（提供商、收件人、渲染内容）
3. Queue → EmailDeliveryWorker：`DeliveryJob`
4. EmailDeliveryWorker → DeliveryTracker：`DeliveryResult`（状态、时间戳、失败时的错误）

### Level 4：契约

**好**——类型化接口，无函数体。包括接口定义、类型定义，反映 Level 3 中约定的交互。

## 顺序图符号

对于 Level 3 交互，使用 ASCII 或 Mermaid。两者均可接受，选择对特定设计更清晰的。

## 接口定义模式

Level 4 契约应为：**最小化**（仅需形式化约定交互的接口）、**自文档化**（类型名和方法名不言自明）、**与 Level 3 对齐**、**语言适配**。
