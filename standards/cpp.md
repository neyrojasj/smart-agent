# C++ Programming Best Practices & Standards

This document contains best practices for modern C++ development that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **C++-specific guidelines**. The general standards take priority.

---

## General Programming Standards (C++-Specific)

These are the core principles from `general.md` applied specifically to C++ programming.

### 🚫 FORBIDDEN: Default Values for Environment Variables

**Never provide default values for required environment variables.**

```cpp
// ❌ FORBIDDEN - Silent fallback
const char* port = std::getenv("PORT");
int port_num = port ? std::stoi(port) : 3000;  // Silent default

// ✅ REQUIRED - Fail if not defined
const char* port = std::getenv("PORT");
if (port == nullptr) {
    throw std::runtime_error("PORT environment variable must be set");
}
int port_num = std::stoi(port);

// ✅ ACCEPTABLE - Only for truly optional features
const char* debug = std::getenv("DEBUG");
bool debug_enabled = debug != nullptr && std::string_view(debug) == "true";
```

### 🚫 FORBIDDEN: Silent Error Swallowing

**Never catch exceptions without handling them properly.**

```cpp
// ❌ FORBIDDEN - Empty catch block
try {
    riskyOperation();
} catch (...) {
    // Exception silently swallowed!
}

// ❌ FORBIDDEN - Catch and ignore
try {
    saveToDatabase(data);
} catch (const std::exception&) {
    // "It's fine, we'll try again later" - NO!
}

// ✅ REQUIRED - Handle, log, and/or rethrow
try {
    riskyOperation();
} catch (const std::exception& e) {
    logger.error("Risky operation failed: {}", e.what());
    throw;  // Re-throw or handle appropriately
}

// ✅ REQUIRED - Catch specific exceptions
try {
    parseConfig(filename);
} catch (const ConfigParseError& e) {
    logger.error("Config parse failed: {} at line {}", e.what(), e.line());
    throw InitializationError("Failed to load configuration", e);
}
```

### 🚫 FORBIDDEN: Catch-All Defaults in Switch Statements

**Never use default cases to hide known enum values.**

```cpp
// ❌ FORBIDDEN - Default hiding known cases
enum class Status { Active, Inactive, Pending, Suspended };

std::string getStatusLabel(Status status) {
    switch (status) {
        case Status::Active:
            return "Active";
        case Status::Inactive:
            return "Inactive";
        default:
            return "Unknown";  // FORBIDDEN: Hides Pending and Suspended!
    }
}

// ✅ REQUIRED - Exhaustive switch (enable -Wswitch-enum)
std::string getStatusLabel(Status status) {
    switch (status) {
        case Status::Active:
            return "Active";
        case Status::Inactive:
            return "Inactive";
        case Status::Pending:
            return "Pending";
        case Status::Suspended:
            return "Suspended";
    }
    // Compiler warns if a case is missing with -Wswitch-enum
    std::unreachable();  // C++23, or throw for earlier standards
}
```

---

## Modern C++ Standards

### Target C++20 or C++23

```cpp
// Compiler flags
// -std=c++23    (Latest)
// -std=c++20    (Recommended minimum for new projects)
// -std=c++17    (Widely supported)

// C++20 Features to use:
// - Concepts for template constraints
// - Ranges for algorithm composition
// - std::span for safe array views
// - std::format for type-safe formatting
// - Coroutines for async operations

// C++23 Features:
// - std::expected for error handling
// - std::unreachable() for impossible code paths
// - Deducing this for simplified CRTP
```

---

## Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Files | snake_case or PascalCase | `user_service.cpp`, `UserService.hpp` |
| Classes/Structs | PascalCase | `UserService`, `HttpClient` |
| Functions | camelCase or snake_case | `getUserById()`, `get_user_by_id()` |
| Variables | camelCase or snake_case | `userName`, `user_name` |
| Member variables | m_ prefix or trailing _ | `m_name`, `name_` |
| Constants | kPascalCase or SCREAMING_SNAKE | `kMaxRetries`, `MAX_RETRIES` |
| Namespaces | lowercase | `myproject::utils` |
| Template params | PascalCase | `typename ValueType` |
| Concepts | PascalCase | `Hashable`, `Printable` |

---

## Resource Management (RAII)

### Smart Pointers

