# General Programming Standards

These are fundamental programming principles that apply across all languages. They prioritize explicit behavior, early failure detection, and code clarity over convenience.

---

## Anti-Hallucination: Ask, Don't Invent

> **When information is missing or ambiguous, ASK the user — never invent, assume, or fill in placeholder values.**

This is the single most important rule for AI-assisted coding. Violating it produces plausible-looking code that is silently wrong.

### NEVER invent

- Color hex values, font names, or design tokens not defined in a project style guide
- API endpoints, URL paths, or service names not found in existing code
- Environment variable names or configuration keys not present in `.env.example` or config files
- Business logic, domain rules, or validation constraints the user hasn't stated
- Database table names, column names, or schema details not visible in migrations/models
- Third-party library APIs — if unsure about a method signature, look it up in the codebase or docs first

### ALWAYS ask when

- A plan or specification is incomplete — list what's missing and ask before proceeding
- You're choosing between two valid approaches — present 2–3 options with tradeoffs
- The user's request is ambiguous — restate your interpretation and confirm before coding
- A required value (secret name, port, domain, config key) isn't in the codebase — ask for it explicitly
- You need to name something user-facing (routes, error messages, labels) — propose and confirm

### Evidence-based answers

- Reference actual file paths and line numbers when explaining existing code
- When suggesting a fix, show what you found vs. what you expected
- If you cannot find evidence in the codebase for a claim, say so: *"I couldn't find X in the codebase. Can you confirm...?"*
- Prefer reading a file over recalling from memory when the file is available

---

## Core Principles

### 1. Fail Fast, Fail Loud

Code should fail immediately and clearly when something is wrong, rather than silently continuing with incorrect behavior.

**Why?**
- Bugs are caught during development, not in production
- Debugging is easier when errors occur close to the source
- Silent failures can cascade into larger, harder-to-diagnose issues

### 2. SOLID Principles

All code should strive to follow the SOLID principles for maintainable, scalable software:

#### **S - Single Responsibility Principle (SRP)**
A class/module should have only one reason to change.

**❌ Bad:**
```typescript
class UserService {
    createUser(data: UserData) { /* ... */ }
    sendWelcomeEmail(user: User) { /* ... */ }  // Email is not user management
    generatePDFReport(user: User) { /* ... */ } // Reporting is not user management
}
```

**✅ Good:**
```typescript
class UserService {
    createUser(data: UserData) { /* ... */ }
}
class EmailService {
    sendWelcomeEmail(user: User) { /* ... */ }
}
class ReportService {
    generatePDFReport(user: User) { /* ... */ }
}
```

#### **O - Open/Closed Principle (OCP)**
Software entities should be open for extension but closed for modification.

**❌ Bad:**
```typescript
function calculateDiscount(customerType: string, amount: number): number {
    if (customerType === "regular") return amount * 0.1;
    if (customerType === "premium") return amount * 0.2;
    if (customerType === "vip") return amount * 0.3;  // Must modify function for new types
    return 0;
}
```

**✅ Good:**
```typescript
interface DiscountStrategy {
    calculate(amount: number): number;
}

class RegularDiscount implements DiscountStrategy {
    calculate(amount: number) { return amount * 0.1; }
}

class PremiumDiscount implements DiscountStrategy {
    calculate(amount: number) { return amount * 0.2; }
}

// New discount types can be added without modifying existing code
```

#### **L - Liskov Substitution Principle (LSP)**
Subtypes must be substitutable for their base types without altering program correctness.

**❌ Bad:**
```typescript
class Rectangle {
    setWidth(w: number) { this.width = w; }
    setHeight(h: number) { this.height = h; }
}

class Square extends Rectangle {
    setWidth(w: number) { this.width = w; this.height = w; }  // Breaks expectations!
    setHeight(h: number) { this.width = h; this.height = h; }
}
```

