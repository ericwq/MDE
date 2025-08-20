# 一种为 Web 应用构建基于 JSON 的 DSL 解决方案的方法
Enrique Chavarriaga, Francisco Jurado, Francy D. Rodríguez

## 摘要
由于其抽象程度，领域特定语言 (DSL) 能够构建简化软件实现的应用。在 Web 应用领域，我们可以找到许多用于服务器端应用的技术和编程语言，它们提供快速、稳健且灵活的解决方案，而用于客户端应用的技术和编程语言则有限，并且大多仅限于直接使用 JavaScript、HTML5、CSS3、JSON 和 XML。本文介绍了一种使用 JSON 语法 (JSON-DSL) 在服务器端和客户端创建基于 DSL 的 Web 应用的新方法。该方法包括一个评估引擎、一个编程模型和一个支持它的集成 Web 开发环境。评估引擎允许执行使用编程模型创建的元素。编程模型则允许定义和规范 JSON-DSL、实现 JavaScript 组件、使用引擎提供的JavaScript模板、使用链接连接器连接到异构信息源，以及与其他小部件、Web 组件和 JavaScript 框架的集成。为了验证我们方法的优势和能力，我们开发了四个案例研究，使用集成的 Web 开发环境来应用编程模型并检查评估引擎中的结果。

## 关键词
领域特定语言；JavaScript；JSON；JSON-DSL；Web 应用；模板引擎

