# C Programming Best Practices & Standards

This document contains best practices for C programming that should be followed when creating or reviewing plans.

> **📌 Important**: This document includes both **General Programming Standards** (applicable to all languages) and **C-specific guidelines**. The general standards take priority.

---

## General Programming Standards (C-Specific)

These are the core principles from `general.md` applied specifically to C programming.

### 🚫 FORBIDDEN: Default Values for Environment Variables

**Never provide default values for required environment variables.**

```c
// ❌ FORBIDDEN - Silent fallback
char *port = getenv("PORT");
if (port == NULL) {
    port = "3000";  // Silent default
}

// ✅ REQUIRED - Fail if not defined
char *port = getenv("PORT");
if (port == NULL) {
    fprintf(stderr, "Error: PORT environment variable must be set\n");
    exit(EXIT_FAILURE);
}

// ✅ ACCEPTABLE - Only for truly optional features
char *debug = getenv("DEBUG");
int debug_enabled = (debug != NULL && strcmp(debug, "true") == 0);
```

### 🚫 FORBIDDEN: Silent Error Swallowing

**Never ignore return values that indicate errors.**

```c
// ❌ FORBIDDEN - Ignoring return value
FILE *fp = fopen("config.txt", "r");
// Continuing without checking if fopen failed!

// ❌ FORBIDDEN - Empty error handling
if (fp == NULL) {
    // Do nothing - error is lost!
}

// ✅ REQUIRED - Handle or propagate errors
FILE *fp = fopen("config.txt", "r");
if (fp == NULL) {
    fprintf(stderr, "Error: Failed to open config.txt: %s\n", strerror(errno));
    return -1;  // Propagate error to caller
}
```

### 🚫 FORBIDDEN: Catch-All Defaults in Switch Statements

**Never use default cases to hide known enum values.**

```c
// ❌ FORBIDDEN - Default hiding known cases
typedef enum {
    STATUS_ACTIVE,
    STATUS_INACTIVE,
    STATUS_PENDING,
    STATUS_SUSPENDED
} Status;

const char* get_status_label(Status status) {
    switch (status) {
        case STATUS_ACTIVE:
            return "Active";
        case STATUS_INACTIVE:
            return "Inactive";
        default:
            return "Unknown";  // FORBIDDEN: Hides PENDING and SUSPENDED!
    }
}

// ✅ REQUIRED - Exhaustive switch (enable -Wswitch compiler warning)
const char* get_status_label(Status status) {
    switch (status) {
        case STATUS_ACTIVE:
            return "Active";
        case STATUS_INACTIVE:
            return "Inactive";
        case STATUS_PENDING:
            return "Pending";
        case STATUS_SUSPENDED:
            return "Suspended";
    }
    // Compiler warns if a case is missing with -Wswitch
    fprintf(stderr, "Error: Unknown status: %d\n", status);
    abort();  // Unreachable if all cases handled
}
```

### 🚫 FORBIDDEN: Ignoring NULL Pointers

**Never dereference pointers without checking for NULL.**

```c
// ❌ FORBIDDEN - Dereferencing without check
void process_user(User *user) {
    printf("Name: %s\n", user->name);  // Crashes if user is NULL!
}

// ✅ REQUIRED - Explicit NULL check
void process_user(User *user) {
    if (user == NULL) {
        fprintf(stderr, "Error: user pointer is NULL\n");
        return;
    }
    printf("Name: %s\n", user->name);
}

// ✅ REQUIRED - Use assertions for programming errors
#include <assert.h>
void process_user(User *user) {
    assert(user != NULL && "user must not be NULL");
    printf("Name: %s\n", user->name);
}
```

---

## C Standard Compliance

### Target C23 (ISO/IEC 9899:2024)

When possible, write code compatible with the latest C standard features:

```c
// C23 Features
#include <stdckdint.h>  // Checked integer arithmetic
#include <stdbit.h>     // Bit manipulation utilities

// Use nullptr instead of NULL (C23)
int *ptr = nullptr;

// Use constexpr for compile-time constants (C23)
constexpr int MAX_SIZE = 1024;

// Type inference with auto and typeof (C23)
auto x = 42;  // int
typeof(x) y = 100;  // Also int
```

### Backwards Compatibility

For broader compatibility, target C11 or C17:

```c
// Compiler flags for different standards
// -std=c23    (Latest, C23)
// -std=c17    (C17/C18)
// -std=c11    (C11)
// -std=c99    (C99)
```

---

## Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Files | lowercase with underscores | `user_service.c`, `user_service.h` |
| Functions | lowercase with underscores | `get_user_by_id()` |
| Variables | lowercase with underscores | `user_count`, `buffer_size` |
| Constants/Macros | SCREAMING_SNAKE_CASE | `MAX_BUFFER_SIZE`, `PI` |
| Types (typedef) | PascalCase or _t suffix | `UserData`, `user_data_t` |
| Enums | SCREAMING_SNAKE_CASE with prefix | `STATUS_ACTIVE`, `COLOR_RED` |
| Struct tags | PascalCase or lowercase | `struct User`, `struct user_data` |

