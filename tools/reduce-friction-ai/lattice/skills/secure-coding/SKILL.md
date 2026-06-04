---
name: secure-coding
description: "在生成或修改代码时应用安全意识的思考。强制执行信任边界意识、输入验证、注入防护、密钥管理和纵深授权。在生成处理用户输入、认证、授权、数据库查询、外部 API、文件操作的代码时使用，或当用户提到'security review'、'secure this'、'check for vulnerabilities'、'trust boundary'、'input validation'或'OWASP'时使用。此 skill 管理生成代码的安全态势——不是架构（参见 architecture）和不是代码工艺（参见 clean-code）。"
---

# Secure Coding（安全编码）

## 配置解析（Config Resolution）

Skill 支持项目自定义。顺序：

1. 在仓库根目录查找 `.lattice/config.yaml`
2. 如果找到，检查 `paths.secure_coding` 获取自定义文档路径
3. 如果自定义路径存在，读取文档并检查 YAML frontmatter `mode`：
   - **`mode: override`**（覆盖模式，或无 mode）：自定义文档完全优先。
     替代嵌入默认值使用。必须全面——唯一参考。
   - **`mode: overlay`**（叠加模式）：首先读取嵌入的 `./references/defaults.md`，然后将
     自定义文档部分叠加在上面。自定义部分替换匹配
     默认部分（按标题匹配）。新部分追加在默认值之后。
4. 如果没有配置、没有路径或路径未找到，读取 `./references/defaults.md`
5. **语言适配**：如果配置中存在 `paths.language_idioms`，读取 **"Error Handling"** 部分并将§1（信任边界识别）错误消息模式适配到语言习惯用法。语言习惯用法优先于伪代码默认值。

Skill 附带的默认值代表有观点的最佳实践。
开箱即用于任何项目。仅当团队有不同特定标准时才覆盖。

## 自我验证清单（Self-Validation Checklist）

生成每个组件后 STOP。在继续之前验证所有项目。如果检查明显失败，在呈现之前修复代码。如果检查是判断调用且有多个有效方法（参见模糊信号），标记——呈现选项和推理而非静默选择。

1. **信任边界**：受信任代码在哪里与不受信任数据相遇？所有边界明确识别？
2. **输入验证**：每个外部输入在到达业务逻辑之前是否在边界使用白名单验证？
3. **查询安全**：所有数据库查询参数化？在查询构建中是否有任何字符串连接？
4. **命令安全**：是否有任何 shell/命令执行？如果是，输入严格白名单？
5. **密钥**：代码中是否有任何 API key、password、token、connection string？如果是→移动到环境变量或密钥管理器。
6. **输出编码**：Output encoded appropriate for render context（HTML、JSON、URL）？
7. **授权**：Authorization verified at service layer，而非仅 controller？每个 endpoint enforce least privilege？
8. **错误消息**：Error message exposed to user avoid reveal internal detail（stack trace、SQL query、file path）？
9. **依赖**：New third-party package necessary？Version pinned or constrained？Any known-vulnerable package added？

## 主动反模式扫描（Active Anti-Pattern Scan）

在验证上述清单后，扫描输出特定反模式。如果找到任何，在呈现代码之前修复。

- [ ] **信任所有输入**：No validation on request param；data flow direct to business logic → validate at boundary with allowlist
- [ ] **SQL 字符串拼接**：User input interpolated into SQL query → use parameterized query or ORM query builder
- [ ] **硬编码密钥**：API key、password、token in source code → use env var or secret manager
- [ ] **缺少授权**：Auth checked at login but not re-verified at service or resource level → check at every layer
- [ ] **权限过宽**：Admin access granted where read-only suffice → apply least privilege
- [ ] **未验证的重定向**：User-controlled URL used in redirect → allowlist permitted destination
- [ ] **冗长的错误消息**：Stack trace or SQL in API response → return generic message, log detail server-side
- [ ] **记录敏感数据**：Password、token、PII in log file → log event, not value; mask sensitive field

## 模糊信号（Ambiguity Signals）

检查通常有多个有效结果。当遇到时，呈现选项而非静默选择。

- **信任边界范围**：Internal API behind trusted gateway may or may not need full boundary validation. Answer depend on deployment topology and threat model.
- **错误消息细节**：How much info "actionable but safe" depend on whether consumer human user, frontend client, or internal service.
- **验证深度**：Whether re-validate data at inner layer (defense-in-depth) or trust boundary validation depend on risk profile and performance requirement.
- **认证与授权失败响应**：Whether return 401 (not authenticated) or 403 (not authorized) depend on whether identity known. Conflating leak info (403 confirm resource exist). When consumer human user, distinguish clear; when consumer internal service, separation may differ.

## 核心原则（Core Principle）

Security about **thinking in trust boundary**. Every data flow cross boundary somewhere -- between user and server, between app and database, between code and third-party API. Question not "could this be exploited?" but "where trusted meet untrusted, and what happen at boundary?"

Atom teach adversarial thinking during code gen, not afterthought. When write code, identify trust boundary as go -- same way skilled dev consider edge case. Cost build security in during gen near zero; cost retrofit after breach catastrophic.

Boundary with clean-code: clean-code say "handle error explicit with actionable message." Secure-coding say "error message shown to user must not reveal internal detail." Both apply; this skill govern security dimension.

Boundary with architecture atom: "check authorization at every layer" (this skill) map direct to loaded architecture layer structure. Architecture atom define *where* each check live (e.g., service layer, not controller); secure-coding define *what* to check (identity confirmed, permission granted, resource owned).

参见 `./references/defaults.md` 获取信任边界识别、输入验证模式、授权检查、密钥管理和注入防护模式。
