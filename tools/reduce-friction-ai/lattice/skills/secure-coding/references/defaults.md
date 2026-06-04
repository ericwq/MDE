# Secure Coding：默认原则（Default Principles）

嵌入的安全代码默认值。综合 OWASP、SANS 和防御性编程为可操作的指南。

自定义 `.lattice/secure-coding.md`（通过 `.lattice/config.yaml`）覆盖此文件。

## 目录（Table of Contents）

1. [信任边界识别](#1-信任边界识别)
2. [输入验证模式](#2-输入验证模式)
3. [参数化查询模式](#3-参数化查询模式)
4. [按上下文输出编码](#4-按上下文输出编码)
5. [授权检查模式](#5-授权检查模式)
6. [密钥管理模式](#6-密钥管理模式)
7. [注入防护模式](#7-注入防护模式)

---

## 1. 信任边界识别（Trust Boundary Identification）

信任边界 = 数据跨越信任级别。首先找到这些。

### 常见信任边界（Common Trust Boundaries）

```
                    不受信任                          受信任
                ┌──────────────┐                  ┌──────────────┐
                │  Browser /   │   HTTP Request   │  Controller  │
                │  Mobile App  │ ───────────────► │  (Boundary)  │
                └──────────────┘                  └──────┬───────┘
                                                         │ validated
                ┌──────────────┐                  ┌──────▼───────┐
                │  External    │   API Response   │  Application │
                │  API         │ ───────────────► │  Service     │
                └──────────────┘                  └──────┬───────┘
                                                         │ validated
                ┌──────────────┐                  ┌──────▼───────┐
                │  Database    │   Query Result   │  Repository  │
                │  (may be     │ ───────────────► │  (Boundary)  │
                │  poisoned)   │                  └──────────────┘
                └──────────────┘
```

### 边界识别清单（Boundary Identification Checklist）

对于每个函数/模块问：

1. **数据从哪里来？** 应用外部 = 跨越边界。
2. **之前验证过吗？** 如果没有，在这里验证。
3. **可能被篡改吗？** Cookies、URL 参数、表单、headers = 用户控制。
4. **来自另一个服务？** 即使是内部服务，如果被入侵或有 bug 也可能返回错误数据。

### 模式：信任边界注释（Trust Boundary Annotation）

```
// POOR: No awareness of trust boundary
function handleRequest(req):
  userId = req.params.userId
  return db.query("SELECT * FROM users WHERE id = " + userId)

// GOOD: Trust boundary explicitly identified and defended
function handleRequest(req):
  // TRUST BOUNDARY: req.params is user-controlled input
  userId = validateAndParseUserId(req.params.userId)  // validate at boundary
  return userRepository.findById(userId)               // parameterized internally
```

---

## 2. 输入验证模式（Input Validation Patterns）

### 按类型（By Type）

**字符串验证：**
```
// POOR: Accept any string
function setUsername(name):
  this.username = name

// GOOD: Validate format, length, and content
function setUsername(name):
  if name is null or name.trim().length == 0:
    throw ValidationError("Username is required")
  if name.length > 50:
    throw ValidationError("Username must be 50 characters or fewer")
  if not matches(name, /^[a-zA-Z0-9_-]+$/):
    throw ValidationError("Username may only contain letters, numbers, hyphens, and underscores")
  this.username = name.trim()
```

**数字验证：**
```
// POOR: Trust the input is a valid number
function setQuantity(qty):
  this.quantity = qty

// GOOD: Type-check, range-check
function setQuantity(qty):
  if not isInteger(qty):
    throw ValidationError("Quantity must be an integer")
  if qty < 1 or qty > 10000:
    throw ValidationError("Quantity must be between 1 and 10,000")
  this.quantity = qty
```

**邮箱验证：**
```
// POOR: Regex-only validation (unreliable for email)
function setEmail(email):
  if not matches(email, /.*@.*/): throw Error("Invalid")
  this.email = email

// GOOD: Structural validation + length limits
function setEmail(email):
  if email is null or email.trim().length == 0:
    throw ValidationError("Email is required")
  if email.length > 254:
    throw ValidationError("Email exceeds maximum length")
  if not isValidEmailFormat(email):  // use a well-tested library
    throw ValidationError("Invalid email format")
  this.email = email.toLowerCase().trim()
```

**URL 验证：**
```
// POOR: Accept any URL
function setCallbackUrl(url):
  this.callbackUrl = url

// GOOD: Validate scheme, domain, and structure
function setCallbackUrl(url):
  parsed = parseUrl(url)
  if parsed.scheme not in ["https"]:
    throw ValidationError("Only HTTPS URLs are allowed")
  if parsed.host in BLOCKED_HOSTS or isPrivateIp(parsed.host):
    throw ValidationError("URL points to a restricted destination")
  this.callbackUrl = parsed.toString()  // normalized
```

**文件路径验证：**
```
// POOR: Accept any path
function readFile(userPath):
  return fs.read(userPath)

// GOOD: Canonicalize and validate against allowlist
function readFile(userPath):
  canonicalPath = fs.realpath(UPLOAD_DIR + "/" + userPath)
  if not canonicalPath.startsWith(UPLOAD_DIR):
    throw SecurityError("Path traversal detected")
  return fs.read(canonicalPath)
```

---

## 3. 参数化查询模式（Parameterized Query Patterns）

### SQL（直接）

```
// POOR: String concatenation -- SQL injection
function findUser(username):
  query = "SELECT * FROM users WHERE username = '" + username + "'"
  return db.execute(query)

// GOOD: Parameterized query
function findUser(username):
  query = "SELECT * FROM users WHERE username = ?"
  return db.execute(query, [username])
```

### ORM 模式（ORM Patterns）

```
// POOR: Raw query with interpolation through ORM
function searchProducts(term):
  return orm.rawQuery("SELECT * FROM products WHERE name LIKE '%" + term + "%'")

// GOOD: ORM query builder with bound parameters
function searchProducts(term):
  return orm.products
    .where("name", "LIKE", "%" + term + "%")  // ORM handles parameterization
    .findMany()
```

### NoSQL（文档数据库）

```
// POOR: User input directly in query operator
function findUser(input):
  return db.users.find({ username: input })
  // if input is { "$gt": "" }, this returns all users

// GOOD: Explicitly extract the expected scalar value
function findUser(input):
  if typeof input != "string":
    throw ValidationError("Username must be a string")
  return db.users.find({ username: input })
```

### 动态查询构建（Dynamic Query Building）

```
// POOR: Dynamic column names from user input
function sortBy(column):
  return db.execute("SELECT * FROM products ORDER BY " + column)

// GOOD: Allowlist for dynamic identifiers
ALLOWED_SORT_COLUMNS = {"name", "price", "created_at"}

function sortBy(column):
  if column not in ALLOWED_SORT_COLUMNS:
    throw ValidationError("Invalid sort column")
  return db.execute("SELECT * FROM products ORDER BY " + column)
  // column is from a fixed allowlist, not user-controlled
```

---

## 4. 按上下文输出编码（Output Encoding by Context）

相同数据在渲染位置不同需要不同的编码。没有单一的"清理"函数。

### HTML 上下文（HTML Context）

```
// POOR: Raw user data in HTML
function renderGreeting(username):
  return "<h1>Hello, " + username + "</h1>"
  // if username is "<script>alert('xss')</script>", this executes

// GOOD: HTML-encode user data
function renderGreeting(username):
  return "<h1>Hello, " + htmlEncode(username) + "</h1>"
  // htmlEncode converts < > & " ' to their HTML entities
```

### JSON 上下文（JSON Context）

```
// POOR: String interpolation into JSON
function buildResponse(message):
  return '{"message": "' + message + '"}'
  // if message contains a quote, this breaks or injects

// GOOD: Use JSON serializer
function buildResponse(message):
  return JSON.stringify({ message: message })
```

### URL 上下文（URL Context）

```
// POOR: Raw user data in URL
function buildLink(searchTerm):
  return "/search?q=" + searchTerm

// GOOD: URL-encode user data
function buildLink(searchTerm):
  return "/search?q=" + urlEncode(searchTerm)
```

### Shell 上下文（Shell Context）

```
// POOR: User data in shell command
function convertFile(filename):
  exec("convert " + filename + " output.png")

// GOOD: Avoid shell entirely; use direct process execution
function convertFile(filename):
  if not matches(filename, /^[a-zA-Z0-9._-]+$/):
    throw ValidationError("Invalid filename")
  execDirect(["convert", filename, "output.png"])  // no shell interpretation
```

---

## 5. 授权检查模式（Authorization Check Patterns）

### 中间件模式（Middleware Pattern）

```
// Authorization enforced at middleware level
// Applies to all routes matching the pattern

router.use("/admin/*", requireRole("admin"))
router.use("/api/orders/:id", requireOwnership("order"))

function requireRole(role):
  return (req, res, next) =>
    if req.user.role != role:
      return res.status(403).json({ error: "Forbidden" })
    next()
```

### 服务层模式（Service-Layer Pattern，纵深防御）

```
// Even with middleware, verify at the service layer
class OrderService:
  function cancelOrder(orderId, requestingUserId):
    order = this.orderRepository.findById(orderId)
    if order is null:
      throw NotFoundError("Order not found")

    // Authorization check at service layer -- defense in depth
    if order.userId != requestingUserId and not this.isAdmin(requestingUserId):
      throw ForbiddenError("Not authorized to cancel this order")

    order.cancel()
    this.orderRepository.save(order)
```

### 装饰器/属性模式（Decorator/Attribute Pattern）

```
// Language-level authorization annotation
@authorize(roles: ["admin", "manager"])
function deleteUser(userId):
  // authorization already verified by decorator
  user = userRepository.findById(userId)
  user.deactivate()
  userRepository.save(user)
```

### 资源级授权（Resource-Level Authorization）

```
// POOR: Only checks if user is authenticated
function getDocument(docId, user):
  if not user.isAuthenticated:
    throw UnauthorizedError()
  return documentRepository.findById(docId)  // any authenticated user can access any document

// GOOD: Checks if user is authorized for this specific resource
function getDocument(docId, user):
  if not user.isAuthenticated:
    throw UnauthorizedError()
  document = documentRepository.findById(docId)
  if not document.isAccessibleBy(user):
    throw ForbiddenError("Not authorized to access this document")
  return document
```

---

## 6. 密钥管理模式（Secrets Management Patterns）

### 环境变量（Environment Variables）

```
// POOR: Hardcoded in source
const DB_PASSWORD = "super_secret_123"
const API_KEY = "sk-abc123def456"

// GOOD: From environment
const DB_PASSWORD = env.require("DB_PASSWORD")  // throws if not set
const API_KEY = env.require("API_KEY")
```

### 密钥管理器集成（Secret Manager Integration）

```
// For production systems with secret rotation
class DatabaseConfig:
  function getConnectionString():
    secret = secretManager.getSecret("db/production/credentials")
    return buildConnectionString(
      host: secret.host,
      port: secret.port,
      user: secret.username,
      password: secret.password
    )
    // secret object goes out of scope and is garbage collected
```

### 安全日志记录（Logging Safely）

```
// POOR: Logging credentials
log.info("Connecting to DB with user=" + dbUser + " password=" + dbPassword)
log.info("API request with key: " + apiKey)

// GOOD: Log the event, not the secret
log.info("Connecting to DB", { user: dbUser, host: dbHost })  // no password
log.info("API request initiated", { endpoint: url, hasApiKey: true })  // existence, not value
```

### 凭证轮换模式（Credential Rotation Pattern）

```
// Design for rotation: accept multiple valid credentials during transition
class ApiAuthenticator:
  function validateKey(providedKey):
    validKeys = secretManager.getSecret("api/valid-keys")  // returns array
    return validKeys.any(key => secureCompare(key, providedKey))
    // during rotation, both old and new keys are valid
```

---

## 7. 注入防护模式（Injection Prevention Patterns）

### SQL 注入（SQL Injection）

```
// VULNERABLE: String concatenation
query = "SELECT * FROM users WHERE email = '" + email + "' AND status = 'active'"
// Attack: email = "' OR '1'='1' --"

// FIXED: Parameterized
query = "SELECT * FROM users WHERE email = ? AND status = 'active'"
db.execute(query, [email])
```

### 命令注入（Command Injection）

```
// VULNERABLE: Shell execution with user input
function compressFile(filename):
  exec("tar -czf archive.tar.gz " + filename)
  // Attack: filename = "file.txt; rm -rf /"

// FIXED: Avoid shell; use direct execution with argument array
function compressFile(filename):
  if not matches(filename, /^[a-zA-Z0-9._-]+$/):
    throw ValidationError("Invalid filename characters")
  execDirect(["tar", "-czf", "archive.tar.gz", filename])
```

### XSS 防护（XSS Prevention）

```
// VULNERABLE: Unencoded output
function renderComment(comment):
  return "<div class='comment'>" + comment.text + "</div>"

// FIXED: Context-aware encoding
function renderComment(comment):
  return "<div class='comment'>" + htmlEncode(comment.text) + "</div>"

// ALSO FIXED: Use a templating engine with auto-escaping
// Most modern frameworks (React, Angular, Jinja2, Thymeleaf) auto-escape by default
// Be cautious with "raw" or "safe" markers that disable escaping
```

### 路径遍历（Path Traversal）

```
// VULNERABLE: Direct path construction
function serveFile(filename):
  path = "/var/uploads/" + filename
  return readFile(path)
  // Attack: filename = "../../etc/passwd"

// FIXED: Canonicalize and validate
function serveFile(filename):
  basePath = "/var/uploads"
  requestedPath = realpath(basePath + "/" + filename)
  if not requestedPath.startsWith(basePath + "/"):
    throw SecurityError("Access denied: path traversal attempt")
  return readFile(requestedPath)
```

### SSRF 防护（SSRF Prevention）

```
// VULNERABLE: Server fetches any URL
function fetchWebhook(url):
  return httpClient.get(url)
  // Attack: url = "http://169.254.169.254/latest/meta-data/" (AWS metadata)

// FIXED: Validate URL scheme and destination
ALLOWED_SCHEMES = {"https"}
BLOCKED_IP_RANGES = ["10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16", "169.254.0.0/16"]

function fetchWebhook(url):
  parsed = parseUrl(url)
  if parsed.scheme not in ALLOWED_SCHEMES:
    throw SecurityError("Only HTTPS URLs are allowed")
  resolvedIp = dns.resolve(parsed.host)
  if isInRange(resolvedIp, BLOCKED_IP_RANGES):
    throw SecurityError("URL resolves to a blocked IP range")
  return httpClient.get(url)
```