```cpp
// ✅ Good - Use unique_ptr for exclusive ownership
auto user = std::make_unique<User>("Alice", 30);

// ✅ Good - Use shared_ptr for shared ownership
auto config = std::make_shared<Config>();

// ❌ Bad - Raw new/delete
User* user = new User("Alice", 30);
// ... easy to forget delete!

// ✅ Good - Factory functions return smart pointers
class Connection {
public:
    static std::unique_ptr<Connection> create(const std::string& url);
    
private:
    Connection() = default;  // Private constructor
};

// ✅ Good - Use weak_ptr to break cycles
class Node {
    std::shared_ptr<Node> next;
    std::weak_ptr<Node> parent;  // Prevents cycle
};
```

### Move Semantics

```cpp
// ✅ Good - Enable move semantics
class Buffer {
public:
    Buffer(Buffer&& other) noexcept 
        : data_(std::exchange(other.data_, nullptr))
        , size_(std::exchange(other.size_, 0)) {}
    
    Buffer& operator=(Buffer&& other) noexcept {
        if (this != &other) {
            delete[] data_;
            data_ = std::exchange(other.data_, nullptr);
            size_ = std::exchange(other.size_, 0);
        }
        return *this;
    }
    
private:
    char* data_;
    size_t size_;
};

// ✅ Good - Pass sink parameters by value and move
void addUser(User user) {
    users_.push_back(std::move(user));
}

// ✅ Good - Return by value (RVO/NRVO applies)
std::vector<int> createVector() {
    std::vector<int> result;
    result.reserve(100);
    // ... populate
    return result;  // Moved or copy-elided
}
```

### Rule of Five/Zero

```cpp
// ✅ Good - Rule of Zero: Let the compiler handle everything
class User {
    std::string name_;
    std::vector<Order> orders_;
    // No manual destructor/copy/move needed!
};

// ✅ Good - Rule of Five: If you define one, define all
class FileHandle {
public:
    explicit FileHandle(const std::string& path);
    ~FileHandle();
    
    FileHandle(const FileHandle&) = delete;
    FileHandle& operator=(const FileHandle&) = delete;
    
    FileHandle(FileHandle&& other) noexcept;
    FileHandle& operator=(FileHandle&& other) noexcept;
    
private:
    FILE* handle_;
};
```

---

## Standard Library Usage

### Containers

```cpp
// ✅ Good - Prefer standard containers
std::vector<int> numbers;           // Dynamic array
std::array<int, 10> fixed;          // Fixed-size array
std::unordered_map<std::string, int> lookup;  // Hash map
std::map<std::string, int> ordered; // Ordered map

// ✅ Good - Reserve capacity when size is known
std::vector<User> users;
users.reserve(expected_count);

// ✅ Good - Use emplace for in-place construction
users.emplace_back("Alice", 30);  // No temporary created
```

### std::optional, std::variant, std::expected

```cpp
// ✅ Good - Use optional for values that may not exist
std::optional<User> findUser(int id) {
    if (auto it = users_.find(id); it != users_.end()) {
        return it->second;
    }
    return std::nullopt;
}

// Usage
if (auto user = findUser(42)) {
    std::cout << user->name() << '\n';
}

// ✅ Good - Use variant for type-safe unions
using ParseResult = std::variant<int, double, std::string>;

// ✅ Good - Use expected for error handling (C++23)
std::expected<User, Error> getUser(int id) {
    if (id <= 0) {
        return std::unexpected(Error::InvalidId);
    }
    // ...
    return User{...};
}
```

### std::string_view and std::span

```cpp
// ✅ Good - Use string_view for read-only string parameters
void processName(std::string_view name) {
    // Avoids copy, works with std::string, const char*, etc.
}

// ✅ Good - Use span for array views (C++20)
void processData(std::span<const int> data) {
    for (int value : data) {
        // ...
    }
}

// Works with any contiguous container
std::vector<int> vec = {1, 2, 3};
std::array<int, 3> arr = {1, 2, 3};
int raw[] = {1, 2, 3};

processData(vec);
processData(arr);
processData(raw);
```

---

## Templates and Concepts

### Concepts (C++20)

```cpp
// ✅ Good - Use concepts to constrain templates
template<typename T>
concept Hashable = requires(T t) {
    { std::hash<T>{}(t) } -> std::convertible_to<std::size_t>;
};

template<Hashable Key, typename Value>
class HashMap {
    // Key must be hashable
};

// ✅ Good - Use standard concepts
template<std::integral T>
T gcd(T a, T b) {
    while (b != 0) {
        T temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// ✅ Good - Abbreviated function templates with auto
void process(const std::ranges::range auto& container) {
    for (const auto& item : container) {
        // ...
    }
}
```

---

## Error Handling

### Exception Safety

