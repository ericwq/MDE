# 第 2 章 编写 Web 服务客户端

<a id="web-services-are-web-sites"></a>
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

<a id="the-sample-application"></a>
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

<a id="making-the-request"></a>
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

### Ruby：rest-open-uri 与 net/http

Ruby 自带两个 HTTP 客户端库：`open-uri` 和更低层的 `net/http`。
如果你安装了 `net/https` 扩展，两者都可以发起 HTTPS 请求。
Windows 版 Ruby 应该开箱即用地支持 HTTPS 请求。
如果你不在 Windows 上，可能需要单独安装 `net/https`。⁑

`open-uri` 库有一个简单优雅的接口，允许你将 URI 当作文件名来对待。
要读取一个网页，你只需打开其 URI 并从 “文件句柄” 中读取数据。
你可以向 `open` 传入一个哈希，包含自定义 HTTP 首部和 `open` 特定的关键字参数。
这允许你设置代理或指定认证信息。

不幸的是，目前 `open-uri` 只支持一种 HTTP 方法：GET。
这就是为什么我对 `open-uri` 做了一些小修改，并将结果作为 `rest-open-uri` Ruby gem 提供。†
我添加了两个关键字参数：`:method`，允许你自定义 HTTP 方法；以及 `:body`，允许你在实体主体中发送数据。

⁑  在 Debian GNU/Linux 和基于 Debian 的系统（如 Ubuntu）上，包名为 `libopenssl-ruby`。如果你的打包系统不包含 `net/https`，你必须从 http://www.nongnu.org/rubypki/ 下载并手动安装。

† 关于 Ruby gems 的更多信息，请参见 http://rubygems.org/。一旦你安装了 `gem` 程序，你可以使用命令 `gem install rest-open-uri` 来安装 `rest-open-uri`。希望我对 `open-uri` 的修改有一天能进入 Ruby 核心代码，届时 `rest-open-uri` gem 将变得多余。

*示例 2-4. 使用 open-uri 的 Ruby 客户端* <a id="example-2-4"></a>

```ruby
#!/usr/bin/ruby -w
# delicious-open-uri.rb
require 'rubygems'
require 'open-uri'
require 'rexml/document'

# 获取 del.icio.us 用户的最近书签，并打印每一个
def print_my_recent_bookmarks(username, password)
  # 发起 HTTPS 请求
  response = open('https://api.del.icio.us/v1/posts/recent',
                  :http_basic_authentication => [username, password])
  
  # 将响应实体主体读取为 XML 文档
  xml = response.read
  
  # 将文档转换为数据结构
  document = REXML::Document.new(xml)
  
  # 遍历每个书签...
  REXML::XPath.each(document, "/posts/post") do |e|
    # 打印书签的描述和 URI
    puts "#{e.attributes['description']}: #{e.attributes['href']}"
  end
end

# 主程序
username, password = ARGV
unless username and password
  puts "Usage: #{$0} [username] [password]"
  exit
end
print_my_recent_bookmarks(username, password)
```

我之前提到过，Ruby 自带的 `open-uri` 只能发起 HTTP GET 请求。
对于许多用途来说，GET 已经足够，但如果你想为像 Amazon S3 这样完全 RESTful 的服务编写 Ruby 客户端，你要么需要使用 `rest-open-uri`，要么转向 Ruby 的低层 HTTP 库：`net/http`。

这个内置库提供了 `Net::HTTP` 类，它有多个用于发起 HTTP 请求的方法（见表 2-1）。
你可以仅使用 Ruby 标准库中的这个类来构建一个完整的 HTTP 客户端。
事实上，`open-uri` 和 `rest-open-uri` 都是基于 `Net::HTTP` 的。
这些库之所以存在，只是因为 `Net::HTTP` 没有提供一个简单易用、且支持 REST 客户端所需所有特性（代理、HTTPS、首部等）的接口。
这就是我推荐你使用 `rest-open-uri` 的原因。

*表 2-1. Ruby HTTP 客户端库的 HTTP 特性矩阵*
<img src="./img/tab-2-1.png" width="90%"/><br/>

### Python：httplib2

Python 标准库自带两个 HTTP 客户端：`urllib2`，它具有类似 Ruby `open-uri` 的文件式接口；
以及 `httplib`，它更类似于 Ruby 的 `Net::HTTP`。
假设你的 Python 编译时包含 SSL 支持，两者都提供透明的 HTTPS 支持。
此外还有一个优秀的第三方库，Joe Gregorio 的 `httplib2`（http://bitworking.org/projects/httplib2/ ），这是我在一般情况下推荐的库。
`httplib2` 是一款出色的软件，几乎支持我希望清单上的所有特性 —— 最值得注意的是透明的缓存功能。
表 2-2 列出了每个库中可用的特性。

*表 2-2. Python HTTP 客户端库的 HTTP 特性矩阵*
<img src="./img/tab-2-2.png" width="90%"/><br/>

*示例 2-5. Python 编写的 del.icio.us 客户端*

