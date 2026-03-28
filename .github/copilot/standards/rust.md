# Rust Best Practices & Standards

This document contains best practices for Rust development that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **Rust-specific guidelines**. The general standards take priority.

---

## Documentation Good Practices (Rust)

- Document public items with rustdoc comments (`///`), including error behavior
- Keep crate-level docs (`//!`) aligned with module boundaries and usage
- Add examples to rustdoc for public APIs when practical
- Update docs when feature flags, env vars, or CLI behavior change
- Prefer comments that explain ownership/lifetime or performance trade-offs

---

## General Standards

> All FORBIDDEN patterns from `general.md` apply. Adapt to idiomatic Rust:
> - **Env vars**: Fail at startup. Use `std::env::var().expect()` or `anyhow::Context` with `?`
> - **Errors**: Never ignore `Result`/`Option`. Use `?` propagation with `anyhow`/`thiserror` context
> - **Pattern matching**: Exhaustive `match` on enums — no wildcard `_` for known variants
> - **Unwrapping**: Never bare `.unwrap()`. Use `.expect("reason")` or return `Result` in public APIs

---

## Project Structure

### Recommended Layout
```
project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs          # Binary entry point
│   ├── lib.rs           # Library entry point
│   ├── config.rs        # Configuration handling
│   ├── error.rs         # Custom error types
│   └── modules/
│       ├── mod.rs
│       └── ...
├── tests/               # Integration tests
│   └── integration_test.rs
├── benches/             # Benchmarks
│   └── benchmark.rs
├── examples/            # Example code
│   └── example.rs
└── docs/               # Documentation
```

---

## Code Style

### Naming Conventions
| Item | Convention | Example |
|------|------------|---------|
| Crates | snake_case | `my_crate` |
| Modules | snake_case | `my_module` |
| Types (structs, enums, traits) | PascalCase | `MyStruct` |
| Functions | snake_case | `my_function` |
| Methods | snake_case | `my_method` |
| Local variables | snake_case | `my_variable` |
| Constants | SCREAMING_SNAKE_CASE | `MY_CONSTANT` |
| Static variables | SCREAMING_SNAKE_CASE | `MY_STATIC` |
| Type parameters | Single uppercase or PascalCase | `T`, `Item` |
| Lifetimes | Short lowercase | `'a`, `'de` |

### Formatting
- Use `rustfmt` for consistent formatting
- Configure in `rustfmt.toml`:
```toml
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Default"
```

---

## Error Handling

### Use `thiserror` for Library Errors
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid configuration: {message}")]
    Config { message: String },
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

### Use `anyhow` for Application Errors
```rust
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("Failed to read configuration file")?;
    
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse configuration")?;
    
    Ok(config)
}
```

### Error Handling Guidelines
- Never use `.unwrap()` in library code
- Use `.expect("meaningful message")` only when panic is intentional
- Propagate errors with `?` operator
- Add context to errors for better debugging
- Define custom error types for domain-specific errors

---

## Memory & Performance

### Ownership Best Practices
```rust
// Prefer borrowing over cloning
fn process(data: &str) -> Result<()> { ... }

// Use Cow for flexible ownership
use std::borrow::Cow;
fn process_flexible(data: Cow<'_, str>) -> String { ... }

// Return owned data from constructors
impl MyStruct {
    pub fn new(name: String) -> Self { ... }
}
```

### Performance Guidelines
- Use `&str` over `String` for function parameters when possible
- Prefer iterators over indexed loops
- Use `Vec::with_capacity()` when size is known
- Avoid unnecessary allocations in hot paths
- Use `#[inline]` judiciously for small, frequently-called functions

### Zero-Cost Abstractions
```rust
// Use iterators (compiled to efficient loops)
let sum: i32 = numbers.iter().filter(|n| **n > 0).sum();

// Use Option/Result methods instead of match
let value = option.unwrap_or_default();
let result = result.map(|v| v * 2)?;
```

---

## Concurrency

### Async Best Practices
```rust
// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<()> {
    // ...
}

// Use async traits with async-trait crate
use async_trait::async_trait;

#[async_trait]
pub trait MyTrait {
    async fn process(&self) -> Result<()>;
}
```

### Thread Safety
```rust
// Use Arc for shared ownership across threads
use std::sync::Arc;
let shared = Arc::new(data);

// Use Mutex/RwLock for interior mutability
use std::sync::{Mutex, RwLock};
let data = Arc::new(Mutex::new(value));

// Prefer RwLock for read-heavy workloads
let data = Arc::new(RwLock::new(value));
```

---

## Testing

### Test Organization
```rust
// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = "test";
        
        // Act
        let result = process(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use my_crate::*;

#[test]
fn test_full_workflow() {
    // Test end-to-end functionality
}
```

