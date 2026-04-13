# 中位数——三个函数的故事

<img src="img/donkey-card.png" width="25%" align="right"/></br>
本文为 [探索生成式AI](exploring-gen-ai.md) 系列的一部分，该系列记录了 Thoughtworks 技术人员在软件开发中运用生成式 AI 技术的探索实践。

|| |
|:---|---:|
|[Birgitta Böckeler](https://birgitta.info/)| |
|<img src="img/bb.jpg" width="30%" /> |Birgitta 是 Thoughtworks 的杰出工程师，同时也是 AI 辅助交付领域专家。她拥有二十余年软件开发、架构设计及技术管理经验。|
| [原文](https://martinfowler.com/articles/exploring-gen-ai/02-median-tale.html) |2023/7/27|

---
这是一个关于生成中位数函数的小故事，它展现了大语言模型辅助编码的实用价值与局限性。

我需要在一个 TypeScript 代码库中计算一组数字的中位数。
以往我通常会上网搜索 “median function javascript”，而这次我尝试让 GitHub Copilot 来协助我完成。

```TypeScript
// calculate the median of a list of numbers
```

Copilot 首先为我生成了规范的方法签名：

```TypeScript
public static median(values:number[]): number {
```

然后给出了该函数的三种不同实现方案。

## 方案一

```TypeScript
public static median(values:number[]): number {
    if(values.length === 0) return 0;

    values.sort((a,b) => a-b);

    var half = Math.floor(values.length / 2);

    if (values.length % 2)
    return values[half];

    return (values[half - 1] + values[half]) / 2.0;
}
```

该实现能够正常运行，但输入参数 `values` 会被 `sort` 函数修改。
我把这个示例给几位同事看后，得到了不同的反馈：
一位同事表示 “这写法很糟糕，这类问题我们经常会忘记写测试覆盖，进而演变成极难调试的错误”；
另一位则认为 “这也不一定绝对不好，要视情况而定”。
我本想知道 IDE 中的静态代码分析工具是否会将其标记为问题，然而无论是 ESLint 还是 Sonar 都未对此发出提示。


## 方案二

```TypeScript
function median2(values: number[]) {
    const sorted = values.slice().sort((a, b) => a - b);
    const middle = Math.floor(sorted.length / 2);
    if (sorted.length % 2 === 0) {
        return (sorted[middle - 1] + sorted[middle]) / 2;
    } else {
        return sorted[middle];
    }
}
```

这种实现方式是可行的，并且不会存在修改输入参数的问题，因为 `slice()` 会返回输入数组的一个副本。

## 方案三

```TypeScript
function median3(values: number[]) {
    const sorted = values.slice().sort((a, b) => a - b);
    const middle = Math.floor(sorted.length / 2);
    return sorted[middle];
}
```

乍一看，这个实现似乎比其他方案更出色——代码更简短，且圈复杂度为零，我们通常会认为这种写法 “更优雅”。
我为它编写的测试也运行通过了，效果很不错！

但后来我意识到，测试时必须同时覆盖偶数个和奇数个数值的情况。
我之前编写的测试只使用了长度为奇数的输入，而该实现在长度为偶数的场景下会出错。
这个例子很好地说明：我们依然需要理解自己在做什么，才能为生成的代码编写出合理的测试。
我们必须清楚中位数的计算逻辑，才能编写出覆盖完整功能的测试用例。


## 不过这和从网上复制代码难道不一样吗？

这样的使用场景恰好完美体现了人们眼中这种 “超强自动补全 (autocomplete on steroids)” 的一大优势：
你不必中断自己的工作流和工具链，就能解决这类问题。
换作平时，我们反正也会从别处复制粘贴代码，之后同样需要仔细审查代码、编写测试。所以风险是一样的——对吧？

唯一的区别在于，使用 Copilot 时，我们无法得知代码的来源。
而在 Stack Overflow 这类平台上，我们至少能多一个判断代码片段质量的依据：点赞数。

顺带一提，“方案一” 的代码几乎和 [Stack Overflow 上该问题](https://stackoverflow.com/questions/45309447/calculating-median-javascript) 最高赞回答给出的代码完全一致，尽管它存在那个小缺陷。不过，有用户在评论区指出了修改输入参数这个问题。

## 先生成测试，还是先写代码？或是两者一起？
那么反过来想会怎样呢？
如果我先让 GitHub Copilot 帮我生成测试用例呢？
我用 Copilot Chat 试了一下，它给出了一组非常完善的测试，其中就包含一个会让 “方案三” 在偶数长度数组场景下执行失败的测试用例。

```TypeScript
it("should return the median of an array of odd length", () => { ... }

it("should return the median of an array of even length", () => { ... }

it("should return the median of an array with negative numbers", () => { ... }

it("should return the median of an array with duplicate values", () => { ... }
```

就中位数这类非常常见的小型函数而言，我甚至会考虑同时使用自动生成的代码和对应的测试代码。
生成的测试可读性很好，我也能轻松判断其覆盖范围，而且它们还能提醒我：需要同时考虑输入长度为偶数和奇数的情况。
不过，对于其他包含更多定制化逻辑的复杂函数，我会选择自己编写测试，以此作为质量把控的手段。
尤其是在处理较大的函数时，我更愿意从头开始、有条理地设计测试用例，而不是先从工具得到部分场景，再去补充缺失的情况。

## 工具本身能否帮我修复生成代码中的缺陷？

我让 Copilot Chat 对 “方案一” 进行重构，使其不会修改输入参数，它给出了合理的修复方案。
不过这个前提是，我已经知道代码需要改进的地方。

我还更宽泛地询问了 ChatGPT，“方案三” 存在哪些问题或可以优化的地方。
它确实指出了该方案在输入长度为偶数时无法正常运行的问题。

## 结论
- 你必须清楚自己在做什么，才能判断生成的代码建议是否合理。
在本例中，我需要理解中位数的计算原理，才能为生成的代码编写合理的测试。

- 工具本身或许能指出生成代码存在的问题或可优化之处——这能否成为未来提升代码质量的可行方向，还是说我们注定只能和 AI 工具进行循环往复的对话？

- 出于质量管控的考虑，我原本对同时生成测试用例和功能代码持怀疑态度。
但即便之后弃用生成的代码，自动生成的测试也能为我提供一些未曾考虑到的测试场景思路。
并且根据函数的复杂程度，如果相关场景易于理解，我也可能考虑直接使用生成的测试用例。
