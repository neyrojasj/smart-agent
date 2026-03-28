# Node.js Best Practices & Standards

This document contains best practices for Node.js/TypeScript development that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **Node.js/TypeScript-specific guidelines**. The general standards take priority.

---

## Documentation Good Practices (Node.js/TypeScript)

- Document exported functions, classes, and interfaces with JSDoc/TSDoc
- For API handlers, document request/response shapes and error codes
- Update README or docs when env vars, scripts, or behavior change
- Keep TypeScript types and docs aligned; avoid docs that contradict actual types
- Include small usage examples for public modules when practical

---

## General Standards

> All FORBIDDEN patterns from `general.md` apply. Adapt to idiomatic TypeScript:
> - **Env vars**: Fail at startup. Use `zod` schema validation on `process.env`
> - **Errors**: Never swallow. Use `{ cause: error }` for error chaining, log with structured context
> - **Switch/conditionals**: Exhaustive checks. Use `const _: never = x` for compile-time safety or `Record<UnionType, T>` maps
> - **Non-null**: Never use `!` assertion. Use explicit null checks or guard clauses with descriptive errors

---

## Project Structure

### Recommended Layout
```
project/
├── package.json
├── package-lock.json
├── tsconfig.json           # TypeScript configuration
├── .eslintrc.js            # ESLint configuration
├── .prettierrc             # Prettier configuration
├── .env.example            # Environment variables template
├── src/
│   ├── index.ts            # Entry point
│   ├── config/             # Configuration management
│   │   └── index.ts
│   ├── controllers/        # Route handlers (for APIs)
│   │   └── userController.ts
│   ├── services/           # Business logic
│   │   └── userService.ts
│   ├── models/             # Data models
│   │   └── user.ts
│   ├── middlewares/        # Express/Fastify middlewares
│   │   └── auth.ts
│   ├── utils/              # Utility functions
│   │   └── helpers.ts
│   ├── types/              # TypeScript type definitions
│   │   └── index.ts
│   └── errors/             # Custom error classes
│       └── AppError.ts
├── tests/                  # Test files
│   ├── unit/
│   └── integration/
├── scripts/                # Build/utility scripts
└── docs/                   # Documentation
```

---

## TypeScript Configuration

### Recommended tsconfig.json
```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "lib": ["ES2022"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "tests"]
}
```

### TypeScript Best Practices
- Enable strict mode (`"strict": true`)
- Use `unknown` instead of `any` when type is truly unknown
- Define explicit return types for public functions
- Use type guards for runtime type checking
- Prefer interfaces for object types, types for unions/primitives

---

## Code Style

### Naming Conventions
| Item | Convention | Example |
|------|------------|---------|
| Files | camelCase or kebab-case | `userService.ts`, `user-service.ts` |
| Classes | PascalCase | `UserService` |
| Interfaces | PascalCase (no I prefix) | `User`, `UserRepository` |
| Types | PascalCase | `UserId`, `ApiResponse` |
| Functions | camelCase | `getUserById` |
| Variables | camelCase | `userName` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRY_COUNT` |
| Enums | PascalCase (members too) | `UserRole.Admin` |

### ESLint Configuration
```javascript
// .eslintrc.js
module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'prettier'
  ],
  parserOptions: {
    project: './tsconfig.json',
  },
  rules: {
    '@typescript-eslint/explicit-function-return-type': 'warn',
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    '@typescript-eslint/no-explicit-any': 'error',
    '@typescript-eslint/prefer-nullish-coalescing': 'error',
    '@typescript-eslint/prefer-optional-chain': 'error',
    'no-console': ['warn', { allow: ['warn', 'error'] }],
  },
};
```

### Prettier Configuration
```json
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 100,
  "arrowParens": "always"
}
```

---

## Error Handling

### Custom Error Classes
```typescript
// src/errors/AppError.ts
export class AppError extends Error {
  constructor(
    public readonly message: string,
    public readonly statusCode: number = 500,
    public readonly code: string = 'INTERNAL_ERROR',
    public readonly isOperational: boolean = true
  ) {
    super(message);
    this.name = this.constructor.name;
    Error.captureStackTrace(this, this.constructor);
  }
}

export class NotFoundError extends AppError {
  constructor(resource: string) {
    super(`${resource} not found`, 404, 'NOT_FOUND');
  }
}

export class ValidationError extends AppError {
  constructor(message: string) {
    super(message, 400, 'VALIDATION_ERROR');
  }
}
```

### Error Handling Patterns
```typescript
// Async error wrapper for Express
export const asyncHandler = (fn: RequestHandler): RequestHandler => {
  return (req, res, next) => {
    Promise.resolve(fn(req, res, next)).catch(next);
  };
};