**✅ Good:**
```typescript
interface Shape {
    getArea(): number;
}

class Rectangle implements Shape {
    constructor(private width: number, private height: number) {}
    getArea() { return this.width * this.height; }
}

class Square implements Shape {
    constructor(private side: number) {}
    getArea() { return this.side * this.side; }
}
```

#### **I - Interface Segregation Principle (ISP)**
Clients should not be forced to depend on interfaces they don't use.

**❌ Bad:**
```typescript
interface Worker {
    work(): void;
    eat(): void;
    sleep(): void;
}

class Robot implements Worker {
    work() { /* ... */ }
    eat() { throw new Error("Robots don't eat"); }   // Forced to implement
    sleep() { throw new Error("Robots don't sleep"); } // Forced to implement
}
```

**✅ Good:**
```typescript
interface Workable { work(): void; }
interface Eatable { eat(): void; }
interface Sleepable { sleep(): void; }

class Human implements Workable, Eatable, Sleepable {
    work() { /* ... */ }
    eat() { /* ... */ }
    sleep() { /* ... */ }
}

class Robot implements Workable {
    work() { /* ... */ }
}
```

#### **D - Dependency Inversion Principle (DIP)**
High-level modules should not depend on low-level modules. Both should depend on abstractions.

**❌ Bad:**
```typescript
class MySQLDatabase {
    save(data: any) { /* MySQL specific */ }
}

class UserRepository {
    private db = new MySQLDatabase();  // Tightly coupled to MySQL
    save(user: User) { this.db.save(user); }
}
```

**✅ Good:**
```typescript
interface Database {
    save(data: any): void;
}

class MySQLDatabase implements Database {
    save(data: any) { /* MySQL specific */ }
}

class PostgresDatabase implements Database {
    save(data: any) { /* Postgres specific */ }
}

class UserRepository {
    constructor(private db: Database) {}  // Depends on abstraction
    save(user: User) { this.db.save(user); }
}
```

### 3. Documentation Is Part of the Code

Documentation must be updated in the same change whenever behavior, API contracts, or configuration change.

**Rules:**
- Update docs when changing runtime behavior or requirements
- Document public APIs with expected inputs, outputs, and failure modes
- Prefer comments that explain intent and trade-offs, not obvious code mechanics
- Keep examples and snippets executable and synchronized with real code
- Remove outdated docs in the same PR/commit to avoid drift

---

## Clean Code Principles

Based on Robert C. Martin's "Clean Code" and industry best practices.

### 3. Meaningful Names

Names should reveal intent and be unambiguous.

**Rules:**
- Use intention-revealing names
- Avoid single-letter names (except `i`, `j`, `k` for loops)
- Avoid abbreviations and encodings
- Class names = nouns (`User`, `OrderProcessor`)
- Method names = verbs (`getUser`, `processOrder`, `validateInput`)

**❌ Bad:**
```typescript
const d = new Date(); // What is 'd'?
const u = getU(id);   // What is 'u'? What does getU do?
const list = getData(); // What data?

function proc(x: any) { /* ... */ }
```

**✅ Good:**
```typescript
const currentDate = new Date();
const user = getUserById(userId);
const activeOrders = getActiveOrders();

function processPayment(paymentDetails: PaymentDetails) { /* ... */ }
```

### 4. Boy Scout Rule

> *"Always leave the code cleaner than you found it."* — Robert C. Martin

When you touch code:
- Fix minor issues you encounter
- Improve unclear variable names
- Extract long methods
- Remove dead code

**Don't:**
- Make unrelated large refactors in the same commit
- "Fix" things without understanding them

### 5. Comments

Comments should explain **why**, not **what**.

**❌ Bad:**
```typescript
// Increment counter by 1
counter++;

// Get the user from database
const user = userRepository.findById(id);

// TODO: fix this later
const result = hackyWorkaround();
```

**✅ Good:**
```typescript
// Counter tracks retry attempts for circuit breaker pattern
counter++;

// Bypass cache due to CAP theorem constraints in distributed system
const user = userRepository.findByIdDirect(id);

// Legal requirement: GDPR Article 17 mandates 30-day retention
const retentionDays = 30;
```

