# 《API 设计模式》章节 ↔ google.aip.dev 对应表

> 说明：本表核对《API 设计模式》（JJ Geewax，Manning 2021）各章与 https://google.aip.dev/ 的对应关系。
> AIP 标题取自 aip.dev 官方站点（general 类 1–236，auth 类 4110–4119）。
> “已标注”指书中/译注中已经写明的 AIP；“建议”指尚未标注、经核对后推荐的对应。

## 总表

| 章 | 章节标题 | 书中已标注 | 对应/建议 AIP | 说明 |
| --- | --- | --- | --- | --- |
| 1 | API 简介 | — | **121** Resource-oriented design | 建议·主线 |
| 2 | API 设计模式简介 | — | 无 | 元话题，AIP 本身即模式集；勉强可引 200 Precedent |
| 3 | 命名 | — | **190** Naming conventions（+ 140 Field names、141 Quantities、142 Time and duration、143 Standardized codes） | 建议·主线 |
| 4 | 资源作用域与层级结构 | — | **122** Resource names（+ 124 Resource association、123 Resource types） | 建议·主线 |
| 5 | 数据类型与默认值 | — | **202** Fields（+ 129 Server-Modified Values and Defaults、149 Unset field values、144 Repeated fields、126 Enumerations、147 Sensitive fields） | 建议·主线 |
| 6 | 资源标识 | — | **122** Resource names（ID 段/别名/完整名）（+ 210 Unicode） | 建议·主线 |
| 7 | 标准方法 | — | **130** Methods + **131** Get / **132** List / **133** Create / **134** Update / **135** Delete | 建议·完全对应 |
| 8 | 部分更新与检索 | — | **161** Field masks（+ 157 Partial responses、134 Update） | 建议·主线 |
| 9 | 自定义方法 | — | **136** Custom methods | 建议·完全对应 |
| 10 | 长时间运行的操作 | — | **151** Long-running operations | 建议·完全对应 |
| 11 | 可重复运行 Job | — | **152** Jobs | 建议·完全对应 |
| 12 | 单例子资源 | — | **156** Singleton resources | 建议·完全对应 |
| 13 | 交叉引用 | — | **122** Resource names（Fields representing another resource / Disallow embedding of resources） | 建议·主线 |
| 14 | 关联资源 | — | **124** Resource association（多对多/多对一） | 建议·完全对应 |
| 15 | 添加和移除自定义方法 | — | **124** Resource association（+ 136 Custom methods） | 建议·部分（无专门 AIP） |
| 16 | 多态 | — | 无专门 AIP；勉强可引 **146** Generic fields（Oneof/Any）或 123 | 建议·勉强 |
| 17 | 复制和移动 | — | 无专门 AIP；概念上归 **136** Custom methods | 建议·部分 |
| 18 | 批量操作 | 231 / 233 / 234 / 235 | Batch methods: Get / Create / Update / Delete | ✅ 准确 |
| 19 | 基于条件的删除 | 165 | Criteria-based delete | ✅ 准确 |
| 20 | 匿名写入 | — | 无 | aip.dev 无对应内容 |
| 21 | 分页 | 158 | Pagination | ✅ 准确 |
| 22 | 过滤 | 160 | Filtering | ✅ 准确 |
| 23 | 导入与导出 | 153 | Import and export | ✅ 准确 |
| 24 | 版本控制与兼容性 | 180 / 184 / 185 | Backwards compatibility / API version identifiers / API Versioning | ✅ 准确（已修正） |
| 25 | 软删除 | 164 | Soft delete | ✅ 准确 |
| 26 | 请求去重 | 155 | Request identification | ✅ 准确 |
| 27 | 请求验证 | 163 | Change validation | ✅ 准确 |
| 28 | 资源修订版本 | 162 | Resource Revisions | ✅ 准确 |
| 29 | 请求重试 | — | **194** Automatic retry configuration | 建议·主线 |
| 30 | 请求认证 | — | 无直接对应 | auth 类 4110–4119 讲的是 Google Cloud 凭据机制，非本书的“请求数字签名”规范 |

## 需要特别说明的几处

### 无直接对应的章节

- **第 2 章（API 设计模式简介）**：元话题，没有对应 AIP。
- **第 16 章（多态）**：AIP 无“多态”独立条目，最接近的是 **AIP-146 Generic fields**（Oneof / Maps / Struct / Any）。
- **第 17 章（复制和移动）**：AIP 无“复制/移动”独立条目，概念上属于 **AIP-136 Custom methods**。
- **第 20 章（匿名写入）**：aip.dev 里**确实没有对应内容**——“匿名写入 / 不可寻址数据”这一模式未被 AIP 收录。
- **第 30 章（请求认证）**：对应的是 Google 内部 auth 类 AIP（4110–4119），其内容是默认凭据、身份令牌（ID Token）、工作负载身份联邦（Workload Identity Federation）等云认证机制，**与本书提出的“对请求进行数字签名”的规范不是一回事**。

### 部分匹配的章节（建议标注时写明“无专门 AIP”）

- **第 15 章**：`add` / `remove` 自定义方法，是关联资源方案的替代，没有专门 AIP，可挂到 **AIP-124** 或 **AIP-136**。
- **第 16 章**：见上，勉强挂 **AIP-146**。
- **第 17 章**：见上，概念上归 **AIP-136**。

### 同一 AIP 覆盖多章的情况

- **AIP-122 Resource names** 同时与两章相关，但落在不同小节：
  - **第 6 章（资源标识）** → Collection identifiers / Resource ID segments / Resource ID aliases / Full resource names；
  - **第 13 章（交叉引用）** → Fields representing another resource / Disallow embedding of resources。

## 统计小结

- **完全/明确对应（已标注）**：ch18、19、21、22、23、24、25、26、27、28（共 10 章）；
- **完全/明确对应（建议补充）**：ch1、3–14、29（共 14 章）；
- **部分/勉强对应**：ch15、16、17（共 3 章）；
- **无对应**：ch2、20、30（共 3 章）。

## 附：核验用到的 AIP 标题来源

以下标题均已向 aip.dev 实际页面核对：

- 121 Resource-oriented design
- 122 Resource names
- 123 Resource types
- 124 Resource association
- 126 Enumerations
- 129 Server-Modified Values and Defaults
- 130 Methods
- 131 Standard methods: Get
- 132 Standard methods: List
- 133 Standard methods: Create
- 134 Standard methods: Update
- 135 Standard methods: Delete
- 136 Custom methods
- 140 Field names
- 141 Quantities
- 142 Time and duration
- 143 Standardized codes
- 144 Repeated fields
- 146 Generic fields
- 147 Sensitive fields
- 149 Unset field values
- 151 Long-running operations
- 152 Jobs
- 153 Import and export
- 155 Request identification
- 156 Singleton resources
- 157 Partial responses
- 158 Pagination
- 160 Filtering
- 161 Field masks
- 162 Resource Revisions
- 163 Change validation
- 164 Soft delete
- 165 Criteria-based delete
- 180 Backwards compatibility
- 184 API version identifiers
- 185 API Versioning
- 190 Naming conventions
- 194 Automatic retry configuration
- 202 Fields
- 210 Unicode
- 231 / 233 / 234 / 235 Batch methods: Get / Create / Update / Delete
- auth 类：4110–4119（Google Cloud 认证机制，与本书第 30 章不对应）