### Naming Best Practices

```c
// ❌ Bad - Single letter names, unclear purpose
int n;
char *s;
void p(int x);

// ✅ Good - Descriptive names
int user_count;
char *username;
void process_request(int request_id);

// ❌ Bad - Reserved prefixes
int _internal_value;    // Reserved for implementation
int __system_value;     // Reserved for compiler

// ✅ Good - Clear prefixes for project scope
int mylib_internal_value;
int mylib_system_value;
```

---

## Memory Management

### Allocation and Deallocation

```c
// ✅ Good - Always check malloc return value
int *array = malloc(count * sizeof(int));
if (array == NULL) {
    fprintf(stderr, "Error: Memory allocation failed\n");
    return NULL;
}

// ✅ Good - Use calloc for zero-initialized memory
int *array = calloc(count, sizeof(int));

// ✅ Good - Check for integer overflow before allocation
if (count > SIZE_MAX / sizeof(int)) {
    fprintf(stderr, "Error: Allocation size would overflow\n");
    return NULL;
}

// ✅ Good - Set pointer to NULL after free
free(array);
array = NULL;  // Prevents use-after-free bugs
```

### Memory Safety Patterns

```c
// ✅ Good - Use sizeof on variable, not type
User *user = malloc(sizeof(*user));  // Automatically correct if type changes

// ✅ Good - Cleanup pattern with goto
int process_data(const char *filename) {
    int result = -1;
    FILE *fp = NULL;
    char *buffer = NULL;
    
    fp = fopen(filename, "r");
    if (fp == NULL) {
        goto cleanup;
    }
    
    buffer = malloc(BUFFER_SIZE);
    if (buffer == NULL) {
        goto cleanup;
    }
    
    // Process data...
    result = 0;
    
cleanup:
    free(buffer);
    if (fp != NULL) {
        fclose(fp);
    }
    return result;
}
```

### Avoiding Common Memory Errors

```c
// ❌ Bad - Memory leak
void process() {
    char *buffer = malloc(1024);
    if (error_condition) {
        return;  // Memory leak!
    }
    free(buffer);
}

// ❌ Bad - Double free
free(ptr);
free(ptr);  // Undefined behavior!

// ❌ Bad - Use after free
free(ptr);
printf("%s\n", ptr);  // Undefined behavior!

// ❌ Bad - Buffer overflow
char buffer[10];
strcpy(buffer, "This string is way too long");  // Overflow!

// ✅ Good - Use bounded string functions
char buffer[10];
strncpy(buffer, input, sizeof(buffer) - 1);
buffer[sizeof(buffer) - 1] = '\0';  // Ensure null termination
```

---

## Header Files

### Include Guards

```c
// ✅ Required - Include guard in every header
#ifndef PROJECT_MODULE_H
#define PROJECT_MODULE_H

// Header contents...

#endif /* PROJECT_MODULE_H */

// ✅ Alternative - #pragma once (widely supported but not standard)
#pragma once

// Header contents...
```

### Header Organization

```c
// user.h - Public interface
#ifndef USER_H
#define USER_H

#include <stddef.h>  // Standard headers first
#include <stdint.h>

// Forward declarations
typedef struct User User;

// Public API
User *user_create(const char *name, int age);
void user_destroy(User *user);
const char *user_get_name(const User *user);
int user_get_age(const User *user);

#endif /* USER_H */
```

### Include Order

```c
// source.c
// 1. Corresponding header (for .c files)
#include "source.h"

// 2. Standard library headers (alphabetical)
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 3. Third-party library headers
#include <openssl/ssl.h>

// 4. Project headers
#include "config.h"
#include "utils.h"
```

---

## Error Handling

### Return Codes Pattern

```c
// Define standard error codes
typedef enum {
    ERR_SUCCESS = 0,
    ERR_NULL_POINTER = -1,
    ERR_INVALID_ARGUMENT = -2,
    ERR_OUT_OF_MEMORY = -3,
    ERR_FILE_NOT_FOUND = -4,
    ERR_PERMISSION_DENIED = -5
} ErrorCode;

// Return error code, output via pointer
ErrorCode read_file(const char *path, char **content, size_t *size) {
    if (path == NULL || content == NULL) {
        return ERR_NULL_POINTER;
    }
    
    FILE *fp = fopen(path, "r");
    if (fp == NULL) {
        if (errno == ENOENT) {
            return ERR_FILE_NOT_FOUND;
        }
        if (errno == EACCES) {
            return ERR_PERMISSION_DENIED;
        }
        return ERR_FILE_NOT_FOUND;
    }
    
    // Read file...
    
    fclose(fp);
    return ERR_SUCCESS;
}

// Usage
char *content = NULL;
size_t size = 0;
ErrorCode err = read_file("config.txt", &content, &size);
if (err != ERR_SUCCESS) {
    fprintf(stderr, "Failed to read file: %d\n", err);
    return EXIT_FAILURE;
}
```