### 6. Avoid Magic Numbers

Replace hard-coded values with named constants.

**❌ Bad:**
```typescript
if (password.length < 8) { /* ... */ }
setTimeout(callback, 86400000);
if (statusCode === 401) { /* ... */ }
```

**✅ Good:**
```typescript
const MIN_PASSWORD_LENGTH = 8;
if (password.length < MIN_PASSWORD_LENGTH) { /* ... */ }

const ONE_DAY_MS = 24 * 60 * 60 * 1000;
setTimeout(callback, ONE_DAY_MS);

const HTTP_UNAUTHORIZED = 401;
if (statusCode === HTTP_UNAUTHORIZED) { /* ... */ }
```

---

## DRY, YAGNI, KISS

Three fundamental principles for clean, maintainable code.

### 7. DRY - Don't Repeat Yourself

Every piece of knowledge should have a single, authoritative representation.

**❌ Bad:**
```typescript
function validateUserEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

function validateOrderEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/; // Duplicated!
    return emailRegex.test(email);
}
```

**✅ Good:**
```typescript
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

function isValidEmail(email: string): boolean {
    return EMAIL_REGEX.test(email);
}

// Use isValidEmail everywhere
```

### 8. YAGNI - You Aren't Gonna Need It

Don't implement functionality until you actually need it.

**❌ Bad:**
```typescript
class UserService {
    createUser(data: UserData) { /* ... */ }
    
    // "We might need these later"
    createBulkUsers(data: UserData[]) { /* Never used */ }
    migrateUserFromLegacy(legacyId: string) { /* Never used */ }
    exportUserToCSV(userId: string) { /* Never used */ }
}
```

**✅ Good:**
```typescript
class UserService {
    createUser(data: UserData) { /* ... */ }
    // Add other methods when actually needed
}
```

### 9. KISS - Keep It Simple, Stupid

Prefer simple solutions over clever ones.

**❌ Bad:**
```typescript
// "Clever" one-liner that no one understands
const result = arr.reduce((a,c)=>({...a,[c.id]:c}),{});
```

**✅ Good:**
```typescript
// Clear and readable
const result: Record<string, Item> = {};
for (const item of items) {
    result[item.id] = item;
}
```

---

## Function Design

Based on Clean Code principles for writing effective functions.

### 10. Small Functions

Functions should be small and do one thing well.

**Guidelines:**
- Aim for < 20 lines per function
- Extract complex logic into well-named helper functions
- If you need to scroll to see the whole function, it's too long

**❌ Bad:**
```typescript
function processOrder(order: Order): ProcessedOrder {
    // 200 lines of validation, calculation, 
    // database operations, email sending...
}
```

**✅ Good:**
```typescript
function processOrder(order: Order): ProcessedOrder {
    validateOrder(order);
    const pricing = calculatePricing(order);
    const savedOrder = saveOrder(order, pricing);
    notifyCustomer(savedOrder);
    return savedOrder;
}
```

### 11. Few Arguments

Prefer fewer function arguments (ideally 0-3).

**❌ Bad:**
```typescript
function createUser(
    name: string,
    email: string,
    age: number,
    address: string,
    phone: string,
    role: string
) { /* ... */ }
```

**✅ Good:**
```typescript
interface CreateUserRequest {
    name: string;
    email: string;
    age: number;
    address: string;
    phone: string;
    role: string;
}

function createUser(request: CreateUserRequest) { /* ... */ }
```

### 12. No Side Effects

Functions should either do something (command) or answer something (query), not both.

**❌ Bad:**
```typescript
function getUser(id: string): User {
    const user = database.find(id);
    user.lastAccessed = new Date(); // Side effect!
    database.save(user);            // Side effect!
    return user;
}
```

**✅ Good:**
```typescript
function getUser(id: string): User {
    return database.find(id);
}

function recordUserAccess(userId: string): void {
    const user = database.find(userId);
    user.lastAccessed = new Date();
    database.save(user);
}
```

