# 一种为客户端 Web 应用构建基于 XML 的 DSL 解决方案的方法
Enrique Chavarriaga, Francisco Jurado, Fernando Díez

2017/04 [original english](origin/approach_chavarriaga_computer_languages_systems_structures_2017_ps.pdf)

----
## 摘要
特定语言（DSL）凭借其提供的抽象层次，在构建应用时，能够减轻软件工程师与领域专家的工作负担。当领域限定于 Client-Side Applications（CSWA）时，通常会结合基于 XML 的语言、框架和组件，以提供快速、稳健且灵活的解决方案。本文提出一种设计方法，可用于为 CSWA 创建基于 XML 的 DSL 解决方案，包含评估引擎、编程模型及轻量级开发环境。该方法可同时运行（evaluate）多个基于 XML 的 DSL 程序，为 CSWA 领域的特定问题提供解决方案。为充分展示此创新方法的能力与潜力，我们将通过 *Anisha* 和 *FeedPsi* 两个案例进行阐释。 *(译注：关于 CSWA 没有定义)*

----
## 关键词
领域特定语言；XML 解释器；JavaScript；Web 应用；XML 编程语言。

----
## 1. 引言
领域特定语言（DSL）通过提供高层次抽象，用于建模解决特定领域问题的规范、结构和功能。DSL 的目标是通过让领域专家在构建高质量可靠系统时更高效地完成任务，从而简化系统的设计、定义和实现，进而提供领域特定解决方案 <sup>[52](#52)</sup>。简而言之，Fowler <sup>[16](#16)</sup> 将 DSL 定义为 “一种表达能力有限、专注于特定领域的计算机编程语言”。

领域特定语言（DSL）的开发需要借助扫描器 (scanner)、解析器和代码生成工具来评估领域特定模型，从而实现相关功能。然而，当基于 XML 标准进行开发时，XML 领域特定语言（XML-DSL）可利用通用解析器，如 XML 简单应用程序接口（SAX）和文档对象模型（DOM）<sup>[53](#53)</sup>。这些 API 已集成于多数编程语言中，使程序员能够访问并修改跨 XML 语言的结构与内容。由此，XML 既能以最契合建模需求的语言存储和交换自动文档化的结构化信息 <sup>[13](#13)</sup>、<sup>[55](#55)</sup>，又能通过标准 API 轻松实现语法规范与功能扩展。因此，XML-DSL 提供了可扩展且易于组合的直观DSL规范，其关联功能可借助目标编程语言现有的 API 快速实现。

过去十年的技术变革彻底改变了可交付的基于网络的信息系统的功能与类型。这些变革要求我们在为客户端网络应用程序（CSWA）提供解决方案时采取全新视角。当前，此类解决方案的设计与实现仍处于探索阶段 <sup>[46](#46)</sup>，主要基于动态网页构建及其相关技术 <sup>[36](#36)</sup>、<sup>[54](#54)</sup>。此外，Web 2.0 <sup>[1](#1)</sup> 提供了构建功能完善、体验愉悦且易于使用的网页所需的技术、服务和工具，其附加价值在于能够实现跨平台部署。

然而，尽管 CSWA 日益重要，且科学界与工业界对 DSL 表现出浓厚兴趣，据我们所知，能为 CSWA 创建 DSL 解决方案的研究仍寥寥无几。因此，本文旨在探讨为 CSWA 创建并实现 XML-DSL 的方法，这正是现有解决方案未能覆盖的领域。为此，本文提出  *Programmable Solutions Interpreter* （Psi），一种构建 CSWA 解决方案的方法，包含评估引擎、编程模型及轻量级开发环境，我们分别将其命名为 *PsiEngine* 、*PsiModel* 和 *PsiEnvironment* 。该方法支持创建并评估面向 CSWA 的 XML-DSL，通过封装功能模块并与其他 Web 组件及框架集成，从而构建快速、稳健且灵活的解决方案。

*PsiEngine* 的核心组件是 *PsiXML Interpreter*（简称 *PsiXML* ），这是一个能够评估 XML-DSL 程序的 JavaScript XML 解释器。 *PsiXML* 能够注册多种 XML-DSL，并能评估用注册的 XML-DSL 编写的多个程序。XML-DSL 程序是一组可编程标签，每组标签都关联着特定的功能。执行 XML-DSL 程序时，其核心在于评估每个 XML 标签的功能。此外，在 *PsiEngine* 中定义并实现的 XML-DSL，能够链接和交换异构信息（支持 XML 与 JSON 格式），同时应用最新的安全策略和良好的编程实践 <sup>[26](#26)</sup>、<sup>[60](#60)</sup>，从而能够为 CSWA 开发灵活的 XML-DSL。

*PsiModel* 为首席程序员们 (lead programmers) 建立了一个编程模型，用于生成 JavaScript 代码及配套文档，主要面向 XML-DSL 和可复用 JavaScript 组件的创建。最后， *PsiEnvironment* 作为轻量级环境，既利用 *PsiModel* 又使用 *PsiEngine* 。

通过这三个要素（ *PsiEngine* 、 *PsiModel* 和 *PsiEnvironment* ），本文目标是，在构建 CSWA 时， 使用 Model Driven Engineering（MDE）奠定基础。MDE 是一种以定义模型为核心的软件方法论，旨在简化信息系统的创建过程 <sup>[49](#49)</sup>。该方法融合了 DSL、XML-DSL 及 Domain-Specific Visual Language（DSVL）<sup>[28](#28)</sup> 等概念，并结合转换引擎与代码生成器。在此背景下，本文详述的 *PsiEngine* 与 *PsiModel* 正是实现前述目标的基石。

为展示 *PsiEngine* 的实现及其功能，同时操作 *PsiModel* 及其关联的 *PsiEnvironment*，我们将提供两个案例研究： *Anisha* 与 *FeedPsi* 。 *Anisha* 旨在构建两种 XML-DSL 以实现基础帧动画，该运行示例详细展示了多程序的解释与执行机制，以及信息绑定过程。与此同时，*FeedPsi* 作为新闻聚合 CSWA，旨在评估与其他网页组件、RSS 服务及框架的集成能力。两项案例均遵循文献 <sup>[59](#59)</sup> 提出的定性案例研究方法论，并将其调整以适应软件工程领域 <sup>[2](#2)</sup>。通过这两项案例研究，我们将涵盖采用本方法编程 XML-DSL 解决方案的核心特性。

本文其余部分结构如下：第 [2](#2-概述与相关工作) 节将重点介绍相关前沿研究；第 [3]() 节将概述本方法的总体框架及核心特征，并包含 *Anisha* 运行实例；第 [4]() 节将阐述 *FeedPsi* 案例研究，总结 *PsiEngine* 的验证细节；第 [5]() 节将详细呈现研究成果；最后第 [6]() 节将通过总结性评论与未来工作展望结束本文。

----
## 2. 概述与相关工作
领域特定语言（DSL）在文献中尚未获得严格定义。如前所述，Fowler <sup>[16](#16)</sup> 将其定义为 “一种表达能力有限、专注于特定领域的计算机编程语言”。在 <sup>[51](#51)</sup> 中，Spinellis 指出 “DSL是为特定应用领域量身定制的编程语言：它并非通用语言，而是精确捕捉该领域的语义特征”。与此同时，Mernik 等人 <sup>[38](#38)</sup> 指出：“领域特定语言（DSL）是为特定应用领域量身定制的语言。相较于通用编程语言，它们在其应用领域内能显著提升表达能力和易用性。”

文献 <sup>[12](#12)</sup> 研究了语言的构成要素，并在考虑语言扩展、语言限制、语言统一、自我扩展及扩展组合等因素的基础上，协助对 DSL 进行分类。据此，我们的工作重点在于为 Web 客户端构建自我扩展语言及扩展组合方案。

----
## 参考文献
#### 1
Anderson, P.</br>
Web 2.0 and Beyond: Principles and Technologies.</br>
London: Chapman and Hall/CRC, 2012.

#### 2
Baxter P, Jack S.</br>
Qualitative Case Study Methodology: Study Design and Implementation for Novice Researchers.</br>
The Qualitative Report;2008:13-4.

#### 3
Betinni L.</br>
Implementing Domain-Specific Languages with Xtext and Xtend.<br>
Birmingham: Packt Publishing Ltd; 2013, p. 1-11.

#### 4
Brown D, Levine J, Mason T.</br>
Lex & Yacc. 2nd ed.</br>
New York: O'Reilly Media, Inc; 1992.

#### 5
Bravenboer M, Trygve K, Vermaas R, Visser E.</br>
Stratego/XT: A language and toolset for program transformation.</br>
Science of Computer Programming. 2008; 72(1–2), p.52-70.</br>
http://dx.doi.org/10.1016/j.scico.2007.11.003.

#### 6
Carter Z.</br>
JS sequence diagrams.</br>
http://jison.org;</br>
2009 [accessed 30.10.16].

#### 7
Cook S, Jones G, Kent S, James D.</br>
Domain-Specific Development with Visual Studio DSL Tools.</br>
Denver: Addison-Wesley Professional; 2007, p. 1-23.

#### 8
Crockford D.</br>
JavaScript: The Good Parts.</br>
Sebastopol: O'Reilly Media, Inc; 2008, ch. 5.

#### 9
Czarnecki K, Eisenecker U.</br>
Generative Programming: Methods, Tools and Applications.</br>
Denver: Addison-Wesley; 2000.

#### 10
Da Silva AR.</br>
Model-driven engineering: a survey supported by a unified conceptual model.</br>
Computer Languages, Systems & Structures.</br>
2015; 43, p.139-155.

#### 11
Dearle F.</br
Groovy for Domain-Specific Languages. Birmingham:</br>
Packt Publishing Ltd; 2010, ch. 1.

#### 12
Erdweg S, Giarrusso PG, Rendel T.</br>
Language composition untangled.</br>
Proceedings of the 12th Workshop on Language Descriptions, Tools, and Applications, LDTA 2012.

#### 13
Fawcett J, Quin L, Ayers D.</br>
Beginning XML. 5th ed.</br>
Wrox Press; 2012.

#### 14
Fanagan D, Matsumoto Y.</br>
The Ruby Programming Language.</br>
Sebastopol: O’reilly Media, Inc.; 2005, ch. 8.

#### 15
Firmenich S, Rossi G, Winckler M, Palanque P.</br>
An approach for supporting distributed user interface orchestration.</br>
International Journal of Human-Computer Studies, 2014;72:1: p. 53- 76.

#### 16
Fowler, M.</br>
Domain Specific Languages.</br>
Denver: Addison-Wesley Professional; 2010, p. 21-27.

#### 17
Ghosh, D.</br>
DSLs in Action.</br>
Greenwich: Manning Publications, 2010, p. 9-15.

#### 18
Greenfield J, Short K.</br>
Software Factories: Assembling Applications with Patterns, Models, Frameworks, and Tools.</br>
New Jersey: Wiley Publishing; 2004.

#### 19
Gronback RC.</br>
Eclipse Modeling Project: A Domain-Specific Language (DSL) Toolkit.</br>
Denver: Addison-Wesley Professional; 2009.

#### 20
Halstead M.</br>
Elements of Software Science.</br>
New York: The Computer Science Library; 1977.

#### 21
Herzberg D et al.</br>
Specifying computer-based counseling systems in health care: A new approach to user-interface and interaction design.</br>
Journal of Biomedical Informatics. 2009; 42:2: p. 347-355.

#### 22
Holzner S.</br>
Secrets of RSS.</br>
Berkeley: Peachpit Press.; 2006, ch. 3.

#### 23
Hudak P.</br>
Building domain-specific embedded languages.</br>
ACM Comput. 1996;196:4.

#### 24
Johnson D.</br>
RSS and Atom in Action: Web 2.0 Building Blocks.</br>
Greenwich: Manning Publications Co.; 2006, p. 70-77.

#### 25
Kelker R.</br>
Clojure for Domain-specific Languages.</br>
Birmingham: Packt Publishing Ltd.; 2013, ch. 1.

#### 26
Kern C.</br>
Securing the tangled web.</br>
Communication ACM 2014;57:9: p. 38-47.

#### 27
Kats L, Kalleberg K, Visser E.</br>
Domain-Specific Languages for Composable Editor Plugins.</br>
Elsevier: Proceedings of the Ninth Workshop on Language Descriptions, Tools, and Applications (LDTA'09),</br>
Electronic Notes in Theoretical Computer Science 2009;253:7, p. 149-163.

#### 28
Kelly S, Tolvanen J.</br>
Domain-Specific Modeling: Enabling Full Code Generation.</br>
Wiley-IEEE Computer Society Press 2008.

#### 29
Kniesel G, Winter V, Siy H, Zand M.</br>
Making aspect-orientation accessible through syntax-based language composition.</br>
IEEE: IET Software 2009;3:1, p. 1-13.

#### 30
Kourie, D.G., Fick, D. & Watson, B.W. (2008).</br>
Virtual machine framework for constructing domain-specific languages.</br>
IET Software. IEEE, 3, 3, 219-237.

#### 31
Kosar T, et al.</br>
Comparing general-purpose and domain-specific languages: An empirical study.</br>
Computer Science and Information Systems 2010;7:2, p. 247–264.

#### 32
Kosar T, Bohra S, Mernik M.</br>
Domain-Specific Languages: A Systematic Mapping Study.</br>
Information and Software Technology 2016;71, p. 77-91.

#### 33
Lee P.</br>
CoffeeScript in Action.</br>
Greenwich: Manning Publications Co.; 2014, ch. 1.

#### 34
Levine J.</br>
Flex & Bison.</br>
Sebastopol: O'Reilly Media, Inc.; 2009.

#### 35
McCabe T.</br>
A Complexity Measure.</br>
IEEE Transactions on Software Engineering 1976;SE- 2:4, p.308-320.

#### 36
McDaniel A.</br>
HTML5: Your visual blueprint<sup>TM</sup> for designing rich web pages and applications.</br>
Indianapolis: Jhon Wiley & Sons, Inc.; 2011, ch. 3-8.

#### 37
McGuire P.</br>
Getting Started with Pyparsing.</br>
Sebastopol: O'Reilly Media, Inc.; 2007.

#### 38
Mernik M, Heering J, Sloane AM.</br>
When and how to develop domain-specific languages.</br>
ACM Comput. Surv. 2005;37:4, p. 316–344.

#### 39
Mernik M, Lenic M, Avdičaušević E, Zumer V.</br>
LISA: An Interactive Environment for Programming Language Development.</br>
Springer Berlin Heidelberg, Lecture Notes in Computer Science 2002;2304, p. 1-4.

#### 40
Nguyen V, Deeds-Rubin S, Tan T, Boehm B.</br>
A SLOC Counting Standard.</br>
University of Southern California, Center for Systems and Software Engineering,</br>
http://sunset.usc.edu/csse/TECHRPTS/2007/usc-csse-2007-737/usc-csse-2007-737.pdf;</br>
2007 [accessed 13.06.16].

#### 41
Oman PW, Hagemeister J, Ash D.</br>
A Definition and Taxonomy for Software Maintainability.</br>
Moscow: Technical Report, University of Idaho, Software Engineering Test Laboratory; 1991.

#### 42
Parr T.</br>
The Definitive ANTLR 4 Reference. 2nd ed.</br>
Raleigh: Pragmatic Bookshelf; 2013, part 1.

#### 43
Pereira MJ, Fonseca J, Henriques PR.</br>
Ontological approach for DSL development.</br>
Computer Languages, Systems & Structures. 2016; 45:1, p.35-52.

#### 44
Prout A, Atlee JM, Day NA, Shaker P.</br>
Code generation for a family of executable modelling notations.</br>
Software and Systems Modeling; 2012;11:2, p. 251-272.

#### 45
Rahien A.</br>
DSLs in Boo: Domain-Specific Languages in .NET.</br>
Greenwich: Manning Publications Co.; 2010, ch. 3-4.

#### 46
Sajja PS, Akerkar R.</br>
Intelligent Technologies for Web Applications.</br>
Minneapolis: CRC Press; 2012, ch. 1.

#### 47
Serrano, A.</br>
Beginning Haskell A Project-Based Approach.</br>
New York: Apress; 2014, part 4.

#### 48
Stilwell J.</br>
npm: Escomplex, v 1.3.</br>
https://www.npmjs.com/package/escomplex;</br>
2014 [accessed 15.07.16].

#### 49
Schmidt DC.</br>
Model-driven engineering.</br>
Computer-IEEE Computer Society 2006;39:2:25.

#### 50
Shim H, Kang B, Kwag K.</br>
Web2Animation - Automatic Generation of 3D Animation from the Web Text.</br>
IEEE/WIC/ACM: Proceedings of the 2009 International Joint Conference on Web Intelligence and Intelligent Agent Technology (WI-IAT '09).

#### 51
Spinellis D.</br>
Notable design patterns for domain-specific languages.</br>
Journal of Systems and Software 2001;56:1, p. 91-99.

#### 52
Voelter M, et al.</br>
DSL Engineering: Designing, Implementing and Using Domain-Specific Languages.</br>
Dslbook.org; 2013, p. 23-38.

#### 53
Document Object Model (DOM) Level 3 Core Specification,</br>
http://www.w3.org/TR /2004/REC-DOM-Level-3-Core-20040407/;</br>
2004 [accessed 30.06.16].

#### 54
Web Design and Applications,</br>
http://www.w3.org/standards/webdesign/;</br>
2014 [accessed 28.06.16].

#### 55
XML Technology,</br>
http://www.w3.org/standards/xml/;</br>
2014 [accessed 27.06.16].

#### 56
Wittenbrink H.</br>
RSS and Atom.</br>
Birmingham: Packt Publishing; 2005, ch. 4.

#### 57
Woodman M.</br>
How to Build an RSS 2.0 Feed.</br>
Sebastopol: O'Reilly Media, Inc.; 2006.

#### 58
White A.</br>
JavaScript Programmer’s Reference.</br>
Indianapolis: Wiley Publishing, Inc.; 2009, ch. 10.

#### 59
Yin RK.</br>
Case Study Research: Design and Methods. 5st ed.</br>
London: Sage Publications, Inc.; 2014.

#### 60
Yue C, Wang H.</br>
A measurement study of insecure javascript practices on the web.</br>
ACM Transaction Web 2013;7:2, p. 1-39.
