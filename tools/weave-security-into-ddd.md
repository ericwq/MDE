# 将身份验证、角色和安全性融入 C# 领域驱动设计（DDD）：领域模型与 ASP.NET MVC 实现之争

[原文](https://www.codestudy.net/blog/how-do-you-weave-authentication-roles-and-security-into-your-ddd/) 2025/11/25

DDD 是一种软件开发方法论，专注于将代码与业务逻辑对齐，强调通用语言 (ubiquitous language)、限界上下文以及领域模型，作为应用程序核心的。
虽然 DDD 擅长对复杂业务规则进行建模，但将安全性 ——特别是身份验证（验证身份）、授权（角色/权限）和访问控制—— 融入其中，往往会引发争论：

安全逻辑应该驻留在领域模型中（作为核心业务关注点），还是委托给 ASP.NET MVC 表现层（作为横切关注点）？

本博客将深入探讨这两种方法、它们的权衡取舍，以及将安全性融入基于 DDD 的 C# 应用程序的实用策略。
到最后，你将理解如何平衡业务完整性与灵活性，以及何时优先考虑以领域为中心还是以框架为驱动的安全性。

---
内容
- [DDD 与安全性：入门概览](#ddd-与安全性入门概览)
- [方法一：领域模型中的安全性](#方法一领域模型中的安全性)
  - [2.1 用户身份和角色作为领域概念](#21-用户身份和角色作为领域概念)
  - [2.2 作为领域行为的授权](#22-作为领域行为的授权)
  - [2.3 用于身份验证逻辑的领域服务](#23-用于身份验证逻辑的领域服务)
  - [2.4 优缺点](#24-优缺点)
- [方法二：ASP.NET MVC 中的安全性](#方法二aspnet-mvc-中的安全性)
  - [3.1 使用 ASP.NET Core Identity 进行身份验证](#31-使用-aspnet-core-identity-进行身份验证)
  - [3.2 通过属性和策略进行授权](#32-通过属性和策略进行授权)
  - [3.3 中间件和操作过滤器](#33-中间件和操作过滤器)
  - [3.4 优缺点](#34-优缺点)
- [大辩论：领域模型 vs. MVC](#大辩论领域模型-vs-mvc)
- [混合方法：融合两者](#混合方法融合两者)
- [C# 中安全 DDD 的最佳实践](#c-中安全-ddd-的最佳实践)
- [结论](#结论)
- [参考资料](#参考资料)

---

## DDD 与安全性：入门概览

在深入探讨实现之前，让我们先厘清关键概念：

- **DDD 核心原则**：
领域模型是业务逻辑的 “真实来源”，实体、值对象、聚合和领域服务封装了规则和行为。

- **DDD 中的安全性**：
身份验证（用户是谁）和授权（他们能做什么）通常是业务关键性的。
例如，银行应用程序可能要求 “只有账户所有者才能转账” —— 这是一条属于领域层的规则。

- **争论焦点**：
这些规则应该驻留在领域模型中（在源头强制执行），还是驻留在 MVC 层中（使用框架工具如 `[Authorize]`）？

## 方法一：领域模型中的安全性

在这种方法中，安全性被视为业务关注点，身份验证/授权逻辑直接嵌入到领域模型中。
这确保了无论领域在何处被访问（例如 MVC、API、后台任务），规则都能被强制执行。

### 2.1 用户身份和角色作为领域概念

在 DDD 中，用户和角色被建模为领域实体或值对象，以强制执行业务不变量。

**示例：带有角色的用户实体**

```csharp
// Value Object for User ID (immutable, ensures uniqueness)
public record UserId(Guid Value);
 
// Value Object for Role (encapsulates role validation)
public record Role(string Name)
{
    public static readonly Role Admin = new("Admin");
    public static readonly Role Customer = new("Customer");
    
    public Role(string name)
    {
        if (string.IsNullOrWhiteSpace(name))
            throw new ArgumentException("Role name cannot be empty.");
        Name = name;
    }
}
 
// Aggregate Root: User (encapsulates identity and roles)
public class User : AggregateRoot<UserId>
{
    private readonly List<Role> _roles = new();
    public IReadOnlyList<Role> Roles => _roles.AsReadOnly();
    public bool IsActive { get; private set; }
    public string Email { get; private set; }
    private string _hashedPassword; // Encapsulated to prevent direct modification
 
    // Constructor enforces invariants (e.g., a user must have at least one role)
    public User(UserId id, string email, string hashedPassword, IEnumerable<Role> roles, bool isActive = true)
        : base(id)
    {
        Email = email ?? throw new ArgumentNullException(nameof(email));
        _hashedPassword = hashedPassword ?? throw new ArgumentNullException(nameof(hashedPassword));
        IsActive = isActive;
 
        if (!roles.Any())
            throw new InvalidOperationException("A user must have at least one role.");
        _roles.AddRange(roles);
    }
 
    // Add a role only if it doesn't already exist (domain invariant)
    public void AddRole(Role role)
    {
        if (_roles.Contains(role))
            throw new InvalidOperationException($"User already has the {role.Name} role.");
        _roles.Add(role);
    }
}
```

此处，`User` 是一个带有不变量（例如 “必须至少有一个角色” ）和行为（例如 `AddRole`）的聚合根。
角色是值对象，以确保一致性。

### 2.2 作为领域行为的授权

关键操作（例如，转账、更新用户角色）受到领域方法内的授权检查的保护，确保只有授权用户才能执行它们。

**示例：银行账户实体中的授权**

```csharp
public class BankAccount : AggregateRoot<AccountId>
{
    private readonly UserId _ownerId;
    private decimal _balance;
 
    public BankAccount(AccountId id, UserId ownerId, decimal initialBalance)
        : base(id)
    {
        _ownerId = ownerId;
        _balance = initialBalance >= 0 
            ? initialBalance 
            : throw new ArgumentException("Initial balance cannot be negative.");
    }
 
    // Authorization: Only the account owner or an Admin can transfer funds
    public void TransferFunds(decimal amount, BankAccount targetAccount, User actingUser)
    {
        // Check if the acting user is authorized (domain-level authorization)
        if (!IsAuthorized(actingUser))
            throw new UnauthorizedAccessException("User is not authorized to transfer funds.");
 
        // Enforce business rules (e.g., positive amount)
        if (amount <= 0)
            throw new ArgumentException("Transfer amount must be positive.");
        if (_balance < amount)
            throw new InvalidOperationException("Insufficient funds.");
 
        _balance -= amount;
        targetAccount._balance += amount;
        AddDomainEvent(new FundsTransferred(Id, targetAccount.Id, amount));
    }
 
    private bool IsAuthorized(User actingUser)
    {
        // Owner or Admin can transfer funds
        return actingUser.Id == _ownerId || actingUser.Roles.Contains(Role.Admin);
    }
}
```

`TransferFunds` 方法在执行转账之前，首先检查 `actingUser` 是否被授权（所有者或管理员）。
这确保了无论领域如何被访问（例如，通过 MVC 或后台服务），该规则都能被强制执行。

### 2.3 用于身份验证逻辑的领域服务

身份验证（验证凭证）可以封装在一个领域服务中，确保它与业务规则保持一致（例如，“被锁定的用户无法进行身份验证” ）。

**示例：用于身份验证的领域服务**

```csharp
public interface IUserAuthenticationService
{
    Task<User> AuthenticateAsync(string email, string password);
}
 
public class UserAuthenticationService : IUserAuthenticationService
{
    private readonly IUserRepository _userRepository;
    private readonly IPasswordHasher _passwordHasher; // Domain service for hashing
 
    public UserAuthenticationService(IUserRepository userRepository, IPasswordHasher passwordHasher)
    {
        _userRepository = userRepository;
        _passwordHasher = passwordHasher;
    }
 
    public async Task<User> AuthenticateAsync(string email, string password)
    {
        var user = await _userRepository.GetByEmailAsync(email);
        if (user == null || !user.IsActive)
            throw new AuthenticationException("User not found or inactive.");
 
        // Validate password against domain-hashed value
        if (!_passwordHasher.Verify(password, user._hashedPassword))
            throw new AuthenticationException("Invalid credentials.");
 
        return user;
    }
}
```

此处，`UserAuthenticationService` 是一个领域服务，它使用 `IUserRepository`（领域抽象）和 `IPasswordHasher`（领域特定的哈希）来对用户进行身份验证，
强制执行诸如 “非活跃用户无法登录” 之类的规则。

### 2.4 优缺点

**优点：**
- **业务关键的安全性**：规则在领域层强制执行，防止通过其他层（例如，直接访问领域的后台任务）绕过。
- **一致性**：安全逻辑集中在模型中，避免在 MVC 控制器或 API 中重复。
- **通用语言**：安全术语（例如 “管理员”、“所有者” ）是领域词汇的一部分，与业务团队保持一致。

**缺点：**
- **领域复杂性**：将身份验证逻辑与业务逻辑混合可能会使领域模型变得臃肿，使其更难维护。
- **紧耦合**：与外部身份验证提供程序（例如 OAuth、Azure AD）集成可能要用特定于框架的代码污染领域。
- **性能开销**：领域级别的检查（例如，为每个操作查询角色）如果未优化，可能会影响性能。

## 方法二：ASP.NET MVC 中的安全性

在这种方法中，安全性在表现层处理，使用 ASP.NET MVC 的内置工具来验证用户身份和授权操作。
领域模型则专注于业务逻辑，将安全性视为 “横切关注点”。

### 3.1 使用 ASP.NET Core Identity 进行身份验证

ASP.NET Core Identity 是一个流行的框架，用于在 MVC 应用中处理用户管理、身份验证和授权。
它抽象了用户存储（例如 SQL Server、Cosmos DB），并为登录/注册提供了预构建的 UI。

**示例：在 MVC 中设置 Identity**

```csharp
// Program.cs: Configure Identity with SQL Server
builder.Services.AddDbContext<AppDbContext>(options =>
    options.UseSqlServer(builder.Configuration.GetConnectionString("DefaultConnection")));
 
builder.Services.AddDefaultIdentity<IdentityUser>(options => options.SignIn.RequireConfirmedAccount = true)
    .AddRoles<IdentityRole>() // Enable roles
    .AddEntityFrameworkStores<AppDbContext>();
 
// Enable authentication/authorization middleware
app.UseAuthentication();
app.UseAuthorization();
```

`Identity` 的 `IdentityUser` 和 `IdentityRole` 取代了自定义的领域实体，由框架处理密码哈希、用户存储和会话管理。

### 3.2 通过属性和策略进行授权

MVC 使用 `[Authorize]` 属性和基于策略的授权来控制对控制器/操作的访问。

**示例：控制器级别的授权**

```csharp
[Authorize(Roles = "Admin")] // Only Admins can access this controller
public class UserManagementController : Controller
{
    private readonly UserManager<IdentityUser> _userManager;
 
    public UserManagementController(UserManager<IdentityUser> userManager)
    {
        _userManager = userManager;
    }
 
    [HttpPost]
    [Authorize(Policy = "RequirePasswordExpiryCheck")] // Custom policy
    public async Task<IActionResult> ResetPassword(string userId, string newPassword)
    {
        var user = await _userManager.FindByIdAsync(userId);
        if (user == null) return NotFound();
 
        var result = await _userManager.ResetPasswordAsync(user, 
            await _userManager.GeneratePasswordResetTokenAsync(user), newPassword);
        return result.Succeeded ? RedirectToAction("Index") : BadRequest(result.Errors);
    }
}
```

策略（例如 `RequirePasswordExpiryCheck`）在 `Program.cs` 中定义：

```csharp
// Program.cs: Define a policy requiring passwords to be changed every 90 days
builder.Services.AddAuthorization(options =>
{
    options.AddPolicy("RequirePasswordExpiryCheck", policy =>
        policy.RequireAssertion(context =>
        {
            var userManager = context.Resource as UserManager<IdentityUser>;
            var user = context.User;
            // Check if user's password is older than 90 days (pseudo-code)
            return userManager?.GetPasswordChangeDateAsync(user).Result < DateTime.UtcNow.AddDays(-90);
        }));
});
```

### 3.3 中间件和操作过滤器

对于自定义的身份验证逻辑（例如 JWT 验证、API 密钥检查），MVC 使用中间件或操作过滤器。

**示例：JWT 身份验证中间件**

```csharp
// Program.cs: Add JWT authentication for APIs
builder.Services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(options =>
    {
        options.TokenValidationParameters = new TokenValidationParameters
        {
            ValidateIssuer = true,
            ValidateAudience = true,
            ValidateLifetime = true,
            ValidateIssuerSigningKey = true,
            ValidIssuer = builder.Configuration["Jwt:Issuer"],
            ValidAudience = builder.Configuration["Jwt:Audience"],
            IssuerSigningKey = new SymmetricSecurityKey(Encoding.UTF8.GetBytes(builder.Configuration["Jwt:Key"]))
        };
    });
```

### 3.4 优缺点

**优点：**
- **关注点分离**：领域模型保持专注于业务逻辑，安全职责委托给框架。
- **框架集成**：利用 ASP.NET 久经考验的工具（Identity、JWT、OAuth）和社区支持。
- **灵活性**：通过 OAuth 轻松集成外部身份验证提供程序（例如 Google、Facebook），而不会污染领域。

**缺点：**
- **安全绕过风险**：如果其他层（例如后台任务）直接访问领域，MVC 的授权检查将被绕过。
- **重复**：授权规则可能跨控制器重复，导致不一致。
- **领域真实性缺失**：安全规则不是领域模型的一部分，因此业务团队可能与实现不一致。

## 大辩论：领域模型 vs. MVC

| 方面 | 领域模型方法 | MVC 方法 |
|------|-------------|----------|
| 强制执行范围 | 在领域被使用的任何地方强制执行 | 仅在 MVC 层强制执行 |
| 与 DDD 的一致性 | 高（安全作为业务规则） | 低（安全作为横切关注点） |
| 框架依赖 | 低（自定义逻辑） | 高（绑定 ASP.NET） |
| 外部身份验证支持 | 较难（需要领域集成） | 较容易（使用 Identity/OAuth） |
| 复杂性 | 较高（领域模型臃肿） | 较低（利用框架工具） |

## 混合方法：融合两者

最务实的解决方案往往结合了两种方法：

- **领域模型**：执行业务关键的授权规则（例如，“只有账户所有者才能转账”）。
- **MVC 层**：处理身份验证（例如 JWT、Identity）和表现层特定的安全（例如 UI 访问控制）。

**示例：混合授权**

```csharp
// Domain: User entity with core authorization (business rule)
public class User : AggregateRoot<UserId>
{
    public bool IsAccountVerified { get; private set; }
 
    public void VerifyAccount()
    {
        IsAccountVerified = true;
        AddDomainEvent(new AccountVerified(Id));
    }
}
 
// MVC: Controller using both domain and framework security
[Authorize] // MVC handles authentication
public class PaymentController : Controller
{
    private readonly IAggregateRepository<Order> _orderRepository;
    private readonly UserManager<IdentityUser> _userManager;
 
    public PaymentController(IAggregateRepository<Order> orderRepository, UserManager<IdentityUser> userManager)
    {
        _orderRepository = orderRepository;
        _userManager = userManager;
    }
 
    [HttpPost]
    public async Task<IActionResult> ProcessPayment(Guid orderId, decimal amount)
    {
        // 1. MVC: Get authenticated user
        var identityUser = await _userManager.GetUserAsync(User);
        var domainUser = await _userManager.Users
            .Where(u => u.Id == identityUser.Id)
            .Select(u => new User(new UserId(Guid.Parse(u.Id)), u.Email, ..., u.IsAccountVerified))
            .FirstAsync();
 
        // 2. Domain: Enforce business rule (verified account required)
        if (!domainUser.IsAccountVerified)
            return BadRequest("Only verified accounts can make payments.");
 
        // 3. Process payment (domain logic)
        var order = await _orderRepository.GetByIdAsync(new OrderId(orderId));
        order.ProcessPayment(amount, domainUser); // Domain method enforces "owner only" rule
        await _orderRepository.SaveAsync(order);
 
        return Ok();
    }
}
```

## C# 中安全 DDD 的最佳实践

- **优先将关键规则放在领域**：将业务关键操作（例如金融交易）的授权嵌入到领域模型中。
- **通过接口解耦**：使用接口（例如 `IUserAuthenticationService`）来抽象身份验证逻辑，允许 MVC 注入框架实现（例如 `Identity` ）。
- **利用 MVC 处理身份验证**：使用 ASP.NET Identity 或 JWT 进行身份验证，因为这些是久经考验的，并能减少样板代码。
- **测试安全规则**：为领域授权编写单元测试（例如，“没有 Admin 角色的用户不能删除记录”）。
- **记录边界**：明确界定哪些安全规则属于领域，哪些属于 MVC，以避免混淆。

## 结论

“领域模型 vs. MVC” 的安全争论，其核心在于平衡业务一致性（对齐）与实用性。
对于大多数应用程序而言，混合方法效果最佳：使用领域模型来强制执行核心业务安全规则，并使用 ASP.NET MVC 来处理身份验证和表现层访问控制。
这确保了安全既与业务对齐，又具备框架效率。

## 参考资料

- Evans, E. (2003). *Domain-Driven Design: Tackling Complexity in the Heart of Software*. Addison-Wesley.
- Vernon, V. (2013). *Implementing Domain-Driven Design*. Addison-Wesley.
- [ASP.NET Core Authentication Documentation](https://learn.microsoft.com/en-us/aspnet/core/security/authentication)
- [ASP.NET Core Authorization Documentation](https://learn.microsoft.com/en-us/aspnet/core/security/authorization)
- [Domain-Driven Design with .NET](https://ardalis.com/domain-driven-design-with-dotnet/) (Ardalis)