---

## Security Best Practices

Essential security principles for all code.

### 13. Input Validation

Never trust external input. Validate at system boundaries.

**❌ Bad:**
```typescript
app.post('/users', (req, res) => {
    const user = createUser(req.body); // Direct use of input!
    res.json(user);
});
```

**✅ Good:**
```typescript
app.post('/users', (req, res) => {
    const validated = userSchema.parse(req.body); // Validate first
    const user = createUser(validated);
    res.json(user);
});
```

### 14. No Secrets in Code

Never hardcode secrets, credentials, or API keys.

**❌ Bad:**
```typescript
const apiKey = "sk-1234567890abcdef";
const dbPassword = "super-secret-password";
```

**✅ Good:**
```typescript
const apiKey = process.env.API_KEY;
if (!apiKey) {
    throw new Error("API_KEY environment variable must be set");
}
```

### 15. Principle of Least Privilege

Grant minimum permissions necessary for functionality.

**Rules:**
- Request only needed permissions/scopes
- Use read-only access when writes aren't needed
- Limit database user permissions
- Use scoped tokens with expiration

### 16. Defense in Depth

Don't rely on a single security layer.

**Apply multiple protections:**
- Input validation AND output encoding
- Authentication AND authorization
- Encryption in transit AND at rest
- Application security AND infrastructure security

---

## Testing Principles

### 17. FIRST Principles for Tests

Tests should be:

| Letter | Principle | Meaning |
|--------|-----------|---------|
| **F** | Fast | Tests should run quickly |
| **I** | Independent | Tests should not depend on each other |
| **R** | Repeatable | Same result every time, any environment |
| **S** | Self-Validating | Pass or fail, no manual interpretation |
| **T** | Timely | Written before or with the code |

### 18. One Concept Per Test

Each test should verify a single behavior.

**❌ Bad:**
```typescript
test('user operations', () => {
    const user = createUser({ name: 'John' });
    expect(user.name).toBe('John');
    
    user.name = 'Jane';
    expect(user.name).toBe('Jane');
    
    deleteUser(user.id);
    expect(getUser(user.id)).toBeNull();
});
```

**✅ Good:**
```typescript
test('createUser sets the provided name', () => {
    const user = createUser({ name: 'John' });
    expect(user.name).toBe('John');
});

test('user name can be updated', () => {
    const user = createUser({ name: 'John' });
    user.name = 'Jane';
    expect(user.name).toBe('Jane');
});

test('deleteUser removes user from database', () => {
    const user = createUser({ name: 'John' });
    deleteUser(user.id);
    expect(getUser(user.id)).toBeNull();
});
```

### 19. Arrange-Act-Assert Pattern

Structure tests clearly:

```typescript
test('calculateTotal applies discount correctly', () => {
    // Arrange
    const cart = new Cart();
    cart.addItem({ price: 100 });
    const discount = new PercentageDiscount(10);
    
    // Act
    const total = cart.calculateTotal(discount);
    
    // Assert
    expect(total).toBe(90);
});
```

---

## Environment Variables

### ❌ FORBIDDEN: Default Values for Environment Variables

**Never provide default values for environment variables.** If a configuration value is required, the application should fail to start if it's not defined.

#### Why This Matters
- Default values hide missing configuration until production
- They create inconsistent behavior between environments
- Missing configs should be caught at deployment, not after hours of debugging

#### Examples

**❌ Bad - Silent fallback to default:**
```rust
// Rust
let port = std::env::var("PORT").unwrap_or("3000".to_string());
```

```typescript
// TypeScript/Node.js
const port = process.env.PORT || 3000;
const apiKey = process.env.API_KEY ?? "default-key";
```

```python
# Python
port = os.getenv("PORT", "3000")
```

**✅ Good - Fail if not defined:**
```rust
// Rust
let port = std::env::var("PORT")
    .expect("PORT environment variable must be set");
```