```python
#!/usr/bin/python2.5
# delicious-httplib2.py
import sys
from xml.etree import ElementTree
import httplib2

# 获取 del.icio.us 用户的最近书签，并打印每一个
def print_my_recent_bookmarks(username, password):
    client = httplib2.Http(".cache")
    client.add_credentials(username, password)
    
    # 发起 HTTP 请求，获取响应和实体主体
    response, xml = client.request('https://api.del.icio.us/v1/posts/recent')
    
    # 将 XML 实体主体转换为数据结构
    doc = ElementTree.fromstring(xml)
    
    # 打印每个书签的信息
    for post in doc.findall('post'):
        print "%s: %s" % (post.attrib['description'], post.attrib['href'])

# 主程序
if len(sys.argv) != 3:
    print "Usage: %s [username] [password]" % sys.argv[0]
    sys.exit()

username, password = sys.argv[1:]
print_my_recent_bookmarks(username, password)
```

**Java：HttpClient**

Java 标准库自带一个 HTTP 客户端 `java.net.HttpURLConnection`。
你可以通过在 `java.net.URL` 对象上调用 `open` 来获取一个实例。
尽管它支持 HTTP 的大部分基本特性，但其 API 编程难度很大。
Apache Jakarta 项目有一个竞争客户端，名为 `HttpClient`（http://jakarta.apache.org/commons/httpclient/ ），其设计更为优秀。
还有 Restlet（http://www.restlet.org/）。
我将在 [第 12 章](ch12.md) 中将 Restlet 作为服务器库进行介绍，但它也是一个 HTTP 客户端库。
`org.restlet.Client` 类使发起简单的 HTTP 请求变得容易，而 `org.restlet.data.Request` 类则封装了发起更复杂请求所需的 `HttpURLConnection` 编程。
表 2-3 列出了每个库中可用的特性。

*表 2-3. Java HTTP 客户端库的 HTTP 特性矩阵*

<img src="./img/tab-2-3.png" width="60%"/><br/>

*示例 2-6. Java 编写的 del.icio.us 客户端*

```java
// DeliciousApp.java
import java.io.*;
import org.apache.commons.httpclient.*;
import org.apache.commons.httpclient.auth.AuthScope;
import org.apache.commons.httpclient.methods.GetMethod;
import org.w3c.dom.*;
import org.xml.sax.SAXException;
import javax.xml.parsers.*;
import javax.xml.xpath.*;

/**
 * 一个命令行应用程序，从 del.icio.us 获取书签并打印到标准输出。
 */
public class DeliciousApp
{
    public static void main(String[] args)
        throws HttpException, IOException, ParserConfigurationException,
               SAXException, XPathExpressionException
    {
        if (args.length != 2)
        {
            System.out.println("Usage: java -classpath [CLASSPATH] "
                               + "DeliciousApp [USERNAME] [PASSWORD]");
            System.out.println("[CLASSPATH] - Must contain commons-codec, " +
                               "commons-logging, and commons-httpclient");
            System.out.println("[USERNAME] - Your del.icio.us username");
            System.out.println("[PASSWORD] - Your del.icio.us password");
            System.out.println();
            System.exit(-1);
        }

        // 设置认证凭据
        Credentials creds = new UsernamePasswordCredentials(args[0], args[1]);
        HttpClient client = new HttpClient();
        client.getState().setCredentials(AuthScope.ANY, creds);

        // 发起 HTTP 请求
        String url = "https://api.del.icio.us/v1/posts/recent";
        GetMethod method = new GetMethod(url);
        client.executeMethod(method);
        InputStream responseBody = method.getResponseBodyAsStream();

        // 将响应实体主体转换为 XML 文档
        DocumentBuilderFactory docBuilderFactory =
            DocumentBuilderFactory.newInstance();
        DocumentBuilder docBuilder =
            docBuilderFactory.newDocumentBuilder();
        Document doc = docBuilder.parse(responseBody);
        method.releaseConnection();

        // 使用 XPath 表达式从 XML 文档中获取书签列表
        XPath xpath = XPathFactory.newInstance().newXPath();
        NodeList bookmarks = (NodeList)xpath.evaluate("/posts/post", doc,
                                                       XPathConstants.NODESET);

        // 遍历书签并打印每一个
        for (int i = 0; i < bookmarks.getLength(); i++)
        {
            NamedNodeMap bookmark = bookmarks.item(i).getAttributes();
            String description = bookmark.getNamedItem("description")
                                          .getNodeValue();
            String uri = bookmark.getNamedItem("href").getNodeValue();
            System.out.println(description + ": " + uri);
        }

        System.exit(0);
    }
}
```

### C#：System.Web.HTTPWebRequest

.NET 公共语言运行时（CLR）定义了用于发起 HTTP 请求的 `HTTPWebRequest`，以及用于向服务器认证客户端的 `NetworkCredential`。
`HTTPWebRequest` 构造函数接受一个 URI。`NetworkCredential` 构造函数接受用户名和密码（见示例 2-7）。

