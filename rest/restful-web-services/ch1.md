# 第 1 章 可编程 Web 及其居民

当你编写计算机程序时，你并不局限于自己能想到的算法。
你所用语言的标准库会提供一些算法。
你可以从书籍或在线找到的第三方库中获得更多。
只有当你处于非常前沿的领域时，才需要自己提出算法。

如果幸运的话，数据也是如此。
有些应用完全由用户输入的数据驱动。
有时数据会自然而然地来到你面前：如果你在分析垃圾邮件，你应该不难获得所需的一切。
你可以下载一些公共数据集 ——单词列表、地理数据、素数列表、公共领域文本—— 就像它们是第三方库一样。
但如果你需要其他类型的数据，情况就不那么乐观了。
数据从何而来？越来越多地，它来自可编程 Web。

当你——作为一个人类 —— 想要查找某主题的书籍时，你可能会将 Web 浏览器指向在线图书馆或书店的 URI，例如 http://www.amazon.com/。

💡 Web 上某个东西的地址的常用术语是 “URL”。
我在本书中通篇使用 “URI”，因为这是 HTTP 标准所规定的。
Web 上的每个 URI 同时也是 URL，所以你可以将我说 “URI” 的地方替换为 “URL”，不会损失任何意义。

你会收到一个网页，即 HTML 格式的文档，由浏览器以图形方式渲染。
你在页面上视觉扫描搜索表单，在文本框中输入你的主题（例如 “web services”），然后提交表单。
此时你的 Web 浏览器发起第二个 HTTP 请求，指向一个包含你主题的 URI。
继续以 Amazon 为例，你的浏览器请求的第二个 URI 类似于 http://amazon.com/s?url=search-alias%3Dstripbooks&field-keywords=web+services 。

amazon.com 的 Web 服务器响应并返回第二个 HTML 格式的文档。
该文档包含你的搜索结果描述、指向其他搜索选项的链接，以及各种商业促销内容（见示例 1-1）。
你的浏览器再次以图形方式渲染文档，你查看后决定下一步操作。

*示例 1-1. amazon.com 返回的部分 HTML 响应*

```html
...
</a>
<a href="http://www.amazon.com/Restful-Web-Services-Leonard-Richardson/dp/...>
<span class="srTitle">RESTful Web Services</span>
by Leonard Richardson and Sam Ruby
<span class="bindingBlock">
(<span class="binding">Paperback</span> - May 1, 2007)
</span>
```

你日常使用的 Web 充满了数据：书籍信息、评论、价格、到达时间、消息、照片和各种杂项内容。
它也充满了服务：搜索引擎、在线商店、博客、Wiki、计算器和游戏。
与其将这些数据和程序全部安装在自己的计算机上，你只需安装一个程序 ——Web 浏览器—— 并通过它访问这些数据和服务。

可编程 Web 也是如此。
主要区别在于，可编程 Web 通常提供的是赤裸裸的、粗糙的 XML 文档，而非带有横幅广告和可爱柔和徽标的精美 HTML 页面。
可编程 Web 不一定是为人类消费而设计的。
它的数据是作为软件程序的输入，以完成某些令人惊叹的任务。

示例 1-2 展示了一个使用可编程 Web 来完成传统人工 Web 任务的 Ruby 脚本：查找与关键词匹配的书籍标题。
它通过 Ruby/Amazon 库（ http://www.caliban.org/ruby/ruby-amazon.shtml ）将 Web 访问隐藏在编程语言接口之下。

*示例 1-2. 使用 Ruby 脚本搜索书籍* <a id="example-1-2"></a>

```ruby
#!/usr/bin/ruby -w
# amazon-book-search.rb
require 'amazon/search'

if ARGV.size != 2
  exit
  puts "Usage: #{$0} [Amazon Web Services AccessKey ID] [text to search for]"
end

access_key, search_request = ARGV
req = Amazon::Search::Request.new(access_key)

# 遍历搜索结果中的每一本书
req.keyword_search(search_request, 'books', Amazon::Search::LIGHT) do |book|
  # 打印书名和作者列表
  puts %{"#{book.product_name}" by #{book.authors.join(', ')}}
end
```

要运行此程序，你需要注册一个 Amazon Web Services 账户（http://aws.amazon.com/），并使用你的 Access Key ID 自定义 Ruby 代码。
以下是该程序的一次示例运行：

```bash
$ ruby amazon-search.rb C1D4NQS41IMK2 "restful web services"
"RESTful Web Services" by Leonard Richardson, Sam Ruby
"Hacking with Ruby: Ruby and Rails for the Real World" by Mark Watson
```

在最好的情况下，可编程 Web 的工作方式与人类 Web 相同。
当 `amazon-book-search.rb` 调用 `Amazon::Search::Request#keyword_search` 方法时，Ruby 程序开始像 Web 浏览器一样运作。
它向一个 URI 发起 HTTP 请求：在此例中，类似于 `http://xml.amazon.com/onca/xml3?KeywordSearch=restful+web+services&mode=books&f=xml&type=lite&page=1` 。
`xml.amazon.com` 的 Web 服务器响应并返回一个 XML 文档。
该文档（见示例 1-3）描述了搜索结果，就像你在 Web 浏览器中看到的 HTML 文档一样，但形式更加结构化。

*示例 1-3. xml.amazon.com 返回的部分 XML 响应*

```xml
...
<ProductName>RESTful Web Services</ProductName>
<Catalog>Book</Catalog>
<Authors>
  <Author>Leonard Richardson</Author>
  <Author>Sam Ruby</Author>
</Authors>
<ReleaseDate>01 May, 2007</ReleaseDate>
...
```

一旦 Web 浏览器提交了 HTTP 请求，它的任务就相当简单。
它只需要以人类可理解的方式呈现响应。
它不需要弄清楚 HTTP 响应的含义：那是人类的工作。
Web 服务客户端则没有这种奢侈。
它是预先编程的，因此它必须既是获取数据的 Web 浏览器，又是决定数据含义的 “人类”。
Web 服务客户端必须自动从 HTTP 响应中提取含义，并基于该含义做出决策。

