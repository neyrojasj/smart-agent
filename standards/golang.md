# Go Programming Best Practices & Standards

This document contains best practices for Go (Golang) development that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **Go-specific guidelines**. The general standards take priority.

---

## General Programming Standards (Go-Specific)

These are the core principles from `general.md` applied specifically to Go programming.

### 🚫 FORBIDDEN: Default Values for Environment Variables

**Never provide default values for required environment variables.**

```go
// ❌ FORBIDDEN - Silent fallback
port := os.Getenv("PORT")
if port == "" {
    port = "3000"  // Silent default
}

// ✅ REQUIRED - Fail if not defined
port := os.Getenv("PORT")
if port == "" {
    log.Fatal("PORT environment variable must be set")
}

// ✅ REQUIRED - Validation function
func mustGetenv(key string) string {
    value := os.Getenv(key)
    if value == "" {
        log.Fatalf("%s environment variable must be set", key)
    }
    return value
}

port := mustGetenv("PORT")
apiKey := mustGetenv("API_KEY")

// ✅ ACCEPTABLE - Only for truly optional features
debug := os.Getenv("DEBUG") == "true"
```

### 🚫 FORBIDDEN: Silent Error Swallowing

**Never ignore returned errors.**

```go
// ❌ FORBIDDEN - Ignoring error
result, _ := riskyOperation()

// ❌ FORBIDDEN - Checking error but not handling
result, err := riskyOperation()
if err != nil {
    // Do nothing - error is lost!
}

// ✅ REQUIRED - Handle or return errors
result, err := riskyOperation()
if err != nil {
    return fmt.Errorf("risky operation failed: %w", err)
}

// ✅ REQUIRED - Log and return for top-level handlers
result, err := riskyOperation()
if err != nil {
    log.Printf("risky operation failed: %v", err)
    return err
}
```

### 🚫 FORBIDDEN: Panic in Libraries

**Never panic in library code. Return errors instead.**

```go
// ❌ FORBIDDEN - Panicking in library
func ParseConfig(path string) Config {
    data, err := os.ReadFile(path)
    if err != nil {
        panic(fmt.Sprintf("failed to read config: %v", err))
    }
    // ...
}

// ✅ REQUIRED - Return errors
func ParseConfig(path string) (Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return Config{}, fmt.Errorf("failed to read config: %w", err)
    }
    // ...
    return config, nil
}
```

---

## Effective Go Principles

### Simplicity Over Cleverness

```go
// ❌ Bad - Overly clever one-liner
func isEven(n int) bool { return n&1 == 0 }

// ✅ Good - Clear and obvious
func isEven(n int) bool {
    return n%2 == 0
}

// ❌ Bad - Unnecessary abstraction
type StringProcessor interface {
    Process(s string) string
}

// ✅ Good - Just use a function if that's all you need
func processString(s string) string {
    return strings.TrimSpace(s)
}
```

---

## Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Packages | lowercase, short, no underscores | `http`, `json`, `user` |
| Files | lowercase with underscores | `user_service.go`, `http_handler.go` |
| Exported funcs/types | PascalCase | `GetUser`, `UserService` |
| Unexported funcs/types | camelCase | `getUser`, `userCache` |
| Variables | camelCase | `userName`, `httpClient` |
| Constants | PascalCase or camelCase | `MaxRetries`, `defaultTimeout` |
| Interfaces | PascalCase, often -er suffix | `Reader`, `Writer`, `Stringer` |
| Acronyms | All caps or all lower | `HTTPClient`, `xmlParser` |

### Naming Best Practices

```go
// ❌ Bad - Redundant package name in identifier
package user
type UserService struct{}  // user.UserService is redundant

// ✅ Good - Package provides context
package user
type Service struct{}  // user.Service is clear

// ❌ Bad - Long, unclear names
func GetAllUsersFromDatabaseByStatus(status string) []User

// ✅ Good - Concise and clear
func (s *Service) ByStatus(status string) []User

// ❌ Bad - Generic names
var data []byte
var info UserInfo

// ✅ Good - Descriptive names
var requestBody []byte
var userProfile UserProfile
```

---

## Error Handling

### Error Wrapping

