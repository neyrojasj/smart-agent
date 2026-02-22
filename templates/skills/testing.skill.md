---
name: testing
description: Create tests with mocks, validate behavior, and improve coverage.
---

# Testing Skill

## Identity

- **Name**: testing
- **Version**: 1.0
- **Description**: Creates tests, manages coverage, and validates code with mocking data.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: test, coverage, spec, mock | "Write tests for UserService" |
| "add...tests" | "Add tests for the new API" |
| "test...coverage" | "Improve test coverage" |
| "create...spec" | "Create a spec for this function" |
| Auto-chain from coding.skill | After code implementation |

---

## Capabilities

What this skill can do:

- ✅ Generate unit tests
- ✅ Generate integration tests
- ✅ Create test fixtures and mocks
- ✅ Set up test data factories
- ✅ Analyze coverage gaps
- ✅ Follow project's existing test patterns

---

## Dependencies

- `context.md` - For project context
- `.copilot/docs/testing.md` - For test strategy
- `coding.skill` - Chains from after implementation
- Existing tests - For pattern matching

---

## Workflow

### Step 1: Understand Test Requirements

```
1. Read .copilot/docs/testing.md for:
   - Test framework (Jest, Vitest, pytest, etc.)
   - Coverage requirements
   - Test structure conventions
   
2. Read existing tests to match patterns
```

### Step 2: Identify Test Scope

| Code Type | Test Type | Priority |
|-----------|-----------|----------|
| Functions | Unit tests | High |
| API endpoints | Integration tests | High |
| Components | Component tests | Medium |
| Utilities | Unit tests | Medium |
| UI flows | E2E tests | Low |

### Step 3: Generate Tests with Mocking

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MANDATORY: USE MOCKING DATA                                            │
│                                                                         │
│  ALL tests MUST use mocked data:                                        │
│  • Mock external API calls                                              │
│  • Mock database operations                                             │
│  • Mock file system operations                                          │
│  • Mock time-dependent functions                                        │
│                                                                         │
│  NEVER make real external calls in tests                                │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 4: Test Structure Template

#### Unit Test Template

```typescript
// [language: TypeScript/Jest example]
describe('[ModuleName]', () => {
  // Setup
  beforeEach(() => {
    // Reset mocks
  });

  describe('[functionName]', () => {
    it('should [expected behavior] when [condition]', () => {
      // Arrange
      const mockData = { /* mocked input */ };
      
      // Act
      const result = functionName(mockData);
      
      // Assert
      expect(result).toEqual(expected);
    });

    it('should handle [edge case]', () => {
      // Test edge case
    });

    it('should throw when [error condition]', () => {
      // Test error handling
    });
  });
});
```

#### Integration Test Template

```typescript
describe('[API Endpoint]', () => {
  // Mocked dependencies
  let mockDb: MockDatabase;
  let mockAuth: MockAuthService;

  beforeAll(() => {
    mockDb = createMockDatabase();
    mockAuth = createMockAuth();
  });

  afterEach(() => {
    mockDb.reset();
  });

  describe('GET /api/[resource]', () => {
    it('should return [expected] when authenticated', async () => {
      // Arrange
      mockAuth.setUser({ id: 'test-user' });
      mockDb.seed([{ /* test data */ }]);

      // Act
      const response = await request(app).get('/api/resource');

      // Assert
      expect(response.status).toBe(200);
      expect(response.body).toMatchObject({ /* expected */ });
    });

    it('should return 401 when not authenticated', async () => {
      mockAuth.setUser(null);
      const response = await request(app).get('/api/resource');
      expect(response.status).toBe(401);
    });
  });
});
```

### Step 5: Mock Data Factories

Create reusable mock data:

```typescript
// tests/factories/user.factory.ts
export const createMockUser = (overrides = {}) => ({
  id: 'test-user-id',
  email: 'test@example.com',
  name: 'Test User',
  createdAt: new Date('2024-01-01'),
  ...overrides
});

// tests/mocks/database.mock.ts
export const createMockDatabase = () => ({
  users: [],
  seed: function(data) { this.users = data; },
  reset: function() { this.users = []; },
  findUser: function(id) { return this.users.find(u => u.id === id); }
});
```

### Step 6: Coverage Requirements

```
┌─────────────────────────────────────────────────────────────────────────┐
│  TEST COVERAGE TARGETS                                                  │
│                                                                         │
│  Minimum coverage (unless project specifies otherwise):                 │
│  • Statements: 80%                                                      │
│  • Branches: 70%                                                        │
│  • Functions: 80%                                                       │
│  • Lines: 80%                                                           │
│                                                                         │
│  Critical paths (auth, payments, etc.): 90%+                            │
└─────────────────────────────────────────────────────────────────────────┘
```

### Step 7: Test File Naming

| Framework | Convention |
|-----------|------------|
| Jest/Vitest | `*.test.ts` or `*.spec.ts` |
| pytest | `test_*.py` or `*_test.py` |
| Go | `*_test.go` |
| Rust | `#[test]` in same file or `tests/` |

### Step 8: Run and Verify

After creating tests:
```
1. Run test suite
2. Verify all tests pass
3. Check coverage meets requirements
4. Report results
```

---

## Test Checklist

Before completing:

- [ ] Tests created for new/modified code
- [ ] Mocking used for all external dependencies
- [ ] Edge cases covered
- [ ] Error conditions tested
- [ ] Tests follow project patterns
- [ ] All tests pass
- [ ] Coverage maintained or improved

---

## Output Format

Return to orchestrator:

```yaml
status: success | error
result:
  tests_created: [list of test files]
  tests_count: [number]
  mocks_created: [list]
  coverage:
    before: "[X%]"
    after: "[Y%]"
  all_passing: true|false
context_updates:
  recent_actions:
    - "Created tests for [component] - [N] tests, [coverage]%"
next_skill: null
user_message: "[Summary of test creation]"
```

---

## Test Exceptions

Testing can be skipped ONLY if:

```
1. Change is purely UI/visual with no logic
2. Testing is technically impossible (document why)
3. User explicitly requests skipping tests
```

If exception applies, document it:
```markdown
⚠️ **Tests skipped**: [Reason]
```

---

## Never Do

- ❌ Create tests that make real external calls
- ❌ Skip testing without valid reason
- ❌ Use real user data in tests
- ❌ Create tests that depend on execution order
- ❌ Leave flaky tests (random failures)
- ❌ Ignore existing test patterns
- ❌ Create tests without assertions

---

## Framework-Specific Notes

### Jest/Vitest (JavaScript/TypeScript)
```javascript
// Mock imports
jest.mock('./database');
// Mock implementations
const mockFn = jest.fn().mockResolvedValue(data);
```

### pytest (Python)
```python
# Fixtures
@pytest.fixture
def mock_db():
    return MockDatabase()

# Mocking
from unittest.mock import patch, MagicMock
```

### Go
```go
// Interface-based mocking
type MockDB struct {
    FindUserFn func(id string) (*User, error)
}

func (m *MockDB) FindUser(id string) (*User, error) {
    return m.FindUserFn(id)
}
```

### Rust
```rust
// Mock traits
#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::mock;
}
```
