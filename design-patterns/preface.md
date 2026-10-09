# 前言

本书不是面向对象技术或设计的入门书。
许多书已经很好地做到了这一点。
本书假设你至少相当熟练地掌握一种面向对象编程语言，而且你也应该有一些面向对象设计的经验。
当我们提到 “类型” 和 “多态”，或者与 “实现” 继承相对的 “接口” 时，你绝对不应该需要赶紧去查最近的词典。

另一方面，这也不是一本高级技术论著。
它是一本设计模式的书，描述面向对象软件设计中特定问题的简单而优雅的解决方案。
设计模式捕捉的是随着时间发展和演化出来的解决方案。
因此它们不是人们最初倾向于生成的设计。
它们反映了开发者为了在软件中获得更大的复用性和灵活性，而进行的无数重新设计和反复修改。
设计模式以简洁且易于应用的形式捕捉这些解决方案。

这些设计模式既不需要不寻常的语言特性，也不需要惊人的编程技巧来让你的朋友和管理者惊叹。
它们都可以用标准面向对象语言实现，尽管可能比临时解决方案要多花一点功夫。
但额外的努力总会在增加的灵活性和可复用性上带来回报。

一旦你理解了这些设计模式，并对它们有过 “啊哈！”（而不只是 “嗯？”）的体验，你就再也不会以同样的方式思考面向对象设计了。
你将拥有一些洞见，能让你自己的设计更灵活、更模块化、更可复用、更易理解 —— 而这正是你最初对面向对象技术感兴趣的原因，对吧？

<ins>一句警告和鼓励：如果你第一遍读这本书时没有完全理解，不要担心。
我们第一次写的时候也没有完全理解！
记住，这不是一本读一遍就放到书架上的书。
我们希望你会发现自己一次又一次地翻阅它，以获取设计洞见和灵感。</ins>

这本书经历了漫长的孕育。
它见证了四个国家、其中三位作者的婚姻，以及两个（互不相关的）后代的诞生。
许多人在它的发展中出了一份力。
特别感谢 Bruce Anderson、Kent Beck 和 Andre Weinand 的启发和建议。
我们还感谢那些审阅手稿草稿的人：Roger Bielefeld、Grady Booch、Tom Cargill、Marshall Cline、Ralph Hyre、Brian Kernighan、Thomas Laliberty、Mark Lorenz、Arthur Riel、Doug Schmidt、Clovis Tondo、Steve Vinoski 和 Rebecca Wirfs-Brock。
我们也感谢 Addison-Wesley 团队的帮助和耐心：Kate Habib、Tiffany Moore、Lisa Raffaele、Pradeepa Siva 和 John Wait。
特别感谢 IBM Research 的 Carl Kessler、Danny Sabbah 和 Mark Wegman 对这项工作的不懈支持。

最后但当然同样重要的是，我们感谢互联网及其他地方所有对模式版本发表评论、给予鼓励之词、并告诉我们我们所做之事值得的人。
这些人包括但不限于 Jon Avotins、Steve Berczuk、Julian Berdych、Matthias Bohlen、John Brant、Allan Clarke、Paul Chisholm、Jens Coldewey、Dave Collins、Jim Coplien、Don Dwiggins、Gabriele Elia、Doug Felt、Brian Foote、Denis Fortin、Ward Harold、Hermann Hueni、Nayeem Islam、Bikramjit Kalra、Paul Keefer、Thomas Kofler、Doug Lea、Dan LaLiberte、James Long、Ann Louise Luu、Pundi Madhavan、Brian Marick、Robert Martin、Dave McComb、Carl McConnell、Christine Mingins、Hanspeter Mossenbock、Eric Newton、Marianne Ozkan、Roxsan Payette、Larry Podmolik、George Radin、Sita Ramakrishnan、Russ Ramirez、Alexander Ran、Dirk Riehle、Bryan Rosenburg、Aamod Sane、Duri Schmidt、Robert Seidl、Xin Shu 和 Bill Walker。

我们不认为这个设计模式集合是完整和静态的；它更像是我们对设计的当前思考的记录。
我们欢迎对它的评论，无论是对我们例子的批评、我们遗漏的参考文献和已知应用，还是我们应该收录的设计模式。
你可以通过 Addison-Wesley 写信给我们，或发送电子邮件至 design-patterns@cs.uiuc.edu 。
你也可以通过向 design-patterns-source@cs.uiuc.edu 发送消息 “send design pattern source” 来获取示例代码部分中代码的软拷贝。
现在还有一个网页 http://st-www.cs.uiuc.edu/users/patterns/DPBook/DPBook.html ，用于发布最新信息和更新。

<p align="right">
Mountain View, California E.G.<br/>
Montreal, Quebec R.H.<br/>
Urbana, Illinois R.J.<br/>
Hawthorne, New York J.V.<br/>
August 1994 </p>
