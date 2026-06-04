---
name: test-quality
description: "在生成或审查测试代码时应用测试质量原则。强制执行 Arrange-Act-Assert 结构、每个测试一个行为、断言质量、测试隔离、有意义的命名和测试数据管理。在编写 tests、审查 test code 时使用，或当用户提到'write tests'、'test this'、'test quality'、'test review'、'improve tests'或'test structure'时使用。此 skill 管理编写单个测试用例的工艺——不是测试什么（由正在实现的代码驱动），而是如何编写 reliable、readable 和 maintainable 的 tests。"
---

# Test Quality（测试质量）

## 配置解析（Config Resolution）

Skill 支持项目自定义。顺序：

1. 在仓库根目录查找 `.lattice/config.yaml`
2. 如果找到，检查 `paths.test_quality` 获取自定义文档路径
3. 如果自定义路径存在，读取文档并检查 YAML frontmatter `mode`：
   - **`mode: override`**（覆盖模式，或无 mode）：自定义文档完全控制。替代默认值使用。必须完整——是唯一参考。
   - **`mode: overlay`**（叠加模式）：首先读取 `./references/defaults.md`，然后将自定义文档叠加在上面。自定义部分替换匹配部分（按标题）。新部分添加在之后。
4. 如果没有配置/路径/文件，读取 `./references/defaults.md`
5. **语言适配**：如果配置中存在 `paths.language_idioms`，读取 **"Testing Patterns"** 部分并将§5（测试命名）、§4（测试隔离）、§6（测试数据构建器）适配到语言测试框架习惯用法。语言习惯用法优先于伪代码默认值。

Skill 附带的默认值。开箱即用。仅当团队有不同标准时才覆盖。

## 自我验证清单（Self-Validation Checklist）

生成每个测试后 STOP。在继续之前检查所有项目。如果失败，修复。如果模糊（参见 Ambiguity Signals），标记——显示选项和推理。

1. **AAA 结构**：arrange、act、assert 用 blank lines 分开？Arrange 或 assert 中是否有任何逻辑（if/loop/try）？
2. **单一行为**：Test verify 一个行为 per loaded doc（默认：每个测试一个行为，name need"and"=split）？
3. **断言质量**：Assert observable behavior，而非 implementation？Specific enough catch regression？
4. **隔离**：Test depend 其他 test output/effects？All mutable state per-test？
5. **测试名称**：Name follow team convention per loaded doc（默认：describe behavior，而非 method）？Failure message clear？
6. **测试数据**：Complex arrange uses builders/factories？Magic values → named constants？（Inline literals fine for trivial tests。）
7. **Mock 边界**：Mock per loaded doc（默认：仅在 arch boundaries——I/O、external——而非 internal collab）？
8. **测试代码作为第一类**：Structured like production code？Shared constants at top、helpers extracted、no dead code、clear file organization？

项目特定检查：如果加载的文档包含验证清单部分，在基本清单后应用这些。

## 主动反模式扫描（Active Anti-Pattern Scan）

在清单后，扫描这些。如果找到，在呈现之前修复。

- [ ] **每个方法一个测试（Test-per-Method）**：One test per method regardless behaviors → One test per scenario、named for behavior
- [ ] **断言轮盘赌（Assertion Roulette）**：Multiple unrelated asserts；unclear which broke → Split to one behavior per test
- [ ] **共享可变状态（Shared Mutable State）**：Pass alone、fail together → Isolate state；per-test setup；no static mutable
- [ ] **测试实现细节（Testing Implementation Details）**：Break on refactor w/ same behavior；mock call counts → Assert observable behavior、not method calls
- [ ] **神秘客人（Mystery Guest）**：Depend external file/db/env var not visible → Inline data 或 use builders；all preconditions visible
- [ ] **默认慢测试（Slow Tests by Default）**：Unit suite take minutes；hit db/network/fs → Mock/fake I/O；use in-memory
- [ ] **条件测试逻辑（Conditional Test Logic）**：Test have if/loop/try -- test need own tests → Remove logic；use parameterized；let asserts fail natural
- [ ] **复制粘贴测试（Copy-Paste Tests）**：Near-identical w/ small changes → Extract shared setup to builders；use parameterized

