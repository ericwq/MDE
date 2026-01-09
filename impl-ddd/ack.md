## 致谢

#### ▶[上一节](preface.md)

衷心感谢 Addison-Wesley 出版社的优秀团队，给予我以他们备受尊敬的出版品牌发行作品的机会。正如我在课堂和演讲中多次强调的，我认为 Addison-Wesley 是一家真正理解 DDD 价值的出版机构。Christopher Guzikowski 和 Chris Zahn（Z 博士）在整个编辑过程中始终支持我的创作。Christopher Guzikowski 致电告知签约邀约的那天，我永生难忘。他始终鼓励我克服作者普遍面临的疑虑，直至出版在望。而 Z 博士则确保了文本最终达到出版标准。谨向我的制作编辑 Elizabeth Ryan，感谢她统筹全书出版事宜。更要感谢无畏的校对编辑 Barbara Wood 。

回溯过去，正是 Eric Evans 耗费五年职业生涯中的大部分时间，撰写了首部关于 DDD 的权威著作。若非他的付出，以及源自 Smalltalk 和设计模式社区的智慧结晶，经 Eric 本人精炼完善，更多开发者恐怕仍在靠临时拼凑的方式交付劣质软件。遗憾的是，这种现象远比想象中普遍。正如 Eric 所言，软件开发质量的低下，以及开发团队毫无创造力且毫无乐趣的工作状态，曾几乎让他彻底离开软件行业。我们衷心感谢 Eric 将精力投入教育事业而非转行，这份奉献值得我们深深感激。

在 2011 年首届 DDD 峰会的尾声，Eric 邀请我与会时，与会者一致认为领导层应制定一套指导方针，帮助更多开发者成功实践 DDD。当时我已完成本书大部分内容，对开发者普遍存在的认知缺口有着深刻理解。我主动撰写了关于 [Aggregates](TODO) 设计 “经验法则” 的文章。这篇题为《Effective Aggregate Design》的三部曲系列，最终构成了本书 [第 10 章](ch10/0.md) 的基础框架。当文章发布于 dddcommunity.org 后，业界对这类权威指导的迫切需求便显而易见。感谢 DDD 领域的其他领导者审阅该文稿并为本书提供宝贵反馈。Eric Evans 与 Paul Rayner 对文稿进行了多次详细审阅。Udi Dahan, Greg Young, Jimmy Nilsson, Niclas Hedhman, 和 Rickard Öberg 也提供了宝贵意见。

特别感谢 Randy Stafford ，这位 DDD 社区的资深成员。几年前在 Denver 参加我的一次 DDD 讲座后，Randy 便鼓励我更深入地参与更广泛的 DDD 社区。不久后，Randy 引荐我认识 Eric Evans ，让我得以阐述凝聚 DDD 社区的构想。尽管我的设想略显宏大且可能难以实现，Eric 却说服我们：组建由核心 DDD 领导者组成的精干团队，反而能创造更具现实意义的价值。2011 年 DDD 峰会正是源于这些讨论。毋庸置疑，若非 Randy 不断鼓励我推进对 DDD 的构想，这本书便不会诞生，甚至可能连 DDD 峰会都无从谈起。尽管 Randy 因忙于 Oracle Coherence 项目未能参与本书撰写，但或许未来我们仍有机会携手创作。

衷心感谢 Rinat Abdullin、Stefan Tilkov 和 Wes Williams 为本书撰写了若干专题章节。要通晓 DDD 的所有知识几乎不可能，更不可能成为软件开发所有领域的专家。正因如此，我邀请特定领域的专家撰写了 [第 4 章](ch4/0.md) 及 [附录 A](appendix.md) 的部分章节。感谢 Stefan Tilkov 分享其非凡的 REST 知识，感谢 Wes Williams 贡献 GemFire 实践经验，更感谢 Rinat Abdullin 持续分享其在聚合体实现事件源方面的不断积累的经验。

我的早期审稿人之一是 Leo Gorodinsk ，他始终支持这个项目。我第一次见到 Leo 是在 Denver 的 DDD 聚会上。他根据自己在 Boulder, Colorado 带领团队实践 DDD 时遇到的困境，为本书提供了许多宝贵建议。希望我的书能像他的严谨审阅帮助到我那样，同样对 Leo 有所裨益。我视 Leo 为 DDD 未来的重要组成部分。

还有很多人至少对我书中的一个章节提供了反馈，有些人对多个章节提供了意见。一些更为批判性的反馈来自 Gojko Adzic、Alberto Brandolini、Udi Dahan、Dan Haywood、Dave Muirhead 和 Stefan Tilkov。具体来说，Dan Haywood 和 Gojko Adzic 提供了大量早期反馈，这些反馈基于我写的最难读的内容。我很高兴他们坚持并纠正了我。Alberto Brandolini 对战略设计的总体见解，以及对 [Context Mapping](TODO)的具体见解，帮助我集中关注这些重要内容的核心。Dave Muirhead 拥有丰富的面向对象设计、领域建模、对象持久化和内存数据网格（包括 GemFire 和 Coherence）经验，他对我书中一些对象持久化的历史和细节有重要影响。除了在 REST 方面的贡献外，Stefan Tilkov 还提供了关于架构的总体见解，尤其是关于 SOA 以及 Pipes 与 Filters 模式的具体见解。最后，Udi Dahan 验证并帮助我厘清了 CQRS、Long-Running Processes（即 Sagas）以及基于 NServiceBus 的消息传递等概念。其他提供宝贵反馈的审稿人包括：Rinat Abdullin、Svein Arne Ackenhausen、Javier Ruiz Aranguren、William Doman、Chuck Durfee、Craig Hoff、Aeden Jameson、Jiwei Wu、Josh Maletz、Tom Marrs、Michael McCarthy、Rob Meidal、Jon Slenk、Aaron Stockton、Tom Stockton、Chris Sutton 以及 Wes Williams。

Scorpio Steele 为本书创作了精彩绝伦的插图。他让 IDDD 团队的每位成员都化身为他们本应成为的超级英雄。另一端则是我的好友 Kerry Gilbert 进行的非技术性编辑审阅。当其他人确保我的技术表述准确无误时，凯瑞则用 “语法锤” 对我进行了严苛的校对。

我的父母在我生命中始终给予我巨大的启发与支持。书中以 “牛仔逻辑” 幽默形象出现的父亲 AJ，绝非仅仅是个牛仔。请别误会，即便只当个出色的牛仔也已足够。除了热爱飞行与驾驶飞机，父亲还是位成就斐然的土木工程师、土地测量师，更是天赋异禀的谈判专家。时至今日，他依然痴迷数学与探索星系。在我十岁左右时，父亲教我如何求解直角三角形，这只是他传授给我的众多知识之一。感谢父亲在我幼年时就培养了我对技术的兴趣。同样要感谢母亲，她是世上最善良的人之一。在我面对人生挑战时，她始终给予鼓励与支持。此外，我身上所有的坚韧都源自于她。我还能继续赞美她，但无论怎样都无法尽述她的美好。

尽管本书献给我的挚爱妻子 Nicole 和我们出色的儿子 Tristan，但若不在此特别致谢，我的感激之情便不完整。正是他们让我得以投入并完成这部著作。若没有他们的支持与鼓励，这项任务根本无法实现。衷心感谢你们，我最亲爱的家人。

#### ▶[下一节](author.md)