// Global error handler
export const errorHandler: ErrorRequestHandler = (err, req, res, next) => {
  if (err instanceof AppError) {
    return res.status(err.statusCode).json({
      status: 'error',
      code: err.code,
      message: err.message,
    });
  }

  // Log unexpected errors
  console.error('Unexpected error:', err);
  
  return res.status(500).json({
    status: 'error',
    code: 'INTERNAL_ERROR',
    message: 'An unexpected error occurred',
  });
};
```

### Error Handling Guidelines
- Always use try-catch for async operations
- Create custom error classes for domain errors
- Never expose internal error details to clients
- Log all errors with appropriate context
- Use error monitoring (Sentry, DataDog, etc.)

---

## Async Patterns

### Promises Best Practices
```typescript
// Use async/await over .then()
async function fetchUser(id: string): Promise<User> {
  const response = await fetch(`/api/users/${id}`);
  if (!response.ok) {
    throw new NotFoundError('User');
  }
  return response.json();
}

// Handle multiple async operations
const [users, posts] = await Promise.all([
  fetchUsers(),
  fetchPosts(),
]);

// Use Promise.allSettled for independent operations
const results = await Promise.allSettled([
  sendEmail(user1),
  sendEmail(user2),
  sendEmail(user3),
]);
```

### Avoid Common Pitfalls
```typescript
// ❌ Bad: Sequential when parallel is possible
for (const id of ids) {
  await processItem(id);
}

// ✅ Good: Parallel processing
await Promise.all(ids.map((id) => processItem(id)));

// ✅ Better: Controlled concurrency with p-limit
import pLimit from 'p-limit';
const limit = pLimit(5);
await Promise.all(ids.map((id) => limit(() => processItem(id))));
```

---

## Security

### Environment Variables
```typescript
// src/config/index.ts
import { z } from 'zod';

const envSchema = z.object({
  NODE_ENV: z.enum(['development', 'production', 'test']),
  PORT: z.string().transform(Number),
  DATABASE_URL: z.string().url(),
  JWT_SECRET: z.string().min(32),
  API_KEY: z.string(),
});

export const config = envSchema.parse(process.env);
```

### Security Best Practices
```typescript
// Use helmet for security headers
import helmet from 'helmet';
app.use(helmet());

// Rate limiting
import rateLimit from 'express-rate-limit';
app.use(rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100,
}));

// Input validation with Zod
import { z } from 'zod';
const createUserSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
  name: z.string().min(2).max(100),
});
```

### Security Checklist
- [ ] Validate all input data
- [ ] Use parameterized queries (prevent SQL injection)
- [ ] Implement rate limiting
- [ ] Use HTTPS in production
- [ ] Store secrets in environment variables
- [ ] Implement proper authentication/authorization
- [ ] Set security headers (helmet)
- [ ] Keep dependencies updated
- [ ] Run `npm audit` regularly

---

## Testing

### Test Structure
```typescript
// tests/unit/userService.test.ts
import { describe, it, expect, beforeEach, vi } from 'vitest';
import { UserService } from '../../src/services/userService';

describe('UserService', () => {
  let userService: UserService;
  let mockRepository: MockUserRepository;

  beforeEach(() => {
    mockRepository = createMockRepository();
    userService = new UserService(mockRepository);
  });

  describe('getUserById', () => {
    it('should return user when found', async () => {
      // Arrange
      const expectedUser = { id: '1', name: 'Test' };
      mockRepository.findById.mockResolvedValue(expectedUser);

      // Act
      const result = await userService.getUserById('1');

      // Assert
      expect(result).toEqual(expectedUser);
    });

    it('should throw NotFoundError when user not found', async () => {
      // Arrange
      mockRepository.findById.mockResolvedValue(null);

      // Act & Assert
      await expect(userService.getUserById('1'))
        .rejects
        .toThrow(NotFoundError);
    });
  });
});
```

### Testing Guidelines
- Use Vitest or Jest for testing
- Aim for 80%+ code coverage on business logic
- Test edge cases and error scenarios
- Use factories for test data
- Mock external dependencies
- Write integration tests for critical paths

---

## Logging

### Structured Logging
```typescript
// src/utils/logger.ts
import pino from 'pino';

export const logger = pino({
  level: process.env.LOG_LEVEL ?? 'info',
  transport: process.env.NODE_ENV === 'development'
    ? { target: 'pino-pretty' }
    : undefined,
});