## 类级别审查（Class-Level Review）

触发条件：(1) completing all tests for a class——new 或 existing。(2) adding or editing any test in an existing class。

STOP before present。Per-test checks verify individual quality。This review verifies the test suite covers the class contract。

### Full review（新类或重大添加）

1. **行为清单（Behavior inventory）**——列出被测类中的每个公共方法/行为。如果类不可用，要求用户枚举。
2. **覆盖矩阵（Coverage matrix）**——映射每个测试 → 它覆盖的行为。零测试的行为 → **blocking**。在用户解决缺口或明确接受之前不要呈现。
3. **错误路径检查（Error path check）**——扫描被测类：显式抛出、条件错误分支、边界守卫。对于每个找到的：是否有测试执行此路径？如果没有 → 按名称标记。零覆盖的错误路径是阻塞的，除非用户明确接受。
4. **行为重复（Behavioral duplication）**——比较所有测试的"then"子句。无论结构差异如何，相同的可观察结果 → 标记为可能的重复。命名两个测试。
5. **平衡信号（Balance signal）**——任何有测试但没有覆盖失败或边界情况的行为 → 作为问题呈现，而非硬失败："`deleteUser` has 1 test (happy path only) — does it have error cases?"

### Edit-scoped review（在现有类中添加或更改一个测试）

仅运行步骤 3-5，范围限定为更改的测试：
- 此测试是否重复任何现有测试的可观察结果？
- 它是否覆盖以前未覆盖的行为或错误路径？

## 模糊信号（Ambiguity Signals）

检查通常有多个有效结果。当遇到时，呈现选项而非静默选择。

- **单元测试 vs 集成测试（Unit vs Integration）**：Service coordinate components -- test isolate (mock) or real collab? Depend coupling & what verify.
- **Mock 深度（Mock Depth）**：Mock direct depend or let call through? Over-mock test implementation; under-mock create slow/flaky.
- **测试粒度（Test Granularity）**：One test multi asserts vs multi tests one assert? When asserts verify facets same behavior, group ok.

## 测试代码作为第一类代码（Test Code as First-Class Code）

Test classes deserve same structural care as production code. They are living documentation -- readers spend as much time here as in source.

**Treat test files like production classes:**
- Shared constants and boundary values at top of file (named, not magic)
- Shared builders/factories extracted to helpers -- not copy-pasted per test
- Setup methods or fixtures for repeated arrange patterns
- Logical grouping: related behaviors together (by feature, by scenario type)
- Dead tests removed, not commented out
- Refactoring applies: extract method when arrange is long, rename when intent unclear, move shared setup when duplicated

**Refactoring opportunities to surface proactively:**
- Multiple tests repeat same arrange → extract to builder or shared fixture
- Same assertion pattern across tests → extract custom assertion helper
- Test file grows beyond ~300 lines → split by behavior group
- Constants scattered inline → collect at top with descriptive names
- Deeply nested test structures → flatten with clear naming

Write tests you would be proud to review. If test code is messy, team stops reading it. Unread tests become unmaintained. Unmaintained tests become liabilities.

## 核心原则（Core Principle）

Test purpose: **describe behavior & fail when behavior break**. Every choice serve this. Hard read, brittle refactor, slow run = not fulfill contract.

Bad test cost negative. Flaky train team ignore. Brittle slow dev. Pass when behavior broke = false confidence. Principles ensure tests assets, not liabilities.

Skill govern HOW write tests -- structure, isolation, asserts, naming. WHAT test driven by code implement & domain rules.

参见 `./references/defaults.md` 获取 AAA 结构示例、断言模式、隔离技术、命名约定、测试数据构建器模式和金字塔分布指南。