*示例 2-7. C# 编写的 del.icio.us 客户端*

```csharp
using System;
using System.IO;
using System.Net;
using System.Xml.XPath;

public class DeliciousApp {
    static string user = "username";
    static string password = "password";
    static Uri uri = new Uri("https://api.del.icio.us/v1/posts/recent");

    static void Main(string[] args) {
        HttpWebRequest request = (HttpWebRequest) WebRequest.Create(uri);
        request.Credentials = new NetworkCredential(user, password);

        HttpWebResponse response = (HttpWebResponse) request.GetResponse();
        XPathDocument xml = new XPathDocument(response.GetResponseStream());
        XPathNavigator navigator = xml.CreateNavigator();

        foreach (XPathNavigator node in navigator.Select("/posts/post")) {
            string description = node.GetAttribute("description", "");
            string href = node.GetAttribute("href", "");
            Console.WriteLine(description + ": " + href);
        }
    }
}
```

### PHP：libcurl

PHP 自带 C 库 libcurl 的绑定，该库几乎可以完成你可能想对 URI 做的任何操作（见示例 2-8）。

*示例 2-8. PHP 编写的 del.icio.us 客户端*

```php
<?php
$user = "username";
$password = "password";

$request = curl_init();
curl_setopt($request, CURLOPT_URL,
            'https://api.del.icio.us/v1/posts/recent');
curl_setopt($request, CURLOPT_USERPWD, "$user:$password");
curl_setopt($request, CURLOPT_RETURNTRANSFER, true);

$response = curl_exec($request);
$xml = simplexml_load_string($response);
curl_close($request);

foreach ($xml->post as $post) {
    print "$post[description]: $post[href]\n";
}
?>
```

### JavaScript：XMLHttpRequest

如果你正在用 JavaScript 编写 Web 服务客户端，你可能打算让它作为 Ajax 应用的一部分在 Web 浏览器中运行。
所有现代 Web 浏览器都为 JavaScript 实现了一个名为 `XMLHttpRequest` 的 HTTP 客户端库。

由于 Ajax 客户端的开发方式与独立客户端不同，我专门用了一章来介绍它们：[第 11 章](ch11.md)。
该章的第一个示例就是一个 del.icio.us 客户端，因此你可以直接跳转到那里，不会影响示例的连贯性。

### 命令行：curl

这个示例略有不同：它完全不使用编程语言。
一个名为 `curl`（ http://curl.haxx.se/ ）的程序是一个功能强大的 HTTP 客户端，可在 Unix 或 Windows 命令行中运行。
它支持大多数 HTTP 方法、自定义首部、多种认证机制、代理、压缩以及许多其他特性。
你可以使用 `curl` 进行快速的单次 HTTP 请求，也可以将其与 Shell 脚本结合使用。

以下是 `curl` 的实际运行，获取用户的 del.icio.us 书签：

```bash
$ curl https://username:password@api.del.icio.us/v1/posts/recent
<?xml version='1.0' standalone='yes'?>
<posts tag="" user="username">
...
</posts>
```

### 其他语言

我没有足够的篇幅或专业知识来通过 del.icio.us 客户端示例深入介绍每种流行编程语言。
不过，我可以为许多我尚未涵盖的语言提供简要的 HTTP 客户端库指引。

**ActionScript**

Flash 应用与 JavaScript 应用一样，通常在 Web 浏览器中运行。
这意味着当你编写 ActionScript Web 服务客户端时，你很可能会使用 [第 11 章](ch11.md) 中描述的 Ajax 架构，而非本章展示的独立架构。

ActionScript 的 `XML` 类提供了类似于 JavaScript 的 `XmlHttpRequest` 的功能。
`XML.load` 方法获取一个 URI 并将响应文档解析为 XML 数据结构。
ActionScript 还提供了一个名为 `LoadVars` 的类，它处理表单编码的键值对而非 XML 文档。

**C**

C 语言的 `libwww` 库是第一个 HTTP 客户端库，但如今大多数 C 程序员使用 `libcurl`（ http://curl.haxx.se/libcurl/ ），它是 `curl` 命令行工具的基础。
之前我提到了 PHP 对 `libcurl` 的绑定，此外还有超过 30 种其他语言的绑定。
如果你不喜欢我的推荐，或者我在本章中没有提到你喜欢的编程语言，你可以考虑使用 `libcurl` 绑定。

**C++**

使用 `libcurl`，可以直接使用，也可以通过一个名为 `cURLpp`（ http://rrette.com/curlpp.html ）的面向对象包装器使用。

**Common Lisp**

`simple-http`（ http://www.enterpriselisp.com/software/simple-http/ ）易于使用，但除了基本的 HTTP GET 和 POST 之外不支持任何其他功能。
AllegroServe Web 服务器库（ http://opensource.franz.com/aserve/ ）包含一个完整的 HTTP 客户端库。

**Perl**

