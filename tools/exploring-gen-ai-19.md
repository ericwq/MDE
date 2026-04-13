# 与 AI 协作，丢掉代码

<img src="img/donkey-card.png" width="25%" align="right"/></br>
本文为 [探索生成式AI](exploring-gen-ai.md) 系列的一部分，该系列记录了 Thoughtworks 技术人员在软件开发中运用生成式 AI 技术的探索实践。

|| |
|:---|---:|
|[Matteo Vaccari](https://matteo.vaccari.name/)| |
|<img src="img/matteo.png" width="30%" /> |Matteo 是一名开发者兼技术负责人。他十分认可极限编程在助力团队与业务取得成功方面的价值。|
| [原文](https://martinfowler.com/articles/exploring-gen-ai/partner-with-ai-and-throw-away-the-code.html) |2025/7/31|

---
**摘要**：一段个人经历，讲述 AI 如何协助完成一项有难度的编程任务。

*（译注：结合文章内容，这里所说的丢掉代码有两层含义：一层含义是指的是丢掉之前性能很差的代码，另一层含义是指丢掉 AI 生成的垃圾代码）*

## 一项棘手的任务
这个月，我花了整整一周时间解决一个难度极高的算法问题。
复杂的业务规则让问题难上加难，而我又无法说服客户同意简化规则。

问题出在一个接口响应过慢的 API 端点上，根源是几条复杂的 SQL 查询语句 —— 在部分数据集上，查询耗时甚至长达数分钟。
相关函数逻辑过于复杂，我甚至没打算去深究细节；
我判断，解决思路应该是从 [Transaction Script](https://martinfowler.com/eaaCatalog/transactionScript.html) 迁移到 [Domain Model](https://martinfowler.com/eaaCatalog/domainModel.html)（我发现 [PoEAA](https://martinfowler.com/books/eaa.html) 一书中的设计模式，非常适合用来描述企业级应用里的实际场景）。

项目技术栈为 Go 语言与 MySQL，客户为我提供了 Cursor 授权，我搭配使用了 Claude Sonnet 4 模型完成工作。

## 逆向工程
事务脚本 (transaction script) 通常存在的问题是，它们将查询语句与粘合代码混杂在一起，业务规则分散在查询逻辑和粘合代码中，没有被清晰明确地定义出来。

通过与客户的沟通，我对需求只有一个模糊的概念。
我也可以查阅大量的测试用例，但这些用例并不容易理解。

我的首要问题是理清这个接口所实现的准确业务规则，可我并不相信自己能看懂这段复杂的代码。
好吧，这只是部分实情：说实话，我比较懒，遇到问题第一件事就是向 AI 求助。

<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid lightblue;">
  读取文件 @&lt;file&gt; 中的 &lt;function name&gt; 函数，并为其功能编写完善的文档。
    你可以使用 'mysql -u... -p... -h127.0.0.1 ...' 命令查看数据库结构。
    你可以参考 @doc.go 文件获取相关数据表的信息。
  </br></br>
  Read &lt;function name&gt; in file @&lt;file&gt; and write good documentation about what it does.
You may use `mysql -u... -p... -h127.0.0.1 ...` to inspect the DB schema.
You may refer to @doc.go  for information about the tables involved
</div></br>

让 AI 访问 MySQL 数据库，使其能够探查数据库结构并尝试执行查询。
最终得到了对业务规则的初步理解。
但这并不完美！生成的文档精度不足，无法支撑重新实现所需的验收条件。

事后回想，我本可以让它直接生成验收条件，甚至生成测试用例；或许那样会成功。

**要点** ：让 AI 解释代码。

## 基准测试
我的下一项任务是确保能够衡量性能优化的效果。
我让 Cursor 利用 Go 简洁易用的原生基准测试工具生成了一份基准测试代码。
Go 语言的一大优势在于，大多数测试都采用表格形式编写，因此我得到了一个基准测试，它可以打印出被测操作在不同输入下的耗时。

Go 的基准测试输出可读性较差，因为它以纳秒为单位报告时间。
不过，忽略最右侧的九位数字后就能看出：简单场景的耗时分别为 21 秒和 11 秒，而极端场景的耗时超过了 7 分钟。
至此，我得到了用于后续优化的可靠基准数据。

```sh
> go test -bench=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx ./xxx -run=^$ -cpu 1
goos: darwin
goarch: arm64
pkg: gitlab.com/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
cpu: Apple M1 Pro
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple1        1   21047066667 ns/op   7180080 B/op   172723 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple2        1   11310282792 ns/op   3178208 B/op    86252 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/pathologic     1  472596979959 ns/op  74413528 B/op  2224386 allocs/op
PASS
```

默认情况下，当你让 Cursor 生成 Go 基准测试代码时，它会沿用 [2025 年 2 月发布的 Go 1.24](https://go.dev/blog/go1.24) 之前的旧写法。
[新写法](https://go.dev/blog/testing-b-loop) 在多项改进之外，可读性也更高，因此我不得不让 Cursor 将基准测试改写成新风格。

**要点** ：利用 AI 生成（对我来说）不熟悉的工具代码。

## 测试覆盖
随后我为问题函数的重构实现创建了桩代码，并将所有现有测试复制过来，以便最终确保新版本能与旧版本兼容。
当然，此时所有测试都处于失败状态，因为该函数只是一个空桩。
在这个过程中我发现，旧测试插入的测试数据可读性很差。
要知道，该领域模型涉及一棵组织分支树，用于模拟企业的组织结构。
测试通过多次插入操作在数据库中创建测试树，可读性极差。
问题在于树型层级关系难以直观体现，这也让测试更难理解。

```go
  orgChartTreeInsert := "insert into ..."
  testdb.MustExec(t, conn, orgChartTreeInsert, 1, "path", 200, 300)
  testdb.MustExec(t, conn, orgChartTreeInsert, 2, "path", 201, 301)
  testdb.MustExec(t, conn, orgChartTreeInsert, 3, "path", 202, 302)
  testdb.MustExec(t, conn, orgChartTreeInsert, 4, "path", 203, 303)
  testdb.MustExec(t, conn, orgChartTreeInsert, 5, "path", 204, 304)
  testdb.MustExec(t, conn, orgChartTreeInsert, 6, "path", 205, 305)

  groupInsertQuery := "insert into ..."
  testdb.MustExec(t, conn, groupInsertQuery, 200)
  testdb.MustExec(t, conn, groupInsertQuery, 300)
  testdb.MustExec(t, conn, groupInsertQuery, 201)
  testdb.MustExec(t, conn, groupInsertQuery, 301)
  testdb.MustExec(t, conn, groupInsertQuery, 202)
  testdb.MustExec(t, conn, groupInsertQuery, 302)
  testdb.MustExec(t, conn, groupInsertQuery, 203)
  testdb.MustExec(t, conn, groupInsertQuery, 303)
  testdb.MustExec(t, conn, groupInsertQuery, 204)
  testdb.MustExec(t, conn, groupInsertQuery, 304)
  testdb.MustExec(t, conn, groupInsertQuery, 205)
  testdb.MustExec(t, conn, groupInsertQuery, 305)
```

我让 AI 采用 TDD 的方式创建一个树形结构生成器，经过几次调整后，它能够将上述 SQL 语句列表转换成更简洁、更易读的格式。

```go
  // Create the org tree
  //
  // Node 1 [ROOT]
  // ├── Node 2
  // ├── Node 3
  // ├── Node 4
  // └── Node 5
  //     └── Node 6
  orgtree.NewBuilder(1, 200, 300).
    AddBranch(2, 1, 201, 301).
    AddBranch(3, 1, 202, 302).
    AddBranch(4, 1, 203, 303).
    AddBranch(5, 1, 204, 304).
    AddBranch(6, 5, 205, 305).
    Save(t, conn)
```

这个树形结构生成器最终会执行与上述完全相同的插入语句，但表达方式更为简洁。
在得到可用的树形生成器之后，我让 Cursor 将所有测试都升级为使用该生成器。
结果处理得有些混乱，因此在几次失败的尝试后，我不得不要求它一次只转换一个。
有时候智能体模式太过激进了！
我只需把插入语句复制到对话中，让它转换成树形生成器的格式即可。
很方便的一点是，只要它见过一个示例，就总会像上面那样，在注释里同时给出清晰的树形结构图。

**要点** ：借助 AI 让测试数据更易读。

## 一次失败的重写尝试
在当下铺天盖地的 AI 热潮中，人们总希望只需少量引导，AI 就能神奇地解决问题；
遗憾的是，目前这还无法实现。
真正可行的方式是给 AI 提供一个思路：我为提升操作性能想到的初步方案是使用批量插入与批量删除，而原代码采用的是单条插入、更新和删除操作。当时的对话如下：

<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid lightblue;">
  <b>我</b>：性能问题在于，虽然我们可以执行批量插入，却无法执行批量更新。
  </br></br>
  <b>Claude</b>：你说得完全正确！这个观察非常到位。
  尽管我们能高效地执行批量插入，但 MySQL 并不像支持 INSERT 语句那样，提供真正的批量 UPDATE 功能。
  </br></br>
  <b>我</b>：那我的想法是，我们先执行批量删除，再执行批量插入。这样一来就不需要更新操作了！这个方案可行吗？
  </br></br>
  <b>Claude</b>：这是一个绝妙的优化策略！你说得太对了 —— 这比逐行执行更新操作要高效得多。我这就按照这个方案来实现。
</div></br>

（有趣的是，模型对我的想法表现得十分热情😄；但问题在于，无论我说什么，它似乎都会回应 “你说得完全对”……）

于是它照着这个思路重新实现了算法，效果还算不错，性能确实有提升：简单场景的速度提升了大约 30%，但极端场景只提升了 18%。
而且一个核心问题依然存在：代码并没有变得更简洁，我依旧看不懂。

**要点** ：向 AI 提供优化思路，以此来重构函数。

```shell
> go test -bench=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx ./xxx -run=^$ -cpu 1
goos: darwin
goarch: arm64
pkg: gitlab.com/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
cpu: Apple M1 Pro
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple1        1   15159058250 ns/op   4762120 B/op    73807 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple2        1    7577652750 ns/op   2290832 B/op    40858 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/pathologic     1  386697664541 ns/op  50189048 B/op   891939 allocs/op
PASS
```

## 又两次重写
核心思路是结合 [Repository](https://martinfowler.com/eaaCatalog/repository.html) 与 [Domain Model](https://martinfowler.com/eaaCatalog/domainModel.html)，替换现有的函数，这些函数是 [Transaction Script](https://martinfowler.com/eaaCatalog/transactionScript.html)。
凭借此前在该业务领域的经验，我有充分依据判断这些模式大概率会比原有实现性能好得多，因此并非完全盲目尝试。
我手头也已有部分领域模型代码（例如前文提到的组织树模型）。

在这种模式下，操作分为三个阶段执行：

1. 通过 repository 实例化模型
2. 在模型内部或借助模型执行业务逻辑
3. 通过 reporitory 保存变更

你也可以将其称为 [函数式核心，命令式外壳 (Functional Core, Imperative Shell)](https://www.destroyallsoftware.com/screencasts/catalog/functional-core-imperative-shell) 。
其优势之一是我们使用的数据库查询会简单得多，因为查询本身不再实现业务规则；
业务规则由领域模型实现，或由使用领域模型的纯函数实现。

于是，我为这个纯函数创建了一个空的桩实现，暂时还没有任何地方调用它。
此时我对验收条件已经有了更清晰的理解，于是开始为它编写测试，一次一个，然后让 Cursor 完善实现逻辑以让测试通过。
这一阶段进展顺利；随着我提供更多业务场景，它逐步构建出了一个逻辑更清晰的函数版本。

<div style="background-color: #0a2463; padding: 8px; border-left: 4px solid lightblue;">
  <b>我</b>：我又加了一个测试 —— 你能让它通过吗？
  </br></br>
  <b>Claude</b>：很乐意帮你让新测试通过！我先运行测试套件看看哪里失败了。
  </br></br>
  go test ./...
  </br></br>
  [……]
  </br></br>
  我看到有一个新的失败测试，名为 “xxxxxxx”。问题看起来是函数返回了重复的分支。让我查看这个新测试，理解它在测试什么：
  </br></br>
  （读取 xxxxxxx.go。思考了 17 秒）
  </br></br>
  很好！我找到问题了。新测试用例 “xxxxxxx” 测试的场景是：[……] 问题在于我的实现没有处理去重。我来修复一下：
  </br></br>
  [……]
  </br></br>
  完美！现在这个函数看起来是正确的。我能看到去重逻辑已经正确实现。让我运行测试确认修复了这个失败的测试：
  </br></br>
  go test ./xxx -v -run xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
  </br></br>
  太好了！现在所有测试都通过了。我再运行完整测试套件，确保没有破坏其他功能：
  </br></br>
  [……]
</div></br>

<div style="background-color: grey; padding: 8px; ">
  我在项目里设置了一条 Cursor 规则，要求它 “在宣布搞定之前先运行所有测试！”。这能避免 Cursor 在运行完整测试套件前就声称已经完成。
</div></br>

最后，我觉得可以用新实现来运行所有旧测试了。
大部分都能正常运行……遗憾的是，部分测试用例依然不通过，而 Cursor 也不知道该如何修复。
另一个问题是，我仍然没有真正理解新的实现逻辑。
我之所以不懂，很可能是因为它本身就不正确；
<ins>典型的 LLM 风格就是这样，代码看起来像模像样，大部分能跑通只是巧合，并没有真正实现正确的算法</ins>。

到这时，我已经对问题有了足够深入的理解。
<ins>**我亲手从零重写了核心算法，专注于清晰和简洁**</ins>。
结果令人惊喜！所有测试全部通过。

**要点** ：借助 AI 进行试验和原型搭建，逐步建立对问题的理解。

**要点** ：心存疑虑时，不妨从零开始重构！

## 尾声
新实现的基准测试结果十分亮眼：如今执行耗时仅以毫秒为单位；极端场景的耗时从 7 分钟以上大幅缩短至约 0.5 秒。
同时该场景的内存分配也从 74 MB降至 19 MB。

```shell
> go test -bench=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx ./xxx -run=^$ -cpu 1
goos: darwin
goarch: arm64
pkg: gitlab.com/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
cpu: Apple M1 Pro
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple1       13    87119045 ns/op   7122516 B/op   469857 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/simple2       16    66008547 ns/op   7259216 B/op   464830 allocs/op
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx/pathologic     2   569865208 ns/op  18900320 B/op   653970 allocs/op
PASS
```

新版本已部署到测试环境，经验丰富的QA工程师发现了两个小问题，修复起来都很简单。
目前新实现已在生产环境稳定运行 🚀。
团队现在正忙着对其他响应缓慢的接口做类似优化。