### Testing Guidelines
- Aim for high test coverage on public APIs
- Use `proptest` or `quickcheck` for property-based testing
- Use `mockall` for mocking in tests
- Test error cases, not just happy paths
- Use `#[should_panic]` for panic tests

---

## Documentation

### 📖 Documentation Philosophy

**All public items MUST be documented.** Documentation is not optional—it's a core part of the code.

> *"Code tells you how, documentation tells you why."*

### Documentation Levels

| Level | What to Document | Example |
|-------|------------------|---------|
| **Crate** | Purpose, usage, features | `//! This crate provides...` |
| **Module** | Module's responsibility | `//! User authentication module` |
| **Public Items** | All pub structs, enums, traits, functions | `/// Authenticates a user` |
| **Complex Private Items** | Non-obvious internal logic | `// Why we use X approach` |

---

### Crate-Level Documentation

Place in `src/lib.rs` or `src/main.rs`:

```rust
//! # My Crate
//!
//! `my_crate` provides utilities for processing data efficiently.
//!
//! ## Quick Start
//!
//! ```rust
//! use my_crate::process;
//!
//! let result = process("input data")?;
//! println!("Result: {}", result);
//! ```
//!
//! ## Features
//!
//! - **Fast**: Optimized for performance
//! - **Safe**: Memory-safe by design
//! - **Async**: Full async/await support
//!
//! ## Feature Flags
//!
//! - `full`: Enable all features
//! - `async`: Enable async support (enabled by default)
//!
//! ## Examples
//!
//! See the [`examples`] directory for complete examples.
```

---

### Module Documentation

```rust
//! User authentication and authorization.
//!
//! This module handles:
//! - User login and logout
//! - Session management
//! - Permission checking
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
//! │   Request   │───▶│   AuthN     │───▶│   AuthZ     │
//! └─────────────┘    └─────────────┘    └─────────────┘
//! ```
//!
//! # Example
//!
//! ```rust
//! use my_crate::auth::{authenticate, authorize};
//!
//! let user = authenticate(&credentials)?;
//! authorize(&user, Permission::Read)?;
//! ```

pub mod login;
pub mod session;
pub mod permissions;
```

---

### Function Documentation

**Full template for public functions:**

```rust
/// Authenticates a user with the provided credentials.
///
/// Validates the username and password against the database,
/// creates a new session, and returns an authentication token.
///
/// # Arguments
///
/// * `credentials` - The user's login credentials
/// * `options` - Optional authentication settings
///
/// # Returns
///
/// Returns an [`AuthToken`] on successful authentication.
///
/// # Errors
///
/// Returns an error if:
/// - The username doesn't exist ([`AuthError::UserNotFound`])
/// - The password is incorrect ([`AuthError::InvalidPassword`])
/// - The account is locked ([`AuthError::AccountLocked`])
/// - Database connection fails ([`AuthError::Database`])
///
/// # Examples
///
/// ```rust
/// use my_crate::auth::{authenticate, Credentials};
///
/// let credentials = Credentials::new("user@example.com", "password123");
/// let token = authenticate(&credentials, None)?;
///
/// println!("Authenticated! Token: {}", token);
/// ```
///
/// ## With Custom Options
///
/// ```rust
/// use my_crate::auth::{authenticate, Credentials, AuthOptions};
///
/// let options = AuthOptions::builder()
///     .remember_me(true)
///     .mfa_required(true)
///     .build();
///
/// let token = authenticate(&credentials, Some(options))?;
/// ```
///
/// # Panics
///
/// Panics if the authentication service is not initialized.
/// Call [`init_auth_service`] before using this function.
///
/// # Security
///
/// - Passwords are never logged
/// - Failed attempts are rate-limited
/// - Tokens expire after 24 hours
pub fn authenticate(
    credentials: &Credentials,
    options: Option<AuthOptions>,
) -> Result<AuthToken, AuthError> {
    // ...
}
```

---

### Struct Documentation

```rust
/// A user account in the system.
///
/// Represents a registered user with their profile information,
/// authentication details, and permissions.
///
/// # Invariants
///
/// - `email` is always a valid email address
/// - `id` is unique across all users
/// - `created_at` <= `updated_at`
///
/// # Example
///
/// ```rust
/// use my_crate::User;
///
/// let user = User::builder()
///     .email("user@example.com")
///     .name("John Doe")
///     .build()?;
///
/// assert_eq!(user.email(), "user@example.com");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    /// Unique identifier for the user.
    ///
    /// Generated using UUIDv4 and guaranteed to be unique.
    id: UserId,

    /// User's email address.
    ///
    /// Must be a valid email format. Used for:
    /// - Login authentication
    /// - Password recovery
    /// - Notifications
    email: String,

    /// User's display name.
    ///
    /// Can contain any UTF-8 characters. Maximum length: 100 chars.
    name: String,

    /// When the account was created.
    created_at: DateTime<Utc>,

    /// When the account was last modified.
    updated_at: DateTime<Utc>,
}
```

---

### Enum Documentation

```rust
/// The result of an authentication attempt.
///
/// Represents all possible outcomes when a user tries to authenticate.
///
/// # Example
///
/// ```rust
/// match authenticate(&credentials) {
///     Ok(AuthResult::Success(token)) => println!("Welcome!"),
///     Ok(AuthResult::MfaRequired(challenge)) => prompt_mfa(challenge),
///     Ok(AuthResult::PasswordExpired) => redirect_to_reset(),
///     Err(e) => handle_error(e),
/// }
/// ```
#[derive(Debug, Clone)]
pub enum AuthResult {
    /// Authentication succeeded.
    ///
    /// Contains the session token for subsequent requests.
    Success(AuthToken),