Perl 的标准 HTTP 库是 `libwww-perl`（也称为 LWP），可从 CPAN 或大多数 Unix 打包系统获得。
`libwww-perl` 历史悠久，是最受好评的 Perl 库之一。
要获得 HTTPS 支持，你还应安装 `Crypt::SSLeay` 模块（可从 CPAN 获得）。

<a id="processing-the-response"></a>
## 处理响应：XML 解析器

实体主体通常是 HTTP 响应中最重要的部分。
就 Web 服务而言，实体主体通常是一个 XML 文档，客户端通过 XML 解析器处理该文档来获取其所需的大部分信息。

现在，有很多 HTTP 客户端库，但它们都有完全相同的任务。
给定一个 URI、一组首部和一个主体文档，客户端的工作是构造一个 HTTP 请求并将其发送到特定的服务器。
有些库拥有比其他库更多的特性：cookie、认证、缓存以及我之前提到的其他特性。
但所有这些额外特性都是在 HTTP 请求内部实现的，通常作为额外的首部。
一个库可能提供面向对象的接口（如 Net::HTTP）或类似文件的接口（如 open-uri），但两种接口做的是同样的事。
只有一种 HTTP 客户端库。

但有三种 XML 解析器。
这不仅是因为某些 XML 解析器拥有其他解析器所缺乏的特性，或者某个接口比另一个更自然。
有两种基本的 XML 解析策略：基于文档的策略，如 DOM 和其他树型解析器；以及基于事件的策略，如 SAX 和 “拉” 式解析器。
你可以为任何编程语言获得树型或 SAX 解析器，以及几乎为任何语言获得拉式解析器。

基于文档的树型策略是三种模型中最简单的。
树型解析器将 XML 文档建模为嵌套的数据结构。
一旦你有了这个数据结构，你就可以使用 XPath 查询、CSS 选择器或自定义导航函数（取决于你的解析器支持什么）来搜索和处理它。
DOM 解析器是一种树型解析器，它实现了 W3C 定义的特定接口。

树型策略易于使用，也是我最常使用的。
使用树型解析器时，文档就像程序中的其他对象一样只是一个对象。
其最大的缺点是必须将文档作为一个整体来处理。
在将整个文档处理成树结构之前，你无法开始处理文档，而且也无法避免将整个文档加载到内存中。
对于简单但非常大的文档来说，这效率很低。
更好的做法是在标签被解析时逐个处理它们。

SAX 风格或拉式解析器将文档转换为事件流，而不是数据结构。
开始和结束标签、XML 注释和实体声明都是事件。

当你需要处理几乎所有事件时，拉式解析器非常有用。
拉式解析器允许你一次处理一个事件，根据需要从流中 “拉取” 下一个事件。
你可以随着事件的到来对单个事件做出响应，或构建数据结构供以后使用 —— 这个数据结构可能比树型解析器构建的要小。
你可以在任何时候停止解析文档，并通过从流中拉取下一个事件来稍后继续。

SAX 解析器更复杂，但当你只关心流式传入的众多事件中的少数几个时，它非常有用。
你通过向 SAX 解析器注册回调方法来驱动它。
一旦定义完回调，你就将解析器释放到文档上。
解析器将文档转换为一系列事件，并处理文档中的每个事件而不停止。
当出现与你的某个回调匹配的事件时，解析器触发该回调，你的自定义代码运行。
一旦回调完成，SAX 解析器继续处理事件而不停止。

基于文档的方法的优势在于它提供了对文档内容的随机访问。
使用基于事件的解析器时，一旦事件被触发，它们就消失了。
如果你想再次触发它们，你需要重新解析文档。
此外，基于事件的解析器在尝试解析错误位置并崩溃之前，不会注意到格式不正确的 XML 文档有问题。
在将文档传入基于事件的解析器之前，你需要确保文档格式良好，否则就要接受这样一个事实：你的回调方法可能会为一个最终证明不合格的文档而被触发。

一些编程语言自带一套标准的 XML 解析器。
另一些则有规范的第三方解析器库。
出于性能考虑，一些语言也有用 C 编写的快速解析器的绑定。
我现在想再次浏览一下这些语言，并为基于文档和基于事件的 XML 解析器提供建议。
我将根据速度、接口质量、对 XPath 的支持程度（对于树型解析器）、严格程度以及是否支持基于 schema 的验证来评价常用的解析器。
根据应用程序的不同，严格的解析器可能是好事（因为 XML 文档将以正确的方式被解析，否则根本无法解析），
也可能是坏事（因为你想要使用生成糟糕 XML 的服务）。

在上述 del.icio.us 客户端示例中，我不仅展示了如何使用每种语言中我最喜欢的 HTTP 客户端库，还展示了如何使用每种语言中我最喜欢的树型解析器。
为了展示基于事件的解析器是如何工作的，我将再给出两个使用 Ruby 内置 SAX 和拉式解析器的 del.icio.us 客户端示例。

### Ruby：REXML，我猜