### errno Usage

```c
#include <errno.h>
#include <string.h>

// ✅ Good - Check errno immediately after call
FILE *fp = fopen(filename, "r");
if (fp == NULL) {
    int saved_errno = errno;  // Save immediately
    fprintf(stderr, "Failed to open %s: %s\n", filename, strerror(saved_errno));
    return -1;
}
```

---

## Code Style & Formatting

### Indentation and Braces

```c
// Use 4 spaces for indentation (configurable, but be consistent)

// K&R style braces (recommended)
if (condition) {
    do_something();
} else {
    do_other();
}

// Always use braces, even for single statements
// ❌ Bad
if (condition)
    do_something();

// ✅ Good
if (condition) {
    do_something();
}
```

### Line Length and Formatting

```c
// Maximum line length: 80-100 characters

// Breaking long function calls
result = very_long_function_name(
    first_argument,
    second_argument,
    third_argument
);

// Breaking long conditions
if (first_condition &&
    second_condition &&
    third_condition) {
    // ...
}
```

### Spacing

```c
// Spaces around binary operators
int result = a + b * c;
if (x == y && z != 0) { }

// No space after function name
printf("Hello\n");  // ✅ Good
printf ("Hello\n"); // ❌ Bad

// Space after keywords
if (condition) { }   // ✅ Good
if(condition) { }    // ❌ Bad
for (int i = 0; i < n; i++) { }
while (running) { }
```

---

## Const Correctness

```c
// ✅ Good - Use const for read-only parameters
size_t string_length(const char *str) {
    if (str == NULL) {
        return 0;
    }
    // str cannot be modified
    return strlen(str);
}

// ✅ Good - Const pointer to const data
void print_array(const int *const arr, size_t count) {
    for (size_t i = 0; i < count; i++) {
        printf("%d ", arr[i]);
    }
}

// ✅ Good - Return const for internal data
typedef struct {
    char name[64];
} User;

const char *user_get_name(const User *user) {
    return user->name;  // Caller cannot modify
}
```

---

## Static Analysis & Compiler Warnings

### Recommended Compiler Flags

```bash
# GCC/Clang recommended flags
-Wall           # Enable common warnings
-Wextra         # Enable extra warnings
-Werror         # Treat warnings as errors
-Wpedantic      # Strict ISO compliance
-Wconversion    # Warn on implicit conversions
-Wshadow        # Warn on variable shadowing
-Wformat=2      # Printf format checking
-Wswitch        # Warn on missing switch cases
-Wnull-dereference  # Warn on NULL pointer dereference
-Wdouble-promotion  # Warn on float to double promotion

# Example compilation
gcc -std=c17 -Wall -Wextra -Werror -Wpedantic -O2 -o program source.c
```

### Static Analysis Tools

- **clang-tidy**: Modern C/C++ linter with many checks
- **cppcheck**: Static analysis for C/C++
- **Coverity**: Commercial static analysis
- **PVS-Studio**: Commercial static analysis
- **MISRA C**: Industry standard for safety-critical systems

---

## Testing

### Unit Testing with Check or Unity

```c
// Using Unity framework
#include "unity.h"
#include "user.h"

void setUp(void) {
    // Setup before each test
}

void tearDown(void) {
    // Cleanup after each test
}

void test_user_create_should_return_valid_user(void) {
    User *user = user_create("Alice", 30);
    
    TEST_ASSERT_NOT_NULL(user);
    TEST_ASSERT_EQUAL_STRING("Alice", user_get_name(user));
    TEST_ASSERT_EQUAL_INT(30, user_get_age(user));
    
    user_destroy(user);
}

void test_user_create_should_return_null_for_invalid_input(void) {
    User *user = user_create(NULL, 30);
    TEST_ASSERT_NULL(user);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_user_create_should_return_valid_user);
    RUN_TEST(test_user_create_should_return_null_for_invalid_input);
    return UNITY_END();
}
```

---

## Summary Checklist

When reviewing or creating C code:

### General Standards (MUST)
- [ ] **No default env vars** - Fail if required env var is undefined
- [ ] **No silent errors** - All error conditions handled or propagated
- [ ] **No catch-all switch** - Enable -Wswitch, handle all cases explicitly
- [ ] **No NULL dereferencing** - Check pointers before use

### C-Specific Standards
- [ ] All `malloc`/`calloc` return values checked
- [ ] All allocated memory freed (no leaks)
- [ ] No use-after-free or double-free
- [ ] Include guards in all headers
- [ ] Const correctness applied
- [ ] No buffer overflows (use bounded functions)
- [ ] Compiler warnings enabled and code warning-free
- [ ] errno checked immediately after error
- [ ] Static analysis tools run regularly
