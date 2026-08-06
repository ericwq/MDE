# 皮下测试（Subcutaneous Test）

| [Martin Fowler](https://martinfowler.com/)| |
|:---|:---|
|<img src="../tools/img/mf.jpg" width="20%" />| |
|[原文](https://martinfowler.com/bliki/SubcutaneousTest.html)| 2011/2/14|

🔖 测试分类

---

我使用 “皮下测试” 一词来表示在应用程序用户界面（UI）之下直接进行的测试。
当进行应用程序的功能测试时，这种方法尤其有价值：即当你想要测试端到端的行为，但通过 UI 本身进行测试很困难时。

皮下测试可以避免难以测试的表示层技术带来的困难，而且通常比通过 UI 进行测试快得多。
最大的风险在于，除非你坚定地遵循将所有业务逻辑都放在 UI 之外的原则，否则皮下测试可能会遗漏重要的行为。