Ruby 自带一个标准的 XML 解析器库 REXML，它同时支持 DOM 和 SAX 接口，并且有良好的 XPath 支持。
不幸的是，REXML 的内部实现使其处于一个奇怪的中间地带：它过于严格，不能用于解析糟糕的 XML，但又不够严格，无法拒绝所有糟糕的 XML。

我在本书中通篇使用 REXML，因为它是默认选择，而且我只处理格式良好的 XML。
如果你想确保只处理格式良好的 XML，你需要安装 GNOME 项目的 libxml2 库的 Ruby 绑定（见本章后面的 “其他语言”）。

如果你希望能够处理糟糕的标记，最好的选择是 hpricot（ http://code.whytheluckystiff.net/hpricot/ ），可作为 hpricot gem 获得。
它很快（使用 C 扩展），并且具有直观的接口，包括对常见 XPath 表达式的支持。

示例 2-9 是使用 REXML 的 SAX 接口实现的 del.icio.us 客户端。

*示例 2-9. 使用 SAX 解析器的 Ruby 客户端*

```ruby
#!/usr/bin/ruby -w
# delicious-sax.rb
require 'open-uri'
require 'rexml/parsers/sax2parser'

def print_my_recent_bookmarks(username, password)
  # 发起 HTTPS 请求并将实体主体读取为 XML 文档
  xml = open('https://api.del.icio.us/v1/posts/recent',
             :http_basic_authentication => [username, password])
  
  # 创建一个 SAX 解析器，其使命是解析 XML 实体主体
  parser = REXML::Parsers::SAX2Parser.new(xml)
  
  # 当 SAX 解析器遇到 'post' 标签时...
  parser.listen(:start_element, ["post"]) do |uri, tag, fqtag, attributes|
    # ...它应该打印该标签的信息
    puts "#{attributes['description']}: #{attributes['href']}"
  end
  
  # 让解析器完成其解析 XML 实体主体的使命
  parser.parse
end

# 主程序
username, password = ARGV
unless username and password
  puts "Usage: #{$0} [USERNAME] [PASSWORD]"
  exit
end
print_my_recent_bookmarks(username, password)
```

在这个程序中，数据在调用 `SAXParser#parse` 之前不会被解析（甚至不会从 HTTP 连接中读取）。
在此之前，我可以自由调用 `listen` 并设置代码块以响应解析器事件。
在本例中，我唯一感兴趣的事件是 `post` 标签的开始。
每当解析器找到一个 `post` 标签时，我的代码块就会被调用。
这与使用树型解析器解析 XML 文档，并对对象树运行 XPath 表达式“//post”的效果相同。
我的代码块做了什么？
与其他示例程序找到 `post` 标签时的操作相同：打印 `description` 和 `href` 属性的值。

这种实现比等效的树型实现更快，且内存效率更高。
然而，复杂的基于 SAX 的程序比等效的树型程序更难编写。
拉式解析器是一个很好的折中方案。
示例 2-10 展示了一个使用 REXML 拉式解析器接口的客户端实现。

*示例 2-10. 使用 REXML 拉式解析器的 del.icio.us 客户端*

```ruby
#!/usr/bin/ruby -w
# delicious-pull.rb
require 'open-uri'
require 'rexml/parsers/pullparser'

def print_my_recent_bookmarks(username, password)
  # 发起 HTTPS 请求并将实体主体读取为 XML 文档
  xml = open('https://api.del.icio.us/v1/posts/recent',
             :http_basic_authentication => [username, password])
  
  # 将 XML 实体主体送入拉式解析器
  parser = REXML::Parsers::PullParser.new(xml)
  
  # 直到没有更多事件可拉取...
  while parser.has_next?
    # ...拉取下一个事件
    tag = parser.pull
    
    # 如果是 'post' 标签...
    if tag.start_element?
      if tag[0] == 'post'
        # 打印书签信息
        attrs = tag[1]
        puts "#{attrs['description']}: #{attrs['href']}"
      end
    end
  end
end

# 主程序
username, password = ARGV
unless username and password
  puts "Usage: #{$0} [USERNAME] [PASSWORD]"
  exit
end
print_my_recent_bookmarks(username, password)
```

### Python：ElementTree

Python 的 XML 解析器种类繁多。
仅在 Python 2.5 标准库中就有七种不同的 XML 接口。
有关完整详情，请参阅 Python 库参考（ http://docs.python.org/lib/markup.html ）。

对于树型解析，最好的库是 ElementTree（ http://effbot.org/zone/element-index.htm ）。
它速度快，接口合理，从 Python 2.5 起无需安装任何东西，因为它已在标准库中。
缺点是它对 XPath 的支持仅限于简单表达式——当然，标准库中其他任何东西根本不支持 XPath。
如果你需要完整的 XPath 支持，可以尝试 4Suite（ http://4suite.org/ ）。