```typescript
// TypeScript/Node.js
const port = process.env.PORT;
if (!port) {
    throw new Error("PORT environment variable must be set");
}

// Or use a validation library like Zod
import { z } from 'zod';
const envSchema = z.object({
    PORT: z.string(),
    API_KEY: z.string(),
});
const env = envSchema.parse(process.env);
```

```python
# Python
port = os.environ["PORT"]  # Raises KeyError if not set

# Or explicit check
port = os.getenv("PORT")
if port is None:
    raise RuntimeError("PORT environment variable must be set")
```

#### Exception: Truly Optional Configuration
Only use defaults when the behavior is **genuinely optional** and the default is **well-documented**:

```typescript
// OK: Debug mode is optional and defaults to off
const debug = process.env.DEBUG === "true";

// OK: Optional feature flag
const enableBetaFeatures = process.env.ENABLE_BETA === "true";
```

---

## Error Handling

### ❌ FORBIDDEN: Silent Error Swallowing

**Never catch errors and ignore them.** Every error must be either:
1. Handled appropriately
2. Propagated to the caller
3. Logged with sufficient context

#### Examples

**❌ Bad - Silent catch:**
```rust
// Rust
let result = some_operation();
if let Ok(value) = result {
    // use value
}
// Error case is silently ignored!
```

```typescript
// TypeScript
try {
    await riskyOperation();
} catch (e) {
    // Silent catch - error is lost!
}
```

**✅ Good - Handle or propagate:**
```rust
// Rust
let value = some_operation()
    .map_err(|e| anyhow::anyhow!("Failed to perform operation: {}", e))?;
```

```typescript
// TypeScript
try {
    await riskyOperation();
} catch (error) {
    logger.error("Risky operation failed", { error, context });
    throw new OperationError("Risky operation failed", { cause: error });
}
```

### Error Lifecycle Model

Errors behave differently depending on **when** they occur. Choose the right strategy:

| Phase | Strategy | Rationale |
|-------|----------|-----------|
| **Startup / Init** | Fail fast — crash the process | Missing config or broken dependencies should prevent launch |
| **Request / Task** | Propagate with context — return error to caller | Caller decides retry, fallback, or user-facing message |
| **Background / Async** | Log + alert — do not crash | A failed cron job should not take down the service |
| **Cleanup / Shutdown** | Log warning — continue shutdown sequence | Failing to close a socket should not block graceful exit |

**Key rule**: Never use the same error strategy for all phases. Startup errors that are silently logged hide fatal misconfiguration. Background errors that crash the process cause unnecessary downtime.

---

## Pattern Matching & Switch Statements

### ❌ FORBIDDEN: Catch-All Defaults for Known Types

**Never use wildcard/default patterns to "make the code compile"** when you should be handling all known cases explicitly.

#### Why This Matters
- Adding a new enum variant won't trigger a compile error
- Bugs are hidden as the default silently handles unexpected cases
- The code doesn't express its true intent

#### Examples

**❌ Bad - Wildcard hiding potential bugs:**
```rust
// Rust
enum Status {
    Active,
    Inactive,
    Pending,
}

fn handle_status(status: Status) -> &'static str {
    match status {
        Status::Active => "active",
        _ => "other",  // FORBIDDEN: Hides Inactive and Pending handling
    }
}
```

```typescript
// TypeScript
type Status = "active" | "inactive" | "pending";

function handleStatus(status: Status): string {
    switch (status) {
        case "active":
            return "active";
        default:
            return "other";  // FORBIDDEN: What about inactive and pending?
    }
}
```

**✅ Good - Exhaustive matching:**
```rust
// Rust
fn handle_status(status: Status) -> &'static str {
    match status {
        Status::Active => "active",
        Status::Inactive => "inactive",
        Status::Pending => "pending",
    }
    // Compiler will error if a new variant is added!
}
```

```typescript
// TypeScript - Use exhaustive check
function handleStatus(status: Status): string {
    switch (status) {
        case "active":
            return "active";
        case "inactive":
            return "inactive";
        case "pending":
            return "pending";
        default:
            // Exhaustive check - will error if a case is missing
            const _exhaustive: never = status;
            throw new Error(`Unhandled status: ${status}`);
    }
}
```