在 [示例 1-2](#example-1-2) 中，Web 服务客户端解析 XML 文档，提取一些有趣的信息（书名和作者），并将这些信息打印到标准输出。
程序 `amazon-book-search.rb` 实际上是一个小型、专用的 Web 浏览器，将数据中继给人类读者。
它完全可以对 Amazon 图书数据做其他事情，一些完全不依赖人工干预的事情：比如将书名存入数据库，或者使用作者信息来驱动推荐引擎。

而且数据也不必总是流向客户端。
就像你可以将人类 Web 的某些部分按自己的意愿弯曲一样（通过在博客上发帖或购买书籍），你也可以编写客户端来修改可编程 Web。
你可以将其用作存储空间，或作为另一个你无需自己编写的算法来源。
这取决于你需要什么服务，以及你是否能找到其他人来提供它。

*示例 1-4. 使用 s3sh 和 S3 操作可编程 Web*

```bash
$ s3sh
>> Service.buckets.collect { |b| b.name }
=> ["example.com"]
>> my_bucket = Bucket.find("example.com")
>> contents = open("disk_file.txt").read
=> "This text is the contents of the file disk_file.txt"
>> S3Object.store("mydir/mydocument.txt", contents, my_bucket.name)
>> my_bucket['directory/document.txt'].value
=> "This text is the contents of the file disk_file.txt"
```

这是一个修改可编程 Web 的 Web 服务客户端示例：用于 Ruby 的 `s3sh` 命令 shell（ http://amazon.rubyforge.org/ ）。
它是众多针对 Amazon 另一个 Web 服务 —— S3（Simple Storage Service，简单存储服务，http://aws.amazon.com/s3 ）编写的客户端之一。
在 [第 3 章](./ch3.md) 中我将详细介绍 S3 的工作原理，所以如果你有兴趣自己使用 `s3sh`，可以在那里进一步了解。

要理解这段 `s3sh` 的交互记录，你只需要知道 Amazon S3 允许其客户端在带标签的容器（“buckets”）中存储带标签的数据片段（“objects”）。
`s3sh` 程序在 S3 之上构建了一个交互式编程接口。
其他客户端则使用 S3 作为备份工具或 Web 主机。
这是一个非常灵活的服务。

在本章中，我调查了可编程 Web 的当前状态。
正在使用哪些技术？它们被用来实现哪些架构？哪些设计风格最流行？
我将展示一些真实的代码和真实的 HTTP 会话，
但本章的主要目标是让你开始将万维网视为一种将计算机程序相互连接的方式，其条件与它将人类相互连接的方式相同。

## 可编程 Web 上的事物类型

可编程 Web 基于 HTTP 和 XML。
它的一些部分提供 HTML、JavaScript Object Notation（JSON）、纯文本或二进制文档，但大多数部分使用 XML。
而且这一切都基于 HTTP：如果不使用 HTTP，你就不在 Web 上。⁑
除了这一小片共识之外，剩下的几乎只有争议。
术语尚未固定，不同的人以各种方式使用常见术语（如本书主题 “REST”），这些方式混合成一种模糊而混乱的杂烩。
所缺少的是一种对可编程 Web 进行分类的连贯方法。
有了它，各个术语的含义就会变得清晰。

⁑ 感谢 Big Web Services 的 WS-Addressing 标准，现在可以创建不在 Web 上的 Web 服务了：即使用电子邮件或 TCP 而非 HTTP 作为传输协议的服务。我并不认为所有东西都必须要在 Web 上，但似乎确实应该给这种奇特的景象起个 Web 服务之外的名字。这点其实并不重要，因为在实践中几乎所有人都使用 HTTP。因此才放在脚注里。我所知道的唯一例外是 eBay 的 Web 服务，它可以通过电子邮件和 HTTP 两种方式向你发送 SOAP 文档。

将可编程 Web 想象成一个生态系统，如同海洋，其中栖息着各种奇特的生物。
古代的科学家和水手根据生物的表面外观对海洋生物进行分类：鲸鱼被归入鱼类。
现代科学家则根据生物在生命演化树中的位置来分类：鲸鱼如今与其他哺乳动物归为一类。
对栖息在可编程 Web 中的服务进行分类，也有两种类似的方式：按它们所使用的技术（URI、SOAP、XML-RPC 等）分类，或按底层的架构和设计理念分类。

通常，这两种海洋生物分类系统是相安无事的。
你不需要做 DNA 检测就知道金枪鱼比海葵更像石斑鱼。
但如果你真的想理解为什么鲸鱼不能在水下呼吸，你就需要停止将它们归类为鱼（按表面外观），而开始将它们归类为哺乳动物（按底层架构）。† 

† Melville 在《白鲸》第 22 章（“鲸类学”）中花了大量篇幅论证鲸鱼是鱼。
这听起来很傻，但他并不是否认鲸鱼有肺且分泌乳汁；他是在主张基于外貌来定义 “鱼”，而非 Linnaeus 的 “基于自然法则”（ex lege naturae）的定义。

当涉及到对可编程 Web 进行分类时，当今大多数术语都是根据服务的表面外观 ——即它们所使用的技术—— 来分类的。
这些分类在大多数情况下是有效的，但它们在概念上有所欠缺，并会导致类似 “鲸鱼是鱼” 的错误。
<ins>我将呈现一个基于架构的分类法，展示技术选择是如何从底层设计原则中衍生出来的。
我将揭示一些后续全书都会反复提及的划分，但我的主要目的是聚焦于可编程 Web 中那些可以合理地与 “REST” 一词相关联的部分。</ins>

## HTTP：信封中的文档

如果我要对海洋动物进行分类，我会从它们共同的特性开始：DNA、细胞结构、胚胎发育规律。
然后我会展示动物如何通过偏离共性来实现彼此的特化。
要对可编程 Web 进行分类，我想先从 HTTP 的概述开始，这是所有 Web 服务共同的协议。

HTTP 是一种基于文档的协议，客户端将文档放入信封中发送给服务器。
服务器则以将响应文档放入信封中发回客户端作为回馈。
HTTP 对信封的格式有严格的标准，但对于信封内部的内容则不太在意。
示例 1-5 展示了一个示例信封：当我访问 oreilly.com 首页时，我的 Web 浏览器发送的 HTTP 请求。
为了适应印刷页面，我截断了两行。

*示例 1-5. 对 http://www.oreilly.com/index.html 的 HTTP GET 请求* <a id="example-1-5"></a>

```
GET /index.html HTTP/1.1
Host: www.oreilly.com
User-Agent: Mozilla/5.0 (X11; U; Linux i686; en-US; rv:1.7.12)...
Accept: text/xml,application/xml,application/xhtml+xml,text/html;q=0.9,...
Accept-Language: us,en;q=0.5
Accept-Encoding: gzip,deflate
Accept-Charset: ISO-8859-15,utf-8;q=0.7,*;q=0.7
Keep-Alive: 300
Connection: keep-alive
```

如果你不熟悉 HTTP，现在正是指出 HTTP 请求主要部分的好时机。
我在全书都会使用这些术语。

* HTTP 方法

  在此请求中，方法是“GET”。在其他关于 REST 的讨论中，你可能会看到它被称为 “HTTP 动词” 或 “HTTP 动作”。

  HTTP 方法的名称类似于编程语言中的方法名：它指示客户端期望服务器如何处理此信封。
  在此情况下，客户端（我的 Web 浏览器）正试图从服务器（ www.oreilly.com ）GET（获取）一些信息。

* 路径

  这是主机名右侧的 URI 部分：此处，`http://www.oreilly.com/index.html` 变成了 “/index.html”。
  用信封的比喻来说，路径就是信封上的地址。在本书中，我有时会用 “URI” 作为路径的简写。

* 请求首部

  这些是元数据片段：键值对，就像贴在信封上的信息标签。
  此请求有八个首部：Host、User-Agent、Accept 等。
  有一套标准的 HTTP 首部列表（参见 [附录 C](./appendix-c.md) ），应用程序也可以定义自己的首部。

* 实体主体（entity-body），也称为文档或表述

  这是信封内的文档。
  此特定请求没有实体主体，这意味着信封是空的！
  这对于 GET 请求来说是典型的，因为完成请求所需的所有信息都在路径和首部中。

HTTP 响应也是信封中的文档。
它的形式与 HTTP 请求几乎相同。
示例 1-6 展示了当我发起 [示例 1-5](#example-1-5) 中的请求时，oreilly.com 的服务器发送给我的 Web 浏览器的响应（已修剪）。

*示例 1-6. 对 http://www.oreilly.com/index.html 的 HTTP GET 请求的响应*

```
HTTP/1.1 200 OK
Date: Fri, 17 Nov 2006 15:36:32 GMT
Server: Apache
Last-Modified: Fri, 17 Nov 2006 09:05:32 GMT
Etag: "7359b7-a7fa-455d8264"
Accept-Ranges: bytes
Content-Length: 43302
Content-Type: text/html
X-Cache: MISS from www.oreilly.com
Keep-Alive: timeout=15, max=1000
Connection: Keep-Alive

<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
...
<title>oreilly.com -- Welcome to O'Reilly Media, Inc.</title>
...
```

响应可分为三部分：

* HTTP 响应码

  这个数字代码告诉客户端其请求是成功还是失败，以及客户端应如何看待此信封及其内容。
  在此例中，GET 操作必定成功了，因为响应码是 200（“OK”）。
  我在 [附录 B](./appendix-b.md) 中描述了 HTTP 响应码。

* 响应首部

  与请求首部一样，这些是贴在信封上的信息标签。
  此响应有 11 个首部：Date、Server 等。

* 实体主体（entity-body）或表述

  同样，这是信封内的文档，而这次确实有一个！
  实体主体是对我 GET 请求的满足。
  响应的其余部分只是一个带有标签的信封，告诉 Web 浏览器如何处理该文档。

  这些标签中最重要的一个值得单独提及。
  响应首部 `Content-Type` 给出了实体主体的媒体类型。
  在此例中，媒体类型是 `text/html`。
  这让我的 Web 浏览器知道它可以将实体主体渲染为 HTML 文档：一个网页。

  有一套标准的媒体类型列表（ http://www.iana.org/assignments/media-types/ ）。
  最常见的媒体类型包括文本文档（`text/html`）、结构化数据文档（`application/xml`）和图像（`image/jpeg`）。
  在其他关于 REST 或 HTTP 的讨论中，你可能会看到媒体类型被称为 “MIME type”、“内容类型” 或 “数据类型”。

## 方法信息

HTTP 是可编程 Web 上所有 “动物” 唯一的共同点。
现在我将向你展示 Web 服务如何相互区分。
当今的 Web 服务对两个大问题有不同的回答。
如果你知道一个 Web 服务如何回答这些问题，你就能很好地了解它与 Web 的协同工作程度。

第一个问题是客户端如何向服务器传达其意图。
服务器如何知道某个请求是检索某些数据的请求，而不是删除相同数据或用不同数据覆盖它的请求？
为什么服务器应该做这件事而不是那件事？

<ins>我把关于如何处理数据的信息称为 *方法信息*（method information）。
在 Web 服务中传达方法信息的一种方式是将其放在 HTTP 方法中。
由于这是 RESTful Web 服务的方式，我将在后面详细讨论。
现在，请注意五种最常见的 HTTP 方法是 GET、HEAD、PUT、DELETE 和 POST。
这足以区分 “检索某些数据”（GET）、“删除相同数据”（DELETE）和 “用不同数据覆盖它”（PUT）。</ins>

HTTP 方法名称的一大优势在于它们是标准化的。
当然，HTTP 方法名称的空间比编程语言中方法名称的空间要有限得多。
一些 Web 服务更倾向于在 HTTP 请求的其他地方查找特定于应用的方法名称：通常是在 URI 路径或请求文档中。

示例 1-7 是一个 Web 服务客户端，该服务将其方法信息放在路径中：即 Yahoo! 在线照片分享应用 Flickr 的 Web 服务。
该示例程序用于在 Flickr 中搜索照片。
要运行此程序，你需要创建一个 Flickr 账户并申请 API 密钥（http://www.flickr.com/services/api/keys/apply/）。

*示例 1-7. 搜索 Flickr 上的图片*

```ruby
#!/usr/bin/ruby -w
# flickr-photo-search.rb
require 'open-uri'
require 'rexml/document'

# 返回 Flickr 照片小尺寸版本的 URI
def small_photo_uri(photo)
  server = photo.attribute('server')
  id = photo.attribute('id')
  secret = photo.attribute('secret')
  return "http://static.flickr.com/#{server}/#{id}_#{secret}_m.jpg"
end

# 搜索 Flickr 上匹配特定标签的照片，并打印每个搜索结果的 URI
def print_each_photo(api_key, tag)
  # 构建 URI
  uri = "http://www.flickr.com/services/rest?method=flickr.photos.search" +
        "&api_key=#{api_key}&tags=#{tag}"
  
  # 发起 HTTP 请求并获取实体主体
  response = open(uri).read
  
  # 将实体主体解析为 XML 文档
  doc = REXML::Document.new(response)
  
  # 遍历找到的每张照片...
  REXML::XPath.each(doc, '//photo') do |photo|
    # ...生成并打印其 URI
    puts small_photo_uri(photo) if photo
  end
end

# 主程序
if ARGV.size < 2
  puts "Usage: #{$0} [Flickr API key] [search term]"
  exit
end

api_key, tag = ARGV
print_each_photo(api_key, tag)
```

<div style="background-color: darkgreen; padding: 8px; border-left: 4px solid lightgreen;">
  <center>XPath：速成指南</center>

XPath 是一种领域特定语言，用于在不编写大量代码的情况下对 XML 文档进行切片和切块。
它有许多令人生畏的特性，但你只需一点点知识就能应付。
关键在于将 XPath 表达式视为从 XML 文档中提取标签或其他元素的规则。
本书中 XPath 表达式不多，但我会解释我使用的每一个。

要将 XPath 表达式转换成通俗语言，请从右向左读。表达式 `//photo` 的意思是：

```
  查找每个 `photo` 标签     photo
  无论它在文档中的什么位置  //
```

Ruby 代码 `REXML::XPath.each(doc, '//photo')` 是一种遍历每个 `photo` 标签的简便方法，无需遍历 XML 树。
</div><br/>

该程序向类似 `http://www.flickr.com/services/rest?method=flickr.photos.search&api_key=xxx&tag=penguins` 的 URI 发起 HTTP 请求。
服务器如何知道客户端想要做什么？
嗯，方法名称很显然是 `flickr.photos.search`。
只不过：HTTP 方法是 GET，而我正在获取信息，所以可能方法信息是个幌子。
也许方法信息实际上是在 HTTP 动作中传达的。

但这个假设很快就不成立了，因为 Flickr API 支持许多方法，不仅仅是 “获取” 类型的方法如 `flickr.photos.search` 和 `flickr.people.findByEmail`，还有像 `flickr.photos.addTags`、`flickr.photos.comments.deleteComment` 等等。
所有这些方法都是通过 HTTP GET 请求调用的，无论它们是否 “获取” 任何数据。
很明显，Flickr 将方法信息放在 `method` 查询变量中，并期望客户端忽略 HTTP 方法所传达的信息。

相比之下，典型的 SOAP 服务将其方法信息放在实体主体和 HTTP 首部中。
示例 1-8 是一个 Ruby 脚本，它使用 Google 基于 SOAP 的 API 搜索 Web。

*示例 1-8. 使用 Google 的搜索服务搜索 Web* <a id="example-1-8"></a>

```ruby
#!/usr/bin/ruby -w
# google-search.rb
require 'soap/wsdlDriver'

# 执行 Google 搜索并打印每个搜索结果的标题
def print_page_titles(license_key, query)
  wsdl_uri = 'http://api.google.com/GoogleSearch.wsdl'
  driver = SOAP::WSDLDriverFactory.new(wsdl_uri).create_rpc_driver
  result_set = driver.doGoogleSearch(license_key, query, 0, 10, true, ' ',
                                     false, ' ', ' ', ' ')
  result_set.resultElements.each { |result| puts result.title }
end

# 主程序
if ARGV.size < 2
  puts "Usage: #{$0} [Google license key] [query]"
  exit
end

license_key, query = ARGV
print_page_titles(license_key, query)
```

💡 在我写这本书的时候，Google 宣布它将弃用其 SOAP 搜索服务，转而采用一种 RESTful 的、面向资源的服务（不幸的是，该服务在使用上受到法律限制，而 SOAP 服务则没有）。
我没有更改这个示例，因为 Google 的 SOAP 服务仍然是我所知道的最佳示例，而且我并不期望你真的去运行这个程序。
我只是想让你看看代码，以及代码所依赖的 SOAP 和 WSDL 文档。

好的，这可能没有太大帮助，因为 WSDL 库隐藏了大部分细节。
实际情况是这样的：当你调用 `doGoogleSearch` 方法时，WSDL 库会向 Google SOAP 服务的 “端点” 发起一个 POST 请求，该端点位于 URI `http://api.google.com/search/beta2`。
这个单一的 URI 是所有 API 调用的目的地，并且只对它发起 POST 请求。
所有这些细节都包含在位于 `http://api.google.com/GoogleSearch.wsdl` 的 WSDL 文件中，其中包含了像 `doGoogleSearch` 定义这样的详细信息（示例 1-9）。

*示例 1-9. Google 搜索服务 WSDL 描述的一部分*

```xml
<operation name="doGoogleSearch">
  <input message="typens:doGoogleSearch"/>
  <output message="typens:doGoogleSearchResponse"/>
</operation>
```

由于 URI 和 HTTP 方法从不变化，方法信息 ——即 “doGoogleSearch” —— 不能放在这两个地方。
相反，它被放入 POST 请求的实体主体中。
示例 1-10 展示了你可能发起的一个搜索 “REST” 的 HTTP 请求。

*示例 1-10. 一个 SOAP RPC 调用示例*

```http
POST search/beta2 HTTP/1.1
Host: api.google.com
Content-Type: application/soap+xml
SOAPAction: urn:GoogleSearchAction

<?xml version="1.0" encoding="UTF-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
  <soap:Body>
    <gs:doGoogleSearch xmlns:gs="urn:GoogleSearch">
      <q>REST</q>
      ...
    </gs:doGoogleSearch>
  </soap:Body>
</soap:Envelope>
```

方法信息是 “doGoogleSearch”。
它既是 SOAP Envelope 中 XML 标签的名称，也是 WSDL 文件中操作的名称，还是示例 1-8 中 Ruby 方法的名称。
它也出现在 `SOAPAction` HTTP 请求首部的值中：一些 SOAP 实现在那里查找它，而不是在实体主体内部。

让我们通过考虑一个不同的例子来让这一切回到原点：不是 Google SOAP 搜索 API，而是 Google 搜索引擎本身。
要使用你的 Web 浏览器在 Google 的数据集中搜索 “REST”，你会向 `http://www.google.com/search?q=REST` 发送一个 GET 请求，并收到一个 HTML 响应。
方法信息保存在 HTTP 方法中：你是在 GET（获取）一个搜索结果列表。

## 范围信息（Scoping Information）

<ins>Web 服务回答方式不同的另一个大问题是：客户端如何告诉服务器要操作数据集的哪一部分？
假设服务器理解客户端想要（比如）删除某些数据，它如何知道客户端想要删除哪些数据？
为什么服务器应该操作这份数据而不是那份数据？</ins>

<ins>我把这类信息称为 *范围信息*（scoping information）。
放置它的一个明显位置是 URI 路径。</ins>
这正是大多数网站的做法。
再次考虑一个搜索引擎 URI，如 `http://www.google.com/search?q=REST`。
在那里，方法信息是 “GET”，范围信息是 “/search?q=REST”。
客户端试图 GET（获取）关于 REST 的搜索结果列表，而不是试图 GET 其他东西：
例如，关于 jellyfish 的搜索结果列表（其范围信息将是 “/search?q=jellyfish” ），或 Google 首页（那将是 “/” ）。

许多 Web 服务将范围信息放在路径中。
Flickr 就是其中之一：Flickr API URI 中的大多数查询变量都是范围信息。
`tags=penguin` 限定了 `flickr.photos.search` 方法的范围，使其只搜索标记为 “penguin” 的照片。
在方法信息定义编程语言意义上方法名的服务中，范围信息可以看作是该方法的一组参数。
你可以合理地期望看到 `flickr.photos.search(tags=penguin)` 作为某种编程语言中的一行代码。

另一种选择是将范围信息放在实体主体中。
典型的 SOAP Web 服务就是这样做的。
[示例 1-10](#example-1-10) 包含一个 `q` 标签，其内容为字符串“REST”。那就是范围信息，方便地嵌套在提供方法信息的 `doGoogleSearch` 标签内部。

服务设计决定了哪些信息是方法信息，哪些是范围信息。这在 Flickr 和 Google 等案例中最为明显，在这些案例中，网站和 Web 服务做同样的事情，但有不同的设计。
这两个 URI 包含相同的信息：

- `http://flickr.com/photos/tags/penguin`
- `http://api.flickr.com/services/rest/?method=flickr.photos.search&tags=penguin`

在第一个 URI 中，方法信息是 “GET”，范围信息是 “标记为‘penguin’的照片”。
在第二个 URI 中，方法信息是 “执行照片搜索”，范围信息是 “penguin”。
从技术角度来看，两者没有区别：它们都使用 HTTP GET。
差异只有在架构层面才变得明显，当你退后一步注意到像 `flickr.photos.delete` 这样的方法名值时，它会将 HTTP 的 GET 方法带到它本不该去的地方。

另一个例子：在 Google SOAP API 中，你正在执行搜索这一事实是方法信息（`doGoogleSearch`）。
搜索查询是范围信息（`q`）。
在 Google 网站上，“search” 和 “q” 的值都是范围信息。
方法信息是 HTTP 标准的 GET。
（如果 Google SOAP API 提供了一个名为 `doGoogleSearchForREST` 的方法，那它就是在如此宽泛地定义方法信息，以至于你不需要任何范围信息来执行对 REST 的搜索。）

## 竞争架构

现在我已经确定了 Web 服务回答方式不同的两个主要问题，我可以根据它们对这些问题的回答来对 Web 服务进行分组。
在我的研究中，我识别出了三种常见的 Web 服务架构：RESTful 面向资源型、RPC 风格型和 REST-RPC 混合型。
我将依次介绍每一种。

### RESTful、面向资源的架构

本书的主要主题是可以被视为 RESTful 的 Web 服务架构：即那些在 Roy Fielding 论文中提出的标准下获得高分的架构。
当然，很多架构在技术上都是 RESTful 的，‡
但我想专注于那些最适合 Web 服务的架构。
所以当我谈论 RESTful Web 服务时，我指的是看起来像 Web 的服务。
我将这类服务称为 *面向资源*（resource-oriented）的。
在 [第 3 章](ch3.md) 中，我将在一个真实的 Web 服务 ——Amazon 的 Simple Storage Service（S3）—— 的上下文中介绍面向资源的 REST 的基本概念。
从 [第 5 章](ch5.md) 开始，我将向你讲解 REST 的定义性特征，并为 RESTful Web 服务定义一个良好的架构：面向资源架构（Resource-Oriented Architecture）。

‡ 比你想象的要多。
Google 用于 Web 搜索的 SOAP API 在技术上具有 RESTful 架构。
许多其他只读的 SOAP 和 XML-RPC 服务也是如此。
但这些对于 Web 服务来说是不良架构，因为它们看起来一点也不像 Web。

<ins>在 RESTful 架构中，方法信息放在 HTTP 方法中。
在面向资源架构中，范围信息放在 URI 中。</ins>
这种组合非常强大。
给定一个面向资源的 RESTful Web 服务 HTTP 请求的第一行（“GET /reports/open-bugs HTTP/1.1”），你基本上应该能理解客户端想要做什么。
请求的其余部分只是细节；事实上，你可以仅使用一行 HTTP 来发起许多请求。
如果 HTTP 方法与方法信息不匹配，则该服务就不是 RESTful 的。
如果范围信息不在 URI 中，则该服务就不是面向资源的。
这些不是唯一的要求，但它们是很好的经验法则。

一些众所周知的 RESTful、面向资源 Web 服务示例包括：

- 暴露 Atom Publishing Protocol（ http://www.ietf.org/html.charters/atompub-charter.html ）及其变体（如 GData，http://code.google.com/apis/gdata/ ）的服务
- Amazon 的 Simple Storage Service（S3，http://aws.amazon.com/s3 ）
- Yahoo! 的大部分 Web 服务（ http://developer.yahoo.com/ ）
- 大多数不使用 SOAP 的只读 Web 服务
- 静态网站
- 许多 Web 应用，特别是像搜索引擎这样的只读应用

每当我介绍非 RESTful 架构以及非面向资源架构时，我都有一些潜在动机。
在本章中，我想在可编程 Web 的更大背景下，将 RESTful Web 服务放在一个适当的位置。
在 [第 2 章](ch2.md) 中，我将扩大本书对真实 Web 服务的覆盖范围，并展示无论服务是否完全符合我偏好的架构，你都可以使用相同的客户端工具。
在 [第 10 章](ch10.md) 中，我将就一场关于可编程 Web 应该是什么样子的长期争论提出论点。

### RPC 风格架构

RPC 风格的 Web 服务接收来自客户端装满数据的信封，并返回一个类似的信封。
方法和范围信息保存在信封内部，或贴在信封上的标签中。
信封的类型对我的分类来说并不重要，但 HTTP 是一种流行的信封格式，因为任何配得上“Web 服务” 名称的服务无论如何都必须使用 HTTP。
SOAP 是另一种流行的信封格式（通过 HTTP 传输 SOAP 文档是将 SOAP 信封放在 HTTP 信封内部）。
每个 RPC 风格的服务都定义一套全新的词汇表。
计算机程序也是这样工作的：每次你编写一个程序，你都会定义具有不同名称的函数。
相比之下，所有 RESTful Web 服务共享一套标准的 HTTP 方法词汇表。
RESTful 服务中的每个对象都响应相同的统一接口。

XML-RPC 协议是 RPC 架构最明显的例子。
XML-RPC 如今基本上是一种遗留协议，但我打算以它作为起点，因为它相对简单且易于解释。
示例 1-11 展示了一个 XML-RPC 服务的 Ruby 客户端，该服务允许你查找任何带有通用产品代码（UPC）的商品。

*示例 1-11. XML-RPC 示例：按 UPC 查找产品*

```ruby
#!/usr/bin/ruby -w
# xmlrpc-upc.rb
require 'xmlrpc/client'

def find_product(upc)
  server = XMLRPC::Client.new2('http://www.upcdatabase.com/rpc')
  begin
    response = server.call('lookupUPC', upc)
  rescue XMLRPC::FaultException => e
    puts "Error: "
    puts e.faultCode
    puts e.faultString
  end
end

puts find_product("001441000055")['description']
# "Trader Joe's Thai Rice Noodles"
```

XML-RPC 服务模拟像 C 这样的编程语言。
你使用一些参数（“001441000055”）调用一个函数（`lookupUPC`），并得到一个返回值。
方法数据（函数名）和范围数据（参数）被放入一个 XML 文档中。
示例 1-12 给出了一个示例文档。

*示例 1-12. 描述 XML-RPC 请求的 XML 文档*

```xml
<?xml version="1.0" ?>
<methodCall>
  <methodName>lookupUPC</methodName>
  <params>
    <param><value><string>001441000055</string></value></param>
  </params>
</methodCall>
```

这个 XML 文档被放入一个信封中以便传输到服务器。
信封是一个带有方法、URI 和首部的 HTTP 请求（见示例 1-13）。
XML 文档成为 HTTP 信封内部的实体主体。

*示例 1-13. 包含描述 XML-RPC 请求的 XML 文档的 HTTP 信封*

```http
POST /rpc HTTP/1.1
Host: www.upcdatabase.com
User-Agent: XMLRPC::Client (Ruby 1.8.4)
Content-Type: text/xml; charset=utf-8
Content-Length: 158
Connection: keep-alive

<?xml version="1.0" ?>
<methodCall>
  <methodName>lookupUPC</methodName>
  ...
</methodCall>
```

XML 文档根据你调用的方法而变化，但 HTTP 信封始终相同。
无论你对 UPC 数据库服务执行什么操作，URI 始终是 `http://www.upcdatabase.com/rpc`，HTTP 方法始终是 POST。
简而言之，XML-RPC 服务忽略了 HTTP 的大部分特性。
它只暴露一个 URI（“端点”），并且在该 URI 上只支持一种方法（POST）。

对于范围信息的不同值，RESTful 服务会暴露不同的 URI，而 RPC 风格服务通常为每个 “文档处理器” 暴露一个 URI：即能够打开信封并将其转换为软件命令的东西。
为了进行比较，示例 1-14 展示了如果 UPC 数据库是一个 RESTful Web 服务，代码可能的样子。

*示例 1-14. 假设的代码示例：RESTful UPC 查找服务*

```ruby
require 'open-uri'
upc_data = open('http://www.upcdatabase.com/upc/00598491').read()
...
```

在此，方法信息包含在 HTTP 方法中。
默认的 HTTP 方法是 GET，在此场景中相当于 `lookupUPC`。
范围信息包含在 URI 中。
该假设服务暴露了大量的 URI：每个可能的 UPC 都有一个 URI。
相比之下，HTTP 信封是空的：HTTP GET 请求根本不包含实体主体。

关于 RPC 风格服务客户端的另一个示例，请回顾 [示例 1-8](#example-1-8)。
Google 的 SOAP 搜索 API 是一个使用 SOAP 作为其信封格式的 RPC 风格服务。

大量或专门使用 HTTP POST 的服务很可能是 RPC 风格服务。
同样，这不是一个确定的标志，但这是一个提示，表明该服务不太倾向于将其方法信息放在 HTTP 方法中。
一个大量使用 HTTP POST 的、原本是 RESTful 的服务往往会趋向于 REST-RPC 混合架构。

一些知名的 RPC 风格 Web 服务示例：

- 所有使用 XML-RPC 的服务
- 几乎所有的 SOAP 服务（关于这一有争议说法的辩护，请参见本章后面的 “可编程 Web 上的技术” 部分）
- 少数 Web 应用（通常是设计不良的）

### REST-RPC 混合架构

这是我创造的一个术语，用于描述介于 RESTful Web 服务和纯 RPC 风格服务之间的 Web 服务。这些服务通常由熟悉现实世界 Web 应用、但对 REST 理论了解不多的程序员创建。

再来看一下 Flickr Web 服务使用的这个 URI：`http://www.flickr.com/services/rest?api_key=xxx&method=flickr.photos.search&tags=penguin`。
尽管 URI 中包含 “rest”，但这显然是设计为 RPC 风格服务，使用 HTTP 作为其信封格式。
它像 RESTful 面向资源服务一样，将范围信息（ “标记为‘penguin’的照片” ）放在 URI 中。
但方法信息（ “搜索照片” ）也放在 URI 中。
在 RESTful 服务中，方法信息应放在 HTTP 方法（GET）中，剩下的内容则成为范围信息。
实际上，该服务只是将 HTTP 用作信封格式，随意地将方法和范围信息放在任何地方。
这是一个 RPC 风格服务。结案。

不过……请看示例 1-15。

*示例 1-15. 对 Flickr Web 服务的 HTTP 请求示例*

```http
GET services/rest?api_key=xxx&method=flickr.photos.search&tags=penguin HTTP/1.1
Host: www.flickr.com
```

这是客户端远程调用该过程时发起的 HTTP 请求。
现在看起来方法信息似乎在 HTTP 方法中。
我发送了一个 GET 请求来获取某些东西。
我在获取什么？
一个针对标记为 “penguin” 照片的搜索结果列表。
曾经看起来像方法信息（ “photoSearch()” ）的东西，现在看起来像范围信息（ “photos/tag/penguin” ）。
现在这个 Web 服务看起来是 RESTful 的了。

当一个 RPC 风格服务使用纯 HTTP 作为其信封格式，并且方法和范围信息恰巧都位于 HTTP 请求的 URI 部分时，就会产生这种视觉错觉。
如果 HTTP 方法是 GET，并且 Web 服务请求的目的是 “获取” 信息，就很难判断方法信息是在 HTTP 方法中还是在 URI 中。
查看在线路上传输的 HTTP 请求，你会看到与 RESTful Web 服务相同的请求。
它们可能包含像 `method=flickr.photos.search` 这样的元素，但这可以被解释为范围信息，就像 “photos/” 和 “search/” 是范围信息一样。
这些 RPC 风格服务或多或少偶然地具有了 RESTful Web 服务的元素。
它们只是将 HTTP 用作方便的信封格式，但使用方式与 RESTful 服务可能做的有所重叠。

许多只读 Web 服务完全符合 RESTful 和面向资源的要求，即使它们是以 RPC 风格设计的！
但如果服务允许客户端写入数据集，就会出现客户端使用的 HTTP 方法与真实方法信息不匹配的情况。
这会使服务无法达到应有的 RESTful 程度。
这类服务就是我所说的 REST-RPC 混合型。

举一个例子。Flickr Web API 要求客户端即使要修改数据集也使用 HTTP GET。
要删除一张照片，你需要向一个包含 `method=flickr.photos.delete` 的 URI 发起 GET 请求。
正如我将在 “将数据集拆分为资源 [115]” 中展示的，这根本不是 GET 的用途。
Flickr Web API 是一个 REST-RPC 混合体：当客户端通过 GET 检索数据时是 RESTful 的，当客户端修改数据集时则是 RPC 风格的。

一些知名的 REST-RPC 混合服务示例包括：

- del.icio.us API
- “RESTful” 的 Flickr Web API
- 许多其他号称 RESTful 的 Web 服务
- 大多数 Web 应用

从设计角度来看，我认为没有人会特意将服务设计成 REST-RPC 混合体。
由于 HTTP 的工作方式，任何使用纯 HTTP 并暴露多个 URI 的 RPC 风格服务都倾向于最终变成 RESTful 或混合型。
许多程序员设计 Web 服务的方式与他们设计 Web 应用的方式完全相同，最终得到的是混合型服务。

混合架构的存在引起了很多混淆。
这种风格对于设计过 Web 应用的人来说很自然，而且人们常常声称混合架构是 RESTful 的：毕竟，它们的工作方式与人类 Web “相同”。
人们花了大量时间试图区分 RESTful Web 服务和这些神秘的 “其他” 服务。
我将这些 “其他” 服务归类为 REST-RPC 混合体，这只是一长串新词中最新的一个。
我认为这个特定的新词是看待这些常见但令人困惑的服务最准确、最有用的方式。
如果你遇到过其他描述它们的方式（在撰写本文时，“HTTP+POX” 是最流行的），你可能想继续阅读，我会用本书的观点来解释这些其他说法。

## 人类 Web 就在可编程 Web 上

在前面的章节中，我声称所有静态网站都是 RESTful 的。
我声称 Web 应用属于三类中的一类，其中大多数是 REST-RPC 混合型。
由于人类 Web 完全由静态网站和 Web 应用组成，这意味着整个人类 Web 也都在可编程 Web 上！
至此，这应该不会让你感到惊讶了。
Web 浏览器是一个软件程序，它发起 HTTP 请求并以某种方式处理响应（通过向人类展示）。
这正是 Web 服务客户端所做的。
如果它在 Web 上，它就是 Web 服务。

我写本书的目标不是让可编程 Web 变得更大。
这几乎是不可能的：可编程 Web 已经涵盖了几乎所有带有 HTTP 接口的东西。
<ins>我的目标是帮助让可编程 Web 变得更好：更统一、结构更清晰，并最大限度地利用 HTTP 的特性。</ins>

### 可编程 Web 上的技术

我已经根据底层架构对 Web 服务进行了分类，区分了鱼和鲸鱼。
现在我可以检查它们所使用的技术，而不会混淆技术和架构。

### HTTP

所有 Web 服务都使用 HTTP，但使用方式不同。
对 RESTful Web 服务的请求将方法信息放在 HTTP 方法中，将范围信息放在 URI 中。
RPC 风格 Web 服务倾向于忽略 HTTP 方法，在 URI、HTTP 首部或实体主体中寻找方法和范围信息。
一些 RPC 风格 Web 服务将 HTTP 用作包含文档的信封，而另一些则仅将其用作包含另一个信封的未标记信封。

### URI

同样，所有 Web 服务都使用 URI，但方式不同。
我接下来要说的是一种概括，但相当准确。
RESTful 面向资源服务为客户端可能想要操作的每一条数据都暴露一个 URI。
REST-RPC 混合服务为客户端可能执行的每个操作都暴露一个 URI：一个用于获取数据的 URI，另一个用于删除相同数据的 URI。
RPC 风格服务为每个能够处理远程过程调用（RPC）的进程暴露一个 URI。
通常只有一个这样的 URI：服务 “端点”。

### XML-RPC

少数（主要是遗留的）Web 服务在 HTTP 之上使用 XML-RPC。
XML-RPC 是一种用于表示函数调用及其返回值的数据结构格式。
顾名思义，它明确设计为使用 RPC 风格。

### SOAP

许多 Web 服务在 HTTP 之上使用 SOAP。
SOAP 是一种信封格式，与 HTTP 类似，但它是基于 XML 的信封格式。

现在我要说一些有争议的话。
粗略地说，目前所有使用 SOAP 的 Web 服务都具有 RPC 架构。
这之所以有争议，是因为许多 SOAP 程序员认为 RPC 架构已经过时，更倾向于称其服务为 “面向消息” 或 “面向文档” 的服务。

好吧，所有 Web 服务都是面向消息的，因为 HTTP 本身是面向消息的。
HTTP 请求只是一个消息：一个内含文档的信封。
问题在于该文档说了什么。
基于 SOAP 的服务要求客户端在 HTTP 信封内部再放入一个信封（SOAP 文档）。
同样，真正的问题在于信封内部说了什么。
SOAP 信封可以包含任何 XML 数据，就像 HTTP 信封可以在其实体主体中包含任何数据一样。
但在现有的每一个 SOAP 服务中，SOAP 信封都包含一个类似 XML-RPC 格式的 RPC 调用描述。

有多种方式可以重新排列这个 RPC 描述并赋予它不同的标签 ——“document/literal” 或 “wrapped/literal”—— 但无论如何切分，你得到的都是一个具有大量方法信息词汇表的服务，一个在文档内部而非信封上查找范围信息的服务。
这些是 RPC 架构的定义性特征。

我要强调的是，这不是关于 SOAP 本身的事实，而是关于它当前使用方式的事实。
SOAP 和 HTTP 一样，只是将数据放入信封的一种方式。
然而目前，放入该信封的唯一数据是类似 XML-RPC 的、关于如何调用远程函数或该函数返回值的数据。
我将在 [第 10 章](ch10.md) 更详细地论证这一点。

### WS-*

这些标准为 SOAP 信封定义了特殊的 XML “标签”。
这些标签类似于 HTTP 首部。

### WSDL

Web 服务描述语言（WSDL）是一种用于描述基于 SOAP 的 Web 服务的 XML 词汇表。
客户端可以加载 WSDL 文件，并确切知道它可以调用哪些 RPC 风格方法、这些方法期望哪些参数，以及它们返回哪些数据类型。
几乎所有现有的 SOAP 服务都暴露一个 WSDL 文件，并且大多数 SOAP 服务如果没有其 WSDL 文件作为指南，将很难使用。
正如我在 [第 10 章](ch10.md) 中所讨论的，WSDL 在维持 SOAP 与 RPC 风格的关联方面，比其他任何技术都负有更大的责任。

### WADL

Web 应用描述语言（WADL）是一种用于描述 RESTful Web 服务的 XML 词汇表。
与 WSDL 类似，通用客户端可以加载 WADL 文件，并立即能够访问相应 Web 服务的全部功能。
我将在 [第 9 章](ch9.md) 讨论 WADL。

由于 RESTful 服务具有更简单的接口，WADL 对这些服务的必要性远不如 WSDL 对 RPC 风格 SOAP 服务那样重要。
这是一件好事，因为截至撰写本文时，几乎没有真正的 Web 服务提供官方的 WADL 文件。
Yahoo! 的 Web 搜索服务是其中之一。

## 剩余术语

信不信由你，在关于 REST 的讨论中，还有一些常见的术语我尚未提及。
我没有提及它们，是因为我认为它们不准确或完全超出了本书的范围。
但我有责任解释我为什么这么认为，以便你决定是否同意。
如果你没有听说过这些术语，可以跳过本节。

**面向服务架构（Service-Oriented Architecture，SOA）**

这是一个行业流行词。
我不打算在此赘述，原因有二。
首先，这个术语定义得不太明确。
其次，就它被定义的程度而言，它大致意思是：“一种基于 Web 服务生产和消费的软件架构。”
在本书中，我讨论的是单个服务的设计。
一本关于面向服务架构的书应该在稍高的层面上展开，展示如何将服务用作软件组件，如何将它们集成到一个连贯的整体中。我不在本书中涵盖这类内容。

**SOAP 作为 REST 的竞争对手**

如果你参与 Web 服务辩论，你会经常听到这种说法。
你不会在这里听到，因为它给人错误的印象。
RESTful 架构的主要竞争对手是 RPC 架构，而不是像 SOAP 这样的特定技术。
诚然，基本上每一个现存的 SOAP 服务都具有 RPC 架构，但 SOAP 只是像 HTTP 一样，将文档放入带有标签的信封中的一种方式。
SOAP 与 RPC 架构的关联主要是历史偶然性和当前一代自动化工具的结果。

这里确实存在一种紧张关系，但这不是我将在本书中过多讨论的内容。
大致来说，这是将文档放入 SOAP 信封再放入 HTTP 信封的服务，与仅使用 HTTP 信封的服务之间的紧张关系。

**HTTP+POX**

代表 HTTP 加上纯旧 XML（Plain Old XML）。
这个术语大致涵盖了我称之为 REST-RPC 混合服务的那些服务。
它们与 RESTful 设计有重叠，尤其是在检索数据方面，但其基本架构是面向 RPC 的。

我不喜欢这个术语，因为 “纯旧 XML” 不准确。
这些服务的有趣之处不在于它们生成纯旧 XML 文档（而非包裹在 SOAP 信封中的 XML 文档）。
有些这类服务根本不提供 XML：它们提供 JSON、纯文本或二进制文件。
不，这些服务的有趣之处在于它们的 RPC 架构。
这才是它们与 REST 对立的原因。

**STREST**

指 Service-Trampled REST。
这是 REST-RPC 混合架构的另一个术语。
它比 HTTP+POX 更准确，因为它传达了 RESTful 架构被其他东西（在此例中为 RPC 风格）接管的概念。
这是一个巧妙的缩写，但我不喜欢它，因为它认同了一个迷思，即唯一真正的 Web 服务是 RPC 风格的服务。
毕竟，践踏你的 REST 的那个 “服务” 是一个 RPC 服务。
如果你认为 REST 服务是真正的服务，那么喊 “救命！我本来有一些 REST，然后这个 Service 掺和进来了！”
就没有意义。RPC-Trampled REST 会更准确，但那是个糟糕的缩写。

**高 REST 与低 REST（High and low REST）**

这是区分真正 RESTful 服务和那些我称之为 REST-RPC 混合服务的另一种方式。
高 REST 服务就是那些严格遵循 Fielding 论文的服务。
除其他方面外，它们将方法信息放在 HTTP 方法中，将范围信息放在 URI 中。
低 REST 服务则被认为偏离了正统。
由于低 REST 服务倾向于向特定方向（朝向 RPC 风格）偏离正统，我更倾向于使用更具体的术语。