Beautiful Soup（ http://www.crummy.com/software/BeautifulSoup/ ）是一个速度较慢的树型解析器，对无效 XML 非常宽容，并提供对文档的编程接口。
它还自动处理大多数字符集转换，让你能够处理 Unicode 数据。

对于 SAX 风格解析，最佳选择是标准库中的 `xml.sax` 模块。
PyXML（ http://pyxml.sourceforge.net/ ）套件包含一个拉式解析器。

### Java：javax.xml、Xerces 或 XMLPull

Java 1.5 包含了由 Apache Xerces 项目编写的 XML 解析器。
核心类位于 `javax.xml.*` 包中（例如 `javax.xml.xpath`）。
DOM 接口位于 `org.w3c.dom.*`，SAX 接口位于 `org.xml.sax.*`。
如果你使用较早版本的 Java，你可以自行安装 Xerces，并利用与 Java 1.5 中相同的接口（ http://xerces.apache.org/xerces2-j/ ）。

有多种适用于 Java 的拉式解析器。
Sun 的 Web Services Developer Pack 在 `javax.xml.stream` 包中包含一个拉式解析器。

对于解析糟糕的 XML，你可以尝试 TagSoup（ http://home.ccil.org/~cowan/XML/tagsoup/ ）。

### C#：System.Xml.XmlReader

.NET 公共语言运行时带有一个拉式解析器接口，与更典型（也更复杂）的 SAX 风格接口形成对比。
你也可以使用 `XmlDocument` 创建完整的 W3C DOM 树。`XPathDocument` 类允许你遍历树中与 XPath 表达式匹配的节点。

如果你需要处理损坏的 XML 文档，请查看 Chris Lovett 的 SgmlReader，网址为 http://www.gotdotnet.com/Community/UserSamples/ 。

### PHP

你可以使用 `xml_parser_create` 函数创建 SAX 风格解析器，以及使用 `XMLReader` 扩展创建拉式解析器。
DOM PHP 扩展（包含在 PHP 5 中）为 GNOME 项目的 libxml2 C 库提供了树型接口。
你可能更轻松地使用 SimpleXML，这是一个树型解析器，不是官方的 DOM 实现。
这就是我在示例 2-8 中使用的。

还有一个纯 PHP DOM 解析器，名为 DOMIT!（ http://sourceforge.net/projects/domit-xmlparser ）。

### JavaScript：responseXML

如果你正在使用 `XMLHttpRequest` 编写 Ajax 客户端，你完全不必担心 XML 解析器。
如果你发起请求且响应实体主体为 XML 格式，Web 浏览器会使用其自身的树型解析器进行解析，并通过 `XMLHttpRequest` 对象的 `responseXML` 属性使其可用。
你使用 JavaScript DOM 方法来操作此文档：与用于操作浏览器中显示的 HTML 文档的方法相同。
[第 11 章](ch11.md) 有更多关于如何使用 `responseXML` 的信息，以及如何使用 `responseData` 成员处理非 XML 文档。

有一个第三方 XML 解析器，XML for \<SCRIPT>（ http://xmljs.sourceforge.net/ ），它独立于客户端 Web 浏览器内置的解析器工作。
“XML for \<SCRIPT>” 提供 DOM 和 SAX 接口，并支持 XPath 查询。

### 其他语言

**ActionScript**

当你使用 `XML.load` 加载 URI 时，它会被自动解析为一个 XML 对象，该对象暴露树型接口。

**C**

Expat（http://expat.sourceforge.net/）是最流行的 SAX 风格解析器。GNOME 项目的 libxml2（http://xmlsoft.org/）包含 DOM、拉式和 SAX 解析器。

**C++**

你可以使用上述任意一种 C 解析器，或面向对象的 Xerces-C++ 解析器（ http://xml.apache.org/xerces-c/ ）。
与 Java 版本的 Xerces 一样，Xerces-C++ 同时暴露 DOM 和 SAX 接口。

**Common Lisp**

使用 SXML（ http://common-lisp.net/project/s-xml/ ）。
它暴露类似 SAX 的接口，并且还可以将 XML 文档转换为树状的 S 表达式或 Lisp 数据结构。

**Perl**

与 Python 类似，Perl 有多种 XML 解析器。
它们都可以在 CPAN 上获得。
`XML::XPath` 支持 XPath，`XML::Simple` 将 XML 文档转换为标准的 Perl 数据结构。
对于 SAX 风格解析，使用 `XML::SAX::PurePerl`。
对于拉式解析，使用 `XML::LibXML::Reader`。
Perl XML FAQ（ http://perl-xml.sourceforge.net/faq/ ）概述了最流行的 Perl XML 库。

<a id="json-parsers"></a>
## JSON 解析器：处理序列化数据

大多数 Web 服务返回 XML 文档，但越来越多的服务返回序列化为 JSON 格式字符串的简单数据结构（数字、数组、哈希等）。
JSON 通常由预期被 Ajax 应用客户端消费的服务生成。
其理念是，浏览器从 JSON 数据结构获取 JavaScript 数据结构比从 XML 文档容易得多。
每个 Web 浏览器对其 XML 解析器提供略有不同的 JavaScript 接口，
但 JSON 字符串只不过是一个严格受限的 JavaScript 程序，因此它在每个浏览器中的工作方式相同。

