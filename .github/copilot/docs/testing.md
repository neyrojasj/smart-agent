<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# Testing Strategy

## Overview

| Attribute | Value |
|-----------|-------|
| Primary Framework | [Jest / Vitest / pytest / cargo test] |
| E2E Framework | [Playwright / Cypress / None] |
| Coverage Tool | [c8 / istanbul / coverage.py] |
| CI Integration | [Yes / No] |

---

## Commands

### Running Tests

| Command | Purpose |
|---------|---------|
| `[npm test]` | Run all tests |
| `[npm run test:unit]` | Run unit tests only |
| `[npm run test:integration]` | Run integration tests only |
| `[npm run test:e2e]` | Run E2E tests |
| `[npm run test:watch]` | Watch mode |
| `[npm run test:coverage]` | Generate coverage report |

### Debugging Tests

```bash
# Run single test file
[npm test -- path/to/test.ts]

# Run tests matching pattern
[npm test -- --grep "pattern"]

# Debug in VS Code
# Use the built-in test runner or launch configuration
```

---

## Test Structure

```
tests/
├── unit/                  # Fast, isolated tests
│   ├── [module]/         # Organized by module
│   └── ...
├── integration/          # Tests with dependencies
│   ├── [feature]/       # Organized by feature
│   └── ...
├── e2e/                  # End-to-end tests
│   └── [flow]/          # Organized by user flow
├── fixtures/             # Shared test data
│   ├── [resource].ts    # Factory functions
│   └── ...
└── helpers/              # Test utilities
    └── setup.ts         # Global setup
```

---

## Coverage

### Targets

| Type | Target | Current |
|------|--------|---------|
| Statements | [80%] | [X%] |
| Branches | [75%] | [X%] |
| Functions | [80%] | [X%] |
| Lines | [80%] | [X%] |

### Excluded Paths

```
[List paths excluded from coverage]
- **/node_modules/**
- **/dist/**
- **/*.d.ts
- **/test/**
```

### Viewing Reports

```bash
# Generate HTML report
[npm run test:coverage]

# Open report
open coverage/index.html
```

---

## Test Types

### Unit Tests

**Purpose**: Test individual functions/classes in isolation

**Conventions**:
- File naming: `*.test.ts` or `*.spec.ts`
- Co-located with source or in `tests/unit/`
- Mock all external dependencies
- Fast execution (< 100ms per test)

**Example**:
```typescript
describe('UserService', () => {
  describe('validateEmail', () => {
    it('should return true for valid email', () => {
      expect(validateEmail('user@example.com')).toBe(true);
    });
    
    it('should return false for invalid email', () => {
      expect(validateEmail('invalid')).toBe(false);
    });
  });
});
```

### Integration Tests

**Purpose**: Test component interactions with real dependencies

**Conventions**:
- Located in `tests/integration/`
- Use test database/services
- Reset state between tests
- May be slower (seconds per test)

**Setup**:
```bash
# Start test dependencies
[docker-compose -f docker-compose.test.yml up -d]
```

### E2E Tests

**Purpose**: Test complete user flows

**Conventions**:
- Located in `tests/e2e/`
- Run against real or staging environment
- Organized by user journey
- Slowest tests (run last in CI)

---

## Mocking

### Strategy

| Dependency | Mock Method |
|------------|-------------|
| HTTP calls | [msw / nock / responses] |
| Database | [Test DB / in-memory] |
| Time | [jest.useFakeTimers() / freezegun] |
| File system | [memfs / mock-fs] |

### Example

```typescript
// Mock HTTP calls
import { setupServer } from 'msw/node';

const server = setupServer(
  rest.get('/api/users', (req, res, ctx) => {
    return res(ctx.json([{ id: 1, name: 'Test' }]));
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());
```

---

## Fixtures & Factories

### Location

`tests/fixtures/` or `tests/factories/`

### Pattern

```typescript
// tests/fixtures/user.ts
export const createUser = (overrides = {}) => ({
  id: 'user-123',
  email: 'test@example.com',
  name: 'Test User',
  ...overrides
});
```

---

## CI Configuration

```yaml
# Tests run in CI pipeline
# Location: [.github/workflows/test.yml or similar]

test:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: Install
      run: [npm ci]
    - name: Test
      run: [npm test]
    - name: Upload coverage
      uses: codecov/codecov-action@v3
```

---
*Last updated: [DATE]*