```cpp
// ✅ Good - Strong exception guarantee
class Container {
public:
    void add(const Item& item) {
        // Copy first, then swap (strong guarantee)
        auto new_items = items_;  // May throw
        new_items.push_back(item);  // May throw
        items_ = std::move(new_items);  // noexcept
    }
};

// ✅ Good - Mark functions that don't throw
void swap(Container& other) noexcept {
    items_.swap(other.items_);
}

// ✅ Good - Use noexcept for move operations
Container(Container&& other) noexcept = default;
Container& operator=(Container&& other) noexcept = default;
```

### std::expected (C++23)

```cpp
#include <expected>

enum class ParseError { InvalidFormat, OutOfRange, Empty };

std::expected<int, ParseError> parseInt(std::string_view str) {
    if (str.empty()) {
        return std::unexpected(ParseError::Empty);
    }
    // Parse...
    return value;
}

// Usage
auto result = parseInt("42");
if (result) {
    std::cout << "Parsed: " << *result << '\n';
} else {
    std::cout << "Error: " << static_cast<int>(result.error()) << '\n';
}
```

---

## Concurrency

### Thread Safety

```cpp
#include <mutex>
#include <shared_mutex>
#include <atomic>

// ✅ Good - Use lock guards
class ThreadSafeCounter {
public:
    void increment() {
        std::lock_guard lock(mutex_);
        ++count_;
    }
    
    int get() const {
        std::lock_guard lock(mutex_);
        return count_;
    }
    
private:
    mutable std::mutex mutex_;
    int count_ = 0;
};

// ✅ Good - Use shared_mutex for read-heavy workloads
class Cache {
public:
    std::optional<Value> get(const Key& key) const {
        std::shared_lock lock(mutex_);  // Multiple readers OK
        auto it = data_.find(key);
        return it != data_.end() ? std::optional(it->second) : std::nullopt;
    }
    
    void set(const Key& key, Value value) {
        std::unique_lock lock(mutex_);  // Exclusive access
        data_[key] = std::move(value);
    }
    
private:
    mutable std::shared_mutex mutex_;
    std::unordered_map<Key, Value> data_;
};

// ✅ Good - Use atomics for simple counters
std::atomic<int> counter{0};
counter.fetch_add(1, std::memory_order_relaxed);
```

### std::jthread (C++20)

```cpp
// ✅ Good - Use jthread for automatic joining
void process() {
    std::jthread worker([](std::stop_token token) {
        while (!token.stop_requested()) {
            // Do work...
        }
    });
    // jthread automatically joins on destruction
}
```

---

## Build System (CMake)

### Modern CMake

```cmake
cmake_minimum_required(VERSION 3.20)
project(MyProject VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Create library
add_library(mylib
    src/user.cpp
    src/order.cpp
)

target_include_directories(mylib
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

# Enable warnings
target_compile_options(mylib PRIVATE
    $<$<CXX_COMPILER_ID:GNU,Clang>:-Wall -Wextra -Wpedantic -Werror>
    $<$<CXX_COMPILER_ID:MSVC>:/W4 /WX>
)

# Testing
enable_testing()
add_subdirectory(tests)
```

---

## Compiler Warnings

```bash
# GCC/Clang recommended flags
-Wall               # Common warnings
-Wextra             # Extra warnings
-Wpedantic          # Strict standard compliance
-Werror             # Treat warnings as errors
-Wconversion        # Implicit conversions
-Wshadow            # Variable shadowing
-Wnon-virtual-dtor  # Non-virtual destructors in base classes
-Wold-style-cast    # C-style casts
-Wcast-align        # Pointer alignment issues
-Woverloaded-virtual # Hidden overloaded virtuals
-Wsign-conversion   # Sign conversion issues
-Wnull-dereference  # Null pointer dereference
-Wdouble-promotion  # Float to double promotion
-Wformat=2          # Printf format checking
```

---

## Summary Checklist

When reviewing or creating C++ code:

### General Standards (MUST)
- [ ] **No default env vars** - Fail if required env var is undefined
- [ ] **No silent errors** - All exceptions handled or propagated
- [ ] **No catch-all switch** - Enable -Wswitch-enum, handle all cases

### C++-Specific Standards
- [ ] Using smart pointers (no raw new/delete)
- [ ] RAII for all resources
- [ ] Move semantics implemented correctly
- [ ] Rule of Zero or Rule of Five followed
- [ ] noexcept on move operations and destructors
- [ ] const correctness applied
- [ ] std::string_view for read-only string params
- [ ] Concepts used for template constraints (C++20+)
- [ ] Modern CMake practices
- [ ] All warnings enabled and code warning-free