当然，JSON 不绑定于 JavaScript，就像 JavaScript 不绑定于 Java 一样。
JSON 为基于 XML 的数据序列化方法（如 XML Schema）提供了一种轻量级替代方案。
JSON 网站（ http://www.json.org/ ）提供了多种语言的实现链接，我推荐你去该网站，而不是为每种语言提及一个 JSON 库。

JSON 是一种简单且与语言无关的方式，用于将编程语言数据结构（数字、数组、哈希等）格式化为字符串。
示例 2-11 是一个简单数据结构的 JSON 表示：一个混合类型数组。

*示例 2-11. JSON 格式的混合类型数组*

```json
[3, "three"]
```

相比之下，示例 2-12 是相同数据的一种可能的 XML 表示。

*示例 2-12. XML-RPC 格式的混合类型数组* <a id="example-2-12"></a>

```xml
<value>
  <array>
    <data>
      <value><i4>3</i4></value>
      <value><string>three</string></value>
    </data>
  </array>
</value>
```

由于 JSON 字符串只不过是一个严格受限的 JavaScript 程序，你可以通过对字符串调用 `eval` 来 “解析” JSON。
这非常快，但除非你控制提供 JSON 的 Web 服务，否则你不应该这样做。
未经测试或不受信任的 Web 服务可能向客户端发送有缺陷或恶意的 JavaScript 程序，而不是真正的 JSON 结构。
对于 [第 11 章](ch11.md) 中的 JavaScript 示例，我使用一个用 JavaScript 编写的 JSON 解析器，可从 json.org 获得（见示例 2-13）。

*示例 2-13. JavaScript 中的 JSON 演示*

```html
<!-- json-demo.html -->
<!-- 在实际应用中，你应该将 json.js 保存在本地，而不是每次都从 json.org 获取 -->
<script type="text/javascript" src="http://www.json.org/json.js">
</script>
<script type="text/javascript">
array = [3, "three"]
alert("Converted array into JSON string: '" + array.toJSONString() + "'")

json = "[4, \"four\"]"
alert("Converted JSON '" + json + "' into array:")
array2 = json.parseJSON()
for (i=0; i < array2.length; i++)
{
    alert("Element #" + i + " is " + array2[i])
}
</script>
```

Dojo JavaScript 框架在 `dojo.json` 包中有一个 JSON 库，因此如果你使用 Dojo，则无需安装任何额外的东西。
ECMAScript 标准的未来版本可能会将 JSON 序列化和反序列化方法定义为 JavaScript 语言的一部分，从而使第三方库过时。

在本书的 Ruby 示例中，我将使用来自 `json` Ruby gem 的 JSON 解析器。
两个最重要的方法是 `Object#to_json` 和 `JSON.parse`。
尝试通过 `irb` 解释器运行示例 2-14 中的 Ruby 代码。

*示例 2-14. Ruby 中的 JSON 演示*

```ruby
# json-demo.rb
require 'rubygems'
require 'json'

[3, "three"].to_json # => "[3,\"three\"]"
JSON.parse('[4, "four"]') # => [4, "four"]
```

目前，Yahoo! Web Services 是最受欢迎的提供 JSON 的公共 Web 服务（ http://developer.yahoo.com/common/json.html ）。
示例 2-15 展示了一个用 Ruby 编写的命令行程序，它使用 Yahoo! News Web 服务获取当前新闻故事的 JSON 表示。

*示例 2-15. 使用 Yahoo! Web 服务搜索 Web（JSON 版）*

```ruby
#!/usr/bin/ruby
# yahoo-web-search-json.rb
require 'rubygems'
require 'json'
require 'open-uri'
$KCODE = 'UTF8'

# 搜索 Web 上的某个术语，并打印匹配网页的标题
def search(term)
  base_uri = 'http://api.search.yahoo.com/NewsSearchService/V1/newsSearch'
  
  # 发起 HTTP 请求并将响应实体主体读取为 JSON 文档
  json = open(base_uri + "?appid=restbook&output=json&query=#{term}").read
  
  # 将 JSON 文档解析为 Ruby 数据结构
  json = JSON.parse(json)
  
  # 遍历数据结构...
  json['ResultSet']['Result'].each do
    # ...并打印每个网页的标题
    |r| puts r['Title']
  end
end

# 主程序
unless ARGV[0]
  puts "Usage: #{$0} [search term]"
  exit
end
search(ARGV[0])
```

