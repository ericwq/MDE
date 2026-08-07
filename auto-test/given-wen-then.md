# Given When Then

| [Martin Fowler](https://martinfowler.com/)| |
|:---|:---|
|<img src="../tools/img/mf.jpg" width="20%" />| |
|[原文](https://martinfowler.com/bliki/GivenWhenThen.html)| 2013/8/21|

🔖 测试 🔖 领域特定语言

---
Given-When-Then 是一种表示测试的风格 ——或者如它的倡导者所说—— 通过 [SpecificationByExample](https://martinfowler.com/bliki/SpecificationByExample.html) 来指定系统行为。
这是由 [Daniel Terhorst-North](http://dannorth.net/about/) 和 Chris Matts 在 [行为驱动开发](http://dannorth.net/introducing-bdd/)（BDD）中提出的一种方法。¹
它作为许多测试框架（如 Cucumber）的结构化方法出现。
你也可以将其视为 [四阶段测试](http://xunitpatterns.com/Four%20Phase%20Test.html) 模式的一种重构 (reformulation)。

其核心思想是将编写场景（或测试）分解为三个部分：

- **Given（给定）** 部分描述在你开始指定该场景中的行为之前，世界的初始状态。
你可以将其视为测试的前置条件。

- **When（当）** 部分是你所指定的行为。

- **Then（那么）** 部分描述由于指定行为而期望发生的变化。

既然我们讨论的是使用示例作为规格说明，那么通过一个示例来展示这一点是合理的。²

**示例：**

```
Feature: User trades stocks
  Scenario: User requests a sell before close of trading
    Given I have 100 shares of MSFT stock
       And I have 150 shares of APPL stock
       And the time is before close of trading

    When I ask to sell 20 shares of MSFT stock
     
    Then I should have 80 shares of MSFT stock
      And I should have 150 shares of APPL stock
      And a sell order for 20 shares of MSFT stock should have been executed
```

上面的示例使用了 Cucumber ³，这是编写 [业务面测试](https://martinfowler.com/bliki/BusinessFacingTest.html) 的一种流行方式，但你可以在任何类型的测试中使用 Given-When-Then 风格。
有些人喜欢将 Given-When-Then 作为注释放在单元测试中来标记非正式块 ⁴。
我也看到过这种约定用于组织非正式散文。

在这种方法中，通常会看到使用 “And” 来组合每个子句中的多个表达式。

我将 Given 描述为前置条件状态的描述，因为这是我更喜欢的思考方式。
然而，测试框架将 Given 解释为一组命令，用于在执行 When 命令之前将被测系统置于正确的状态。
（这也是为什么其他命名惯例通常称之为 “setup” 的原因。）测试框架为 Then 命令提供各种查询方法 —— 这些方法应无副作用。

尽管 Given-When-Then 风格是 BDD 的典型特征，但在编写测试或通过示例进行规格说明时，其基本思想相当普遍。
[Meszaros](https://martinfowler.com/books/xunit) 将此模式描述为 [四阶段测试](http://xunitpatterns.com/Four%20Phase%20Test.html)。
他的四个阶段是 Setup（Given）、Exercise（When）、Verify（Then）和 Teardown ⁵。
Bill Wake 则提出了 [Arrange、Act、Assert](http://xp123.com/articles/3a-arrange-act-assert/) 的表述。

### 注释：

¹ 在对此的审阅评论中，Dan 提到 Ivan Moore 在提出这一方法时给予了大量灵感。

² 来自 [Pete Hodgson](http://blog.thepete.net/) 。

³ 或者严格来说，它使用的是 Gherkin，这是 Cucumber 的 DSL 名称。

⁴ 测试框架倾向于遵循 xunit 或 BDD 的命名风格，后者倾向于以Given-When-Then风格命名方法。

⁵ Teardown 在实现测试时并不总是必需的（特别是如果你使用的是 [Automated Teardown](http://xunitpatterns.com/Automated%20Teardown.html) ），并且对通过示例进行规格说明的沟通方面没有太大贡献。
因此，在 BDD 风格中看不到它是合理的。