    /// Multi-factor authentication is required.
    ///
    /// The user must complete the MFA challenge before proceeding.
    /// Contains the challenge details for the configured MFA method.
    MfaRequired(MfaChallenge),

    /// The password has expired and must be reset.
    ///
    /// User should be redirected to the password reset flow.
    PasswordExpired,

    /// The account requires email verification.
    ///
    /// A verification email has been sent to the registered address.
    EmailVerificationRequired,
}
```

---

### Trait Documentation

```rust
/// A repository for user data access.
///
/// Provides CRUD operations for [`User`] entities. Implementations
/// must be thread-safe and handle concurrent access properly.
///
/// # Implementors
///
/// - [`PostgresUserRepository`] - PostgreSQL implementation
/// - [`InMemoryUserRepository`] - In-memory for testing
///
/// # Example
///
/// ```rust
/// use my_crate::UserRepository;
///
/// async fn get_user_name(repo: &impl UserRepository, id: UserId) -> Result<String> {
///     let user = repo.find_by_id(id).await?
///         .ok_or(UserError::NotFound)?;
///     Ok(user.name().to_string())
/// }
/// ```
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Finds a user by their unique identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique ID
    ///
    /// # Returns
    ///
    /// - `Ok(Some(user))` if the user exists
    /// - `Ok(None)` if no user with that ID exists
    /// - `Err(_)` if a database error occurred
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>>;

    /// Finds a user by their email address.
    ///
    /// Email lookup is case-insensitive.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;

    /// Persists a new user to the repository.
    ///
    /// # Errors
    ///
    /// Returns [`RepositoryError::DuplicateEmail`] if the email
    /// is already registered.
    async fn save(&self, user: &User) -> Result<()>;

    /// Deletes a user from the repository.
    ///
    /// This is a soft delete—the user record is marked as deleted
    /// but retained for audit purposes.
    async fn delete(&self, id: UserId) -> Result<()>;
}
```

---

### Method Documentation Shortcuts

For simple getters/setters, use concise docs:

```rust
impl User {
    /// Returns the user's unique identifier.
    #[inline]
    pub fn id(&self) -> UserId {
        self.id
    }

    /// Returns the user's email address.
    #[inline]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Returns `true` if the user account is active.
    pub fn is_active(&self) -> bool {
        self.status == Status::Active
    }

    /// Sets the user's display name.
    ///
    /// # Panics
    ///
    /// Panics if `name` exceeds 100 characters.
    pub fn set_name(&mut self, name: impl Into<String>) {
        let name = name.into();
        assert!(name.len() <= 100, "Name exceeds maximum length");
        self.name = name;
        self.updated_at = Utc::now();
    }
}
```

---

### Documentation Tests

**All examples should be runnable tests:**

```rust
/// Parses a duration string into a [`Duration`].
///
/// # Format
///
/// Supports the following formats:
/// - `Ns` - seconds (e.g., "30s")
/// - `Nm` - minutes (e.g., "5m")
/// - `Nh` - hours (e.g., "2h")
/// - `Nd` - days (e.g., "7d")
///
/// # Examples
///
/// ```
/// use my_crate::parse_duration;
///
/// assert_eq!(parse_duration("30s")?, Duration::from_secs(30));
/// assert_eq!(parse_duration("5m")?, Duration::from_secs(300));
/// assert_eq!(parse_duration("2h")?, Duration::from_secs(7200));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Errors
///
/// ```
/// use my_crate::parse_duration;
///
/// assert!(parse_duration("invalid").is_err());
/// assert!(parse_duration("").is_err());
/// assert!(parse_duration("-5s").is_err());
/// ```
pub fn parse_duration(s: &str) -> Result<Duration, ParseError> {
    // ...
}
```

---

### Internal Documentation (Comments)

Use regular comments for implementation details:

```rust
pub fn complex_algorithm(data: &[u8]) -> Result<Output> {
    // We use a two-pass approach here because:
    // 1. First pass identifies boundaries (O(n))
    // 2. Second pass processes segments in parallel
    // This is faster than single-pass for data > 1MB
    
    // First pass: find boundaries
    let boundaries = find_boundaries(data);
    
    // PERF: Parallel processing gives ~3x speedup on 8 cores
    // See benchmarks/algorithm_bench.rs for measurements
    let segments: Vec<_> = boundaries
        .par_iter()
        .map(|b| process_segment(data, b))
        .collect();
    
    // TODO(#123): Consider using SIMD for the merge step
    merge_segments(segments)
}
```

---

### Documentation Lints

Enable in `Cargo.toml` or `lib.rs`:

```rust
// In lib.rs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
```

Or in `Cargo.toml`:

```toml
[lints.rust]
missing_docs = "warn"

