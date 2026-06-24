# FastAPI 中的整洁架构与领域驱动设计（DDD）

[![Berkay Sonel](img/berkay-sonel.webp) Berkay Sonel](https://dev.to/berkaysson) 2026/6/14

---
\#fastapi \#ddd \#cleancode \#architecture

构建可扩展的后端应用程序需要清晰的关注点分离。
虽然 FastAPI 允许快速原型开发和灵活性，但应用像整洁架构和 DDD 这样的结构化模式，可以在项目增长时帮助维护性。

下面是一种在 FastAPI 应用程序中实现这些架构模式的结构化方法。

---
## 架构目录结构

为了将核心业务逻辑与外部框架、数据库和交付机制分离，代码库可以组织为不同的层：

```
backend/
├── domain/         # Core business entities and abstract interfaces (Zero external dependencies)
├── application/    # Application logic and use cases
├── infrastructure/ # Database implementations, file storage, external APIs
└── api/            # Presentation layer (FastAPI routers, dependency injection, schemas)
```

---
## 实现各层

### 1. 领域层（核心业务规则）

领域层代表应用程序的核心。
它完全独立于外部框架、数据库引擎或第三方库。
它定义了领域实体、值对象和 repository 接口（ports）。

#### 实体占位符

```python
# domain/entities/document.py

class Document:
    """Represents a core business domain entity."""
    def __init__(self, id: str, status: str):
        self.id = id
        self.status = status

    def process(self) -> None:
        """Domain-specific business logic."""
        self.status = "PROCESSING"
```

#### 抽象接口（Port）占位符

```python
# domain/interfaces/repository.py
from abc import ABC, abstractmethod

class IDocumentRepository(ABC):
    """Abstract interface defining database capabilities."""

    @abstractmethod
    def save(self, document: Document) -> None:
        pass

    @abstractmethod
    def find_by_id(self, id: str) -> Document:
        pass
```

### 2. 应用层（用例）

应用层协调执行流程。
它通过依赖领域层中定义的抽象接口（而非具体的数据库或基础设施类）来实现特定的用例。

#### 用例占位符

```python
# application/use_cases/process_document.py
from domain.interfaces.repository import IDocumentRepository

class ProcessDocumentUseCase:
    """Encapsulates a specific application workflow."""

    def __init__(self, repository: IDocumentRepository):
        # Dependencies are injected via the constructor
        self.repository = repository

    def execute(self, document_id: str) -> None:
        # Retrieve domain entity through the abstraction
        document = self.repository.find_by_id(document_id)

        # Execute domain business logic
        document.process()

        # Save updated domain state
        self.repository.save(document)
```

### **3. 基础设施层（Adapters）**

基础设施层包含领域中所定义抽象端口的具体实现。
这是数据库引擎（SQLAlchemy、Tortoise 等）、文件存储系统和外部服务客户端被集成的地方。

#### 仓储适配器占位符

```python
# infrastructure/database/sqlite_repository.py
from domain.entities.document import Document
from domain.interfaces.repository import IDocumentRepository

class SqliteDocumentRepository(IDocumentRepository):
    """Concrete SQLite implementation of the repository interface."""

    def save(self, document: Document) -> None:
        # Framework-specific database write logic (e.g., SQLAlchemy / raw SQL)
        pass

    def find_by_id(self, id: str) -> Document:
        # Framework-specific database read logic
        pass
```

### 4. 表现层（FastAPI 路由与依赖注入）

在这种设置中，FastAPI 严格充当交付机制。
API 层处理路由、序列化（Pydantic）、HTTP 状态码以及依赖注入（DI），以将具体的基础设施适配器组装到应用程序的用例中。

#### 依赖配置与路由占位符

```python
# api/dependencies.py
from fastapi import Depends
from infrastructure.database.sqlite_repository import SqliteDocumentRepository
from application.use_cases.process_document import ProcessDocumentUseCase

# Singleton or session factory definitions
_repository_instance = SqliteDocumentRepository()

def get_repository() -> SqliteDocumentRepository:
    return _repository_instance

def get_process_document_use_case(
    repository: SqliteDocumentRepository = Depends(get_repository)
) -> ProcessDocumentUseCase:
    return ProcessDocumentUseCase(repository)
```

```python
# api/routes.py
from fastapi import APIRouter, Depends, status
from api.dependencies import get_process_document_use_case
from application.use_cases.process_document import ProcessDocumentUseCase

router = APIRouter()

@router.post("/documents/{document_id}/process", status_code=status.HTTP_200_OK)
def process_document(
    document_id: str,
    use_case: ProcessDocumentUseCase = Depends(get_process_document_use_case)
):
    use_case.execute(document_id)
    return {"message": "Processing started"}
```

---

## 架构优势

- **数据库与框架独立性**：
核心业务逻辑与外部工具解耦。
从 SQLite 迁移到 PostgreSQL，或替换队列库，只需编写新的适配器，而无需修改核心应用规则。

- **可测试性**：
业务规则可以被独立验证。
由于用例依赖于接口，单元测试可以使用模拟适配器快速运行，无需数据库连接。

- **关注点分离**：
开发人员可以在清晰定义的边界内处理 API 端点、数据库查询和业务规则，从而降低在无关模块中引入意外副作用的风险。
