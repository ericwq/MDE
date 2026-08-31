# 第 2 章 编写 Web 服务客户端

## Web 服务就是网站

在 [第 1 章](ch1.md) 中，我展示了一些针对现有公共 Web 服务的客户端快速示例。
其中一些服务具有面向资源的 RESTful 架构，一些具有 RPC 风格架构，还有一些是混合型。
大多数情况下，我通过包装库访问这些服务，而不是自己发起 HTTP 请求。

你并不总能依赖你喜欢的 Web 服务存在便捷的包装库，尤其是当你自己编写了该 Web 服务时。
幸运的是，直接处理 HTTP 请求和响应来编写程序很容易。
在本章中，我将展示如何使用多种编程语言为 RESTful 和混合架构服务编写客户端。

示例 2-1 是一个针对 RESTful Web 服务 —— Yahoo! Web 搜索的裸 HTTP 客户端。
你可以将其与 [示例 1-8](ch1.md#example-1-8)
（前一章中针对 Google Web 搜索的 RPC 风格 SOAP 接口的客户端）进行比较。

*示例 2-1. 使用 Yahoo! Web 服务搜索 Web* <a id="example-2-1"></a>

```ruby
#!/usr/bin/ruby
# yahoo-web-search.rb
require 'open-uri'
require 'rexml/document'
require 'cgi'

BASE_URI = 'http://api.search.yahoo.com/WebSearchService/V1/webSearch'

def print_page_titles(term)
  # 获取资源：一个包含搜索结果的 XML 文档
  term = CGI::escape(term)
  xml = open(BASE_URI + "?appid=restbook&query=#{term}").read
  
  # 将 XML 文档解析为数据结构
  document = REXML::Document.new(xml)
  
  # 使用 XPath 查找数据结构中有趣的部分
  REXML::XPath.each(document, '/ResultSet/Result/Title/[]') do |title|
    puts title
  end
end

(puts "Usage: #{$0} [search term]"; exit) if ARGV.empty?
print_page_titles(ARGV.join(' '))
```

这段 “Web 服务” 代码看起来就像通用的 HTTP 客户端代码。
它使用 Ruby 的标准 `open-uri` 库来发起 HTTP 请求，并使用 Ruby 的标准 `REXML` 库来解析输出。
我会使用相同的工具来获取和处理网页。
这两个 URI：

- `http://api.search.yahoo.com/WebSearchService/V1/webSearch?appid=restbook&query=jellyfish`
- `http://search.yahoo.com/search?p=jellyfish`

指向同一事物的不同表现形式：“针对查询 ‘jellyfish’ 的搜索结果列表。”
一个 URI 提供 HTML，供 Web 浏览器使用；
另一个提供 XML，供自动化客户端使用。

<div style="background-color: darkgreen; padding: 8px; border-left: 4px solid lightgreen;">
<center>XPath 详解</center>

从右向左读，表达式 `/ResultSet/Result/Title/[]` 的意思是：

```
查找直接子节点                  []
每个 `Title` 标签               Title/
是 `Result` 标签的直接子节点    Result/
是 `ResultSet` 标签的直接子节点 ResultSet/
位于文档的根节点                /
```

如果你查看 Yahoo! 搜索服务提供的 XML 文件，你会看到一个 `ResultSet` 标签，其中包含 `Result` 标签，每个 `Result` 标签又包含一个 `Title` 标签。
这些标签的内容正是我在 [示例 2-1](#example-2-1) 中要提取的。

</div><br/>

并没有什么神奇之处能让一个 HTTP 请求变成 Web 服务请求。
你完全可以仅使用编程语言的 HTTP 客户端库来向 RESTful 或混合 Web 服务发起请求。
你可以使用标准的 XML 解析器处理响应结果。
每个 Web 服务请求都涉及相同的三个步骤：

1. 构造要放入 HTTP 请求的数据：HTTP 方法、URI、任何 HTTP 首部，以及（对于使用 PUT 或 POST 方法的请求）需要放入请求实体主体中的任何文档。
2. 将数据格式化为 HTTP 请求，并发送给相应的 HTTP 服务器。
3. 将响应数据 ——响应码、任何首部以及任何实体主体—— 解析为程序其余部分所需的数据结构。

在本章中，我将展示不同的编程语言和库如何实现这三个步骤。

### 包装器、WADL 和 ActiveResource

尽管 Web 服务请求只是 HTTP 请求，但任何给定的 Web 服务都具有整个万维网所缺乏的逻辑和结构。
如果你每次发起 Web 服务请求都遵循三步算法，你的代码将会变得一团糟，而且你将永远无法利用底层的结构。

相反，作为聪明的程序员，你会很快注意到对特定服务请求的模式，并编写包装方法来抽象掉 HTTP 访问的细节。
[示例 2-1](#example-2-1) 中定义的 `print_page_titles` 方法就是一个原始包装器。
随着 Web 服务的普及，其用户会发布各种语言的精良包装库。
一些服务提供商提供官方包装器：Amazon 为其 RESTful S3 服务提供了五种不同语言的客户端。
但这并未阻止外部程序员编写自己的 S3 客户端库，如 jbucket 和 s3sh。

包装器使服务编程变得容易，因为包装库的 API 是针对特定服务定制的。
你完全不必考虑 HTTP。
缺点在于每个包装器略有不同：学习一个包装器并不能让你为下一个做好准备。

这有点令人失望。
毕竟，这些服务只是发起 HTTP 请求的三步算法的变体。
难道不应该有某种方法来抽象服务之间的差异，某个库可以作为 RESTful 和混合服务整个空间的包装器吗？

<ins>这就是服务描述的问题。
我们需要一种词汇表能够描述各种 RESTful 和混合服务的语言。
用这种语言编写的文档可以为通用 Web 服务客户端编写脚本，使其像定制包装器一样工作。</ins>
SOAP RPC 社区已围绕 WSDL 作为其服务描述语言统一起来。
REST 社区尚未围绕某种描述语言统一起来，因此在本书中，我尽自己的一份力来推广 WADL，作为 WSDL 的面向资源替代方案。
我认为这是解决整个问题的最简单、最优雅的方案。
我在本章中展示了一个简单的 WADL 客户端，并在 “WADL” 部分进行了详细介绍。

还有一种名为 ActiveResource 的通用客户端，仍在开发中。
ActiveResource 使得为使用 Ruby on Rails 框架编写的多种 Web 服务编写客户端变得容易。
我将在 [第 3 章](ch3.md) 末尾介绍 ActiveResource。

<img src="./img/fig-2-1.png" width="90%"/><br/>
*图 2-1. del.icio.us 截图*

## del.icio.us：示例应用

在本章中，我将从客户端的角度，逐步讲解 Web 服务请求的生命周期。
尽管本书中的大部分代码示例是用 Ruby 编写的，但本章我将展示多种编程语言的代码。
本章通篇使用的示例是社交书签网站 del.icio.us（ http://del.icio.us/ ）提供的 Web 服务。
你可以访问 http://del.icio.us/help/api/ 阅读该 Web 服务的文字描述。

💡 如果你不熟悉 del.icio.us，这里简要介绍一下。
del.icio.us 是一个网站，它的功能类似于你的 Web 浏览器书签功能，但它是公开的且组织得更好（见图 2-1）。
当你将一个链接保存到 del.icio.us 时，它会与你的账户关联，以便你以后找到它。你也可以与他人分享你的书签。

💡  你可以将称为标签（tags）的短字符串与 URI 关联。
标签非常灵活，它们使你以后能轻松找到 URI，使 URI 能够分组，并且当多人对同一 URI 打标签时，他们为该 URI 创建了一个机器可读的词汇表。

del.icio.us Web 服务让你能够以编程方式访问你的书签。
你可以编写程序来收藏 URI、将你的浏览器书签转换为 del.icio.us 书签，或获取你过去收藏的 URI。
可视化 del.icio.us Web 服务的最佳方式是先使用一段时间面向人类的网站。
del.icio.us 网站与 del.icio.us Web 服务之间没有根本区别，但存在一些差异：

- 网站根地址为 `http://del.icio.us/`，而 Web 服务根地址为 `https://api.del.icio.us/v1/`。
网站通过 HTTP 与客户端通信，而 Web 服务使用安全的 HTTPS。

- 网站和 Web 服务暴露不同的 URI 结构。
要从网站获取你最近的书签，你访问 `https://del.icio.us/{your-username}`。
要从 Web 服务获取你最近的书签，你访问 `https://api.del.icio.us/v1/posts/recent`。

- 网站提供 HTML 文档，而 Web 服务提供 XML 文档。
格式不同，但包含相同的数据。

- 网站让你无需登录甚至无需账户即可查看大量信息。
而 Web 服务要求你对每个请求进行身份验证。

- 两者都提供个人书签管理功能，但网站还具有社交功能。
在网站上，你可以查看其他人收藏的 URI 列表、收藏了特定 URI 的用户列表、带有特定标签的 URI 列表，以及热门书签列表。
Web 服务只允许你查看自己的书签。

这些差异很重要，但它们并没有使 Web 服务成为与网站不同的东西。
Web 服务是一个精简版的网站，它使用 HTTPS 并提供外观古怪的文档。
（你也可以反过来，将网站视为功能更丰富的 Web 服务，尽管 del.icio.us 的管理员不鼓励这种观点。）
这是我反复强调的主题：Web 服务应该遵循与网站相同的规则。

除了与网站的相似性之外，del.icio.us Web 服务并没有非常 RESTful 的设计。
程序员们以一种暗示 RPC 风格而非面向资源设计的方式来布局服务 URI。
所有对 del.icio.us Web 服务的请求都使用 HTTP GET 方法：真正的方法信息放在 URI 中，并可能与 “GET” 产生冲突。
几个示例 URI 可以说明这一点：看看 `https://api.del.icio.us/v1/posts/add` 和 `https://api.del.icio.us/v1/tags/rename`。
尽管没有显式的 `methodName` 变量，但 del.icio.us API 与我在 [第 1 章](ch1.md) 中介绍的 Flickr API 非常相似。
方法信息（“add” 和 “rename”）保存在 URI 中，而非 HTTP 方法中。

那么，为什么我选择 del.icio.us 作为本章示例服务的对象呢？原因有三。

首先，del.icio.us 是一个易于理解的应用程序，其 Web 服务流行且易于使用。

其次，我想明确指出，我在后续章节中的论述是规范性的，而非描述性的。
当你实现 Web 服务时，遵循 REST 的约束将赋予你的客户端一个良好、可用的、行为像 Web 一样的 Web 服务。
但当你实现 Web 服务客户端时，你只能按服务现有的方式工作。
唯一的替代方案是游说对方更改或抵制该服务。
如果 Web 服务设计者从未听说过 REST，或者认为混合服务就是 “RESTful” 的，你几乎无能为力。
大多数现有服务都是混合型或纯粹的 RPC 服务。
一个只能消费最纯粹 REST 服务的傲慢客户端并没有多大用处，并且在可预见的未来也不会变得有用。
服务器应该是理想主义的；客户端必须是务实的。
这是 Postel 法则的一个变体：“在自己发送时保守，在接收他人时开放。”

第三，在 [第 7 章](ch7.md) 中，我将展示一个类似于 del.icio.us 但基于 RESTful 原则设计的书签跟踪 Web 服务。
我现在就想向你介绍社交书签领域，这样当我介绍 REST 原则和我的面向资源架构时，你已经在思考它了。
在 [第 7 章](ch7.md) 中，当我设计并实现一个针对类似 del.icio.us 功能的 RESTful 接口时，你将看到其中的差异。

### 示例客户端的功能

在接下来的各节中，我将向你展示多种编程语言的简单 del.icio.us 客户端。
所有这些客户端都执行完全相同的操作，值得明确说明是什么操作。
首先，它们打开一个到 `api.del.icio.us` 服务器上 443 端口（标准 HTTPS 端口）的 TCP/IP 套接字连接。
然后，它们发送类似示例 2-2 中的 HTTP 请求。
del.icio.us Web 服务会返回类似示例 2-3 中的 HTTP 响应，然后关闭套接字连接。
与所有 HTTP 响应一样，这个响应有三部分：状态码、一组首部和实体主体。
在此例中，实体主体是一个 XML 文档。

*示例 2-2. 对 del.icio.us Web 服务的一个可能请求*

```http
GET /v1/posts/recent HTTP/1.1
Host: api.del.icio.us
Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=
```

*示例 2-3. del.icio.us Web 服务的一个可能响应*

```http
200 OK
Content-Type: text/xml
Date: Sun, 29 Oct 2006 15:09:36 GMT
Connection: close

<?xml version='1.0' standalone='yes'?>
<posts tag="" user="username">
  <post href="http://www.foo.com/" description="foo" extended=""
        hash="14d59bdc067e3c1f8f792f51010ae5ac" tag="foo"
        time="2006-10-29T02:56:12Z" />
  <post href="http://amphibians.com/" description="Amphibian Mania"
        extended="" hash="688b7b2f2241bc54a0b267b69f438805" tag="frogs toads"
        time="2006-10-28T02:55:53Z" />
</posts>
```

我编写的客户端只对实体主体部分感兴趣。
具体来说，它们只对 `post` 标签的 `href` 和 `description` 属性感兴趣。
它们会将 XML 文档解析为数据结构，并使用 XPath 表达式 `/posts/post` 来遍历 `post` 标签。
它们会将每个 del.icio.us 书签的 `href` 和 `description` 属性打印到标准输出：

```
foo: http://www.foo.com/
Amphibian Mania: http://amphibians.com/
```

<div style="background-color: darkgreen; padding: 8px; border-left: 4px solid lightgreen;">
<center>XPath 详解</center>

从右向左读，XPath 表达式 `/posts/post` 的意思是：

```
查找每个 `post` 标签（`post`）——  
该标签是 `posts` 标签的直接子节点（`posts/`）——  
而 `posts` 标签位于文档的根节点（`/`）。
```

</div><br/>

## 发起请求：HTTP 库

每种现代编程语言都有一个或多个用于发起 HTTP 请求的库。
不过，并非所有这些库都同样有用。
要构建一个功能完整的通用 Web 服务客户端，你需要具备以下特性的 HTTP 库：

- 必须支持 HTTPS 和 SSL 证书验证。
Web 服务与网站一样，使用 HTTPS 来保护与客户端的通信。
许多 Web 服务（del.icio.us 是其中之一）根本不接受纯 HTTP 请求。
库的 HTTPS 支持通常依赖于是否存在用 C 编写的外部 SSL 库。

- 必须至少支持五种主要的 HTTP 方法：GET、HEAD、POST、PUT 和 DELETE。
有些库只支持 GET 和 POST。
还有些库为了简化设计，只支持 GET。

仅支持 GET 和 POST 的客户端也能做很多事情：HTML 表单只支持这两种方法，因此整个人类 Web 对你都是开放的。
甚至只支持 GET 也能应付，因为许多 Web 服务（包括 del.icio.us 和 Flickr）在不该使用 GET 的地方也使用了 GET。
但如果你要为所有 Web 服务客户端选择一个库，或者正在编写像 WADL 客户端这样的通用客户端，你就需要一个支持所有五种方法的库。
额外支持 OPTIONS 和 TRACE 等方法，以及 MOVE 等 WebDAV 扩展方法，则更为理想。

- 必须允许程序员自定义作为 PUT 或 POST 请求实体主体发送的数据。

- 必须允许程序员自定义请求的 HTTP 首部。

- 必须允许程序员访问 HTTP 响应的响应码和首部，而不仅仅是实体主体。

- 必须能够通过 HTTP 代理进行通信。
普通程序员可能不会考虑这一点，但企业环境中的许多 HTTP 客户端只能通过代理工作。
像 HTTP 代理这样的中介也是 REST 元架构的标准组成部分，尽管我不会在这方面详细展开。

### 可选特性

HTTP 库还有一些特性，能在你为 RESTful 和混合服务编写客户端时让工作更轻松。
这些特性大多归结为对 HTTP 首部的了解，因此从技术上讲是可选的。
只要你的库允许你访问请求和响应 HTTP 首部，你就可以自己实现它们。
库支持的优势在于你无需担心细节。

- HTTP 库应能自动以压缩形式请求数据以节省带宽，并透明地解压接收到的数据。
这里的请求首部是 `Accept-Encoding`，响应首部是 `Encoding`。
我将在 [第 8 章](ch8.md) 中更详细地讨论这些。

- 应能自动缓存对请求的响应。
当你第二次请求某个 URI 时，如果服务器上的对象未发生变化，它应返回缓存中的条目。
这里的 HTTP 首部包括请求中的 `ETag` 和 `If-Modified-Since`，以及响应中的 `ETag` 和 `Last-Modified`。
我也将在 [第 8 章](ch8.md) 中讨论这些。

- 应能透明地支持最常见的 HTTP 认证方式：Basic、Digest 和 WSSE。
支持自定义的、公司特有的认证方式（如 Amazon 的方式）或提供支持这些方式的插件也很有用。
请求首部是 `Authorization`，响应首部（要求认证的那个）是 `WWW-Authenticate`。
我将在 [第 8 章](ch8.md) 中介绍标准的 HTTP 认证方式以及 WSSE。
我将在 [第 3 章](ch3.md) 中介绍 Amazon 的自定义认证方式。

- 应能透明地跟随 HTTP 重定向，同时避免无限重定向和重定向循环。
这应作为用户的便捷选项，而非对每个重定向都自动执行。
Web 服务合理地发送状态码 303（“See Other”）时，并不意味着客户端应立即去获取那个其他 URI！

- 应能解析和创建 HTTP cookie 字符串，而非强制程序员手动设置 `Cookie` 首部。
这对于避开 cookie 的 RESTful 服务来说不太重要，但如果你要使用人类 Web，这非常重要。

当你针对特定服务编写代码时，可能可以不需要其中部分或全部特性。
Ruby 标准的 `open-uri` 库只支持 GET 请求。
如果你在为 del.icio.us 编写客户端，这没问题，因为该 Web 服务只期望 GET 请求。
但尝试将 `open-uri` 用于 Amazon S3（使用 GET、HEAD、PUT 和 DELETE），你会很快碰壁。
在接下来的几节中，我将为一些流行的编程语言推荐优秀的 HTTP 客户端库。
