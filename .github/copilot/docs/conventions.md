<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# Code Conventions

## Language Configuration

| Tool | Config File | Purpose |
|------|-------------|---------|
| [TypeScript] | `tsconfig.json` | Type checking settings |
| [ESLint] | `.eslintrc.js` | Linting rules |
| [Prettier] | `.prettierrc` | Code formatting |

---

## Naming Conventions

### Files & Directories

| Type | Convention | Example |
|------|------------|---------|
| Directories | kebab-case | `user-management/` |
| Source files | kebab-case | `user-service.ts` |
| Test files | kebab-case + suffix | `user-service.test.ts` |
| React components | PascalCase | `UserProfile.tsx` |
| Config files | kebab-case | `jest.config.js` |

### Code Elements

| Type | Convention | Example |
|------|------------|---------|
| Classes | PascalCase | `UserService` |
| Interfaces | PascalCase (no I prefix) | `User`, `UserRepository` |
| Types | PascalCase | `CreateUserInput` |
| Functions | camelCase | `getUserById` |
| Variables | camelCase | `userCount` |
| Constants | SCREAMING_SNAKE | `MAX_RETRIES` |
| Enums | PascalCase | `UserRole.Admin` |
| Boolean vars | is/has/can prefix | `isActive`, `hasPermission` |

### Database

| Type | Convention | Example |
|------|------------|---------|
| Tables | snake_case (plural) | `users`, `user_roles` |
| Columns | snake_case | `created_at`, `is_active` |
| Indexes | table_column_idx | `users_email_idx` |

---

## Code Style

### Imports Order

```typescript
// 1. Node.js built-ins
import fs from 'fs';
import path from 'path';

// 2. External dependencies
import express from 'express';
import { z } from 'zod';

// 3. Internal modules (absolute paths)
import { UserService } from '@/services/user-service';
import { logger } from '@/utils/logger';

// 4. Relative imports
import { validateInput } from './validators';
import type { User } from './types';
```

### Function Structure

```typescript
/**
 * Brief description of what the function does.
 * 
 * @param param1 - Description of first parameter
 * @param param2 - Description of second parameter
 * @returns Description of return value
 * @throws {ErrorType} When this error occurs
 */
export async function functionName(
  param1: Type1,
  param2: Type2
): Promise<ReturnType> {
  // Early returns for edge cases
  if (!param1) {
    throw new ValidationError('param1 is required');
  }

  // Main logic
  const result = await doSomething(param1, param2);

  // Return
  return result;
}
```

### Error Handling

```typescript
// Define custom errors
class NotFoundError extends Error {
  constructor(resource: string, id: string) {
    super(`${resource} with id ${id} not found`);
    this.name = 'NotFoundError';
  }
}

// Use specific error types
try {
  const user = await userService.findById(id);
} catch (error) {
  if (error instanceof NotFoundError) {
    // Handle not found
  } else {
    // Re-throw unexpected errors
    throw error;
  }
}
```

---

## Design Patterns

### Used Patterns

| Pattern | Where Used | Purpose |
|---------|------------|---------|
| Repository | Data access | Abstract database operations |
| Service | Business logic | Encapsulate domain logic |
| Factory | Object creation | Centralize complex construction |
| Dependency Injection | Throughout | Improve testability |

### Repository Pattern Example

```typescript
interface UserRepository {
  findById(id: string): Promise<User | null>;
  findByEmail(email: string): Promise<User | null>;
  create(data: CreateUserInput): Promise<User>;
  update(id: string, data: UpdateUserInput): Promise<User>;
  delete(id: string): Promise<void>;
}
```

### Service Pattern Example

```typescript
class UserService {
  constructor(
    private userRepo: UserRepository,
    private emailService: EmailService
  ) {}

  async createUser(input: CreateUserInput): Promise<User> {
    // Business logic here
    const user = await this.userRepo.create(input);
    await this.emailService.sendWelcome(user.email);
    return user;
  }
}
```

---

## Git Conventions

### Branch Naming

| Type | Format | Example |
|------|--------|---------|
| Feature | `feature/description` | `feature/user-auth` |
| Bug fix | `fix/description` | `fix/login-redirect` |
| Hotfix | `hotfix/description` | `hotfix/security-patch` |
| Chore | `chore/description` | `chore/update-deps` |
| Docs | `docs/description` | `docs/api-readme` |

### Commit Messages

Format: `type(scope): description`

| Type | Use For |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation |
| `style` | Formatting (no code change) |
| `refactor` | Code restructuring |
| `test` | Adding/updating tests |
| `chore` | Maintenance tasks |

**Examples:**
```
feat(auth): add JWT refresh token support
fix(api): handle null user in profile endpoint  
docs(readme): update installation instructions
refactor(user): extract validation to separate module
```

### Pull Requests

**Title**: Same format as commits
**Description template**:
```markdown
## Summary
Brief description of changes

## Changes
- Change 1
- Change 2

## Testing
How to test these changes

## Checklist
- [ ] Tests pass
- [ ] Linting passes
- [ ] Documentation updated
```

---

## Documentation

### Code Comments

```typescript
// ✅ Good: Explain WHY, not what
// Using binary search because the list is always sorted
// and can contain millions of items
const index = binarySearch(items, target);

// ❌ Bad: Explains obvious code
// Loop through users
for (const user of users) { ... }
```

### JSDoc

Required for:
- Public APIs
- Complex functions
- Non-obvious parameters

```typescript
/**
 * Calculates the optimal batch size based on system resources.
 * 
 * @param totalItems - Total number of items to process
 * @param memoryLimit - Available memory in MB (default: 512)
 * @returns Optimal batch size, minimum 10, maximum 1000
 * 
 * @example
 * const batchSize = calculateBatchSize(10000, 256);
 * // Returns: 250
 */
```

---
*Last updated: [DATE]*
