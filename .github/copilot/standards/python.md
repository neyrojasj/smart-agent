# Python Programming Best Practices & Standards

This document contains best practices for Python development that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **Python-specific guidelines**. The general standards take priority.

---

## Documentation Good Practices (Python)

- Add docstrings to public modules, classes, and functions (PEP 257 style)
- Document function parameters, return values, and raised exceptions
- Keep examples in docstrings runnable and synchronized with current behavior
- Update docs when CLI flags, env vars, or config contracts change
- Prefer concise comments that explain intent, edge cases, and trade-offs

---

## General Standards

> All FORBIDDEN patterns from `general.md` apply. Adapt to idiomatic Python:
> - **Env vars**: Fail at startup. Use `os.environ["KEY"]` (raises `KeyError`) or `pydantic_settings.BaseSettings`
> - **Errors**: Never bare `except:` or empty `except` blocks. Catch specific exceptions, log with `exc_info=True`, re-raise with `from e`
> - **Pattern matching**: Exhaustive `match` (3.10+) or dict mapping. No catch-all `_` for known enum values

---

## PEP 8 Style Guide

### Code Layout

```python
# Indentation: 4 spaces (never tabs)

# Maximum line length: 79 characters for code, 72 for docstrings/comments

# Blank lines:
# - 2 blank lines around top-level definitions
# - 1 blank line around method definitions


class MyClass:
    """Class docstring."""
    
    def method_one(self):
        """Method docstring."""
        pass
    
    def method_two(self):
        """Method docstring."""
        pass


def top_level_function():
    """Function docstring."""
    pass
```

### Imports

```python
# ✅ Good - Imports at top, grouped and ordered
# 1. Standard library
import os
import sys
from pathlib import Path

# 2. Third-party packages
import requests
from pydantic import BaseModel

# 3. Local imports
from myapp.config import settings
from myapp.models import User

# ❌ Bad - Wildcard imports
from mymodule import *  # Never do this

# ❌ Bad - Multiple imports on one line
import os, sys, json
```

---

## Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Modules | lowercase with underscores | `user_service.py` |
| Packages | lowercase, no underscores preferred | `mypackage` |
| Classes | PascalCase | `UserService`, `HttpClient` |
| Functions | lowercase with underscores | `get_user_by_id` |
| Variables | lowercase with underscores | `user_name`, `total_count` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRIES`, `DEFAULT_TIMEOUT` |
| Private | leading underscore | `_internal_method` |
| "Protected" | leading underscore | `_protected_attr` |
| Name-mangled | double leading underscore | `__private` |

---

## Type Hints

### Modern Type Annotations (Python 3.10+)

```python
from typing import Any
from collections.abc import Sequence, Mapping

# ✅ Good - Use built-in types directly (3.9+)
def process_items(items: list[str]) -> dict[str, int]:
    return {item: len(item) for item in items}

# ✅ Good - Union with | (3.10+)
def find_user(user_id: int) -> User | None:
    return users.get(user_id)

# ✅ Good - Use Sequence/Mapping for read-only params
def process(items: Sequence[str]) -> None:
    for item in items:
        print(item)

# ✅ Good - TypedDict for structured dicts
from typing import TypedDict

class UserDict(TypedDict):
    id: int
    name: str
    email: str

# ✅ Good - Literal for exact values
from typing import Literal

def set_log_level(level: Literal["DEBUG", "INFO", "WARNING", "ERROR"]) -> None:
    pass
```

### Type Checking with mypy

```python
# Run mypy for static type checking
# mypy --strict myapp/

# pyproject.toml configuration
# [tool.mypy]
# python_version = "3.12"
# strict = true
# warn_return_any = true
# warn_unused_configs = true
```

---

## Error Handling

### Context Managers

```python
# ✅ Good - Use context managers for resource cleanup
with open("file.txt") as f:
    content = f.read()

# ✅ Good - Multiple resources
with open("input.txt") as infile, open("output.txt", "w") as outfile:
    outfile.write(infile.read().upper())

# ✅ Good - Custom context manager
from contextlib import contextmanager

@contextmanager
def database_transaction(conn):
    cursor = conn.cursor()
    try:
        yield cursor
        conn.commit()
    except Exception:
        conn.rollback()
        raise
    finally:
        cursor.close()
```

### Custom Exceptions

```python
# ✅ Good - Define domain-specific exceptions
class ServiceError(Exception):
    """Base exception for service layer."""
    pass

class NotFoundError(ServiceError):
    """Resource not found."""
    def __init__(self, resource: str, id: str):
        self.resource = resource
        self.id = id
        super().__init__(f"{resource} with id {id} not found")

class ValidationError(ServiceError):
    """Validation failed."""
    def __init__(self, field: str, message: str):
        self.field = field
        self.message = message
        super().__init__(f"Validation error on {field}: {message}")

# Usage
def get_user(user_id: str) -> User:
    user = db.get(user_id)
    if user is None:
        raise NotFoundError("User", user_id)
    return user
```

---

## Project Structure

### Recommended Layout

```
project/
├── pyproject.toml          # Project configuration
├── README.md
├── src/
│   └── myapp/
│       ├── __init__.py
│       ├── main.py         # Entry point
│       ├── config.py       # Configuration
│       ├── models/
│       │   ├── __init__.py
│       │   └── user.py
│       ├── services/
│       │   ├── __init__.py
│       │   └── user_service.py
│       └── api/
│           ├── __init__.py
│           └── routes.py
├── tests/
│   ├── __init__.py
│   ├── conftest.py         # pytest fixtures
│   ├── unit/
│   │   └── test_user_service.py
│   └── integration/
│       └── test_api.py
└── scripts/
    └── run_migrations.py