[lints.rustdoc]
missing_crate_level_docs = "warn"
broken_intra_doc_links = "warn"
```

---

### Documentation Checklist

When documenting Rust code:

- [ ] **Crate-level docs** with purpose, quick start, and examples
- [ ] **Module docs** explaining the module's responsibility
- [ ] **All public items** have `///` documentation
- [ ] **Function docs** include Arguments, Returns, Errors, Examples
- [ ] **Struct fields** have inline `///` documentation
- [ ] **Enum variants** have inline `///` documentation
- [ ] **Examples are runnable** (doc tests pass)
- [ ] **Links work** (no broken intra-doc links)
- [ ] **Panics documented** if function can panic
- [ ] **Safety documented** for unsafe code
- [ ] **Complexity noted** for non-obvious algorithms

---

## Dependencies

### Cargo.toml Best Practices
```toml
[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
rust-version = "1.70"
description = "Brief description"
license = "MIT OR Apache-2.0"
repository = "https://github.com/..."

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[features]
default = []
full = ["feature1", "feature2"]
```

### Dependency Guidelines
- Pin major versions: `"1"` or `"1.0"`
- Use workspace dependencies for monorepos
- Audit dependencies with `cargo audit`
- Minimize feature flags to reduce compile time
- Prefer well-maintained, widely-used crates

---

## Security

### Security Checklist
- [ ] No `.unwrap()` on user input
- [ ] Validate all external data
- [ ] Use constant-time comparison for secrets
- [ ] Sanitize log output (no secrets)
- [ ] Use `secrecy` crate for sensitive data
- [ ] Run `cargo audit` regularly
- [ ] Enable Clippy security lints

### Secure Coding
```rust
// Use secrecy for sensitive data
use secrecy::{Secret, ExposeSecret};

struct Config {
    api_key: Secret<String>,
}

// Constant-time comparison
use subtle::ConstantTimeEq;
```

---

## Clippy Lints

### Recommended Clippy Configuration
```toml
# Cargo.toml or .cargo/config.toml
[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
cargo = "warn"

# Allow specific lints as needed
module_name_repetitions = "allow"
must_use_candidate = "allow"
```

### Common Clippy Fixes
- Use `if let` instead of `match` for single patterns
- Prefer `map_or` over `map().unwrap_or()`
- Use `collect()` type hints: `collect::<Vec<_>>()`
- Replace `clone()` with `to_owned()` for strings

---

## CI/CD Recommendations

### GitHub Actions Workflow
```yaml
name: CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Format check
        run: cargo fmt --check
      
      - name: Clippy
        run: cargo clippy -- -D warnings
      
      - name: Test
        run: cargo test
      
      - name: Audit
        run: cargo audit
```

---

## Summary Checklist

When reviewing or creating Rust code:

### General Standards (MUST)
- [ ] **No default env vars** - All env vars fail if undefined
- [ ] **No silent errors** - All `Result`/`Option` handled or propagated
- [ ] **No catch-all patterns** - Exhaustive matching on known enums
- [ ] **No bare `.unwrap()`** - Use `.expect("reason")` or handle properly

### Documentation Standards (MUST)
- [ ] **Crate-level docs** - `//!` docs in lib.rs with purpose and examples
- [ ] **Module docs** - `//!` docs explaining module responsibility
- [ ] **All public items documented** - Every pub fn/struct/enum/trait
- [ ] **Function docs complete** - Arguments, Returns, Errors, Examples
- [ ] **Struct/Enum fields documented** - Inline `///` for each field/variant
- [ ] **Examples are runnable** - Doc tests compile and pass
- [ ] **Panics/Safety documented** - If applicable
- [ ] **No broken links** - All intra-doc links resolve

### Rust-Specific Standards
- [ ] Follows naming conventions
- [ ] Proper error handling (no unwrap in library code)
- [ ] Tests for new functionality
- [ ] No Clippy warnings
- [ ] Formatted with rustfmt
- [ ] Dependencies audited
- [ ] Performance considerations addressed
