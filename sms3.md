# 领域专用语言开发工具的系统性映射研究
An ́ıbal Iung<sup>[1](#1)</sup> · Joa ̃ o Carbonell<sup>[1](#1)</sup> · Luciano Marchezan<sup>[1](#1)</sup> · Elder Rodrigues<sup>[1](#1)</sup> · Maicon Bernardino<sup>[1](#1)</sup> · Fabio Paulo Basso<sup>[1](#1)</sup> · Bruno Medeiros<sup>[1](#1)</sup>


2020/08 [original english](origin/iung2020.pdf)

----
## 摘要
领域特定语言（DSL）是专用于特定应用领域的编程或建模语言。支持 DSL 实现的工具种类繁多，使得选择合适工具的决策过程变得困难。因此，识别并映射这些工具的特性对于学术界和工业界在 DSL 开发中的决策具有重要意义。目标：本研究旨在识别并梳理 2012 至 2019 年间文献中提及的、用于开发 DSL 的工具、语言工作台（ Language Workbenchese, LW ）或框架。方法：采用文献的系统性映射研究（ Systematic Mapping Study, SMS ）方法，聚焦 DSL 开发工具。结果： 我们共识别出 59 种工具（含 9 种商业许可工具与 41 种非商业许可工具），并从 230 篇论文中分析其功能特性。结论：现有工具数量庞大且覆盖功能广泛。此外，我们观察到开发者通常采用单一类型的表示法 (notation) 来实现 DSL（文本或图形）。我们还探讨了研究空白，例如：缺乏支持元元模型转换，以及对建模工具互操作性的支持。

----
## 关键词
DSL · Domain-specific language · Language workbench ·
Model driven engineering · DSL-supporting tools · Systematic mapping study · Systematic review

----
## 1. 引言
特定领域的计算机系统实施正变得日益复杂，需要整合多个知识领域。例如，现代 Web 系统的编码需要考虑多个实现要点，包括可用性、安全性、持久性及业务规则。为使这些不同要点能独立于编码技术进行开发，软件工程师正采用领域特定语言（DSL）的开发模式（[Fowler 2010](#fowler-m-2010)），这是对特定领域特征进行建模和编码的最常用方法之一。DSL 已被应用于众多领域，包括性能测试开发（[Bernardino et al. 2016](#bernardino-m-zorzo-af-rodrigues-em-2016)）、机器深度学习（[Zhao and Huang 2018](#zhao-t-huang-x-2018)）、面向对象的领域驱动设计（[Le et al. 2018](#le-dm-dang-d-h-nguyen-v-h-2018)）、学校课表自动生成（[Ribić et al. 2018](#ribic-́-s-turcˇinhodzˇic-́-r-muratovic-́-ribic-́-a-kosar-t-2018)）以及用于通信与查询的数据库语言 （[Fowler 2010](#fowler-m-2010)）。

在此背景下，为设计捕获软件需求的所有元素，需要开发图形化和/或文本化的 DSL，以向最终用户提供最佳前端体验。DSL 能实现比软件工程日常实践中常用编程/建模语言更高的抽象层次。例如，当前开发实践中，开发者使用集成开发环境（IDE）中的代码编辑器，通过通用语言（ General Purpose Languages, GPL）（[Fowler 2010](#fowler-m-2010)）如 C 和 Python， 实现代码。与此同时，基于平台无关性理念的方法正在改变这种开发模式，通过引入更高层次的语言，使其与 C 和 Python 等具体实现无关。

要实现这种平台独立性，即通过与具体实现细节无关的软件开发项目，可能需要创建新的 DSL 来捕捉领域概念。然而，创建新 DSL 并非易事，需要借助多种 DSL 开发工具。这些工具支持 DSL 的开发与维护过程，确保其一致性、演进性和可维护性。此外，这些工具可能包含代码生成器、验证器、模型检查器，以及词法分析器、语义分析器和语法分析器。通常，这些集成工具属于名为语言工作台（Languages Workbenches, LWs）的工具箱（[Fowler 2010](#fowler-m-2010)），其使用流程通常以工具链的形式呈现（[Jakumeit et al. 2014](#jakumeitebuchwaldswagelaarddanlhegedusa-́herrmannsdorfermhorntkalninaekrause-c-lano-k-lepper-m-rensink-a-rose-l-watzoldt-s-mazanek-s-2014)）。

为实现某些软件开发流程任务的自动化（[Whittle et al. 2015](#whittle-j-hutchinson-j-rouncefield-m-ha-̊kan-b-rogardt-h-2015)），在启用工具链之前必须谨慎选择每项工具（[Basso et al. 2017](#basso-fp-werner-cml-de-oliveira-tc-2017)）。这一过程颇为复杂，因为某些工具仅能协助 DSL 开发的单一环节（[Mussbacher et al. 2014](#mussbacher-g-amyot-d-breu-r-bruel-j-m-cheng-bhc-collet-p-combemale-b-france-rb-hel--dal-r-hill-j-kienzle-j-scho-̈ttle-m-steimann-f-stikkolorum-d-whittle-j-2014)），而另一些工具则能覆盖多个环节。这种多样性虽有益于实践，却也增加了理解的难度，既要厘清启动 DSL 辅助流程所需的核心功能，又要识别哪些互补工具能在特定场景下支持语言构建（[Liebel et al. 2014](#liebel-g-marko-n-tichy-m-leitner-a-hansson-jo-̈rgen-2014)）。

尽管该领域文献中充斥着关于 DSL 开发工具的参考文献，却缺乏对其质量属性的最新图谱。在此背景下，一项关于这些工具的映射研究，调研有助于技术决策的功能与信息，将会帮助软件工程师在特定开发场景中选择最优方案。欢迎提交关于工具链决策的论文（[Liebel et al. 2014](#liebel-g-marko-n-tichy-m-leitner-a-hansson-jo-̈rgen-2014)），因为缺乏此类映射会，阻碍选择合适的元生成器，这是构建特定领域专用工具，以支持某种软件工程任务的的关键环节（[Jakumeit et al. 2014](#jakumeitebuchwaldswagelaarddanlhegedusa-́herrmannsdorfermhorntkalninaekrause-c-lano-k-lepper-m-rensink-a-rose-l-watzoldt-s-mazanek-s-2014)）。当前，该选择过程需要耗费时间，研究诸多影响决策的关键要素，包括技术层面，如哪些工具满足需求和描述性数据（用于理解其运作机制）。例如：类似 MetaEdit+（[Kelly et al. 1996](#kelly-s-lyytinen-k-rossi-m-1996)）这类工具虽在技术层面支持完整的 DSL 开发流程且应用广泛，但 MetaEdit+ 的商业许可模式可能与某些业务场景存在冲突。因此，在特定情境下，开发者可能需要寻找免费和/或非商业化的替代方案。

为进一步决策而对这些工具进行特征描述时，探讨 DSL 开发过程及其应用的研究至关重要。当前的特征化研究（[Pérez et al. 2013](#pe-́rez-f-valderas-p-fons-j-2013)；[Arkin and Tekinerdogan 2014](#arkin-e-tekinerdogan-b-2014)）虽提及了 DSL 创建中使用的工具，却未提供技术特征等足够细节。其他研究则聚焦于 DSL 领域的系统性映射（[do Nascimento et al. 2012](#do-nascimento-lm-viana-dl-neto-pas-martins-da-garcia-vc-meira-sr-2012)；[Erdweg et al. 2013](#erdweg-s-van-der-storm-t-volter-m-boersma-m-bosman-r-cook-wr-gerritsen-a-hulshout-a-2013)； [Erdweg et al. 2015](#erdweg-s-van-der-storm-t-vo-̈lter-m-tratt-l-bosman-r-cook-wr-gerritsen-a-hulshout-a-kelly-s-loh-a-et-al-2015); [Kosar et al. 2016](#kosar-t-bohra-s-mernik-m-2016); [Méndez-Acuna et al. 2016](#me-́ndez-acuna-d-galindo-j-degueule-t-combemale-b-baudry-b-2016)），但这些综述并未梳理，支持特定 DSL 与 DSML 开发生命周期阶段的，工具及其功能特性。如高亮显示与错误标记。受限于此，DSL 构建的技术决策仍面临巨大挑战。

本文采用系统性映射研究（Systematic Mapping Study, SMS ）方法（[Kitchenham et al. 2011](#kitchenham-ba-budgen-d-brereton-op-2011)），全面梳理了支持 DSL 开发的工具。我们不仅阐明了 DSL 在不同领域的应用场景，更呈现了 DSL 开发工具研究领域的现状综述，同时指明了未来研究机遇与空白领域。该综述包含对跨应用领域的分析，即 DSL 被提出的领域，以及构建 DSL 所采用的实践方法。据我们所知，本 SMS 首次系统性地阐述了构建 DSL 的技术特性，涵盖结构特征及商业要素，如许可类型和适用性。因此，本研究为该领域做出了重要贡献。

本文结构如下：第 [2]() 节介绍全文使用的术语与概念；第 [3]() 节阐述 SMS 规划；第 [4]() 节报告对 DSL 开发工具的 SMS 如何执行；第 [5]() 节呈现为解答研究问题所收集的数据；第 [6]() 节探讨研究空白；第 [7]() 节阐述有效性面临的威胁（threats to validity）；最后第 [8]() 节总结全文。

----
## 原文注释
#### 1
通讯员: Daniel Amyot
- An ́ıbal Iung netoiiung@gmail.com
扩展的作者信息详见文章末页。

----
## 参考文献
#### A ̊ kesson A, Hedin G (2017)
Jatte : A tunable tree editor for integrated DSLs, vol 2

#### Akhundov J, Werner M, Schaus V, Gerndt A (2016)
Using timed automata to check space mission feasibility in the early design phases.</br>
In: 2016 IEEE Aerospace Conference, pp 1–9

#### Alvarez C, Casallas R (2013)
MTC Flow: A Tool to Design, Develop and Deploy Model Transformation Chains.</br>
In: Proceedings of the Workshop on ACadeMics Tooling with Eclipse, (ACME’13).</br>
ACM, New York, NY, USA, pp 7:1–7:9

#### A ́ ngel MS, de Lara J, Neubauer P, Wimmer M (2018)
Automated modelling assistance by integrating heterogeneous information sources.</br>
Computer Languages, Systems & Structures 53:90–120

#### Antonelli HL, da Silva EAN, Fortes RPM (2015)
A Model-driven Development for Creating Accessible Web Menus.</br>
Procedia Computer Science 67:95–104

#### Arcaini P, Mirandola R, Riccobene E, Scandurra P (2019)
A Pattern-Oriented Design Framework for Self-Adaptive Software Systems.</br>
In: 2019 IEEE International Conference on Software Architecture Companion (ICSA-C), IEEE, pp 166–169

#### Arendt T, Taentzer G, Weber A (2013)
Quality assurance of textual models within eclipse using OCL and model transformations.</br>
In: CEUR Workshop Proceedings, vol 1092, Miami, FL, United states, pp 1–10

#### Arkin E, Tekinerdogan B (2014)
Domain specific language for deployment of parallel applications on parallel computing platforms.</br>
In: ACM International Conference Proceeding Series (ICPS’14), Vienna, Austria, p University of Vienna

#### Azadi M. E, Azadi M. E, Challenger M (2015)
DSML4CP: A Domain-specific Modeling Language for Concurrent Programming.</br>
Computer Languages, Systems and Structures 44:319–341

#### Barbosa A, Silva F, Coutinho L, Santos D, Teles A (2019)
A Domain-Specific Modeling Language for Specification of Clinical Scores in Mobile Health.</br>
In: Proceedings of the 14th International Conference on Evaluation of Novel Approaches to Software Engineering, SCITEPRESS-Science and Technology
Publications, Lda, pp 104–113

#### Barisˇic ́ A, Amaral V, Goula ̃o M. (2017)
Usability driven DSL development with USE-ME.</br>
Computer Languages, Systems and Structures 51:1339–1351

#### Barisˇic ́ A, Blouin D, Amaral V, Goula ̃o M (2017)
A Requirements Engineering Approach for Usability driven DSL Development.</br>
In: Proceedings of the 10th ACM SIGPLAN International Conference on Software Language Engineering, (SLE’17). ACM, New York, NY, USA, pp 115–128

#### Bartman B, Newman CD, Collard ML, Maletic JI (2017)
SrcQL: A syntax-aware query language for source code.</br>
In: 24th IEEE International Conference on Software Analysis, Evolution, and Reengineering (SANER’17), pp 467–471

#### Basso FP (2017)
RAS++: representing hybrid reuse assets for MDE as a service, Ph.D. Thesis, Universidade Federal do Rio de Janeiro

#### Basso FP, Werner CML, de Oliveira TC (2017)
Revisiting Criteria for Description of MDE Artifacts.<br>
In: 2017 IEEE/ACM Joint 5th International Workshop on Software Engineering for Systems-of-Systems and 11th Workshop on Distributed Software Development, Software Ecosystems and Systems-of-Systems, JSOS@ICSE, Buenos Aires, Argentina, May 23, 2017, pp 27–33

#### Bencharqui H, Haidrar S, Anwar A (2019)
Dealing with Requirement Inconsistencies Based on ReqDL Lan- guage.</br>
In: 2019 International Conference on Wireless Technologies, Embedded and Intelligent Systems (WITS), IEEE, pp 1–6

#### Bennani S, El Hamlaoui M, Nassar M, Ebersold S, Coulette B (2018)
Collaborative model-based matching of heterogeneous models.</br>
In: 2018 IEEE 22nd International Conference on Computer Supported Cooperative Work in Design ((CSCWD)), IEEE, pp 443–448

#### Bergenti F (2014)
An Introduction to the JADEL Programming Language.</br>
In: 2014 IEEE 26th International Conference on Tools with Artificial Intelligence (ICTAI’14), pp 974–978

#### Bermu ́dez R. FJ, Sa ́nchez Ramo ́n O ́ , Garc ́ıa M. J (2017)
A tool to support the definition and enactment of model-driven migration processes.</br>
Journal of Systems and Software 128:106–129

#### Bernardino M, Zorzo AF, Rodrigues EM (2016)
Canopus: A Domain-Specific Language for Modeling Performance Testing.</br>
In: 2016 IEEE International Conference on Software Testing, Verification and Validation (ICST’16), pp 157–167

#### Bernardino M, Rodrigues E, Zorzo A, Marchezan L (2017)
A Systematic Mapping Study on Model-Based Testing: Tools and Models.</br>
IET Software (2017) 11:141–155

#### Bettini L (2014)
Developing user interfaces with EMF parsley.</br>
In: Proceedings of the 9th International Conference on Software Paradigm Trends (ICSOFT-PT’14), Vienna, Austria, pp 58–66

#### Bettini L (2019)
Type errors for the IDE with Xtext and Xsemantics.</br>
Open Computer Science 9(1):52–79

#### Bocciarelli P, D’Ambrogio A, Paglia E, Giglio A (2018)</br>
On the Performance Prediction Capabilities of the eBPMN-based Model-driven Method for Business Process Simulation.</br>
In: CIISE, pp 71–78

#### Bonnet S, Voirin JL, Exertier D, Normand V (2016)
Not (strictly) relying on SysML for MBSE: Language, tooling and development perspectives: The Arcadia/Capella rationale, pp 1–6

#### Boßelmann S, Naujokat S, Steffen B (2018)
On the difficulty of drawing the line. In: International Symposium on Leveraging Applications of Formal Methods, Springer, pp 340–356

#### Boubeta-Puig J, D ́ıaz G, Macia ́ H, Valero V, Ortiz G (2017)
MEdit4CEP-CPN: an approach for complex event processing modeling by prioritized Colored Petri Nets. Information Systems 81:267–289

#### Bousse E, Leroy D, Combemale B, Wimmer M, Baudry B (2018)
Omniscient debugging for executable DSLs.</br>
Journal of Systems and Software 137:261–288

#### Bousse E, Mayerhofer T, Combemale B, Baudry B (2019)
Advanced and efficient execution trace management for executable domain-specific modeling languages.</br>
Software & Systems Modeling 18(1): 385–421

#### Brdjanin D, Banjac D, Banjac G, Maric S (2018)
An Online Business Process Model-driven Generator of the Conceptual Database Model.</br>
In: Proceedings of the 8th International Conference on Web Intelligence, Mining and Semantics, ACM, p 16

#### Bruneliere H, Cabot J, Clasen C, Jouault F, Be ́zivin J (2010)
Towards model driven tool interoperability: bridging Eclipse and Microsoft modeling tools.</br>
In: European Conference on Modelling Foundations and Applications, (ECMFA’2010), Springer, pp 32–47

#### Buisson J, Rehab S (2018)
Effective Bridging Between Ecore and Coq: Case of a Type-Checker with Proof Carrying Code.</br>
In: International Symposium on Modelling and Implementation of Complex Systems, Springer, pp 259–273

#### Bu ̈nder H (2019)
Decoupling Language and Editor-The Impact of the Language Server Protocol on Textual Domain-Specific Languages.</br>
In: Proceedings of the 7th International Conference on Model-Driven Engineering and Software Development, SCITEPRESS-Science and Technology Publications, Lda, pp 129–140

#### Burdusel A, Zschaler S, Stru ̈ber D (2018)
MDEoptimiser: a search based model engineering tool.</br>
In: Pro- ceedings of the 21st ACM/IEEE International Conference on Model Driven Engineering Languages and Systems: Companion Proceedings, ACM, pp 12–16

#### Butting A, Dalibor M, Leonhardt G, Rumpe B, Wortmann A (2018)
Deriving Fluent Internal Domain-specific Languages from Grammars.</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language Engineering. SLE 2018, 2018, pp 187–199

#### Butting A, Eikermann R, Kautz O, Rumpe B, Wortmann A (2018)
Controlled and Extensible Variability of Concrete and Abstract Syntax with Independent Language Features.</br>
In: Proceedings of the 12th International Workshop on Variability Modelling of Software-Intensive Systems, ACM, pp 75–82

#### Caramujo J, da Silva AR, Monfared S, Ribeiro A, Calado P, Breaux T (2019)
RSL-IL4Privacy: a domain-specific language for the rigorous specification of privacy policies.</br>
Requirements Engineering 24(1):1–26

#### Cariou E, Le Goaer O, Brunschwig Le ́a, Barbier F (2018)</br>
A generic solution for weaving business code into executable models. In: MODELS Workshops, pp 251–256

#### Challenger M, Demirkol S, Getir S, Mernik M, Kardas G, Kosar T (2014)
On the use of a domain-specific modeling language in the development of multiagent systems.</br>
Engineering Applications of Artificial Intelligence 28:111–141

#### Chlipala A, Delaware B, Duchovni S, Gross J, Pit-Claudel C, Suriyakarn S, Wang P, Ye K (2017)
The end of history? Using a proof assistant to replace language design with library design.</br>
In: Leibniz International Proceedings in Informatics, (LIPIcs’17), vol 71

#### Combemale B, Barais O, Wortmann A (2017)
Language engineering with the GEMOC studio.</br>
In: Proceedings - 2017 IEEE International Conference on Software Architecture Workshops, (ICSAW’17): Side Track Proceedings, vol 54, pp 189–191

#### Combemale B, Kienzle J, Mussbacher G, Barais O, Bousse E, Cazzola W, Collet P, Degueule T, Heinrich R, Je ́ze ́quel J-M, et al. (2018)
Concern-oriented language development (COLD): Fostering reuse in language engineering.</br>
Computer Languages, Systems & Structures 54:139–155

#### Cook TD, Campbell DT (1979)
Quasi-Experimentation: Design and Analysis Issues for Field Settings, Houghton Mifflin

#### Co ́rdoba-Sa ́nchez I, de Lara J (2016)
Ann: A domain-specific language for the effective design and validation of Java annotations.</br>
Computer Languages, Systems and Structures 45:164–190

#### Corral Diaz MA (2018)
Software Development Tools in Model-Driven Engineering.</br>
In: Proceedings-2017 5th International Conference in Software Engineering Research and Innovation, CONISOFT 2017, 2018-January, volumen 2018, Proceedings-2017 5th International Conference in Software Engineering

#### Coulon F, Degueule T, Van Der Storm T, Combemale B (2018)
Shape-diverse DSLs: languages without borders (vision paper).</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language Engineering, ACM, pp 215–219

#### Crapo AW, Moitra A (2019)
Using OWL ontologies as a domain-specific language for capturing requirements for formal analysis and test case generation.</br>
In: 2019 IEEE 13th International Conference on Semantic Computing (ICSC), IEEE, pp 361–366

#### de Almeida Pereira DI, Malki O, Bon P, Perin M, Collart-Dutilleul S (2018)
An MDA approach for the specification of relay-based diagrams.</br>
In: International Conference on Model and Data Engineering, Springer, pp 17–29

#### De F. CD, Moreira A, Arau ́jo J, Amaral V (2017)
Towards security modeling of E-voting systems

#### de la Vega A, Garc ́ıa-Saiz D, Zorrilla M, Sa ́nchez P (2018)
Flandm: a development framework of domain-specific languages for data mining democratisation.</br>
Computer Languages, Systems & Structures 54:316–336

#### De Sousa LM, Da Silva AR (2016)
A domain specific language for spatial simulation scenarios.</br>
GeoInfor- matica 20(1):117–149

#### de Sousa LM, da Silva AR (2018)
Usability evaluation of the domain specific language for spatial simulation scenarios.</br>
Cogent Engineering 5(1):1436889

#### Dejanovic ́ I, Vaderna R, Milosavljevic ́ G, Vukovic ́ (2017)
TextX: A Python tool for Domain-Specific Languages implementation.</br>
Knowledge-Based Systems 115:1–4

#### Demirkol S, Challenger M, Getir S, Kosar T, Kardas G, Mernik M (2013)
A DSL for the development of software agents working within a semantic web environment.</br>
Computer Science and Information
Systems 10(4 SPEC.ISSUE):1525–1556

#### Denkers J, van Gool L, Visser E (2018)
Migrating custom DSL implementations to a language workbench (tool demo).</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language Engineering, ACM, pp 205–209

#### Derakhshanmanesh M, Ebert Ju ̈rgen, Grieger M, Engels G (2019)
Model-integrating development of software systems: a flexible component-based approach.</br>
Software & Systems Modeling 18(4):2557–2586

#### do Nascimento LM, Viana DL, Neto PAS, Martins DA, Garcia VC, Meira SR (2012)
A systematic mapping study on domain-specific languages.</br>
In: Proceedings of the 7th International Conference on Software Engineering Advances (ICSEA’12), pp 179–187

#### Do ̈rndorfer J, Hopfensperger F, Seel C (2019)
The SenSoMod-Modeler–A Model-Driven Architecture Approach for Mobile Context-Aware Business Applications.</br>
In: International Conference on Advanced Information Systems Engineering, Springer, pp 75–86

#### Dupont G, Mustafiz S, Khendek F, Toeroe M (2018)
Building Domain-specific Modelling Environments with Papyrus: An Experience Report.</br>
In: Proceedings of the 10th International Workshop on Modelling in Software Engineering. MiSE ’18, 2018, pp 49–56

#### Dwarakanath A, Era D, Priyadarshi A, Dubash N, Podder S (2017)
Accelerating Test Automation through a Domain Specific Language

#### Eclipse F (2017) Sirius. Available in: https://www.eclipse.org/sirius/

#### Eclipse F (2017) Sirius. Available in: https://www.eclipse.org/Xtext/

#### Eclipse F (2020) Acceleo. Available in: https://www.eclipse.org/acceleo//

#### Eclipse F (2020) Xbase. Available in: https://wiki.eclipse.org/Xbase

#### Eclipse F (2020) Xpand. Available in: https://www.eclipse.org/modeling/m2t/?project=xpand

#### Efftinge S, Eysholdt M, Ko ̈hnlein J., Zarnekow S, von Massow R, Hasselbring W, Hanus M (2012)
Xbase: Implementing Domain-specific Languages for Java.</br>
SIGPLAN Not. 48(3):112–121

#### Elaasar M, Noyrit F, Badreddin O, Ge ́rard Se ́bastien (2018)
Adaptation and Implementation of the ISO42010 Standard to Software Design and Modeling Tools.
In: International Conference on Model-Driven Engineering and Software Development, Springer, pp 236–258

#### Erdweg S, Van Der Storm T, Volter M, Boersma M, Bosman R, Cook WR, Gerritsen A, Hulshout A (2013)
The State of the Art in Language Workbenches.</br>
In: Software Language Engineering, (SLE’13).</br>
Springer International Publishing, Cham, pp 197–217

#### Erdweg S, Van Der Storm T, Vo ̈lter M, Tratt L, Bosman R, Cook WR, Gerritsen A, Hulshout A, Kelly S, Loh A, et al. (2015)
Evaluating and comparing language workbenches: Existing results and benchmarks for the future.</br>
Computer Languages, Systems & Structures 44:24–47

#### Essadi N, Anwar A (2018)
Towards A Language Interface Design to Coordinate Between Heterogeneous DSMLs.</br>
In: 2018 IEEE 5th International Congress on Information Science and Technology (CiSt), IEEE, pp 12–17

#### Falkner K, Chiprianov V, Falkner N, Szabo C, Puddy G (2013)
Modeling scenarios for the performance prediction of distributed real-time embedded systems.</br>
In: Military Communications and Information Systems Conference (MilCIS’13) 2013, pp 1–6

####  Forbrig P (2018)
Supporting Collaborative Decision Making in Software Engineering.</br>
In: In Proceedings of The 2018 Workshop on PhD Software Engineering Education: Challenges, Trends and Programs (SWEPHD2018)

#### Fowler M (2010)
Domain-specific languages, Pearson Education

#### Gamboa M, Syriani E (2019)
Improving user productivity in modeling tools by explicitly modeling workflows.</br>
Software & Systems Modeling 18(4):2441–2463

#### Garca-Daz V, Espada JP, Crespo RG, Pelayo G, Bustelo BC, Cueva Lovelle JM (2018)
An approach to improve the accuracy of probabilistic classifiers for decision support systems in sentiment analysis.</br>
Applied Soft Computing 67(C):822–833

#### Gargantini A, Radavelli M (2018)
Migrating combinatorial interaction test modeling and generation to the web.</br>
In: 2018 IEEE International Conference on Software Testing, Verification and Validation Workshops (ICSTW), IEEE, pp 308–317

#### Gargantini A, Vavassori P (2012)
CITLAB: A Laboratory for Combinatorial Interaction Testing.</br>
In: Proceedings of the 2012 IEEE Fifth International Conference on Software Testing, Verification and Validation,
(ICST’12). IEEE Computer Society, Washington, DC, USA, pp 559–568

#### Garmendia A, Guerra E, De Lara J, Garc ́ıa-Dom ́ınguez A, Kolovos D (2019)</br>
Scaling-up domain-specific modelling languages through modularity services.</br>
Information and Software Technology 115:97–118

#### Gavran I, Mailahn O, Muller R, Peifer R, Zufferey D (2018)
Tool: accessible automated reasoning for human robot collaboration.</br>
In: Proceedings of the 2018 ACM SIGPLAN International Symposium on New Ideas, New Paradigms, and Reflections on Programming and Software, ACM, pp 44–56

#### GEMOC, I (2017)
GEMOC Studio.</br>
Available in: http://gemoc.org/studio.html

#### Gibbs I, Dascalu S, Harris Jr. FC (2015)
A Separation-based UI Architecture with a DSL for Role Specialization.</br>
Journal of Systems and Software. 101(C):69–85

#### Go ́mez-Abajo P, Guerra E, de Lara J, Merayo MG (2018)
A tool for domain-independent model mutation.</br>
Science of Computer Programming 163:85–92

#### Gonnord L, Mosser S (2018)
Practicing domain-specific languages: from code to models.</br>
In: Proceedings of the 21st ACM/IEEE International Conference on Model Driven Engineering Languages and Systems:
Companion Proceedings, ACM, pp 106–113

#### Gossen F, Margaria T, Murtovi A, Naujokat S, Steffen B (2018)
DSLs for decision services: a tutorial introduction to language-driven engineering.</br>
In: International Symposium on Leveraging Applications of
Formal Methods, Springer, pp 546–564

#### Granchelli G, Cardarelli M, Francesco PD, Malavolta I, Iovino L, Salle AD (2017)
Towards recovering the software architecture of microservice-based systems.</br>
In: Proceedings - 2017 IEEE International Conference on Software Architecture Workshops, (ICSAW’17): Side Track Proceedings, pp 46–53

#### Guelfi N, Jahic ́ B, Ries B (2017)
TESMA: Requirements and design of a tool for educational programs.</br>
Information (Switzerland) 8(1):37

#### Haitzer T, Zdun U (2014)
Semi-automated architectural abstraction specifications for supporting software evolution.</br>
Science of Computer Programming 90(PART B):135–160

#### Hasan S, Dubey A, Chhokra A, Mahadevan N, Karsai G, Koutsoukos X (2017)
A modeling framework to integrate exogenous tools for identifying critical components in power systems,</br>
2017 Workshop on Modeling and Simulation of Cyber-Physical Energy Systems (MSCPES’17), pp 1–6

#### Ha ̈ser F, Felderer M, Breu R (2016)
An integrated tool environment for experimentation in domain specific language engineering.</br>
In: ACM International Conference Proceeding Series, (ICPS’16), vol 01-03-June

#### Ha ̈ser F, Felderer M, Breu R (2018)
Evaluation of an Integrated Tool Environment for Experimentation in DSL Engineering.</br>
In: International Conference on Software Quality, Springer, pp 147–168

#### Heinrich R, Strittmatter M, Reussner RH (2019)
A Layered Reference Architecture for Metamodels to Tailor Quality Modeling and Analysis.</br>
IEEE Transactions on Software Engineering

#### Heitko ̈tter H (2012)
A Framework for Creating Domain-specific Process Modeling Languages.</br>
Icsoft, pp 127–136

#### Henriques PR, Pereira MJV, Mernik M, Lenicˇ M., Avdicˇausˇevic ́ E, Zˇumer V (2002)
Automatic generation of language-based tools.</br>
Electronic notes in theoretical computer science 65(3):77–96

#### Herrera AS-B (2014)
Enhancing xtext for general purpose languages.</br>
In: CEUR Workshop Proceedings, vol 1321, Valencia, Spain

#### Hinkel G, Goldschmidt T, Burger E, Reussner R (2017)
Using internal domain-specific languages to inherit tool support and modularity for model transformations.</br>
Software and Systems Modeling, pp 1–27

#### Hiya S, Hisazumi K, Fukuda A, Nakanishi T (2013)
clooca: Web based tool for Domain Specific Modeling.</br>
In: Demos/Posters/StudentResearch@ ACM/IEEE 16th International Conference on Model Driven Engineering Languages and Systems (MoDELS’13), pp 31–35

#### Hoffmann B, Chalmers K, Urquhart N, Farrenkopf T, Guckert M (2018)
Towards Reducing Complexity of Multi-agent Simulations by Applying Model-Driven Techniques.</br>
In: International Conference on Practical Applications of Agents and Multi-Agent Systems, Springer, pp 187–199

#### Hoisl B, Sobernig S, Strembeck M (2017)
Reusable and generic design decisions for developing UML-based domain-specific languages.</br>
Information and Software Technology 92:49–74

#### Hojaji F, Zamani B, Hamou-Lhadj A, Mayerhofer T, Bousse E (2019)
Lossless compaction of model execution traces.</br>
Software & Systems Modeling, pp 1–32

#### HoseinDoost S, Adamzadeh T, Zamani B, Fatemi A (2017)
A model-driven framework for developing multiagent systems in emergency response environments.
Software and Systems Modeling, pp 1–28

#### Hoyos JR, Garcia-Molina J, Botia JA (2013)
A Domain-specific Language for Context Modeling in Context- aware Systems.</br>
Journal of Systems and Software 86(11):2890–2905

#### Huang C, Osaka A, Kamei Y, Ubayashi N (2015)
Automated DSL Construction Based on Software Product Lines.</br>
In: Proceedings of the 3rd International Conference on Model-Driven Engineering and Soft- ware Development, (MODELSWARD’15).
SCITEPRESS - Science and Technology Publications, Lda, Portugal, pp 247–254

#### Idani A, Ledru Y, Wakrime AA, Ayed RB, Bon P (2019)
Towards a tool-based domain specific approach for railway systems modeling and validation.</br>
In: International Conference on Reliability, Safety, and Security of Railway Systems, Springer, pp 23–40

#### Iglesias A, Iglesias-Urkia M, Lo ́pez-Davalillo B, Charramendieta S, Urbieta A (2019)
Trilateral: Software product line based multidomain iot artifact generation for industrial cps.</br>
In: Proceedings of the 7th International Conference on Model-Driven Engineering and Software Development, Modelsward

#### Iliasov A, Romanovsky A (2013)
SafeCap domain language for reasoning about safety and capacity.</br>
Proceedings - 2012 Workshop on Dependable Transportation Systems/Recent Advances in Software Dependability, (WDTS-RASD’12), pp 1–10

#### Iung A?B, Carbonell J, Marchezan L, Rodrigues E, Bernardino M, Basso F, Medeiros B (2020)
Systematic Mapping Study on Domain-Specific Language Development Tools - Data Repository, Zenodo.
https:// doi.org/10.5281/zenodo.3963379

#### Jacob F, Wynne A, Liu Y, Gray J (2014)
Domain-specific languages for developing and deploying signature discovery workflows.</br>
Computing in Science and Engineering 16(1):52–64

#### Jafer S, Chhaya B, Durak U (2017)
Graphical specification of flight scenarios with aviation scenario definition language (ASDL).</br>
In: AIAA Modeling and Simulation Technologies Conference, 2017 (AIAA SciTech’17)

#### Jager S, Maschotta R, Jungebloud T, Wichmann A, Zimmermann A (2016)
Creation of domain-specific languages for executable system models with the Eclipse Modeling Project.</br>
In: 10th Annual International Systems Conference, (SysCon’16) - Proceedings

#### JakumeitE,BuchwaldS,WagelaarD,DanL,HegedusA ́,HerrmannsdorferM,HornT,KalninaE,Krause C, Lano K, Lepper M, Rensink A, Rose L, Watzoldt S, Mazanek S (2014)
A survey and comparison of transformation tools based on the transformation tool contest.</br>
Science of Computer Programming 85, Part A(0):41 – 99.</br>
http://dx.doi.org/10.1016/j.scico.2013.10.009

#### JetBrains (2017)
MPS.</br>
Available in: https://www.jetbrains.com/mps/

#### Jeusfeld MA (2009)
Metamodel.</br>
In: Encyclopedia of Database Systems. Springer US, Boston, MA, pp 1727– 1730

#### Je ́ze ́quel J-M, Combemale B, Barais O, Monperrus M, Fouquet F (2015)
Mashup of Metalanguages and Its Implementation in the Kermeta Language Workbench.</br>
Software and Systems Modeling 14(2):905–920

#### Jinzhi L, To ̈rngren M, Chen De-Jiu, Wang J (2017)
A tool integration language to formalize co-simulation tool-chains for Cyber-Physical System (CPS).</br>
In: 1st Workshop on Formal Co-Simulation of Cyber- Physical Systems A satellite event of SEFM2017-15th International conference on Software Engineering and Formal Methods, Springer

#### Johnson RE (1997)
frameworks = (Components + Patterns).</br>
Communications of ACM 40(10):39–42

#### Jr A, Benedito F, Coutinho L, Silva F, Roriz M, Endler M (2019)
A mobility restriction authoring tool approach based on a domain specific modeling language and model transformation.</br>
In: International Conference on Enterprise Information Systems (ICEIS), pp 525–534

#### Jrad AB, Bhiri S, Tata S (2019)
STRATFram: A framework for describing and evaluating elasticity strategies for service-based business processes in the cloud.</br>
Future Generation Computer Systems 97:69–89

#### Kahani N (2018)
AutoModel: A Domain-Specific Language for Automatic Modeling of Real-Time Embedded Systems.</br>
In: 2018 IEEE/ACM 40th International Conference on Software Engineering: Companion (ICSE-Companion), pp 515–517

#### Kalnins A, Barzdins J (2019)
Metamodel specialization for graphical language support.</br>
Software & Systems Modeling 18(3):1699–1735

#### Kanav S (2018)
A modular approach to integrate verification tools in model based development.</br>
In: Pro- ceedings of the 21st ACM/IEEE International Conference on Model Driven Engineering Languages and Systems: Companion Proceedings, ACM, pp 150–155

#### Karol S, Nett T, Castrillon J, Sbalzarini IF (2018)
A Domain-Specific Language and Editor for Parallel Particle Methods.</br>
ACM Trans. Math. Softw. 44:34:1–34:32.</br>
https://doi.org/10.1145/3175659

#### Kelly S, Lyytinen K, Rossi M (1996)
MetaEdit+: A Fully Configurable Multi-User and Multi-Tool CASE and CAME Environment.</br>
In: 8th International Conference on Advances Information System Engineering (CAiSE’96). Springer-Verlag, London, UK, pp 1–21

#### Kern H (2016)
Model Interoperability between Meta-Modeling Environments by using M3-Level-Based Bridges.</br>
Ph.D. Thesis, Universita ̈t Leipzig

#### Kitchenham BA, Budgen D, Brereton OP (2010)
The value of mapping studies-A participant-observer case study.</br>
In: (EASE’10), vol 10, pp 25–33

#### Kitchenham BA, Budgen D, Brereton OP (2011)
Using mapping studies as the basis for further research–a participant-observer case study.
Information and Software Technology 53(6):638–651

#### Korenkov Y, Loginov I, Lazdin A (2015)
PEG-based language workbench.</br>
In: Conference of Open Innovation Association, (FRUCT’15), vol 2015-June, Yaroslavl, Russia, pp 75–81

#### Kosar T, Bohra S, Mernik M (2016)
Domain-specific languages: A systematic mapping study.</br>
Information and Software Technology 71:77–91

#### Koschke R, Schmidt U, Berger B (2018)
\[engineering paper\] built-in clone detection in meta languages.</br>
In: 2018 IEEE 18th International Working Conference on Source Code Analysis and Manipulation (SCAM), pp 165–170

#### Koster N, Wrede S, Cimiano P (2018)
A Model Driven Approach for Eased Knowledge Storage and Retrieval in Interactive HRI Systems.
In: 2018 Second IEEE International Conference on Robotic Computing (IRC), 2018, pp 113–120

#### Ko ̈vesda ́n G, Lengyel L (2019)
Meta3: a code generator framework for domain-specific languages.</br>
Software & Systems Modeling 18(4):2421–2439

#### Kowalski M, Magott J (2012)
Time coordination of heterogeneous distance protections using a domain specific language.</br>
E-Informatica Software Engineering Journal 6(1):7–26

#### Krasts O, Kleins A, Teilans A (2012)
Domain specific language for securities settlement systems.</br>
In: Digital Information Processing and Communications (ICDIPC’12), Second International Conference on, pp 80–83

#### Kru ̈ger S, Spa ̈th J, Ali K, Bodden E, Mezini M (2018)
CrySL: An Extensible Approach to Validating the Correct Usage of Cryptographic APIs.</br>
In: 32nd European Conference on Object-Oriented Programming
(ECOOP 2018), Schloss Dagstuhl-Leibniz-Zentrum fuer Informatik

#### Landis JR, Koch GG (1977)
The measurement of observer agreement for categorical data.</br> biometrics, pp 159–174

#### Le DM, Dang D-H, Nguyen V-H (2018)
On Domain Driven Design Using Annotation-Based Domain Specific Language.</br>
Computer Languages, Systems & Structures

#### Le G. O, Waltham S (2013)
Yet Another DSL for Cross-platforms Mobile Development.</br>
In: Proceedings of the First Workshop on the Globalization of Domain Specific Languages, (GlobalDSL’13).</br>
ACM, New York, NY, USA, pp 28–33

#### Lelandais B, Oudot M-P, Combemale B (2018)
Fostering Metamodels and Grammars Within a Dedicated Environment for HPC: The NabLab Environment (Tool Demo).</br>
In: Proceedings of the 11th ACM SIG- PLAN International Conference on Software Language Engineering, SLE 2018.</br>
ACM, New York, NY, USA, pp 200–204

#### Lemazurier L, Chapurlat V, Grosseteˆte A (2017)
An MBSE Approach to Pass from Requirements to Functional Architecture.</br>
IFAC-PapersOnLine 50(1):7260–7265

#### Li X-S, Tao X-P, Song W, Dong K (2018)
AocML: A Domain-Specific Language for Model-Driven Development of Activity-Oriented Context-Aware Applications.</br>
Journal of Computer Science and Technology 33(5):900–917

#### Liebel G, Marko N, Tichy M, Leitner A, Hansson Jo ̈rgen (2014)
Assessing the state-of-practice of model-based engineering in the embedded systems domain.</br>
In: Model-Driven Engineering Languages and Systems, MODELS’14, pp 166–182

#### Lo ́pez-Ferna ́ndez JJ, Garmendia A, Guerra E, de Lara J (2019)
An example is worth a thousand words: Creating graphical modelling environments by example.</br>
Software & Systems Modeling 18(2):961–993

#### Ma T, Sallai J (2017)
MiW: A domain specific modeling environment for complex molecular systems.</br>
Procedia Computer Science 108:1232–1241

#### Mac ́ıas F, Wolter U, Rutle A, Dura ́n F, Rodriguez-Echeverria R (2019)
Multilevel coupled model transformations for precise and reusable definition of model behaviour.</br>
Journal of Logical and Algebraic Methods in Programming 106:167–195

#### Makedonski P, Adamis G, Ka ̈a ̈rik M, Kristoffersen F, Carignani M, Ulrich A, Grabowski J (2019)
Test descriptions with ETSI TDL.</br>
Software Quality Journal, pp 1–33

#### Marchezan L, Bolfe G, Rodrigues E, Bernardino M, Basso FP (2019)
Thoth: A Web-based Tool to Support Systematic Reviews.</br>
In: 2019 ACM/IEEE International Symposium on Empirical Software Engineering and Measurement (ESEM), pp 1–6

#### Maro S, Stegho ̈fer J-P, Anjorin A, Tichy M, Gelin L (2015)
On Integrating Graphical and Textual Editors for a UML Profile Based Domain Specific Language: An Industrial Experience.</br>
In: Proceedings of the 2015 ACM SIGPLAN International Conference on Software Language Engineering, (SLE’15).</br>
ACM, New York, NY, USA, pp 1–12

#### Maro ́ti M, Kecske ́s T, Kereske ́nyi R, Broll B, Vo ̈lgyesi P, Jura ́cz L, Levendoszky T, Le ́deczi A. (2014)
Next generation (Meta)modeling: Web- and cloud-based collaborative tool infrastructure.</br>
CEUR Workshop Proceedings 1237:41–60

#### Maschotta R, Wichmann A, Zimmermann A, Gruber K (2019)
Integrated Automotive Requirements Engineering with a SysML-Based Domain-Specific Language.</br>
In: 2019 IEEE International Conference on Mechatronics (ICM), vol 1, IEEE, pp 402–409

#### Mavridou A, Kecskes T, Zhang Q, Sztipanovits J (2018)
A common integrated framework for heterogeneous modeling services.</br>
CEUR Workshop Proceedings 2245:416–422

#### Mavropoulos O, Mouratidis H, Fish A, Panaousis E (2017)
ASTo: A tool for security analysis of IoT systems.</br>
In: Proceedings - 2017 15th IEEE/ACIS International Conference on Software Engineering Research, Management and Applications, (SERA’17), pp 395–400

#### Mayr-Dorn C, Laaber C (2017)
A Domain-Specific Language for Coordinating Collaboration.</br>
2017 43rd Euromicro Conference on Software Engineering and Advanced Applications (SEAA’17), pp 57–60

#### Me ́ndez-Acuna D, Galindo J, Degueule T, Combemale B, Baudry B (2016)
Leveraging software product lines engineering in the development of external DSLs: a systematic literature review.</br>
Computer Languages, Systems & Structures 46:206–235

#### Mendivelso LF, Garce ́s K, Casallas R (2018)
Metric-centered and technology-independent architectural views for software comprehension.</br>
Journal of Software Engineering Research and Development 6(1):16

#### Merino MV, Vinju J, van der Storm T (2018)
Bacata ́: a language parametric notebook generator (tool demo).</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language
Engineering, ACM, pp 210–214

#### Mernik M, Heering J, Sloane AM (2005)
When and how to develop domain-specific languages.</br>
ACM computing surveys (CSUR) 37(4):316–344

#### MetaCase (2017)
MetaEdit+.</br>
Available in: http://www.metacase.com/mep/

#### Mettouris C, Achilleos A, Kapitsaki G, Papadopoulos GA (2018)
The UbiCARS Model-Driven Framework: Automating Development of Recommender Systems for Commerce.</br>
In: European Conference on Ambient Intelligence, Springer, pp 37–53

#### Mezhuyev V, Al-Emran M, Fatehah M, Hong NgChin (2018)
Factors affecting the Metamodelling Acceptance: A Case Study from Software Development Companies in Malaysia.</br>
IEEE Access 6:49476– 49485

#### Mohamad RP, Kolovos DS, Paige RF (2015)
Resource requirement analysis for web applications running in a virtualised environment.</br>
Proceedings of the International Conference on Cloud Computing Technology
and Science, (CLOUDCOM’15) 2015-Febru(February):632–637

#### Molina AI, Gallardo J, Redondo MA, Ortega M, Giraldo WJ (2013)
Metamodel-driven Definition of a Visual Modeling Language for Specifying Interactive Groupware Applications: An Empirical Study.</br>
Journal of Systems and Software 86(7):1772–1789

#### Molina PJ (2019)
Quid: prototyping web components on the web.</br>
In: Proceedings of the ACM SIGCHI Symposium on Engineering Interactive Computing Systems, ACM, p 3

#### Montenegro-Marin CE, Cueva-Lovelle JM, Sanjua ́n-Mart ́ınez O, Garc ́ıa-D ́ıaz V (2012)
Domain specific language for the generation of learning management systems modules.</br>
Journal of Web Engineering 11(1):23

#### Monthe VM, Nana L, Kouamou GE, Tangha C (2016)
RsaML: A domain specific modeling language for describing robotic software architectures with integration of real-time properties.</br>
In: CEUR Workshop Proceedings, vol 1697

#### Montrieux L, Yu Y, Wermelinger M (2013)
Developing a domain-specific plug-in for a modelling platform: The good, the bad, the ugly.</br>
In: 2013 3rd International Workshop on developing Tools as Plug-ins (TOPI’13), pp 1–6

#### Morgan R, Grossmann G, Schrefl M, Stumptner M, Payne T (2018)
VizDSL: a visual DSL for interactive information visualization.</br>
In: International Conference on Advanced Information Systems Engineering, Springer, pp 440–455

#### Mosteller D, Haustermann M, Moldt D, Schmitz D (2018)
Graphical Simulation Feedback in Petri Net- based Domain-Specific Languages within a Meta-Modeling Environment.</br>
In: PNSE@ Petri Nets/ACSD, pp 57–76

#### Mussbacher G, Amyot D, Breu R, Bruel J-M, Cheng BHC, Collet P, Combemale B, France RB, Hel- dal R, Hill J, Kienzle J, Scho ̈ttle M, Steimann F, Stikkolorum D, Whittle J (2014)
The relevance of model-driven engineering thirty years from now.</br>
In: Model-Driven Engineering Languages and Systems, pp 183–200

#### Nagele T, Hooman J (2017)
Rapid Construction of Co-Simulations of Cyber-Physical Systems in HLA Using a DSL.</br>
2017 43rd Euromicro Conference on Software Engineering and Advanced Applications (SEAA’17), pp 247–251

#### Nakamura H, Nagano R, Hisazumi K, Kamei Y, Ubayashi N, Fukuda A (2012)
QORAL: An External Domain-Specific Language for Mining Software Repositories.</br>
In: Proceedings of the 2012 Fourth Inter- national Workshop on Empirical Software Engineering in Practice, (IWESEP’12).</br>
IEEE Computer Society, Washington, DC, USA, pp 23–29

#### Naujokat S, Lybecait M, Kopetzki D, Steffen B (2017)
CINCO: A simplicity-driven approach to full generation of domain-specific graphical modeling tools.</br>
International Journal on Software Tools for Technology Transfer, pp 1–28

#### Nazir A, Alam M, Malik SUR, Akhunzada A, Cheema MN, Khan MK, Ziang Y, Khan T, Khan A (2017)
A high-level domain-specific language for SIEM (design, development and formal verification).</br>
Cluster Computing 20(3):2423–2437

#### Neubauer P, Bill R, Mayerhofer T, Wimmer M (2017)
Automated generation of consistency-achieving model editors.</br>
In: 24th International Conference on Software Analysis, Evolution and Reengineering (SANER’17), 2017 IEEE, IEEE, pp 127–137

#### Nordmann A, Wrede S, Steil J (2015)
Modeling of movement control architectures based on motion prim- itives using domain-specific languages.</br>
In: Proceedings - IEEE International Conference on Robotics and Automation (ICRA’15), vol 2015-June, Seattle, WA, United states, pp 5032–5039

#### Ober I, Palyart M, Bruel J-M, Lugato D (2018)
On the use of models for high-performance scientific computing applications: an experience report.</br>
Software & Systems Modeling 17(1):319–342

#### Oliveira B, Belo O (2017)
On the specification of extract, transform, and load patterns behavior: A domain- specific language approach.</br>
Expert Systems, 34(1)

#### OMG (2019)
META-OBJECT FACILITY.
Available in: https://www.omg.org/spec/MOF/2.4.1/PDF

#### Ouared A, Ouhammou Y, Bellatreche L (2018)
QoSMOS: QoS metrics management tool suite. Computer Languages, Systems & Structures 54:236–251

#### Pasternak M, Kahani N, Bagherzadeh M, Dingel J, Cordy JR (2018)
Simgen: a tool for generating simulations and visualizations of embedded systems on the unity game engine.</br>
In: Proceedings of the 21st ACM/IEEE International Conference on Model Driven Engineering Languages and Systems: Companion Proceedings, ACM, pp 42–46

#### Pescador A, Garmendia A, Guerra E, Sanchez Cuadrado JS, De Lara J (2015)
Pattern-based develop- ment of Domain-Specific Modelling Languages.</br>
In: 2015 ACM/IEEE 18th International Conference on Model Driven Engineering Languages and Systems, (MODELS’15) - Proceedings, Ottawa, ON, Canada, pp 166–175

#### Pescador A, Lara JD (2016)
DSL-Maps: From Requirements to Design of Domain-Specific Languages.</br>
31st IEEE/ACM International Conference on Automated Software Engineering (ASE’16), pp 438–443

#### Petersen K, Feldt R, Mujtaba S, Mattsson M (2008)
Systematic Mapping Studies in Software Engineering.</br>
12th Int. Conf. on Evaluation and Assessment in Software Engineering 17(1):1–10

#### Pietrusewicz K (2019)
Metamodelling for Design of Mechatronic and Cyber-Physical Systems.</br>
Applied Sciences 9(3):376

#### Pomante L, Candia S, Incerto E (2015)
A Model-Driven approach for the development of an IDE for Spacecraft on-board software.</br>
In: 2015 IEEE Aerospace Conference, pp 1–17

#### Pe ́rez F, Valderas P, Fons J (2013)
A domain-specific language for enabling doctors to specify biomechanical protocols.</br>
In: 2013 IEEE Symposium on Visual Languages and Human Centric Computing, (IEEE-VL/HCC’13), pp 99–102

#### Pe ́rez-Berenguer D, Garc ́ıa-Molina J (2019)
INDIeAuthor: A Metamodel-Based Textual Language for Authoring Educational Courses.</br>
IEEE Access 7:51396–51416.</br>
https://doi.org/10.1109/ACCESS.2019.2911884

#### Pro ̈ll R., Rumpold A, Bauer B (2018)
Applying integrated domain-specific modeling for multi-concerns development of complex systems.</br>
In: Pires LF, Hammoudi S, Selic B (eds) Model-Driven Engineering and Software Development.</br>
Springer International Publishing, Cham, pp 247–271

#### Rabiser R, Thanhofer-Pilisch J, Vierhauser M, Gru ̈nbacher P, Egyed A (2018)
Developing and evolving a DSL-based approach for runtime monitoring of systems of systems.</br>
Automated Software Engineering 25(4):875–915

#### Rapos EJ, Stephan M (2019)
IML: Towards an Instructional Modeling Language.</br>
In: Proceedings of the 7th International Conference on Model-Driven Engineering and Software Development, SCITEPRESS- Science and Technology Publications, Lda, pp 417–425

#### Ratiu D, Ulrich A (2017)
Increasing usability of spin-based C code verification using a harness defini- tion language: Leveraging model-driven code checking to practitioners.</br>
In: SPIN 2017 - Proceedings of the 24th ACM SIGSOFT International SPIN Symposium on Model Checking of Software (SPIN’17), pp 60–69

#### Ratiu D, Voelter M (2016)
Automated Testing of DSL Implementations - Experiences from Building mbeddr.</br>
In: 2016 IEEE/ACM 11th International Workshop in Automation of Software Test (AST’16), pp 15–21

#### Ratiu D, Ulrich A (2019)
An integrated environment for Spin-based C code checking.</br>
International Journal on Software Tools for Technology Transfer 21(3):267–286

#### Rein P, Hirschfeld R, Taeumel M (2016)
Gramada: Immediacy in programming language development.</br>
In: Onward! 2016 - Proceedings of the 2016 ACM International Symposium on New Ideas, New Paradigms,
and Reflections on Programming and Software (SPLASH’16), pp 165–179

#### Rensink A, Aksit M (2018)
A Java Bytecode Metamodel for Composable Program Analyses.</br>
In: Software Technologies: Applications and Foundations: STAF 2017 Collocated Workshops, Marburg, Germany,
July 17-21, 2017, Revised Selected Papers, vol 10748, Springer, p 30

#### Ribeiro A, Da S AR (2014)
XIS-Mobile: A DSL for mobile applications.</br>
In: Proceedings of the ACM Symposium on Applied Computing, (SIGAPP’14), pp 1316–1323

#### Ribeiro A, De S. L, Da S. AR (2016)
Comparative analysis of workbenches to support DSMLs: Discussion with non-trivial model-driven development needs.</br>
In: Proceedings of the 4th International Conference on Model-Driven Engineering and Software Development (MODELSWARD’16), pp 323–330

#### Ribeiro A, da Silva AR (2017)
RSLingo4Privacy Studio - A Tool to Improve the Specification and Analysis of Privacy Policies.</br>
Proceedings of the 19th International Conference on Enterprise Information Systems (ICEIS’17), pp 52–63

#### Ribic ́ S, Turcˇinhodzˇic ́ R, Muratovic ́-Ribic ́ A, Kosar T (2018)
REDOSPLAT: A readable domain-specific language for timetabling requirements definition.</br>
Computer Languages, Systems & Structures 54:252– 272

#### Rieger C, Kuchen H (2018)
A process-oriented modeling approach for graphical development of mobile business apps.</br>
Computer Languages, Systems & Structures 53:43–58.</br>
https://doi.org/10.1016/j.cl.2018.01.001

#### Ries B, Capozucca A, Guelfi N (2018)
Messir: a text-first DSL-based approach for UML requirements engi- neering (tool demo).</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language Engineering, ACM, pp 103–107

#### Ristic ́ S (2017)
How to apply model-driven paradigm in information system (Re) engineering.
In: 2017 IEEE 14th International Scientific Conference on Informatics, IEEE, pp 6–11

#### Rocha H, Durelli RS, Terra R, Bessa S, Valente MT (2017)
DCL 2.0: modular and reusable specification of architectural constraints.</br>
Journal of the Brazilian Computer Society, 23(1)

#### Rodriguez-Echeverria R, Izquierdo JLC, Wimmer M, Cabot J (2018)
Towards a Language Server Protocol Infrastructure for Graphical Modeling.</br>
In: Proceedings of the 21th ACM/IEEE International Conference on Model Driven Engineering Languages and Systems, ACM, pp 370–380

#### Rose LM, Kolovos DS, Paige RF (2012)
EuGENia Live: A Flexible Graphical Modelling Tool.</br>
In: Proceedings of the 2012 Extreme Modeling Workshop (XM’2012), XM’12. ACM, New York, NY, USA, pp 15–20

#### Ruiz-Rube I, Person T, Dodero JM, Mota JM, Sa ́nchez-Jara JM (2019)
Applying static code analysis for domain-specific languages.</br>
Software & Systems Modeling, pp 1–16

#### Salehi P, Hamou-Lhadj A, Toeroe M, Khendek F (2018)
A model-driven approach for the generation of configurations for highly available software systems.</br>
Innovations in Systems and Software Engineering 14(4):273–307

#### Sandobalin J, Insfran E, Abrahao S (2017)
An Infrastructure Modelling Tool for Cloud Provisioning.</br>
Proceedings - 2017 IEEE 14th International Conference on Services Computing (SCC’17), pp 354–361

#### Santos AL, Gomes E (2016)
Xdiagram: A declarative textual DSL for describing diagram editors (Tool Demo -).</br>
In: Proceedings of the 2016 ACM SIGPLAN International Conference on Software Language
Engineering, co-located with SPLASH 2016 (SLE’16), (SLE’16), pp 253–257

#### Savic ́ D, Da S AR, Vlajic ́ S, Lazarevic ́ S, Antovic ́ I, Stanojevic ́ V, Milic ́ M (2014)
Preliminary experi- ence using JetBrains MPS to implement a requirements specification language.</br>
Proceedings - 2014 9th International Conference on the Quality of Information and Communications Technology, (QUATIC’14) 1:134–137

#### Schmidt DC (2006)
Model-driven engineering.</br>
COMPUTER-IEEE COMPUTER SOCIETY- 39(2):25

#### Schuts M, Hooman J, Tielemans P (2018)
Industrial Experience with the Migration of Legacy Models Using a DSL.</br>
In: Proceedings of the Real World Domain Specific Languages Workshop 2018. RWDSL2018, 2018, pp 1:1–1:10

#### Schwaiger WSA (2016)
REA Business Management Ontology: Conceptual Modeling of Accounting, Finance and Management Control.</br>
In: CAiSE Forum

#### Selgrad K, Lier A, Do ̈rntlein J., Reiche O, Marc Stamminger M (2016)
A High-Performance Image Pro- cessing DSL for Heterogeneous Architectures.</br>
In: Proceedings of the 9th European Lisp Symposium on European Lisp Symposium, (ELS’16).</br>
European Lisp Scientific Activities Association, pp 5:38–5:37

#### Semera ́th Oszka ́r, Varro ́ Da ́niel (2018)
Iterative Generation of Diverse Models for Testing Specifications of DSL Tools.</br>
In: FASE, pp 227–245

#### Smits J, Visser E (2017)
FlowSpec: Declarative Dataflow Analysis Specification.</br>
In: Proceedings of the 10th ACM SIGPLAN International Conference on Software Language Engineering, (SLE’17). ACM, New York, NY, USA, pp 221–231

#### Sorgalla J, Wizenty P, Rademacher F, Sachweh S, Zu ̈ndorf A. (2018)
AjiL: enabling model-driven microservice development.</br>
In: Proceedings of the 12th European Conference on Software Architecture: Companion Proceedings, ACM, p 1

#### Stevanetic S, Zdun U (2018)
Supporting the analyzability of architectural component models-empirical findings and tool support.</br>
Empirical Software Engineering 23(6):3578–3625

#### Stocker K. AB, Washizaki HB, Fukazawa YB (2017)
Closing the Gap between Unit Test Code and Doc- umentation.</br>
Proceedings - 10th IEEE International Conference on Software Testing, Verification and Validation Workshops (ICSTW’17), pp 304–308

#### Stro ̈mba ̈ck F. (2018)
Storm: A Language Platform for Interacting and Extensible Languages (Tool Demo).</br>
In: Proceedings of the 11th ACM SIGPLAN International Conference on Software Language Engineering. SLE 2018, 2018, pp 60–64

#### Sutii AM, van den Brand M, Verhoeff T (2017)
Exploration of modularity and reusability of domain-specific languages: An expression DSL in MetaMod.</br>
Computer Languages, Systems and Structures

#### Szabo T, Alperovich S, Voelter M, Erdweg S (2016)
An extensible framework for variable-precision data- flow analyses in MPS.</br>
In: Proceedings of the 31st IEEE/ACM International Conference on Automated Software Engineering, (ASE’16), Singapore, Singapore, pp 870–875

#### Tairas R, Cabot J (2015)
Corpus-based Analysis of Domain-specific Languages.</br>
Software and Systems Modeling 14(2):889–904

#### Tariq MU, Florence J, Wolf M (2014)
Design specification of cyber-physical systems: Towards a domain- specific modeling language based on simulink, eclipse modeling framework, and giotto.</br>
In: CEUR Workshop Proceedings, vol 1250, pp 6–15

#### Tekinerdogan B, Arkin E (2019)
ParDSL: a domain-specific language framework for supporting deployment of parallel algorithms.</br>
Software & Systems Modeling 18(5):2907–2935

#### Terzic ́ B., Dimitrieski V, Kordic ́ S., Milosavljevic ́ G., Lukovic ́ I. (2018)
Development and evaluation of MicroBuilder: a Model-Driven tool for the specification of REST Microservice Software Architectures.</br>
Enterprise Information Systems 12(8-9):1034–1057

#### Tezel B, Challenger M, Kardas G (2018)
Dsml4bdi: A modeling tool for bdi agent development.</br>
In: 12th turkish national software engineering symposium (uyms 2018)

#### Theobald M, Palladino L, Virelizier P (2018)
About DSML design based on standard and open-source - REX from SAFRAN tech work using Papyrus-SysML.</br>
International Journal of Electrical and Electronic Engineering and Telecommunications 7:70–75.</br>
https://doi.org/10.18178/ijeetc.7.2.70-75

#### Tikhonova U (2017)
Reusable specification templates for defining dynamic semantics of DSLs.</br>
Software and Systems Modeling, pp 1–30

#### Tisi M, Cheng Z (2018)
CoqTL: an Internal DSL for Model Transformation in Coq.</br>
In: International Conference on Theory and Practice of Model Transformations, Springer, pp 142–156

#### Tolvanen J-P, Kelly S (2018)
Effort Used to Create Domain-Specific Modeling Languages.</br>
In: Proceedings of the 21th ACM/IEEE International Conference on Model Driven Engineering Languages and Systems, ACM, pp 235–244

#### Tragatschnig S, Stevanetic S, Zdun U (2018)
Supporting the evolution of event-driven service-oriented architectures using change patterns.</br>
Information and Software Technology 100:133–146

#### Tran N-H, Chiba Y, Aoki T (2017)
Domain-specific language facilitates scheduling in model checking.</br>
In: 2017 24th Asia-Pacific Software Engineering Conference (APSEC), IEEE, pp 417–426

#### Trobo IP, D ́ıaz VG, Espada JP, Crespo RG, Moreno-Ger P (2019)
Rapid modeling of human-defined AI behavior patterns in games.</br>
Journal of Ambient Intelligence and Humanized Computing 10(7):2683– 2692

#### Uhna ́k P, Pergl R (2016)
The OpenPonk Modeling Platform.</br>
In: Proceedings of the 11th Edition of the Inter- national Workshop on Smalltalk Technologies, (IWST’16).</br>
ACM, New York, NY, USA, pp 14:1–14:11

#### Vaderna R, Vukovic ́ Zˇ , Dejanovic ́ I, Milosavljevic ́ G (2018)
Graph Drawing and Analysis Library and Its Domain-Specific Language for Graphs’ Layout Specifications.</br>
Scientific Programming, pp 2018

#### van den Berg F, Hooman J, Haverkort BR (2018)
A Domain-Specific Language and Toolchain for Perfor- mance Evaluation Based on Measurements.</br>
Lecture Notes in Computer Science (including subseries Lecture Notes in Artificial Intelligence and Lecture Notes in Bioinformatics) 10740 LNCS:295–301.</br>
https://doi.org/10.1007/978-3-319-74947-1 21

#### van den Berg F, Garousi V, Tekinerdogan B, Haverkort BR (2018)
Designing cyber-physical systems with aDSL: A domain-specific language and tool support.</br>
In: 2018 13th Annual Conference on System of Systems Engineering (SoSE), IEEE, pp 225–232

#### van Rozen R, van der Storm T (2017)
Toward live domain-specific languages: From text differencing to adapting models at run time.</br>
Software & Systems Modeling 18:1–17

#### van Rozen R, van der Storm T (2019)
Toward live domain-specific languages.</br>
Software & Systems Modeling 18(1):195–212

#### Viana M, Penteado R, Do P. A, Durelli R (2013)
F3T: From features to frameworks tool.</br>
In: Proceedings - 2013 27th Brazilian Symposium on Software Engineering, (SBES’13), Brasilia, DF, Brazil, pp 89–98

#### Viana MC, Penteado RAD, do Prado AF (2013)
Domain-Specific Modeling Languages to improve framework instantiation.</br>
Journal of Systems and Software 86(12):3123–3139

#### Vieira MA, Carvalho ST (2017)
Model-driven Engineering in the Development of Ubiquitous Applications: Technologies, Tools and Languages.</br>
In: Proceedings of the 23rd Brazillian Symposium on Multimedia and the Web, (WebMedia’17).</br>
ACM, New York, NY, USA, pp 29–32

#### Vinogradov S, Ozhigin A, Ratiu D (2015)
Modern model-based development approach for embedded sys- tems practical experience.</br>
In: 1st IEEE International Symposium on Systems Engineering, (ISSE’15) - Proceedings, Rome, Italy, pp 56–59

#### Visic N, Fill H-G, Buchmann RA, Karagiannis D (2015)
A domain-specific language for modeling method definition: From requirements to grammar.</br>
In: Proceedings - International Conference on Research Challenges in Information Science, (RCIS’15), vol 2015-June, Athens, Greece, pp 286–297

#### Vissers Y, Mengerink JGM, Schiffelers RRH, Serebrenik A, Reniers MA (2017)
Maintenance of specifica- tion models in industry using Edapt.</br>
In: Specification and Design Languages (FDL), 2017 Forum on, IEEE, pp 1–6

#### Vistbakka I, Barash M, Troubitsyna E (2018)
Towards creating a DSL facilitating modelling of dynamic access control in Event-B.</br>
In: International Conference on Abstract State Machines, Alloy, B, TLA, VDM, and Z, Springer, pp 386–391

#### Viyovic ́ V, Maksimovic ́ M, Perisˇic ́ B (2014)
Sirius: A rapid development of DSM graphical editor.</br>
IEEE 18th International Conference on Intelligent Engineering Systems, Proceedings (INES’14), pp 233–238

#### Voelter M, Kolb B, Szabo ́ T, Ratiu D, van Deursen A (2017)
Lessons learned from developing mbeddr: a case study in language engineering with MPS.</br>
Software and Systems Modeling, pp 1–46

#### Voelter M (2018)
Fusing modeling and programming into language-oriented programming.</br>
In: International Symposium on Leveraging Applications of Formal Methods, Springer, pp 309–339

#### Voelter M, Kolb B, Birken K, Tomassetti F, Alff P, Wiart L, Wortmann A, Nordmann A (2019)
Using lan- guage workbenches and domain-specific languages for safety-critical software development.</br>
Software & Systems Modeling 18(4):2507–2530

#### Voelter M, Ratiu D, Schaetz B, Kolb B (2012)
Mbeddr: An extensible c-based programming language and IDE for embedded systems.</br>
In: Proceedings of the 2012 ACM Conference on Systems, Programming, and Applications: Software for Humanity (SPLASH’12), Tucson, AZ, United states, pp 121–140

#### Vo ̈gele C, van Hoorn A, Schulz E, Hasselbring W, Krcmar H (2018)
WESSBAS: extraction of probabilis- tic workload specifications for load testing and performance predictions a model-driven approach for
session-based application systems.</br>
Software & Systems Modeling 17(2):443–477

#### Wachsmuth GH, Konat GDP, Visser E (2014)
Language Design with the Spoofax Language Workbench.</br>
IEEE Software 31(5):35–43

#### Walter T, Parreiras FS, Staab S (2014)
An ontology-based framework for domain-specific modeling.</br>
Software & Systems Modeling 13(1):83–108

#### Whittle J, Hutchinson J, Rouncefield M, Ha ̊kan B, Rogardt H (2015)
A taxonomy of tool-related issues affecting the adoption of model-driven engineering.</br>
Software & Systems Modeling, pp 1–19

#### Wienke J, Wigand D, Koster N, Wrede S (2018)
Model-based performance testing for robotics software components.</br>
In: 2018 Second IEEE International Conference on Robotic Computing (IRC), IEEE, pp 25–32

#### Wigand DL, Nordmann A, Goerlich M, Wrede S (2017)
Modularization of domain-specific languages for extensible component-based robotic systems.</br>
Proceedings - 2017 1st IEEE International Conference on Robotic Computing, (IRC’17), pp 164–171

#### Wohlin C, Runeson P, Ho ̈st M, Ohlsson M, Regnell B (2012)
Experimentation in Software Engineering, Springer

#### Wu H, Gray J, Mernik M (2008)
Grammar-driven generation of domain-specific language debuggers.</br>
Software: Practice and Experience 38(10):1073–1103

#### Yakymets N, Sango M, Dhouib S, Gelin R (2018)
Model-Based Engineering, Safety Analysis and Risk Assessment for Personal Care Robots.</br>
In: 2018 IEEE/RSJ International Conference on Intelligent Robots and Systems (IROS), IEEE, pp 6136–6141

#### Yigitbas E, Anjorin A, Leblebici E, Grieger M (2018)
Bidirectional Method Patterns for Language Editor Migration.</br>
In: European Conference on Modelling Foundations and Applications, Springer, pp 97–114

#### Zabawa P, Hnatkowska B (2017)
CDMM-F–domain languages framework

#### Zarrin B, Baumeister H (2014)
Design of a Domain-Specific Language for Material Flow Analysis Using Microsoft DSL Tools: An Experience Paper.</br>
In: Proceedings of the 14th Workshop on Domain-Specific Modeling, (DSM’14).</br>
ACM, New York, NY, USA, pp 23–28

#### Zarrin B, Baumeister H (2018)
An integrated framework to specify domain-specific modeling languages.</br>
In: 6th International Conference on Model-Driven Engineering and Software Development, pp 83–94

#### Zarrin B, Baumeister H, Sarjoughian H (2018)
An integrated framework to develop domain-specific languages: Extended case study.</br>
In: International Conference on Model-Driven Engineering and Software Development, Springer, pp 159–184

#### Zhao T, Huang X (2018)
Design and implementation of DeepDSL: A DSL for deep learning.</br>
Computer Languages, Systems & Structures 54:39–70

#### Zhou N, Li D, Li S, Wang S, Liu C (2017)
Model-Based Development of Knowledge-Driven Self-Reconfigurable Machine Control Systems.</br>
IEEE Access 5:19909–19919

#### Zhu M, Wang AI (2017)
RAIL: A Domain-Specific Language for Generating NPC Behaviors in Action/Adventure Game.</br>
In: International Conference on Advances in Computer Entertainment, Springer, pp 868–881

#### Zhu Z, Lei Y, Zhu Y, Sarjoughian H (2017)
Cognitive Behaviors Modeling Using UML Profile: Design and Experience.</br>
IEEE Access 5:21694–21708

#### Zikra I (2012)
Implementing the unifying meta-model for enterprise modeling and model-driven development: An experience report.</br>
In: Sandkuhl K, Seigerroth U, Stirna J (eds) The Practice of Enterprise Modeling.</br>
Springer Berlin Heidelberg, Berlin, Heidelberg, pp 172–187

#### Zweihoff P, Naujokat S, Steffen B (2019)
Pyro: Generating Domain-Specific Collaborative Online Modeling Environments.</br>
In: International Conference on Fundamental Approaches to Software Engineering, Springer, pp 101–115

**出版者声明：** Springer Nature 对所刊载地图及机构隶属关系中的管辖权主张保持中立立场。

## 所属机构
An ́ıbal Iung<sup>[1](#affiliations)</sup> · Joa ̃ o Carbonell<sup>[1](#affiliations)</sup> · Luciano Marchezan<sup>[1](#affiliations)</sup> · Elder Rodrigues<sup>[1](#affiliations)</sup> · Maicon Bernardino<sup>[1](#affiliations)</sup> · Fabio Paulo Basso<sup>[1](#affiliations)</sup> · Bruno Medeiros<sup>[1](#affiliations)</sup>1

- Joa ̃o Carbonell joaocarbonellpc@gmail.com
- Luciano Marchezan lucianomarchp@gmail.com
- Elder Rodrigues eldermr@gmail.com
- Maicon Bernardino bernardino@acm.org
- Fabio Paulo Basso fabiopbasso@gmail.com
- Bruno Medeiros brunobragamedeiros@gmail.com

#### affiliations
Federal University of Pampa (Unipampa), Av. Tiaraju ́, 97546-550, Alegrete, RS, Brazil