## 1. 引言
领域特定语言 (DSL) 提供高级别的抽象，用于建模、指定和定义解决领域特定问题的结构、规范和功能。DSL 的目标是简化系统或部分系统的实现过程，使领域专家能够参与可靠、健壮和高质量系统的 开发过程，从而为特定问题提供解决方案 <sup>[1](#1)</sup>、<sup>[2](#2)</sup>。

DSL 部署意味着使用解析器、分析器和代码生成器来评估和执行 DSL 规范背后的代码。此外，为了方便 DSL 的部署，我们可以找到集成开发环境 (IDE)，例如 Visual Studio、Eclipse、NetBeans 和 WebStorm 等，它们提供了用于设计和实现 DSL 的工具，专用语言和框架。专注于 Web 应用开发，当我们必须定义易于集成到 Web 应用构建和部署中的语法时，有两种广泛采用的事实标准：基于 XML 的语法和基于 JSON 的语法。

因此，一方面，当 DSL 基于 XML 标准 <sup>[3](#3)</sup>、<sup>[4](#4)</sup> 时，即 DSL 遵循 XML 语法（XML-DSL），则可以使用文档对象模型（Document Object Model, DOM）<sup>[5](#5)</sup> 等通用解析器来指定特定领域的解决方案，评估和执行 DSL。此外，当这些方法使用 HTML5、SVG <sup>[6](#6)</sup>、MathML <sup>[7](#7)</sup> 和 XSLT <sup>[8](#8)</sup> 等语言时，解决方案在客户端会得到增强，无论是在视觉上还是功能上。作为示例，<ins>我们可以提到 <sup>[9](#9)</sup> 中的工作，在其中我们可以找到 PsiEngine，一个用于 Web 客户端的 XML-DSL 执行引擎，以及一组促进这些 DSL 开发和运行的工具。在 <sup>[9](#9)</sup>、<sup>[10](#10)</sup>中，作者展示了使用基于 XML 的解决方案的 DSL案例研究，这些解决方案使用 PsiEngine 来解决不同的特定领域问题。</ins>

另一方面，JSON 标准 <sup>[11](#11)</sup> 侧重于服务器端和客户端的信息交换。因此，我们可以提到用于链接数据的 JSON （JSON-LD） <sup>[12](#12)</sup>、<sup>[13](#13)</sup>，它允许交换可自动读取和共享的结构化信息。然而，当我们指定遵循 JSON 语法的 DSL（JSON-DSL）时会出现几个问题，即：如何定义 JSON-DSL 语法，我们可以使用哪些解析器、分析器和代码生成工具来运行 DSL，如何评估用 JSON-DSL 编写的程序，以及多个程序和多个 JSON-DSL 是否可以交互。本文提出了一种解决所有这些问题的方法。

尽管 Web 应用的相关性日益增长，并且科学界和工业界对使用此类应用表现出浓厚的兴趣，但文献中很少有研究涉及服务器端和客户端 Web 应用的 JSON-DSL 的规范和评估。关于这个主题的文献主要关注用于解决特定领域问题的 JSON-DSL，而不是用于实现 JSON-DSL 的工具和方法。大多数作品都涉及 JSON-DSL 的规范及其工作原理，无论桌面应用、服务器端还是客户端 Web 应用。因此，仅举几例，Canis <sup>[14](#14)</sup> 是一种允许 JSON 规范用于数据驱动的图形动画的高级语言，JSON-P <sup>[15](#15)</sup> 展示了一个关于开发简单人机对话播放器的案例研究，JS4Geo <sup>[16](#16)</sup> 是用于在 NoSQL 数据库中存储地理数据的规范 JSON schema，而 JSON-LS <sup>[17](#17)</sup> 促进与 BioThings API 的交叉链接以进行知识探索。

因此，在本文中，我们提出了一种构建 JSON-DSL 的架构，称为 RhoArchitecture，以我们之前的 PsiArchitecture <sup>[9](#9)</sup> 命名。该架构包括：
- （a）JSON-DSL Rho 评估引擎（简称 RhoEngine），它是能够运行用不同 JSON-DSL 编写的多个程序的 JavaScript 组件；
- （b）Rho 编程模型（简称 RhoModel），它建立一个编程模型来添加 JavaScript 功能并支持相应的代码生成和文档；
- （c）一个集成的 Web 开发环境，称为 Rho 的 Web 集成开发环境（简称 WebIDERho），允许指定、实现和部署基于 NodeJS 的服务器端和客户端项目，以及可视化类图。

我们的方法允许：
- （i）JSON-DSL 的规范和评估；
- （ii）可以与 DSL 交互的 JavaScript 组件的实现；
- （iii）JavaScript 模板引擎的应用，它可以作为程序员有效和高效地生成用 HTML、JavaScript、CSS 等编码的字符串的一种方式；
- （iv）连接异构信息源（JSON、XML 和文本），嵌入数据并将其与其他小部件、组件和 Web 框架集成。

凭借所有这些功能，我们的目标是为服务器端和客户端 Web 应用程序创建快速、稳健且灵活的解决方案。

通过 RhoArchitecture 的这三个部分（RhoEngine、RhoModel 和 WebIDERho），我们尝试为在 JSON-DSL 的规范、实现和部署中应用模型驱动工程 (MDE) 奠定基础。MDE 是一种软件工程范式，专注于定义领域模型，以简化信息系统 的构建 <sup>[18](#18)</sup>。因此，通过将 JSON-DSL 的概念与代码生成和转换引擎相结合，我们为将 MDE 应用于 Web 应用程序奠定了坚实的基础。

我们将提供四个案例研究，以展示 JSON-DSL 规范和评估的能力，以及 RhoArchitecture 中 JavaScript 组件的实现。第一个案例研究是经典的 “Hello World”，用于展示 JSON-DSL 的实现和执行。第二个案例研究重点介绍了集成管理多个异构信息源（XML、JSON 和文本）的能力。第三个案例研究旨在通过创建 Web 服务来验证服务器端编程，该服务包含 JSON-DSL 规范、模板引擎的使用和网页设计。最后一个使用案例，我们称之为 DrawRho，它以集成的方式验证了 RhoArchitecture 提出的所有特性，包括与其他框架的接口。在所有这些案例研究中，我们都遵循了 <sup>[19](#19)</sup> 建议的定性案例研究方法，并在 <sup>[20](#20)</sup> 中将其应用于软件工程， 以验证我们方法中最相关的特性。

本文的其余部分结构如下：第 2 节重点介绍相关工作；第 3 节概述了 RhoArchitecture 和我们方法的相关特性；第 4 节展示了四个案例研究；第 5 节详细介绍了我们获得的结果；最后，第 6 节以一些结论和未来工作结束本文。

## 2. 概述及相关工作
领域特定语言 (DSL) 一词在文献中没有严格的定义。Fowler  <sup>[2](#2)</sup> 将其定义为“一种专注于特定领域的、表达能力有限的计算机编程语言”。在 <sup>[21](#22)</sup>、<sup>[22](#22)</sup>、<sup>[23](#23)</sup> 中，<ins>作者们一致认为 DSL 是一种针对特定问题的编程语言，它的语法和语义包含与问题域相同的抽象级别，它的目标是促进信息系统的设计、定义和实现，为问题域提供解决方案</ins>。此外，根据 <sup>[2](#2)</sup>、<sup>[23](#23)</sup>， DSL 提供了合适的语法，以便领域专家可以更有效地执行这些任务，并生成更高质量和更可靠的系统。另一方面，<sup>[24](#24)</sup> 中的成果展示研究了语言的语法组成，并通过如下的考虑因素对 DSL 进行分类：语言扩展、语言限制、语言统一、自扩展和扩展组合来。

在 <sup>[25](#25)</sup> 中 ，我们可以找到一项系统映射研究（SMS1），它利用 2011 年之前的出版物，确定了最流行的DSL应用领域（按顺序排列：Web、网络、数据密集型应用、控制系统、低级软件、并行计算、可视化语言、嵌入式系统、实时系统、动态系统等）。此外，他们还开展了多项研究，列出了处理 DSL 的技术、方法和/或流程。最后，SMS1 对不同研究类型和领域进行了比较分析。

在 <sup>[26](#26)</sup> 中， 我们可以找到另一项关于 DSL 的系统映射研究（SMS2），该研究旨在识别 2006 年至 2012 年期间的研究趋势。作者寻找了可能尚未解决的问题，并对他们所谓的文献人口统计数据进行了分析。在 SMS2 中，作者观察到 DSL 社区似乎对开发支持 DSL 开发过程不同阶段（分析、设计和实现）的新技术和方法更感兴趣，而不是研究新工具，而且只有一小部分研究侧重于验证和维护。此外，作者还观察到，大多数研究并没有表明他们用于实现的工具。

<ins>此外，我们可以在 <sup>[27](#27)</sup> 中找到最新的系统映射研究（SMS3），该研究识别并映射了 2012 年至 2019 年间出版物中的工具和 IDE（作者称之为语言工作台LW）。在分析了 230 多篇论文后，他们确定了 59 种工具（9 种采用商业许可，41 种采用非商业许可），并得出结论，这些工具基本涵盖了 <sup>[26](#26)</sup> 中提出的特性（分为以下类别：符号、语义、编辑器、验证、测试和可组合性）</ins>。此外，在 SMS3 中，作者观察到开发人员采用了一种文本或图形符号来实现他们的 DSL。

DSL 的实现涉及使用解析器、分析器和代码生成工具来获得运行 DSL 的功能。一直以来，大多数解释器和编译器都是基于 Lex 和 Yacc <sup>[28](#28)</sup> 或 Flex 和 Bison <sup>[29](#29)</sup> 的。此外，当前的 IDE 提供了专门的工具、插件和语言，以简化 DSL 的设计和实现。例如，Visual Studio 具有用于构建基于模型的 DSL 的 软件开发工具包（SDK）<sup>[30](#30)</sup>，Eclipse 提供了各种用于构建 DSL 的专用插件，如 Stratego/XT <sup>[31](#31)</sup>、LISA <sup>[32](#32)</sup>、Spoofax <sup>[33](#33)</sup>、Antlr <sup>[34](#34)</sup>、Xtext <sup>[35](#35)</sup>、<sup>[36](#36)</sup> 和 Eclipse Modeling Project <sup>[37](#37)</sup> 。从MDE <sup>[18](#18)</sup>、<sup>[38](#38)</sup>、<sup>[39](#39)</sup> 的角度来看，关于用于构建建模语言的软件产品、平台和转换工具的综述可参见 <sup>[40](#40)</sup>。同样，使用通用编程语言，结合特定的设计模式和方法，我们可以构建内部 DSL，例如 Java <sup>[23](#23)</sup>、<sup>[41](#41)</sup>、C# <sup>[42](#42)</sup>、Scala <sup>[23](#23)</sup>、<sup>[43](#43)</sup>、Ruby  <sup>[23](#23)</sup>、Kotlin <sup>[44](#44)</sup>、Rust <sup>[45](#45)</sup>、Groovy <sup>[23](#23)</sup>、<sup>[46](#46)</sup>、Python <sup>[47](#47)</sup>、Clojure <sup>[48](#48)</sup> 和 Haskell <sup>[49](#49)</sup>。

因此，据我们所知，许多用于创建 DSL 的工具主要侧重于创建文本或图形 DSL。然而，目前尚无实现 JSON-DSL 的解决方案。上述 SMS1、SMS2 和 SMS3 并未明确提及 JSON-DSL 的创建，也未提及 Web 客户端 DSL 的创建。上述论文 <sup>[14](#14)</sup>、<sup>[15](#15)</sup>、<sup>[16](#16)</sup>、<sup>[17](#17)</sup> 描述了它们的规范以及 JSON-DSL 功能的实现是如何明确地临时执行的。

由于存在这两个缺点 —— 需要提供用于构建 JSON-DSL 的工具；以及需要一个执行引擎来在 Web 应用程序的服务器端和客户端运行用 JSON-DSL 编写的程序 —— 我们的工作重点是满足这些需求并创建案例研究来验证我们的提案。

## 3. 构建基于 JSON 的领域特定语言的方法
本节详细介绍了基于 JSON 的领域特定语言 (JSON-DSL) 解决方案的规范和实现方法，适用于服务器端和客户端的 Web 应用程序。本节将介绍 RhoArchitecture 及其三个部分（RhoEngine、RhoModel 和 WebIDERho）的核心思想。因此，我们将首先简要概述该方法，然后再进行分解。

### 3.1 简要概述
RhoArchitecture 的基石是 RhoEngine，这是一个 JavaScript 组件，可以运行用不同 JSON-DSL 编写的多个程序。JSON-DSL 是一种遵循 JSON 语法的编程语言，而 RhoLanguage 则是 JSON-DSL 加上实现语法元素功能的 JavaScript 类。JSON-DSL 程序的运行会评估嵌套语法符号的功能，从根开始，并根据语法定义向下钻取。在 RhoEngine 中运行的 JSON-DSL 可以连接和交换异构信息、使用模板引擎、使用组件和 Web 组件、应用安全策略，并遵循良好的编程实践 <sup>[50](#50)</sup>、<sup>[51](#50)</sup>，从而使其代码更加实用、可靠和健壮。

为了使 JSON-DSL 与 RhoEngine 兼容，我们必须遵循 RhoModel，它建立了一个用于生成JavaScript 代码和文档的编程模型。正如我们将要详细介绍的，RhoModel 专注于指定 JSON-DSL，并在服务器端和客户端实现 JavaScript 组件和其他必要的可重用 Web 组件。RhoModel 基于我们之前的 PsiModel <sup>[9](#9)</sup>。

最后，WebIDERho 使用 RhoModel 作为编程模型，并使用 RhoEngine 作为执行环境。WebIDERho 使我们能够在服务器端和客户端定义项目、可视化类图、自动生成文档，并部署基于 NodeJS 的服务器端和客户端 Web 应用程序。

### 3.2 Rho架构
为了更好地理解，[Figure 1](#figure-1) 展示了软件架构，该架构在概念层面上定义了我们处理 JSON-DSL 的方法中所涉及的组件，即 RhoArchitecture。该架构的主要目标是促进以下两个步骤：

1. *指定 JSON-DSL* ：在此步骤中，使用 RhoModel（位于 [Figure 1](#figure-1) 的中下部分）将 JSON-DSL 定义为 RhoLanguage。为此，首先我们必须定义 JSON-DSL 语法，然后实现与语法元素（终端符号和非终端符号）相关的功能。在 RhoModel 中，必须通过从 RhoLanguage 基类继承来将此功能作为一组 JavaScript 类提供，以减轻程序员的任务。与语法的每个元素相关的功能在我们称之为组件（位于 [Figure 1](#figure-1) 的底部）的部分中实现。在提供语法和相关功能之后，RhoModel 可以在评估相应的 JSON-DSL 代码时生成 JavaScript 代码。

2. *评估 JSON-DSL* ：此步骤的主要目标是获取用 RhoLanguage 编写的 RhoCode 程序（ [Figure 1](#figure-1) 左侧）以及必要的资源（即 HTML 片段、图像、视频、css、svg 等），然后使用 RhoEngine 运行 RhoCode。为此，RhoEngine 通过将 RhoCode 转换为 JavaScript 对象（ [Figure 1](#figure-1) 右侧的 RhoObject ）来生成 RhoProgram。此 RhoObject 是通过执行语法的每个嵌套元素的功能生成的，从根元素开始，并根据其语法定义向下钻取。获得的 RhoObject 解决了特定问题，可以在服务器端或客户端的 Web 应用程序中运行。执行此步骤时，将报告编译和执行错误以供处理。

[Figure 1](#figure-1) 顶部的数据源组件封装了管理 JSON、XML 和文本格式信息的功能。因此，RhoArchitecture 中定义的任何对象或类都可以使用这些数据源组件获取外部信息，并在运行时链接其一个或多个属性。因此，该提案允许我们引入异构信息，并通过适当的参数化将其分配给类。因此，该方法可以将数据源与 JSON-DSL 解耦，从而在 Web 应用程序中实现针对特定问题的解决方案时增强其多功能性 (versatility) 和 能力（power)。

在 [Figure 1](#figure-1) 的底部，我们可以看到模板。它们是预先设计的文本，包含变量标签，可以动态调整以生成自定义文本输出。这些模板可用于生成 HTML、SVG、JSON、XML 和 JavaScript 代码，从而加快 Web 应用程序的开发速度。为了自定义管理模板的使用，在 [Figure 1](#figure-1) 的左侧，我们可以看到模板引擎。模板引擎是一个 JavaScript 组件，它允许使用不同的特定语法（通常在内存中预编译）创建自定义模板，以快速高效地生成字符串。市场上有各种各样的模板引擎，特别是 EJS <sup>[52](#52)</sup>、Handlebars <sup>[53](#53)</sup>、<sup>[54](#54)</sup> 和 Hogan  <sup>[55](#55)</sup> 引擎。这些引擎作为插件添加到 RhoArchitecture 中，可由 RhoEngine 轻松使用。此外，RhoArchitecture 实现了一个简单的本机模板引擎，我们称之为 Plain，其功能有限。在创建 JSON-DSL 时使用这些模板引擎，结合数据源绑定，可以成倍地提高这些语言的能力和多功能性。

在 [Figure 1](#figure-1) 的底部，我们可以看到 Web 组件 [56]、[57]、[58]、[59] ，它们是用 HTML、 DOM 、JavaScript 和 CSS构建的小部件或可重用组件，部署在 Web 应用程序中。RhoArchitecture 使用模板引擎自动生成 Web 组件的代码。为了将这些编程组件和工具整合到我们的编程模型中，我们实现了工厂 Web 组件 (Factory Web Components) 和插件模板引擎 (Plugins Templates Engine)（参见 [Figure 1](#figure-1) RhoModel 框左侧）。

#### Figure 1
![Figure 1](pic/1-s2.0-S2590118423000138-gr1.jpg "Figure 1: RhoArchitecture 的功能块图")

[Figure 1 高清图](pic/1-s2.0-S2590118423000138-gr1_lrg.jpg)

到目前为止可以推断，我们方法背后的主要思想是高度重视代码生成，这是软件工程中一个成熟的领域，尤其注重模型驱动工程 <sup>[60](#60)</sup>、<sup>[61](#61)</sup>。代码生成在构建信息系统时可以节省时间、提高效率、提高质量和标准化 <sup>[62](#62)</sup>、<sup>[63](#63)</sup>。因此，在这种情况下，为了简化 RhoLanguages 的规范（我们将在后面的第 3.4 节中详细介绍），我们需要 RhoModel（见 [Figure 1](#figure-1) ）。这允许定义 RhoGrammars
以及基于 RhoEngine 的模板引擎、Web 组件和 JavaScript 组件的实现。RhoModel 将规范与实现分离，这源于我们之前对 PsiModel <sup>[9](#9)</sup> 和自动代码生成的研究成果。

总而言之，使用 RhoArchitecture，为 RhoEngine 实现的任何 JSON-DSL 都能够连接到异构数据源（XML、JSON、文本等），使用模板引擎，与 Web 组件协同工作，以增加语言的多功能性和功能性 (functionality)，并且如果应用安全策略和良好的编程实践，则可以获得相当可靠和强大的执行。


----
#### 1
Voelter M.</br>
DSL Engineering: Designing, Implementing and using Domain-Specific Languages</br>
Dslbook (2013)

#### 2
Fowler M., White T.</br>
Domain-Specific Languages</br>
Addison-Wesley Professional, Denver (2010)

#### 3
W3c</br>
Extensible Markup Language (XML) Version 1.1</br>
W3C Recomm (2006)</br>
https://www.w3.org/standards/xml/ (accessed August 30, 2021)

#### 4
Fawcett J., Quin L., A D.</br>
Beginning XML</br>
(fifth ed.), Wrox Press (2012)

#### 5
W3C</br>
Document Object Model (DOM) Level 3 Core Specification Version 1.0</br>
W3C Recomm (2004)</br>
https://www.w3.org/TR/DOM-Level-3-Core/ (accessed September 3, 2021)

#### 6
W3C</br>
Scalable Vector Graphics (SVG) 2</br>
W3C Candidate Recomm (2018)</br>
https://www.w3.org/TR/SVG2/ (accessed August 30, 2021)

#### 7
W3C</br>
Mathematical Markup Language (MathML) Version 3.0</br>
W3C Recomm (2014)</br>
https://www.w3.org/TR/MathML3/ (accessed September 1, 2021)

#### 8
W3C</br>
XSL Transformation (XSLT) Version 2.0</br>
W3C Recomm (2021)</br>
https://www.w3.org/TR/xslt20/ (accessed September 1, 2021)

#### 9
Chavarriaga E., Jurado F., Díez F.</br>
An approach to build XML-based domain specific languages solutions for client-side web applications</br>
Comput. Lang. Syst. Struct., 49 (2017), 10.1016/j.cl.2017.04.002

#### 10
Chavarriaga E., Jurado F., Díez F.</br>
PsiLight: A lightweight programming language to explore multiple program execution and data-binding in a web-client DSL evaluation engine</br>
J. Univers Comput. Sci., 23 (2017), pp. 953-968

#### 11
ECMA</br>
ECMA-404: The JSON Data Interchange Syntax</br>
(first ed.) (2018)</br>
https://www.ecma-international.org/publications-and-standards/standards/ecma-404/ (accessed September 2, 2021)

#### 12
W3C Recommendation</br>
JSON-LD 1.1: A JSON-Based Serialization for Linked Data (W3C Recommendation 16 July 2020)</br>
(2020) https://www.w3.org/TR/json-ld/

#### 13
Web Payments Working Group</br>
JSON for Linking Data</br>
(2022) https://json-ld.org/

#### 14
Ge T., Zhao Y., Lee B., Ren D., Chen B., Wang Y.</br>
Canis: A high-level language for data-driven chart animations</br>
Comput. Graph. Forum, 39 (2020), pp. 607-617

#### 15
Sarasa-Cabezuelo A., Sierra J.-L.</br>
Grammar-driven development of JSON processing applications</br>
2013 Fed. Conf. Comput. Sci. Inf. Syst. (2013), pp. 1557-1564

#### 16
Frozza A.A., Mello R., dos S.</br>
JS4Geo: a canonical JSON schema for geographic data suitable to NoSQL databases</br>
Geoinformatica, 24 (2020), pp. 987-1019

#### 17
Xin J., Afrasiabi C., Lelong S., Adesara J., Tsueng G., Su A.I., et al.</br>
Cross-linking BioThings APIs through JSON-LD to facilitate knowledge exploration</br>
BMC Bioinformatics, 19 (2018)
1-N.PAG.

#### 18
Schmidt D.C.</br>
Model-driven engineering</br>
Comput, 39 (2006), pp. 25-31

#### 19
Yin R.K.</br>
Case Study Research: Design and Methods</br>
(fifth ed.), Sage Publications, Inc., London (2014)

#### 20
Baxter P., Jack S.</br>
Qualitative case study methodology. Study design and implementation for novice researchers</br>
Qual. Rep. (2008), pp. 13-17

#### 21
Mernik M., Heering J., Sloane A.M.</br>
When and how to develop domain-specific languages</br>
ACM Comput. Surv., 37 (2005), pp. 316-344

#### 22
Spinellis D.</br>
Notable design patterns for domain-specific languages</br>
J. Syst. Softw., 56 (2001), pp. 91-99

#### 23
Ghosh D.</br>
DSLs in Action</br>
Manning Publications, Greenwich (2010)

#### 24
Erdweg S., Giarrusso P.G., Rendel T.</br>
Language composition untangled</br>
Proc. 12th Work. Lang. Descr. Tools, Appl. LDTA 2012 (2012),

#### 25
do Nascimento L.M., Viana D.L., Neto P.A.S., Martins D.A., Garcia V.C., Meira S.R.</br>
A systematic mapping study on domain-specific languages</br>
Seventh Int. Conf. Softw. Eng. Adv. (ICSEA 2012) (2012), pp. 179-187

#### 26
Kosar T., Bohra S., Mernik M.</br>
Domain-specific languages: A systematic mapping study</br>
Inf. Softw. Technol. (2016), p. 71,

#### 27
Iung A., Carbonell J., Marchezan L., Rodrigues E., Bernardino M., Basso F.P., et al.</br>
Systematic mapping study on domain-specific language development tools</br>
Empir. Softw. Eng., 25 (2020), pp. 4205-4249

#### 28
Brown D., Levine J., Mason T.</br>
Lex & Yacc</br>
(second ed.), O’Reilly Media, New York (1992)

#### 29
Levine J.</br>
Flex & Bison</br>
O’Reilly Media, Sebastopol (2009)

#### 30
Microsoft</br>
Modeling SDK for visual studio - Domain-specific languages</br>
(2022)</br>
https://docs.microsoft.com/en-us/visualstudio/modeling/modeling-sdk-for-visual-studio-domain-specific-languages?view=vs-2022

#### 31
Bravenboer M., Kalleberg K.T., Vermaas R., Visser E.</br>
Stratego/XT 0.17. A language and toolset for program transformation</br>
Sci. Comput. Program., 72 (2008), pp. 52-70

#### 32
Mernik M., Lenič M., Avdičaušević E., Žumer V.</br>
LISA: An interactive environment for programming language development</br>
Int. Conf. Compil. Constr. (2002), pp. 1-4

#### 33
Kats L.C.L., Kalleberg K.T., Visser E.</br>
Domain-specific languages for composable editor plugins</br>
Electron. Notes Theor. Comput. Sci. (2010), p. 253,

#### 34
Rajan H.</br>
ANTLR: A brief review</br>
(2022)

#### 35
Bettini L.</br>
Implementing Domain-Specific Languages with Xtext and Xtend</br>
Packt Publishing (2013)

#### 36
Toussaint M., Baar T.</br>
Enriching Textual Xtext-DSLs with a Graphical GEF-Based Editor, LNCS, vol. 10742, Springer Verlag (2018)</br>

#### 37
Gronback R.</br>
Eclipse Modeling Project: A Domain-Specific Language (DSL) Toolkit</br>
Addison-Wesley Professional, Denver (2009)

#### 38
Brambilla M., Cabot J., Wimmer M., Baresi L.</br>
Model-Driven Software Engineering in Practice</br>
(second ed.) (2017)

#### 39
Diez A., Nguyen N., Diez F., Chavarriaga E.</br>
MDE for enterprise application systems</br>
Model. 2013 - Proc. 1st Int. Conf. Model. Eng. Softw. Dev. (2013)

#### 40
Bettini L.</br>
Implementing Domain-Specific Languages with Xtext and Xtend</br>
Packt Publishing Ltd (2016)

#### 41
Bettini L.</br>
Implementing Domain-Specific Languages with Xtext and Xtend</br>
Packt Publishing Ltd (2016)

#### 42
Kourie D.G., Fick D., Watson B.W.</br>
Virtual machine framework for constructing domain-specific languages</br>
IET Softw., 3 (2009), pp. 1-13

#### 43
Pollak D., Layka V., Sacco A.</br>
DSL and Parser Combinator. Begin. Scala 3</br>
Springer (2022), pp. 237-245

#### 44
Subramaniam V.</br>
Programming DSLs in Kotlin</br>
Pragmatic Bookshelf (2021)

#### 45
Segeljakt K.</br>
A Scala DSL for Rust code generation</br>
(2018)

#### 46
Dearle F.</br>
Groovy for Domain-Specific Languages</br>
packt Publishing Ltd (2015)

#### 47
McGuire P.</br>
Getting Started with Pyparsing</br>
O’Reilly Media, Inc. (2007)

#### 48
Kelker R.D</br>.
Clojure for Domain-Specific Languages</br>
Packt Publishing (2013)

#### 49
Valliappan N., Krook R., Russo A., Claessen K.</br>
Towards secure IoT programming in Haskell</br>
(2020)

#### 50
Yue C., Wang H.</br>
A measurement study of insecure javascript practices on the web</br
ACM Trans. Web, 7 (2013), pp. 1-39

#### 51
Czech G., Moser M., Pichler J.</br>
Best practices for domain-specific modeling. A systematic mapping study</br>
2018 44th Euromicro Conf. Softw. Eng. Adv. Appl. (2018), pp. 137-145

#### 52
Eernisse M.</br>
Embedded JavaScript Templating (EJS)</br>
(2012)</br>
https://ejs.co (accessed November 15, 2021)

#### 53
Mardan A.</br>
Template Engines: Pug and Handlebars. Pract. Node. js</br>
Springer (2018), pp. 113-163

#### 54
Katz Y.</br>
Handlebars: minimal templating on steroids</br>
(2021)</br>
https://handlebarsjs.com/ (accessed November 10, 2021)

#### 55
Hogan Velasco A.</br>
JavaScript templating from Twitter</br>
(2021)</br>
http://twitter.github.io/hogan.js/

#### 56
WEBCOMPONENTS.ORG</br>
Discuss & share web components</br>
(2021)</br>
https://www.webcomponents.org/ (accessed October 11, 2021)

#### 57
W3C</br>
Introduction to Web Components. W3C Work Gr Note</br>
(2014)</br>
https://www.w3.org/TR/components-intro/ (accessed October 1, 2021)

#### 58
Gupta A., Ahirwar M., Pandey R.</br>
Creating website as a service using web components</br>
Int. Res. J. Eng. Technol., 6 (2019)

#### 59
Molina P.J.</br>
Quid: prototyping web components on the web</br>
Proc. ACM SIGCHI Symp. Eng. Interact. Comput. Syst. (2019), pp. 1-5

#### 60
Lano K., Xue Q., Kolahdouz-Rahimi S.</br>
Agile specification of code generators for model-driven engineering</br>
ICSEA 2020 (2020), p. 19

#### 61
Barth T., Fromm I.P.</br>
Modeling and code generation for safety critical systems</br>
Embed. World Conf., Vol. 2020 (2020)

#### 62
Sebastián G., Gallud J.A., Tesoriero R.</br>
Code generation using model driven architecture: A systematic mapping study</br>
J. Comput. Lang., 56 (2020), Article 100935

#### 63
Prout A., Atlee J.M., Day N.A., Shaker P.</br>
Code generation for a family of executable modelling notations</br>
Softw. Syst. Model. (2012), p. 11, 10.1007/s10270-010-0176-6