```go
// ✅ Good - Wrap errors with context
func loadConfig(path string) (*Config, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("reading config file %s: %w", path, err)
    }
    
    var config Config
    if err := json.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("parsing config JSON: %w", err)
    }
    
    return &config, nil
}

// ✅ Good - Check for specific errors
if errors.Is(err, os.ErrNotExist) {
    return nil, ErrConfigNotFound
}

// ✅ Good - Extract error type
var parseErr *json.SyntaxError
if errors.As(err, &parseErr) {
    return nil, fmt.Errorf("syntax error at offset %d: %w", parseErr.Offset, err)
}
```

### Custom Error Types

```go
// ✅ Good - Sentinel errors for known conditions
var (
    ErrNotFound     = errors.New("not found")
    ErrUnauthorized = errors.New("unauthorized")
    ErrInvalidInput = errors.New("invalid input")
)

// ✅ Good - Custom error types with context
type ValidationError struct {
    Field   string
    Message string
}

func (e *ValidationError) Error() string {
    return fmt.Sprintf("validation error on %s: %s", e.Field, e.Message)
}

// Usage
func validateUser(u *User) error {
    if u.Email == "" {
        return &ValidationError{Field: "email", Message: "required"}
    }
    return nil
}
```

### Error Handling Flow

```go
// ✅ Good - Handle error cases first, reduce nesting
func processUser(id string) (*User, error) {
    user, err := db.GetUser(id)
    if err != nil {
        return nil, fmt.Errorf("getting user: %w", err)
    }
    
    if !user.IsActive {
        return nil, ErrUserInactive
    }
    
    if err := user.Validate(); err != nil {
        return nil, fmt.Errorf("validating user: %w", err)
    }
    
    // Happy path continues at the left margin
    return user, nil
}
```

---

## Project Structure

### Standard Layout

```
project/
├── cmd/
│   └── myapp/
│       └── main.go         # Application entry point
├── internal/               # Private application code
│   ├── config/
│   ├── handler/
│   ├── service/
│   └── repository/
├── pkg/                    # Public library code (optional)
│   └── mylib/
├── api/                    # API definitions (OpenAPI, protobuf)
├── web/                    # Static web assets
├── scripts/                # Build/deploy scripts
├── go.mod
├── go.sum
└── README.md
```

### Package Design

```go
// ❌ Bad - Generic package names
package utils
package helpers
package common

// ✅ Good - Descriptive package names
package auth
package storage
package validation

// ✅ Good - Small, focused packages
// Each package should have a clear responsibility
package user     // User domain logic
package order    // Order domain logic
package postgres // PostgreSQL-specific implementations
```

---

## Concurrency

### Goroutines

```go
// ❌ Bad - Goroutine without lifecycle management
func startWorker() {
    go func() {
        for {
            // Work forever with no way to stop
        }
    }()
}

// ✅ Good - Use context for cancellation
func startWorker(ctx context.Context) {
    go func() {
        for {
            select {
            case <-ctx.Done():
                return
            default:
                // Do work
            }
        }
    }()
}

// ✅ Good - Wait for goroutines to complete
func processAll(items []Item) error {
    var wg sync.WaitGroup
    errCh := make(chan error, len(items))
    
    for _, item := range items {
        wg.Add(1)
        go func(item Item) {
            defer wg.Done()
            if err := process(item); err != nil {
                errCh <- err
            }
        }(item)
    }
    
    wg.Wait()
    close(errCh)
    
    // Collect errors
    var errs []error
    for err := range errCh {
        errs = append(errs, err)
    }
    
    if len(errs) > 0 {
        return errors.Join(errs...)
    }
    return nil
}
```

### Channels

```go
// ✅ Good - Prefer channels for communication
func worker(jobs <-chan Job, results chan<- Result) {
    for job := range jobs {
        results <- process(job)
    }
}

// ✅ Good - Use buffered channels appropriately
jobs := make(chan Job, 100)  // Buffer size based on expected load

// ✅ Good - Close channels from the sender side
func producer(ch chan<- int) {
    for i := 0; i < 10; i++ {
        ch <- i
    }
    close(ch)  // Sender closes
}
```

### sync Package