// Usage
logger.info({ userId, action: 'login' }, 'User logged in');
logger.error({ err, requestId }, 'Request failed');
```

### Logging Best Practices
- Use structured logging (JSON format)
- Include request IDs for tracing
- Log at appropriate levels (error, warn, info, debug)
- Never log sensitive data (passwords, tokens)
- Include context with every log

---

## Dependency Management

### package.json Best Practices
```json
{
  "name": "my-project",
  "version": "1.0.0",
  "engines": {
    "node": ">=20.0.0"
  },
  "scripts": {
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "tsx watch src/index.ts",
    "test": "vitest",
    "test:coverage": "vitest --coverage",
    "lint": "eslint src --ext .ts",
    "lint:fix": "eslint src --ext .ts --fix",
    "format": "prettier --write src/**/*.ts",
    "typecheck": "tsc --noEmit"
  }
}
```

### Dependency Guidelines
- Use exact versions for critical dependencies
- Run `npm audit` regularly
- Use `npm ci` (or `pnpm install --frozen-lockfile`) in CI/CD pipelines
- Document why each dependency is needed
- Prefer well-maintained packages with good TypeScript support
- Keep `devDependencies` and `dependencies` separate

### Package Managers

| Manager | When to Use |
|---------|-------------|
| **pnpm** | Default recommendation — strict, fast, disk-efficient |
| **npm** | When pnpm is unavailable or project already uses npm |
| **bun** | For scripts, dev tooling, or projects that embrace Bun runtime |

```bash
# ✅ Preferred - pnpm (strict dependency resolution)
pnpm install
pnpm add zod
pnpm run build

# ✅ Also good - bun for speed
bun install
bun add zod
bun run build
```

---

## Performance

### Streams & Backpressure

```typescript
import { pipeline } from 'stream/promises';
import { createReadStream, createWriteStream } from 'fs';
import { Transform } from 'stream';

// ✅ Good - Use pipeline() for automatic backpressure handling
await pipeline(
  createReadStream('large-file.csv'),
  new Transform({
    transform(chunk, encoding, callback) {
      callback(null, processChunk(chunk));
    },
  }),
  createWriteStream('output.csv')
);

// ❌ Bad - Manual pipe without error handling or backpressure
readable.pipe(writable); // Errors are not propagated!
```

### Worker Threads

```typescript
import { Worker, isMainThread, parentPort, workerData } from 'worker_threads';

// ✅ Good - Offload CPU-intensive work to a worker
if (isMainThread) {
  const worker = new Worker(new URL('./worker.ts', import.meta.url), {
    workerData: { input: largeDataset },
  });
  worker.on('message', (result) => handleResult(result));
  worker.on('error', (err) => logger.error('Worker failed', { err }));
} else {
  const result = heavyComputation(workerData.input);
  parentPort!.postMessage(result);
}
```

### Caching

```typescript
// Cache expensive operations
import NodeCache from 'node-cache';
const cache = new NodeCache({ stdTTL: 600 });
```

### Memory Management
- Avoid memory leaks in event listeners
- Use streams for large files
- Implement proper connection pooling
- Monitor memory usage in production
- Use weak references when appropriate

---

## API Design

### RESTful API Guidelines
```typescript
// Route structure
// GET    /users          - List users
// POST   /users          - Create user
// GET    /users/:id      - Get user
// PUT    /users/:id      - Update user (full)
// PATCH  /users/:id      - Update user (partial)
// DELETE /users/:id      - Delete user

// Response format
interface ApiResponse<T> {
  status: 'success' | 'error';
  data?: T;
  error?: {
    code: string;
    message: string;
  };
  meta?: {
    page: number;
    limit: number;
    total: number;
  };
}
```

---

## CI/CD Recommendations

### GitHub Actions Workflow
```yaml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - run: npm ci
      - run: npm run lint
      - run: npm run typecheck
      - run: npm run test:coverage
      - run: npm run build
      - run: npm audit --audit-level=high
```

---

## Summary Checklist

When reviewing or creating Node.js/TypeScript code:

### General Standards (MUST)
- [ ] **No default env vars** - Use Zod validation, fail if undefined
- [ ] **No silent errors** - All catch blocks handle/log/rethrow
- [ ] **No catch-all switch** - Use exhaustive checks with `never`
- [ ] **No unsafe `!` assertions** - Explicit null checks required

### Node.js/TypeScript-Specific Standards
- [ ] TypeScript strict mode enabled
- [ ] No `any` types (use `unknown` if needed)
- [ ] Proper error handling with custom errors
- [ ] Input validation on all endpoints
- [ ] Tests for new functionality
- [ ] No ESLint warnings
- [ ] Formatted with Prettier
- [ ] Dependencies audited
- [ ] Sensitive data not logged
- [ ] Environment variables validated with schema
