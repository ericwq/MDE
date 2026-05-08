# 如何上手 Pi 编码智能体（在 VPS 上）

2026/4/24 Willem pi.dev agentic-engeeringing getting-started how-to

[原文](https://willemvandenende.com/blog/engineering/how-to-get-started-with-the-pi-coding-agent-on-a-vps)

---
有几个人问过我的编码智能体配置。
这是一份关于如何在 VPS（虚拟专用服务器）上使用托管 LLM 服务来设置 Pi 编码智能体的简要指南。
这套方案适用于 [Open Router](https://willemvandenende.com/openrouter.ai)，
也应当适用于任何支持 OpenAI（ChatGPT）API 的服务，包括本地模型、Anthropic、OpenAI 等。
Pi 还支持其他 API，而且在我写这篇文章时发现，Pi 的开箱体验非常出色。

过去一两个月里，我一直在笔记本电脑上的沙箱中使用 Pi。
我在 VPS 上部署了 Claude Code，但也希望 Pi 能在那里运行。
Chris Parsons（他的博客见 [后记](#后记) ）问到了这一点，所以我为自己写了这份操作指南，随后又运行并更新了它。
整个过程比我想象的要简单。

使用 VPS（云端的虚拟机）的思路在于，它为你提供了一个运行智能体的沙箱。
如果智能体删除了你的 home 目录，你只需重建一个即可。
当然还有其他方式来沙箱化智能体，但就目前而言，我发现这是最简单也最让人安心的方式。

## 本指南的步骤
1. 安装 Pi
2. 将你的 API 密钥放入环境变量，以便 Pi 能够访问
3. 告诉 Pi 你的 LLM 托管在哪里，以及你想使用哪个模型
4. 启动 Pi，然后开始使用

至少这是我最初的设想。实际上比这还要简单。
1. 安装 Pi
2. 按照指引，以小步骤完成针对你的模型和提供商的安装。
3. 在 Pi 中键入 /Reload，然后开始使用 (*)

(*) 修复好 `~/.pi/agent/models.json` 中的语法错误 —— 你的所有配置都可以放在这个文件里，除非你决定将其拆分出去。

我当时觉得展示我的操作过程仍然有用 —— Pi 的用户界面比 Claude Code 响应快得多，并且会为你提供指引。
不过一开始我并没有注意到这一点。希望这对你有帮助。
祝你玩得开心！

## 安装 Pi
Pi 假定你已经安装了 [Node.js](https://nodejs.org/en) 。
如果还没有安装，Node.js 官网提供了安装说明，它通常也包含在你 VPS 所用 Linux 发行版的包管理器中。

安装好 Node.js 之后，在终端中运行以下命令：

```sh
npm install -g @mariozechner/pi-coding-agent
```

现在你可以启动 Pi，它会指引你找到其余文档的位置。
以下是它向我展示的内容：

```
Warning: No models available. Use /login to log into a provider via OAuth or API key. See:
   [somewhere on your disk]/lib/node_modules/@mariozechner/pi-coding-agent/docs/providers.md
   [somewhere on your disk]//node/24.0.1/lib/node_modules/@mariozechner/pi-coding-agent/docs/models.md
```

这是 Pi 最让我喜欢的一点，令人意外：你所使用版本的文档就在你自己的机器上，而且它会不遗余力地把你和你的模型指向这些文档，
这样一来，你就能在对话中自行摸索出如何使用和扩展它。

不过我们现在还不能开始对话，因为我们还没有配置 provider，也没有配置模型。

所以我们需要告诉 Pi 两件事：
- 你的 provider 是什么（托管你模型的服务方或服务器）
- 那里有哪些模型可用

关于第二点，你需要比我预想中更多的细节，所以才有了这篇文章。
我将以 OpenRouter 作为 provider，去那里找到我能找到的最便宜的模型 —— 我们只是想快速发送一个提示，看看 Pi、provider 和模型三者能否协同工作。

在没有配置模型的情况下，你在 Pi 中仍然可以使用 `!` 来运行 shell 命令。
我将对 provider 的文档运行 `cat` 命令，看看如何配置一个 provider。

```sh
!cat  [..]/lib/node_modules/@mariozechner/pi-coding-agent/docs/providers.md
```

现在我们可以编辑 `$HOME/pi/agent/models.json` 来设置 provider 端点和 API 密钥。
你也可以在此文件中指定模型，但对于入门来说并非必需。
在 UI 中使用 `/model` 命令可以让你搜索已提供的模型。

我已在另一台机器上配置过 Pi，所以我直接问了它。
下一节是与 Qwen3.6:27b 共同撰写的：

## 告诉 Pi 你的 LLM 托管在哪里（由 Qwen 撰写）
Pi 内置了对 OpenRouter 的支持。
你只需要在 `~/.pi/agent/models.json` 中进行配置即可。

快速配置

创建或编辑 `~/.pi/agent/models.json` 文件：

```json
   {
     "providers": {
       "openrouter": {
         "baseUrl": "https://openrouter.ai/api/v1",
         "apiKey": "OPENROUTER_API_KEY",
         "api": "openai-completions"
       }
     }
   }
```

这样就完成了 —— 不需要配置 `models` 数组。
Pi 会自动加载所有内置的 OpenRouter 模型。
你的 API 密钥可以是：
- 一个环境变量名（例如 `"OPENROUTER_API_KEY"`）
- 一个直接的密钥字符串（例如 `"sk-or-…"`）
- 一个 shell 命令（例如 `"!op read 'op://vault/item/credential'"`）

配置完成后

1. 打开 Pi，运行 `/model` 查看可用的 OpenRouter 模型
2. 使用 `/model openrouter/anthropic/claude-sonnet-4`（或你想要的任何模型）来选择模型

关于排序、忽略、max_price、preferred_min_throughput 等选项，请参阅 models 文档中的完整 `compat.openRouterRouting` 配置说明。

## 第一条提示

最后，我们可以发送提示了。
我从 Claude Code 那里学到，当你只是想看看模型是否能正常工作时，让模型写一首俳句是一个入门的好方法。
有时我会把我的挫败感写进提示里，但这次不会。

> Model: qwen3.5:35b-a3b

我：写一首俳句

（推理 token：）

> 用户让我写一首俳句。俳句是日本诗歌的一种形式，共三行，音节模式为 5-7-5。我来写一首关于自然的俳句，这是俳句的传统主题。
> 
> 细雨轻柔落，水珠亲吻绿叶间，春息满庭院。

希望这对你有效。祝你使用愉快！

## 后记

Chris Parsons 在与我讨论他的[How I use AI to Code](https://www.chrismdp.com/coding-with-ai/) 时，问到我上手 Pi 需要做哪些准备等问题。
鉴于大家也一直在询问我的配置以及我如何使用它，我觉得从这篇文章开始回答是个不错的选择。
如果身边能有更多人使用开源智能体、开源权重模型和开源模型，那就太好了。