```go
// ✅ Good - Use sync.Mutex for simple mutual exclusion
type SafeCounter struct {
    mu    sync.Mutex
    count int
}

func (c *SafeCounter) Increment() {
    c.mu.Lock()
    defer c.mu.Unlock()
    c.count++
}

// ✅ Good - Use sync.RWMutex for read-heavy workloads
type Cache struct {
    mu   sync.RWMutex
    data map[string]string
}

func (c *Cache) Get(key string) (string, bool) {
    c.mu.RLock()
    defer c.mu.RUnlock()
    val, ok := c.data[key]
    return val, ok
}

// ✅ Good - Use sync.Once for one-time initialization
var (
    once   sync.Once
    config *Config
)

func GetConfig() *Config {
    once.Do(func() {
        config = loadConfig()
    })
    return config
}
```

---

## Interfaces

### Small Interfaces

```go
// ✅ Good - Small, focused interfaces
type Reader interface {
    Read(p []byte) (n int, err error)
}

type Writer interface {
    Write(p []byte) (n int, err error)
}

// ✅ Good - Compose interfaces
type ReadWriter interface {
    Reader
    Writer
}

// ❌ Bad - Large, monolithic interfaces
type UserManager interface {
    Create(user *User) error
    Update(user *User) error
    Delete(id string) error
    Get(id string) (*User, error)
    List() ([]*User, error)
    ValidateEmail(email string) error
    SendWelcomeEmail(user *User) error
    // Too many responsibilities!
}
```

### Accept Interfaces, Return Structs

```go
// ✅ Good - Accept interface, return concrete type
func NewUserService(repo UserRepository) *UserService {
    return &UserService{repo: repo}
}

// This allows:
// - Easy mocking in tests
// - Flexibility in implementations
// - Clear, concrete return types
```

---

## Testing

### Table-Driven Tests

```go
func TestAdd(t *testing.T) {
    tests := []struct {
        name     string
        a, b     int
        expected int
    }{
        {"positive numbers", 2, 3, 5},
        {"negative numbers", -2, -3, -5},
        {"zero", 0, 0, 0},
        {"mixed", -2, 3, 1},
    }
    
    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            result := Add(tt.a, tt.b)
            if result != tt.expected {
                t.Errorf("Add(%d, %d) = %d; want %d", 
                    tt.a, tt.b, result, tt.expected)
            }
        })
    }
}
```

### Testing with Interfaces

```go
// Define interface for dependencies
type UserRepository interface {
    Get(id string) (*User, error)
    Save(user *User) error
}

// Mock implementation for tests
type mockUserRepo struct {
    users map[string]*User
    err   error
}

func (m *mockUserRepo) Get(id string) (*User, error) {
    if m.err != nil {
        return nil, m.err
    }
    return m.users[id], nil
}

// Use in tests
func TestUserService_GetUser(t *testing.T) {
    repo := &mockUserRepo{
        users: map[string]*User{
            "123": {ID: "123", Name: "Alice"},
        },
    }
    
    service := NewUserService(repo)
    user, err := service.GetUser("123")
    
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if user.Name != "Alice" {
        t.Errorf("got name %q; want %q", user.Name, "Alice")
    }
}
```

---

## Tools

### Essential Tools

```bash
# Format code (always run before commit)
gofmt -w .
# or
go fmt ./...

# Static analysis
go vet ./...

# Comprehensive linting
golangci-lint run

# Run tests with coverage
go test -cover ./...
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

### golangci-lint Configuration

```yaml
# .golangci.yml
run:
  timeout: 5m

linters:
  enable:
    - errcheck
    - gosimple
    - govet
    - ineffassign
    - staticcheck
    - unused
    - gofmt
    - goimports
    - revive
    - unconvert
    - unparam
    - misspell

linters-settings:
  errcheck:
    check-type-assertions: true
  govet:
    check-shadowing: true
```

---

## Summary Checklist

When reviewing or creating Go code:

### General Standards (MUST)
- [ ] **No default env vars** - Use `mustGetenv` pattern
- [ ] **No silent errors** - Never use `_` for errors
- [ ] **No panic in libraries** - Return errors instead

### Go-Specific Standards
- [ ] All errors checked and handled
- [ ] Error wrapping with `%w` for context
- [ ] Context used for cancellation
- [ ] Goroutines have lifecycle management
- [ ] Small, focused interfaces
- [ ] Descriptive package names (not utils/helpers)
- [ ] Table-driven tests
- [ ] `gofmt` and `go vet` pass
- [ ] No data races (use `-race` flag)
- [ ] `defer` for cleanup operations
