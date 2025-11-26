# General Programming Standards

These are fundamental programming principles that apply across all languages. They prioritize explicit behavior, early failure detection, and code clarity over convenience.

---

## Core Principles

### 1. Fail Fast, Fail Loud

Code should fail immediately and clearly when something is wrong, rather than silently continuing with incorrect behavior.

**Why?**
- Bugs are caught during development, not in production
- Debugging is easier when errors occur close to the source
- Silent failures can cascade into larger, harder-to-diagnose issues

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

**Never use `.unwrap()`, `!`, or force-unwrapping without explicit justification.**

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

- [ ] **No default environment variables** for required configuration
- [ ] **No silent error swallowing** - all errors handled or propagated
- [ ] **No wildcard patterns** hiding known enum/union cases
- [ ] **No unsafe unwrapping** without explicit justification
- [ ] **Error logs include context** - what, why, and relevant IDs
- [ ] **Fail fast** - detect problems early, not in production

---

## Applying These Standards

These guidelines should be applied in order of priority:

1. **Compilation/Startup failures** > Runtime errors > Silent failures
2. **Explicit handling** > Implicit defaults
3. **Loud errors** > Quiet errors > No errors

When in doubt, ask: *"If something goes wrong here, will I know about it immediately?"*

If the answer is no, the code needs to be more explicit.
