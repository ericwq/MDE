# Mocks 不是 Stubs

“Mock Objects" 这一术语已经成为一个流行词，用于描述在测试中模仿真实对象的特例对象。
大多数语言环境现在都有框架可以轻松创建 `mock objects`。
然而，人们常常没有意识到的是，`mock objects` 只是特例测试对象的一种形式，它支持一种不同的测试风格。
在本文中，我将解释 `mock objects` 如何工作，它们如何鼓励基于行为验证的测试，以及围绕它们的社区如何使用它们来发展一种不同的测试风格。

| [Martin Fowler](https://martinfowler.com/)| |
|:---|:---|
|<img src="../tools/img/mf.jpg" width="20%" />| |
|[原文](https://martinfowler.com/articles/mocksArentStubs.html)| 2007/1/2|

🔖 测试 

---

我第一次接触到 “mock object” 这个术语是在几年前，在 [极限编程](https://martinfowler.com/bliki/ExtremeProgramming.html)（XP）社区中。
从那以后，我越来越多地遇到 mock objects。
部分原因是，mock objects 的许多主要开发者曾是我在 Thoughtworks 不同时期的同事。
部分原因是，我在受 XP 影响的测试文献中越来越频繁地看到它们。

但我也经常看到 mock objects 被描述得很糟糕。
特别是，我经常看到它们与 stubs 相混淆 —— stubs 是测试环境中一种常见的辅助工具。
我理解这种混淆 —— 我自己也曾一度认为它们是相似的，但与 mock 开发者的对话逐渐让一些关于 mock 的理解穿透了我那坚硬的龟壳脑颅。

<ins>这种差异实际上是两种不同的差异。
一方面，在测试结果如何验证上存在差异：即状态验证和行为验证之间的区别。
另一方面，则是一种关于测试与设计如何协同的完全不同的理念，我在此称之为 [测试驱动开发](https://martinfowler.com/bliki/TestDrivenDevelopment.html) 的经典风格和 mockist 风格</ins>。

## 常规测试

我将通过一个简单的例子来说明这两种风格。
（示例使用 Java，但原理适用于任何面向对象语言。）
我们想要获取一个订单对象并从仓库 (warehouse) 对象中填充它。
订单非常简单，只有一种产品和一个数量。
仓库保存不同产品的库存。
当我们要求订单从仓库中自行填充时，有两种可能的响应。
如果仓库中有足够的产品来填充订单，则订单被标记为已填充，并且仓库中该产品的数量会相应减少。
如果仓库中没有足够的产品，则订单不被填充，仓库中也不会发生任何变化。

这两种行为意味着几个测试，看起来像是相当常规的 JUnit 测试。

```java
public class OrderStateTester extends TestCase {
  private static String TALISKER = "Talisker";
  private static String HIGHLAND_PARK = "Highland Park";
  private Warehouse warehouse = new WarehouseImpl();

  protected void setUp() throws Exception {
    warehouse.add(TALISKER, 50);
    warehouse.add(HIGHLAND_PARK, 25);
  }
  public void testOrderIsFilledIfEnoughInWarehouse() {
    Order order = new Order(TALISKER, 50);
    order.fill(warehouse);
    assertTrue(order.isFilled());
    assertEquals(0, warehouse.getInventory(TALISKER));
  }
  public void testOrderDoesNotRemoveIfNotEnough() {
    Order order = new Order(TALISKER, 51);
    order.fill(warehouse);
    assertFalse(order.isFilled());
    assertEquals(50, warehouse.getInventory(TALISKER));
  }
}
```

xUnit 测试遵循典型的四阶段序列：setup、exercise、verify、teardown。
在本例中，setup 阶段部分在`setUp`方法中完成（设置仓库），部分在测试方法中完成（设置订单）。
对`order.fill`的调用是 exercise 阶段，即在此阶段我们促使对象执行我们想要测试的行为。
`assert`语句则是 verification 阶段，检查被调用的方法是否正确完成了其任务。
在本例中没有显式的 teardown 阶段，垃圾回收器为我们隐式地完成了这项工作。

在 setup 过程中，我们组合了两种对象。
`Order`是我们正在测试的类，但要使`Order.fill`工作，我们还需要一个`Warehouse`的实例。
在这种情况下，`Order`是我们关注测试的对象。
测试人员喜欢使用类似`object-under-test`或`system-under-test`这样的术语来指代此类对象。
这两个术语说起来都很拗口，但既然它们是广泛接受的术语，我就勉强使用。
遵循 Meszaros 的用法，我将使用 *System Under Test（SUT）* ，或其缩写形式。

因此，对于这个测试，我需要 SUT（`Order`）和一个协作者（`warehouse`）。
我需要仓库有两个原因：一是为了让被测试的行为能够工作（因为`Order.fill`会调用仓库的方法）；
二是为了验证（因为`Order.fill`的结果之一是对仓库状态的潜在更改）。
随着我们进一步探讨这个话题，你会看到我们会非常重视 SUT 和协作者之间的区分。
（在本文早期版本中，我将 SUT 称为 “主要对象”，协作者称为 “次要对象”。）

这种测试风格使用 **状态验证（state verification）** ：即通过检查 SUT 及其协作者在方法执行后的状态，来确定被调用的方法是否正确工作。
正如我们将看到的，mock objects 提供了一种不同的验证方法。

## 使用 Mock Objects 的测试

现在我将针对相同的行为使用 mock objects。
对于这段代码，我使用的是 jMock 库来定义 mock。
jMock 是一个 Java mock object 库。
虽然还有其他 mock object 库，但这个是该技术创始者编写的最新库，因此是一个不错的起点。

```java
public class OrderInteractionTester extends MockObjectTestCase {
  private static String TALISKER = "Talisker";

  public void testFillingRemovesInventoryIfInStock() {
    //setup - data
    Order order = new Order(TALISKER, 50);
    Mock warehouseMock = new Mock(Warehouse.class);
    
    //setup - expectations
    warehouseMock.expects(once()).method("hasInventory")
      .with(eq(TALISKER),eq(50))
      .will(returnValue(true));
    warehouseMock.expects(once()).method("remove")
      .with(eq(TALISKER), eq(50))
      .after("hasInventory");

    //exercise
    order.fill((Warehouse) warehouseMock.proxy());
    
    //verify
    warehouseMock.verify();
    assertTrue(order.isFilled());
  }

  public void testFillingDoesNotRemoveIfNotEnoughInStock() {
    Order order = new Order(TALISKER, 51);    
    Mock warehouse = mock(Warehouse.class);
      
    warehouse.expects(once()).method("hasInventory")
      .withAnyArguments()
      .will(returnValue(false));

    order.fill((Warehouse) warehouse.proxy());

    assertFalse(order.isFilled());
  }
```

先集中看 `testFillingRemovesInventoryIfInStock`，我在后面的测试中做了一些简化处理。

首先，setup 阶段非常不同。
首先，它分为两部分：数据和期望。
数据部分设置我们感兴趣的对象，在这个意义上，它类似于传统的 setup。
区别在于所创建的对象。
SUT 是相同的 —— 一个订单。
然而，协作者不是一个仓库对象，而是一个 mock warehouse —— 从技术上讲，是`Mock`类的一个实例。

setup 的第二部分是在 mock 对象上设置 *期望（expectations）* 。
期望表明当 SUT 被执行时，应该在 mock 上调用哪些方法。

一旦所有期望都设置好，我执行 SUT。
执行之后，我进行验证，这有两个方面。
我对 SUT 运行断言 —— 与之前非常相似。
然而，我也验证 mock —— 检查它们是否按照其期望被调用。

这里的关键区别在于，我们如何验证订单在与仓库交互时做了正确的事情。
使用 *状态验证（state verification）* 时，我们通过对仓库状态进行断言来做到这一点。
而 mock objects 使用 *行为验证（behavior verification）*，我们转而检查订单是否对仓库进行了正确的调用。
我们通过在 setup 期间告诉 mock 期望什么，并在验证期间要求 mock 自行验证来执行此检查。
只有订单使用断言进行检查，如果该方法不改变订单的状态，则根本没有断言。

在第二个测试中，我做了几件不同的事情。
首先，我以不同的方式创建 mock，使用`MockObjectTestCase`中的`mock`方法，而不是构造函数。
这是 jMock 库中的一个便捷方法，意味着我不需要在之后显式调用`verify`，任何使用便捷方法创建的 mock 都会在测试结束时自动验证。
我本可以在第一个测试中也这样做，但我想要更明确地展示验证过程，以展示使用 mocks 进行测试的工作方式。

第二个测试用例中的第二个不同之处在于，我使用`withAnyArguments`放宽了对期望的约束。
原因在于，第一个测试检查了传递给仓库的数字是否正确，因此第二个测试无需重复该测试元素。
如果以后需要更改订单的逻辑，那么只有一个测试会失败，从而减轻迁移测试的工作量。
事实证明，我完全可以省略`withAnyArguments`，因为这是默认行为。

### 使用 EasyMock

目前有许多 mock object 库。
我经常遇到的一个是 EasyMock，包括其 Java 和 .NET 版本。
EasyMock 也支持行为验证，但在风格上与 jMock 有一些差异，值得讨论。
以下是同样的测试：

```java
public class OrderEasyTester extends TestCase {
  private static String TALISKER = "Talisker";
  
  private MockControl warehouseControl;
  private Warehouse warehouseMock;
  
  public void setUp() {
    warehouseControl = MockControl.createControl(Warehouse.class);
    warehouseMock = (Warehouse) warehouseControl.getMock();    
  }

  public void testFillingRemovesInventoryIfInStock() {
    //setup - data
    Order order = new Order(TALISKER, 50);
    
    //setup - expectations
    warehouseMock.hasInventory(TALISKER, 50);
    warehouseControl.setReturnValue(true);
    warehouseMock.remove(TALISKER, 50);
    warehouseControl.replay();

    //exercise
    order.fill(warehouseMock);
    
    //verify
    warehouseControl.verify();
    assertTrue(order.isFilled());
  }

  public void testFillingDoesNotRemoveIfNotEnoughInStock() {
    Order order = new Order(TALISKER, 51);    

    warehouseMock.hasInventory(TALISKER, 51);
    warehouseControl.setReturnValue(false);
    warehouseControl.replay();

    order.fill((Warehouse) warehouseMock);

    assertFalse(order.isFilled());
    warehouseControl.verify();
  }
}
```

EasyMock 使用 *记录/回放（record/replay）* 模式来设置期望。
对于你想要 mock 的每个对象，你创建一个控制对象和一个 mock 对象。
mock 满足协作者的接口，控制对象则为你提供额外的功能。
要指示一个期望，你调用 mock 上的方法，并传入你期望的参数。
如果你想要一个返回值，紧接着调用控制对象的相应方法。
一旦你完成期望的设置，你在控制对象上调用`replay` —— 此时 mock 完成记录并准备好响应主要对象。
完成后，你在控制对象上调用`verify`。

虽然人们通常在第一眼看到记录/回放模式时会感到困惑，但他们会很快习惯。
<ins>与 jMock 的约束相比，它有一个优势，即你对 mock 进行的是实际的方法调用，而不是在字符串中指定方法名称</ins>。
这意味着你可以在 IDE 中使用代码补全，并且任何方法名称的重构都会自动更新测试。
缺点是你不能有更宽松的约束。

jMock 的开发人员正在开发一个新版本，该版本将使用其他技术来允许你使用实际的方法调用。

## Mocks 与 Stubs 的区别

当 mock objects 首次被引入时，许多人很容易将它们与测试中常见的 stubs 概念相混淆。
从那时起，人们似乎对它们之间的差异有了更好的理解（我希望本文的早期版本对此有所帮助）。
然而，要完全理解人们如何使用 mocks，理解 mocks 和其他类型的测试替身 (test doubles) 是很重要的。
（“替身 (doubles)” ？如果这对你来说是一个新术语，不用担心，稍等几段一切都会清楚。）

当你进行这样的测试时，你一次只关注软件的一个元素 —— 因此通常称为单元测试。
问题在于，要让单个单元工作，你通常需要其他单元 —— 因此在我们示例中需要某种仓库。

在我上面展示的两种测试风格中，第一种情况使用了一个真实的仓库对象，第二种情况使用了一个 mock 仓库，它当然不是一个真实的仓库对象。
使用 mocks 是在测试中不使用真实仓库的一种方式，但在这类测试中还有其他形式的非真实对象。

讨论这个话题的词汇很快会变得混乱 —— 各种词语都被使用：stub、mock、fake、dummy。
在本文中，我将遵循 Gerard Meszaros 书中的词汇。
这不是每个人都使用的，但我认为这是一个好的词汇，而且由于这是我的文章，我可以选择使用哪些词。

Meszaros 使用 **Test Double（测试替身）** 作为任何用于测试目的、代替真实对象的伪装对象的通用术语。
这个名字来源于电影中的 “特技替身 (Stunt Double)” 的概念。
（他的目标之一是避免使用任何已被广泛使用的名称。）
Meszaros 随后定义了五种特定类型的替身：

- **Dummy**：被传递但从未被实际使用。通常仅用于填充参数列表。

- **Fake**：确实有工作实现，但通常采取某种捷径，使其不适合生产环境（ [内存数据库](./test-database.md) 是一个很好的例子）。

- **Stub**：为测试期间发出的调用提供预设的答案，通常对编程中设定的测试之外的任何内容都不做响应。

- **Spy**：一种存根，同时根据调用方式记录一些信息。例如，一个电子邮件服务记录发送给它的消息数量。

- **Mock**：即我们在这里讨论的对象：预先编程了期望，这些期望形成了它们预期接收的调用规范。

<ins>在这些类型的替身中，只有 mock objects 坚持使用 *行为验证* 。
其他替身可以，而且通常确实使用 *状态验证* </ins>。
mock objects 在执行阶段实际上确实表现得像其他替身一样，
因为它们需要让 SUT 相信它正在与其真实的协作者通信 —— 但 mocks 在 setup 和 verification 阶段有所不同。

为了进一步探讨测试替身，我们需要扩展我们的示例。
许多人只在真实对象难以使用时才使用测试替身。
一个更常见的测试替身场景是，如果我们规定在填充订单失败时发送一封电子邮件。
问题在于，我们不想在测试期间向客户发送实际的电子邮件。
因此，我们为邮件系统创建一个测试替身，一个我们可以控制和操纵的替身。

在这里，我们可以开始看到 mocks 和 stubs 之间的区别。
如果我们要为这个邮件行为编写测试，我们可以编写一个像这样的简单 stub。

```java
public interface MailService {
  public void send (Message msg);
}

public class MailServiceStub implements MailService {
  private List<Message> messages = new ArrayList<Message>();
  public void send (Message msg) {
    messages.add(msg);
  }
  public int numberSent() {
    return messages.size();
  }
}
```

然后我们可以像这样在 stub 上使用状态验证。

```java
class OrderStateTester...
  public void testOrderSendsMailIfUnfilled() {
    Order order = new Order(TALISKER, 51);
    MailServiceStub mailer = new MailServiceStub();
    order.setMailer(mailer);
    order.fill(warehouse);
    assertEquals(1, mailer.numberSent());
  }
```

当然，这是一个非常简单的测试 —— 仅测试消息已被发送。
我们尚未测试它是否发送给了正确的人，或包含正确的内容，但这足以说明问题。

使用 mocks 时，这个测试看起来会相当不同。

```java
class OrderInteractionTester...
  public void testOrderSendsMailIfUnfilled() {
    Order order = new Order(TALISKER, 51);
    Mock warehouse = mock(Warehouse.class);
    Mock mailer = mock(MailService.class);
    order.setMailer((MailService) mailer.proxy());

    mailer.expects(once()).method("send");
    warehouse.expects(once()).method("hasInventory")
      .withAnyArguments()
      .will(returnValue(false));

    order.fill((Warehouse) warehouse.proxy());
  }
}
```

<ins>在这两种情况下，我都使用测试替身来代替真正的邮件服务。
区别在于，stub 使用 *状态验证* ，而 mock 使用 *行为验证* </ins>。

为了在 stub 上使用状态验证，我需要在 stub 上添加一些额外的方法来辅助验证。
因此，stub 实现了 `MailService`，但增加了额外的测试方法。

Mock objects 总是使用行为验证，而 stub 可以任选其一。
Meszaros 将使用行为验证的 stub 称为 *Test Spy（测试间谍）* 。
区别在于替身具体如何运行和验证，我将留给你自己去探索。

## 经典测试与 Mockist 测试

现在我可以探讨第二种二分法：经典测试与 Mockist TDD 之间的区别。
这里的关键问题在于何时使用 mock（或其他替身）。

<ins>**经典 TDD** 风格是尽可能使用真实对象，只有在真实对象难以使用时才使用替身。
因此，经典 TDD 实践者会使用真实的仓库，而为邮件服务使用替身。
替身的类型实际上并不那么重要</ins>。

<ins>然而，**Mockist TDD** 实践者会为任何具有有趣行为的对象始终使用 mock。
在这种情况下，仓库和邮件服务都会使用 mock </ins>。

尽管各种 mock 框架是为 mockist 测试而设计的，但许多经典派也发现它们对创建替身很有用。

Mockist 风格的一个重要分支是 [行为驱动开发](http://dannorth.net/introducing-bdd/)（BDD）。
BDD 最初由我的同事 Daniel Terhorst-North 开发，作为一种通过关注 TDD 如何作为设计技术运作来更好地帮助人们学习 TDD 的技术。
这导致将测试重命名为行为，以更好地探索 TDD 在思考对象需要做什么方面的帮助。
BDD 采用 mockist 方法，但它在此基础上进行了扩展，包括其命名风格，以及将分析整合到其技术中的愿望。
我在这里不再深入讨论，因为与本文唯一相关的是，BDD 是另一种倾向于使用 mockist 测试的 TDD 变体。
我将留给你通过链接获取更多信息。

你有时会看到 “Detroit 风格” 用于指代 “经典”，“London 风格” 用于指代 “mockist”。
这暗示了 XP 最初是在底特律的 C3 项目中开发的，而 mockist 风格是由伦敦的早期 XP 采用者开发的。
我还应提到，许多 mockist TDD 实践者不喜欢这个术语，实际上不喜欢任何暗示经典风格和 mockist 风格之间存在差异的术语。
他们认为在这两种风格之间做出有用的区分是没有必要的。

## **在差异之间做出选择**

在本文中，我解释了两组差异：状态验证与行为验证 / 经典TDD与Mockist TDD。在它们之间做出选择时，需要考虑哪些论据？我将从状态验证与行为验证的选择开始。

首先要考虑的是上下文。我们是在考虑一个容易的协作，如订单和仓库，还是一个困难的协作，如订单和邮件服务？

如果是一个容易的协作，那么选择很简单。如果我是一个经典TDD实践者，我不使用mock、stub或任何类型的替身。我使用真实对象和状态验证。如果我是一个Mockist TDD实践者，我使用mock和行为验证。完全没有决策。

如果是一个困难的协作，那么如果我是Mockist，就没有决策——我只使用mocks和行为验证。如果我是经典派，那么我确实有选择，但选择哪个并不重要。通常经典派会根据具体情况逐案决定，为每种情况使用最便捷的路径。

因此，正如我们所看到的，状态验证与行为验证大多不是一个重要的决策。真正的问题在于经典TDD与Mockist TDD之间。事实证明，状态验证和行为验证的特性确实会影响该讨论，而这也将是我主要精力集中的地方。

但在进行之前，让我提一个边缘情况。偶尔你会遇到一些即使不是困难协作，也难以使用状态验证的东西。一个很好的例子是缓存。缓存的全部意义在于你无法从其状态中判断是缓存命中还是未命中——这是一个即使对于铁杆经典TDD实践者来说，行为验证也是明智选择的情况。我相信在两个方向上还有其他例外。

当我们深入探讨经典/Mockist的选择时，有很多因素需要考虑，因此我将它们大致分组。