将此与 [示例 2-1](#example-2-1) 中的程序 `yahoo-web-search.rb` 进行比较。
该程序具有相同的基本结构，但工作方式不同。
它请求格式为 XML 的搜索结果，解析 XML，并使用 XPath 查询来提取结果标题。
此程序将 JSON 数据结构解析为原生语言数据结构（一个哈希），并使用原生语言操作符而非 XPath 进行遍历。

如果 JSON 如此简单，为什么不对所有内容使用它呢？
你可以这样做，但我不推荐。
JSON 适合表示一般的数据结构，而 Web 主要提供文档：不规则的、自描述的数据结构，它们相互链接。
XML 和 HTML 专门用于表示文档。
网页的 JSON 表示会难以阅读，就像 [示例 2-12](#example-2-12) 中数组的 XML 表示难以阅读一样。
当你需要描述不适合文档范例的数据结构时，JSON 很有用：例如，一个简单列表或一个哈希。

<a id="clients-with-wadl"></a>
## 使用 WADL 轻松开发客户端

到目前为止，我展示的代码使用了多种语言，但它们始终遵循相同的三步模式。
要调用 Web 服务，我构建 HTTP 请求的各个元素（方法、URI、首部和实体主体）。
我使用 HTTP 库将这些数据转换为真实的 HTTP 请求，并由库将该请求发送到相应的服务器。
然后我使用 XML 解析器将响应解析为数据结构或事件流。
一旦发起请求，我就可以随意使用响应数据。
在这方面，所有 RESTful Web 服务和大多数混合服务都是相同的。
此外，正如我将在接下来的章节中展示的，所有 RESTful Web 服务都以相同的方式使用 HTTP：HTTP 具有所谓的统一接口。

我能否利用这种相似性？
将这种模式抽象为一个通用的 “REST 库”，使其能够访问任何支持统一接口的 Web 服务？
这是有先例的。
Web 服务描述语言（WSDL）以足够的细节描述 RPC 风格 Web 服务之间的差异，使得通用库在给定适当的 WSDL 文件的情况下，可以访问任何 RPC 风格 SOAP 服务。

<ins>对于 RESTful 和混合服务，我推荐使用 Web 应用描述语言（WADL）。
WADL 文件描述你可以合法地对服务发起的 HTTP 请求：你可以访问哪些 URI，这些 URI 期望你发送哪些数据，以及它们返回哪些数据。
WADL 库可以解析此文件，并将可能的服务请求空间建模为原生语言 API。</ins>

我将在 [第 9 章](ch9.md) 中更详细地描述 WADL，但这里先简单了解一下。
示例 2-16 中显示的 del.icio.us 客户端等同于 [示例 2-4](#example-2-4) 中的 Ruby 客户端，但它使用 Ruby 的 WADL 库和我为 del.icio.us 创建的一个非官方 WADL 文件
（我将在 [第 8 章](ch8.md) 中展示该 WADL 文件）。

*示例 2-16. 用于 del.icio.us 的 Ruby/WADL 客户端*

```ruby
#!/usr/bin/ruby
# delicious-wadl-ruby.rb
require 'wadl'

if ARGV.size != 2
  puts "Usage: #{$0} [username] [password]"
  exit
end

username, password = ARGV

# 从 WADL 文件加载应用
delicious = WADL::Application.from_wadl(open("delicious.wadl"))

# 向应用提供认证信息
service = delicious.v1.with_basic_auth(username, password)

begin
  # 查找“最近书签”功能
  recent_posts = service.posts.recent
  
  # 对于每个最近书签...
  recent_posts.get.representation.each_by_param('post') do |post|
    # 打印其描述和 URI
    puts "#{post.attributes['description']}: #{post.attributes['href']}"
  end
rescue WADL::Faults::AuthorizationRequired
  puts "Invalid authentication information!"
end
```

在后台，这段代码发出的 HTTP 请求与本章中看到的其他 del.icio.us 客户端完全相同。
细节隐藏在 `delicious.wadl` WADL 文件中，该文件由 `WADL::Application.from_WADL` 内部的 WADL 客户端库解释。
这段代码不容易被识别为 Web 服务客户端。
这是一件好事：这意味着库正在完成其工作。
然而，当我们在 [第 9 章](ch9.md) 中回到这段代码时，你将看到它遵循了与那些自行发起 HTTP 请求的示例同样多的 REST 原则。
WADL 抽象了 HTTP 的细节，但没有抽象底层的 RESTful 接口。

截至撰写本文时，WADL 的采用率非常低。
如果你想要为某个服务使用 WADL 客户端，而不是编写特定语言的客户端，你可能需要自己编写 WADL 文件。
为别人的服务编写一个非官方的 WADL 文件并不困难：我已经为 del.icio.us 和其他几个服务这样做过。
你甚至可以编写一个 WADL 文件，让你能够将原本为人类使用而设计的 Web 应用作为 Web 服务使用。
WADL 旨在描述 RESTful Web 服务，但它几乎可以描述 Web 上的任何内容。

一个名为 ActiveResource 的 Ruby 库采用了不同的策略。
它只适用于特定类型的 Web 服务，但它通过一个简单的面向对象接口隐藏了 RESTful HTTP 访问的细节。
我将在下一章中介绍 ActiveResource，在介绍一些 REST 术语之后。