#### Exception: Truly Unknown External Data
Defaults are acceptable when handling **external data** that may contain unexpected values:

```rust
// OK: Parsing external API response
fn parse_external_status(s: &str) -> Status {
    match s {
        "active" => Status::Active,
        "inactive" => Status::Inactive,
        "pending" => Status::Pending,
        unknown => {
            log::warn!("Unknown status received: {}", unknown);
            Status::Pending  // Explicit fallback with logging
        }
    }
}
```

---

## Null/None/Undefined Handling

### ❌ FORBIDDEN: Ignoring Optional Values

**Never force-unwrap nullable values without explicit justification.** This includes `.unwrap()` (Rust), `!` non-null assertions (TypeScript), force-unwrap (Swift), and unchecked dereferences (C/C++).

#### Examples

**❌ Bad - Unsafe unwrapping:**
```rust
// Rust
let value = optional_value.unwrap();  // Panics if None
```

```typescript
// TypeScript
const value = possiblyNull!.property;  // Throws if null
```

**✅ Good - Explicit handling:**
```rust
// Rust
let value = optional_value
    .ok_or_else(|| anyhow::anyhow!("Expected value to be present"))?;

// Or with context
let value = optional_value
    .expect("Value must be present after validation step");
```

```typescript
// TypeScript
if (!possiblyNull) {
    throw new Error("Expected value to be present");
}
const value = possiblyNull.property;

// Or with optional chaining + explicit check
const value = possiblyNull?.property;
if (value === undefined) {
    throw new Error("Property not found");
}
```

---

## Logging & Observability

### Required Context in Error Logs

When logging errors, always include:
1. **What** operation failed
2. **Why** it failed (the error message)
3. **Context** (relevant IDs, parameters, state)

**❌ Bad:**
```typescript
console.log("Error occurred");
logger.error(error.message);
```

**✅ Good:**
```typescript
logger.error("Failed to process user order", {
    userId: user.id,
    orderId: order.id,
    error: error.message,
    stack: error.stack,
});
```

---

## Summary Checklist

When writing or reviewing code, verify:

### Core Principles

- [ ] **SOLID principles applied** - Single responsibility, proper abstractions, substitutable types
- [ ] **Fail fast** - detect problems early, not in production

### Clean Code

- [ ] **Meaningful names** - Variables, functions, classes reveal intent
- [ ] **Small functions** - < 20 lines, single responsibility
- [ ] **No magic numbers** - Use named constants
- [ ] **Comments explain why** - Not what the code does

### DRY, YAGNI, KISS

- [ ] **No duplication** - Single source of truth for each concept
- [ ] **No speculative code** - Only implement what's needed now
- [ ] **Simple over clever** - Prefer readable solutions

### Error Handling

- [ ] **No silent error swallowing** - All errors handled or propagated
- [ ] **No wildcard patterns** hiding known enum/union cases
- [ ] **No unsafe unwrapping** without explicit justification
- [ ] **Error logs include context** - what, why, and relevant IDs

### Configuration

- [ ] **No default environment variables** for required configuration

### Security

- [ ] **Input validated** at system boundaries
- [ ] **No secrets in code** - Use environment variables
- [ ] **Least privilege** - Minimum permissions granted

### Testing

- [ ] **Tests follow FIRST** - Fast, Independent, Repeatable, Self-validating, Timely
- [ ] **One concept per test** - Single behavior verification
- [ ] **Arrange-Act-Assert** pattern used

---

## Applying These Standards

These guidelines should be applied in order of priority:

1. **Compilation/Startup failures** > Runtime errors > Silent failures
2. **Explicit handling** > Implicit defaults
3. **Loud errors** > Quiet errors > No errors

When in doubt, ask: *"If something goes wrong here, will I know about it immediately?"*

If the answer is no, the code needs to be more explicit.
