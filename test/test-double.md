# 测试替身（Test Double）

| [Martin Fowler](https://martinfowler.com/)| |
|:---|:---|
|<img src="../tools/img/mf.jpg" width="20%" />| |
|[原文](https://martinfowler.com/bliki/TestDouble.html)| 2006/1/17|

🔖 测试

---
[Gerard Meszaros 正在撰写](https://martinfowler.com/books/meszaros.html) 一本关于使用各种 [Xunit](https://martinfowler.com/bliki/Xunit.html) 框架模式的书。
他遇到的一个棘手问题是，对于 stubs、mocks、fakes、dummies 以及人们用于在测试中替换系统部分的各种其他术语，名称各异。
为了应对这个问题，他提出了自己的术语体系，我认为值得进一步传播。

他使用的通用术语是 [Test Double](http://xunitpatterns.com/Test%20Double.html)（试想电影中的特技替身）。
Test Double 是一个通用术语，指任何为了测试目的而替换生产对象的情况。
Gerard 列出了几种不同类型的替身：

- **Dummy**：
被传递但从未被实际使用。通常仅用于填充参数列表。

- **Fake**：
确实有工作实现，但通常采取某种捷径，使其不适合生产环境（例如，[内存中的测试数据库](https://martinfowler.com/bliki/InMemoryTestDatabase.html) 是一个很好的例子）。

- **Stub**：
为测试期间发出的调用提供预设的答案，通常对编程中设定的测试之外的任何内容都不做响应。

- **Spy**：
一种 Stub，同时根据调用方式记录一些信息。
例如，一个电子邮件服务记录发送给它的消息数量。

- **Mock**：
预先编程了期望，这些期望形成了它们预期接收的调用规范。
如果收到意外调用，它们可以抛出异常，并在验证期间进行检查，以确保它们收到了所有预期调用。

## 延伸阅读

我在 [Mocks Aren't Stubs](https://martinfowler.com/articles/mocksArentStubs.html) 一文中进一步阐述了 Mocks、Doubles 等的使用。