```

### pyproject.toml

```toml
[project]
name = "myapp"
version = "1.0.0"
requires-python = ">=3.11"
dependencies = [
    "fastapi>=0.100.0",
    "pydantic>=2.0.0",
    "sqlalchemy>=2.0.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0.0",
    "pytest-cov>=4.0.0",
    "mypy>=1.0.0",
    "ruff>=0.1.0",
]

[tool.ruff]
line-length = 88
target-version = "py311"

[tool.ruff.lint]
select = ["E", "F", "W", "I", "UP", "B", "SIM"]

[tool.mypy]
python_version = "3.11"
strict = true
```

---

## Testing

### pytest Patterns

```python
# tests/conftest.py
import pytest
from myapp.models import User

@pytest.fixture
def sample_user() -> User:
    return User(id="123", name="Alice", email="alice@example.com")

@pytest.fixture
def mock_db(mocker):
    return mocker.patch("myapp.services.user_service.db")


# tests/unit/test_user_service.py
import pytest
from myapp.services.user_service import UserService
from myapp.exceptions import NotFoundError

class TestUserService:
    def test_get_user_returns_user_when_found(self, mock_db, sample_user):
        # Arrange
        mock_db.get.return_value = sample_user
        service = UserService(mock_db)
        
        # Act
        result = service.get_user("123")
        
        # Assert
        assert result.id == "123"
        assert result.name == "Alice"
    
    def test_get_user_raises_not_found_when_missing(self, mock_db):
        # Arrange
        mock_db.get.return_value = None
        service = UserService(mock_db)
        
        # Act & Assert
        with pytest.raises(NotFoundError) as exc_info:
            service.get_user("999")
        
        assert exc_info.value.resource == "User"
        assert exc_info.value.id == "999"
```

### Parametrized Tests

```python
import pytest

@pytest.mark.parametrize("input,expected", [
    ("hello", "HELLO"),
    ("world", "WORLD"),
    ("", ""),
    ("123", "123"),
])
def test_uppercase(input: str, expected: str):
    assert input.upper() == expected
```

---

## Virtual Environments

### Modern Tools

```bash
# venv (built-in)
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
.venv\Scripts\activate     # Windows

# uv (fast, modern alternative)
uv venv
uv pip install -e ".[dev]"

# pipx for CLI tools
pipx install ruff
pipx install mypy
```

---

## Linting and Formatting

### Ruff (Recommended)

```bash
# Lint and format
ruff check .
ruff format .

# Fix auto-fixable issues
ruff check --fix .
```

### Pre-commit Configuration

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.1.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format
  
  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.0.0
    hooks:
      - id: mypy
        additional_dependencies: [pydantic]
```

---

## Common Patterns

### Dataclasses and Pydantic

```python
from dataclasses import dataclass
from pydantic import BaseModel, EmailStr, Field

# ✅ Good - Dataclass for simple data containers
@dataclass
class Point:
    x: float
    y: float

# ✅ Good - Pydantic for validation
class CreateUserRequest(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    email: EmailStr
    age: int = Field(ge=0, le=150)

# Automatic validation
user = CreateUserRequest(name="Alice", email="alice@example.com", age=30)
```

### Pathlib for File Operations

```python
from pathlib import Path

# ✅ Good - Use pathlib instead of os.path
config_path = Path(__file__).parent / "config.yaml"
data_dir = Path.home() / ".myapp" / "data"

# Create directories
data_dir.mkdir(parents=True, exist_ok=True)

# Read/write files
content = config_path.read_text()
output_path.write_text(result)

# Iterate files
for file in data_dir.glob("*.json"):
    process(file)
```

### Async Concurrency (Python 3.11+)

```python
import asyncio

# ✅ Good - TaskGroup for structured concurrency (3.11+)
async def fetch_all(urls: list[str]) -> list[Response]:
    async with asyncio.TaskGroup() as tg:
        tasks = [tg.create_task(fetch(url)) for url in urls]
    return [t.result() for t in tasks]
    # If any task fails, all others are cancelled and
    # errors are raised as an ExceptionGroup

# ✅ Good - Handle ExceptionGroup (3.11+)
try:
    results = await fetch_all(urls)
except* ConnectionError as eg:
    for exc in eg.exceptions:
        logger.error(f"Connection failed: {exc}")
except* TimeoutError as eg:
    logger.warning(f"{len(eg.exceptions)} requests timed out")
```

### Package Management

```bash
# ✅ Preferred - uv (fast Rust-based resolver)
uv init myproject          # New project with pyproject.toml
uv add requests pydantic   # Add dependencies
uv sync                    # Install from lockfile
uv run pytest              # Run in managed environment

# ✅ Also acceptable - pip with constraints
pip install -r requirements.txt
```

---

## Summary Checklist

When reviewing or creating Python code:

### General Standards (MUST)
- [ ] **No default env vars** - Use pydantic-settings or explicit checks
- [ ] **No silent errors** - No bare except, no empty except blocks
- [ ] **No catch-all patterns** - Handle all enum cases explicitly

### Python-Specific Standards
- [ ] Type hints on all public functions
- [ ] mypy --strict passes
- [ ] PEP 8 compliant (run ruff)
- [ ] Context managers for resources
- [ ] Custom exceptions for domain errors
- [ ] pytest for testing with good coverage
- [ ] pathlib for file operations
- [ ] Virtual environment documented
- [ ] pyproject.toml for configuration
- [ ] No mutable default arguments
