#!/bin/bash
# =============================================================================
# Smart Agent Installer
# Installs the smart agent and documentation structure into your project
# =============================================================================

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://raw.githubusercontent.com/neyrojasj/planning-copilot/main"
COPILOT_DIR=".copilot"
GITHUB_DIR=".github"

# Flags
INSTALL_STANDARDS=true
INSTALL_MINIMAL=false

# =============================================================================
# Helper Functions
# =============================================================================

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║                    Smart Agent Installer                      ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

show_help() {
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --with-standards    Install with language standards (default)"
    echo "  --no-standards      Skip language standards installation"
    echo "  --minimal           Install only the agent, no standards or extras"
    echo "  --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      # Full installation with standards (default)"
    echo "  $0 --no-standards       # Install without standards"
    echo "  $0 --minimal            # Install only the smart agent"
}

# =============================================================================
# Installation Functions
# =============================================================================

create_directory_structure() {
    print_info "Creating directory structure..."
    
    # Create .copilot directory structure
    mkdir -p "$COPILOT_DIR/docs/decisions"
    mkdir -p "$COPILOT_DIR/plans"
    mkdir -p "$COPILOT_DIR/prompts"
    mkdir -p "$COPILOT_DIR/tmp"
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        mkdir -p "$COPILOT_DIR/standards"
    fi
    
    # Create .github directory if it doesn't exist
    mkdir -p "$GITHUB_DIR/agents"
    
    print_success "Directory structure created"
}

create_gitignore() {
    print_info "Creating .copilot/.gitignore..."
    
    cat > "$COPILOT_DIR/.gitignore" << 'EOF'
# Smart Agent - Gitignore
# These files are local and should not be committed

# Temporary files
tmp/

# Plans directory (local planning state)
plans/

# Project summary (auto-generated)
project_summary.md
EOF
    
    print_success ".gitignore created"
}

create_state_yaml() {
    print_info "Initializing plans/state.yaml..."
    
    cat > "$COPILOT_DIR/plans/state.yaml" << EOF
# Smart Agent - Plans State File
# This file tracks all plans and their statuses

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

plans: {}

summary:
  draft: 0
  pending_review: 0
  approved: 0
  in_progress: 0
  completed: 0
  archived: 0
  rejected: 0
EOF
    
    print_success "state.yaml initialized"
}

create_docs_index() {
    print_info "Initializing docs/index.yaml..."
    
    cat > "$COPILOT_DIR/docs/index.yaml" << EOF
# Smart Agent - Documentation Search Index
# ALWAYS read this file first - it's your navigation map!

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

project:
  name: null
  type: null
  primary_language: null
  framework: null
  stage: development

documents:
  overview:
    file: "overview.md"
    title: "Project Overview"
    summary: null
    keywords: [purpose, quick-start, getting-started, about, features]
    last_updated: null
    
  architecture:
    file: "architecture.md"
    title: "System Architecture"
    summary: null
    keywords: [layers, structure, modules, data-flow, directories, components]
    last_updated: null
    
  tech-stack:
    file: "tech-stack.md"
    title: "Technology Stack"
    summary: null
    keywords: [dependencies, frameworks, libraries, versions, runtime, database]
    last_updated: null
    
  api:
    file: "api.md"
    title: "API Documentation"
    summary: null
    keywords: [endpoints, routes, rest, http, requests]
    has_api: false
    last_updated: null
    
  testing:
    file: "testing.md"
    title: "Testing Strategy"
    summary: null
    keywords: [tests, coverage, unit, integration, commands]
    last_updated: null
    
  development:
    file: "development.md"
    title: "Development Guide"
    summary: null
    keywords: [setup, install, scripts, env, commands, run, build]
    last_updated: null
    
  conventions:
    file: "conventions.md"
    title: "Code Conventions"
    summary: null
    keywords: [style, naming, patterns, linting, formatting, git]
    last_updated: null

decisions:
  count: 0
  recent: []

cross_references: {}

quick_commands:
  dev: null
  build: null
  test: null
  lint: null
EOF
    
    print_success "docs/index.yaml initialized"
}

create_docs_decisions_index() {
    print_info "Initializing docs/decisions/index.yaml..."
    
    cat > "$COPILOT_DIR/docs/decisions/index.yaml" << EOF
# Smart Agent - Decision Index

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
next_id: 1

decisions: {}

by_category:
  architecture: []
  api: []
  security: []
  testing: []
  infrastructure: []
  dependencies: []
  patterns: []
  other: []

by_status:
  proposed: []
  accepted: []
  deprecated: []
  superseded: []

summary:
  total: 0
  proposed: 0
  accepted: 0
  deprecated: 0
  superseded: 0
EOF
    
    print_success "docs/decisions/index.yaml initialized"
}

install_smart_agent() {
    print_info "Installing smart agent to .github/agents/..."
    
    # Download or copy the smart agent
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/agents/smart.agent.md" -o "$GITHUB_DIR/agents/smart.agent.md" 2>/dev/null || {
            print_warning "Could not download from remote, creating local copy..."
            create_smart_agent_local
        }
    else
        create_smart_agent_local
    fi
    
    print_success "Smart agent installed"
}

install_copilot_instructions() {
    print_info "Installing copilot-instructions.md..."
    
    # Download or create copilot-instructions.md
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/templates/copilot-instructions.md" -o "$GITHUB_DIR/copilot-instructions.md" 2>/dev/null || {
            print_warning "Could not download from remote, creating local copy..."
            create_copilot_instructions_local
        }
    else
        create_copilot_instructions_local
    fi
    
    print_success "copilot-instructions.md installed"
}

create_copilot_instructions_local() {
    cat > "$GITHUB_DIR/copilot-instructions.md" << 'INSTRUCTIONS_EOF'
# Copilot Instructions

## Default Agent Mode

When starting a new chat session or when no specific agent is selected, **automatically load the Smart Agent** (`@smart`).

The Smart Agent ensures all code changes are planned, tracked, and explicitly approved before implementation, with documentation maintained in the `.copilot/docs/` folder.

## Agent Loading Priority

1. If user explicitly selects an agent → Use that agent
2. If no agent selected → Load `@smart` agent automatically
3. Always check `.github/agents/` for available custom agents

## Available Agents

### @smart (Default)
- **Purpose**: Plan, track, implement code changes with docs-first approach
- **Location**: `.github/agents/smart.agent.md`
- **When to use**: Any task that involves code modifications

## Required Behavior

### Always On First Interaction:
1. Load `.copilot/docs/index.yaml` search index (PRIMARY CONTEXT)
2. Navigate to relevant docs based on index keywords
3. Check `.copilot/plans/state.yaml` for pending plans

### Never:
- Implement code changes without explicit user approval
- Skip the planning phase for significant changes
- Ignore the documentation in `.copilot/docs/`

## Project Configuration

- **Documentation**: `.copilot/docs/` (single source of truth)
- **Search Index**: `.copilot/docs/index.yaml`
- **Plans**: `.copilot/plans/`
- **Standards**: `.copilot/standards/` (if installed)

## Workflow Summary

```
User Request → Load Index → Read Docs → Create Plan → Approval → Implement → Update Docs
```

After every completed request, documentation must be updated to reflect changes.
INSTRUCTIONS_EOF
}

create_smart_agent_local() {
    cat > "$GITHUB_DIR/agents/smart.agent.md" << 'AGENT_EOF'
---
description: Docs-first agent that plans, tracks, and implements code changes with search-indexed documentation.
name: Smart
tools: ['fetch', 'githubRepo', 'search', 'usages']
handoffs:
  - label: Setup Project
    agent: agent
    prompt: "Initialize or update the project documentation. Run the setup prompt at .copilot/prompts/setup-project.md to analyze the codebase and populate .copilot/docs/ with comprehensive documentation."
    send: false
  - label: Rebuild Search Index
    agent: agent
    prompt: "Rebuild the search index. Re-scan all documentation in .copilot/docs/ and regenerate .copilot/docs/index.yaml with updated summaries, keywords, and cross-references."
    send: false
  - label: Implement Approved Plan
    agent: agent
    prompt: "Implement the approved plan. Check .copilot/plans/state.yaml for the most recently approved plan and implement it. After completion, update relevant docs."
    send: false
  - label: Show Plan Status
    agent: agent
    prompt: "Show current implementation plan status from .copilot/plans/state.yaml as a dashboard."
    send: true
---

# Smart Agent

You are a **Smart Agent** for GitHub Copilot with a documentation-first approach.

## Core Principle: Documentation as Context

Your primary context source is `.copilot/docs/` with `index.yaml` as your navigation map.

---

## Step 0: Load Search Index (ALWAYS FIRST!)

```
ON EVERY EXECUTION:
1. Read .copilot/docs/index.yaml
2. This index contains:
   - Project identity and quick reference
   - Document summaries with keywords
   - Cross-references between docs
3. Use keywords to navigate to relevant docs
4. ONLY read full docs when keywords match the task
```

---

## Documentation Structure

```
.copilot/
├── docs/
│   ├── index.yaml          # Search index (load first!)
│   ├── overview.md         # Project overview and identity
│   ├── architecture.md     # System design and components
│   ├── tech-stack.md       # Languages, frameworks, dependencies
│   ├── api.md              # API endpoints and contracts
│   ├── testing.md          # Testing strategy and commands
│   ├── development.md      # Dev workflow and commands
│   ├── conventions.md      # Code style and patterns
│   └── decisions/
│       ├── index.yaml      # Decision log
│       └── ADR-XXX.md      # Individual decisions
├── plans/
│   └── state.yaml          # Plan tracking
└── standards/              # Language guidelines (optional)
```

---

## Workflow

### For Any Task:

1. **Load Index** → Read `index.yaml` keywords
2. **Find Docs** → Navigate to docs matching task keywords
3. **Understand** → Read only relevant sections
4. **Plan** → Create plan if needed
5. **Implement** → Execute after approval
6. **Update Docs** → Reflect changes in documentation

### Post-Completion (MANDATORY):

After completing ANY request that changes the codebase:
1. Update affected documentation files
2. Update `index.yaml` keywords if new concepts added
3. Add ADR if architectural decision was made

---

## Plan States

| State | Description |
|-------|-------------|
| `draft` | Being created |
| `pending_review` | Ready for approval |
| `approved` | Ready to implement |
| `in_progress` | Being implemented |
| `completed` | Done |

---

## Commands

| Command | Action |
|---------|--------|
| `plan new <desc>` | Create plan |
| `plan approve <ID>` | Approve plan |
| `plan implement <ID>` | Start implementation |
| `docs update` | Update documentation |
| `docs rebuild` | Rebuild search index |

---

## Critical Rules

🚨 **ALWAYS** load `index.yaml` first
🚨 **NEVER** implement without approval
🚨 **ALWAYS** update docs after changes
🚨 **NEVER** duplicate information across docs
AGENT_EOF
}

install_standards() {
    if [ "$INSTALL_STANDARDS" = false ]; then
        return
    fi
    
    print_info "Installing language standards..."
    
    # Install General standards (always first - contains core principles)
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/standards/general.md" -o "$COPILOT_DIR/standards/general.md" 2>/dev/null || {
            print_warning "Could not download General standards from remote"
        }
        curl -sSL "$REPO_URL/standards/rust.md" -o "$COPILOT_DIR/standards/rust.md" 2>/dev/null || {
            print_warning "Could not download Rust standards from remote"
        }
        curl -sSL "$REPO_URL/standards/nodejs.md" -o "$COPILOT_DIR/standards/nodejs.md" 2>/dev/null || {
            print_warning "Could not download Node.js standards from remote"
        }
        curl -sSL "$REPO_URL/standards/c.md" -o "$COPILOT_DIR/standards/c.md" 2>/dev/null || {
            print_warning "Could not download C standards from remote"
        }
        curl -sSL "$REPO_URL/standards/cpp.md" -o "$COPILOT_DIR/standards/cpp.md" 2>/dev/null || {
            print_warning "Could not download C++ standards from remote"
        }
        curl -sSL "$REPO_URL/standards/golang.md" -o "$COPILOT_DIR/standards/golang.md" 2>/dev/null || {
            print_warning "Could not download Go standards from remote"
        }
        curl -sSL "$REPO_URL/standards/python.md" -o "$COPILOT_DIR/standards/python.md" 2>/dev/null || {
            print_warning "Could not download Python standards from remote"
        }
    fi
    
    # Check if files were created
    if [ -f "$COPILOT_DIR/standards/general.md" ]; then
        print_success "General programming standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/rust.md" ]; then
        print_success "Rust standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/nodejs.md" ]; then
        print_success "Node.js standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/c.md" ]; then
        print_success "C standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/cpp.md" ]; then
        print_success "C++ standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/golang.md" ]; then
        print_success "Go standards installed"
    fi
    
    if [ -f "$COPILOT_DIR/standards/python.md" ]; then
        print_success "Python standards installed"
    fi
}

install_prompts() {
    print_info "Installing prompt files..."
    
    # Download the setup-project prompt
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/templates/prompts/setup-project.md" -o "$COPILOT_DIR/prompts/setup-project.md" 2>/dev/null || {
            print_warning "Could not download setup-project.md from remote"
            create_setup_prompt_local
        }
        curl -sSL "$REPO_URL/templates/prompts/code-audit.md" -o "$COPILOT_DIR/prompts/code-audit.md" 2>/dev/null || {
            print_warning "Could not download code-audit.md from remote"
            create_code_audit_prompt_local
        }
    else
        create_setup_prompt_local
        create_code_audit_prompt_local
    fi
    
    if [ -f "$COPILOT_DIR/prompts/setup-project.md" ]; then
        print_success "Setup project prompt installed"
    fi
    
    if [ -f "$COPILOT_DIR/prompts/code-audit.md" ]; then
        print_success "Code audit prompt installed"
    fi
}

create_setup_prompt_local() {
    cat > "$COPILOT_DIR/prompts/setup-project.md" << 'PROMPT_EOF'
# Setup Project Documentation

Execute this prompt to fully initialize or update the Smart Agent documentation for this project.

## Instructions

Perform ALL of the following steps in order. Do not skip any steps.

---

## Step 1: Analyze Project Structure

Scan the entire project and identify:
- Root directory structure and key folders
- All programming languages used (by file extension and content)
- Package/dependency files (package.json, Cargo.toml, pyproject.toml, go.mod, etc.)
- Configuration files (.env.example, config files, etc.)
- Build/bundler configuration (webpack, vite, tsconfig, etc.)
- CI/CD configuration (.github/workflows, .gitlab-ci.yml, etc.)
- Existing documentation (README.md, docs/, etc.)

---

## Step 2: Create/Update Documentation Files

### 2.1: Overview (docs/overview.md)
- Project name, description, purpose
- Key features and capabilities
- Quick start instructions

### 2.2: Architecture (docs/architecture.md)
- System components and their relationships
- Data flow diagrams (text-based)
- Key design patterns used

### 2.3: Tech Stack (docs/tech-stack.md)
- Languages with versions
- Frameworks and libraries
- Development tools
- All dependencies from package files

### 2.4: API Documentation (docs/api.md)
- Endpoints (if web service)
- Public interfaces
- Data models/schemas

### 2.5: Testing (docs/testing.md)
- Test framework and tools
- Test file locations
- Test commands
- Coverage requirements

### 2.6: Development (docs/development.md)
- Setup instructions
- Build commands
- Run commands
- Environment variables

### 2.7: Conventions (docs/conventions.md)
- Code style patterns found
- Naming conventions
- File organization patterns
- Commit message format (if detectable)

---

## Step 3: Build Search Index

Update `.copilot/docs/index.yaml` with:
- Project identity (name, type, primary language)
- Document summaries (2-3 sentences each)
- Keywords for each document
- Cross-references between related docs
- Quick commands for common operations

---

## Step 4: Initialize Other Files

- Create `.copilot/plans/state.yaml` if not exists
- Create `.copilot/docs/decisions/index.yaml` if not exists
- Update `.copilot/instructions.md` with any GitHub config found

---

## Step 5: Report Summary

Provide a summary of:
- Project name and tech stack
- Documentation files created/updated
- Key findings about the codebase
- Recommendations for improvement

---

## Important Notes

1. **Use real data** - Do not use placeholder text. Analyze the actual codebase.
2. **Be thorough** - Scan all relevant files, not just root level.
3. **No duplication** - Each piece of information lives in ONE place.
4. **Update index** - The search index must reflect all documentation.
PROMPT_EOF
}

create_code_audit_prompt_local() {
    cat > "$COPILOT_DIR/prompts/code-audit.md" << 'AUDIT_EOF'
# Code Audit

> Perform a comprehensive code audit based on installed best practices

## Pre-Requisites Check (CRITICAL - DO THIS FIRST!)

```
CHECK if .copilot/standards/ directory exists AND contains at least one .md file

IF no standards found:
  ❌ STOP IMMEDIATELY
  
  Display:
  "⛔ **Code Audit Cannot Proceed**
  
  No best practices standards found in `.copilot/standards/`.
  
  **To fix this:**
  1. Run the Smart Agent installer with standards: `./install-with-standards.sh`
  2. Or manually add standard files to `.copilot/standards/`
  
  Available standards:
  - `general.md` - General coding best practices
  - `nodejs.md` - Node.js/TypeScript specific
  - `rust.md` - Rust specific
  
  Cannot audit code without defined standards to audit against."
  
  EXIT - Do not proceed with audit
```

## Audit Process

### Step 1: Load Standards

```
Read ALL .md files from .copilot/standards/
For each standard file:
  - Extract the rules and guidelines
  - Note the category (general, language-specific, framework-specific)
  - Build a checklist of items to verify
```

### Step 2: Load Project Context

```
Read .copilot/docs/index.yaml
From index, determine:
  - Primary language(s)
  - Framework(s) in use
  - Project structure
  - Key modules to audit

Only load standards relevant to the project's tech stack
```

### Step 3: Scan Codebase

```
For each applicable standard category:
  
  1. SECURITY
     - Check for hardcoded secrets/credentials
     - Verify input validation patterns
     - Check authentication/authorization implementations
     - Look for SQL injection vulnerabilities
     - Check for XSS vulnerabilities
     
  2. CODE QUALITY
     - Verify naming conventions
     - Check for code duplication
     - Verify error handling patterns
     - Check function/file length
     - Verify comments and documentation
     
  3. ARCHITECTURE
     - Verify layer separation
     - Check dependency injection patterns
     - Verify single responsibility principle
     - Check for circular dependencies
     
  4. TESTING
     - Check test coverage presence
     - Verify test naming conventions
     - Look for missing edge case tests
     
  5. PERFORMANCE
     - Check for N+1 query patterns
     - Verify async/await usage
     - Look for memory leak patterns
     - Check for unnecessary computations
     
  6. DEPENDENCIES
     - Check for outdated packages
     - Verify no unused dependencies
     - Check for security vulnerabilities
```

### Step 4: Generate Report

Create audit report at `.copilot/tmp/audit-report-[TIMESTAMP].md`:

```markdown
# Code Audit Report

**Project:** [project name]
**Date:** [current date]
**Standards Applied:** [list of standard files used]

## Summary

| Category | Issues Found | Severity |
|----------|--------------|----------|
| Security | X | 🔴 Critical / 🟡 Warning |
| Code Quality | X | 🟡 Warning / 🟢 Info |
| Architecture | X | ... |
| Testing | X | ... |
| Performance | X | ... |
| Dependencies | X | ... |

**Total Issues:** X
**Critical:** X | **Warning:** X | **Info:** X

## Critical Issues (Fix Immediately)

### [ISSUE-001] [Title]
- **Category:** Security
- **Severity:** 🔴 Critical
- **Location:** `path/to/file.ts:LINE`
- **Standard Violated:** [Reference to standard]
- **Description:** [What's wrong]
- **Recommendation:** [How to fix]
- **Example Fix:**
  \`\`\`typescript
  // Before
  [problematic code]
  
  // After
  [fixed code]
  \`\`\`

## Warnings (Fix Soon)

### [ISSUE-XXX] ...

## Informational (Consider Fixing)

### [ISSUE-XXX] ...

## Passed Checks ✅

- [List of standards that passed]

## Recommendations

### Immediate Actions
1. [ ] Fix critical security issues
2. [ ] ...

### Short-term (This Sprint)
1. [ ] Address warning-level issues
2. [ ] ...

### Long-term (Backlog)
1. [ ] Consider refactoring suggestions
2. [ ] ...

---
*Generated by Smart Agent Code Audit*
*Standards version: [date of standards files]*
```

## Output Format

After audit completes, display:

```
📊 **Code Audit Complete**

**Standards Applied:** [count] files from `.copilot/standards/`

**Results Summary:**
- 🔴 Critical: X issues
- 🟡 Warning: X issues  
- 🟢 Info: X suggestions

**Full Report:** `.copilot/tmp/audit-report-[TIMESTAMP].md`

**Top Priority Actions:**
1. [Most critical issue]
2. [Second most critical]
3. [Third most critical]

Run `@smart` and ask to "fix audit issue [ISSUE-ID]" to address specific issues.
```

## Notes

- Only audit files matching the project's primary languages
- Skip `node_modules/`, `vendor/`, `dist/`, `build/`, `.git/`
- Skip files in `.copilot/tmp/`
- Focus on patterns, not style (leave style to linters)
- Reference specific line numbers when possible
- Provide actionable, copy-paste-ready fixes
AUDIT_EOF
}

create_instructions_template() {
    print_info "Creating instructions template..."
    
    cat > "$COPILOT_DIR/instructions.md" << 'EOF'
# Project Instructions

This file contains project-specific instructions for the Planning Agent.
Edit this file to customize the agent's behavior for your project.

## GitHub Configuration

<!-- The planning agent will document any findings from .github/ here -->

## Custom Rules

<!-- Add your project-specific rules here -->

## Preferences

<!-- Add any preferences for how plans should be created -->

## Notes

<!-- Any additional notes for the planning agent -->
EOF
    
    print_success "Instructions template created"
}

# =============================================================================
# Main Installation Flow
# =============================================================================

parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --with-standards)
                INSTALL_STANDARDS=true
                shift
                ;;
            --no-standards)
                INSTALL_STANDARDS=false
                shift
                ;;
            --minimal)
                INSTALL_MINIMAL=true
                INSTALL_STANDARDS=false
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

interactive_mode() {
    # Standards are installed by default unless --minimal or --no-standards was passed
    # No interactive prompt needed - just proceed with defaults
    if [ "$INSTALL_MINIMAL" = true ]; then
        INSTALL_STANDARDS=false
    fi
}

main() {
    parse_arguments "$@"
    
    print_banner
    
    # Check if we're in a git repository
    if [ ! -d ".git" ]; then
        print_warning "Not in a git repository. The .copilot folder will still be created."
    fi
    
    # Interactive mode if no flags provided
    interactive_mode
    
    echo ""
    print_info "Starting installation..."
    echo ""
    
    # Run installation steps
    create_directory_structure
    create_gitignore
    create_state_yaml
    create_docs_index
    create_docs_decisions_index
    install_smart_agent
    install_copilot_instructions
    install_prompts
    create_instructions_template
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        install_standards
    fi
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                    Installation Complete!                      ${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "The following has been installed:"
    echo "  • Smart agent:          .github/agents/smart.agent.md"
    echo "  • Copilot instructions: .github/copilot-instructions.md"
    echo "  • Setup prompt:         .copilot/prompts/setup-project.md"
    echo "  • Copilot folder:       .copilot/"
    echo "  • Documentation:        .copilot/docs/"
    echo "  • Search index:         .copilot/docs/index.yaml"
    echo "  • Plans tracker:        .copilot/plans/state.yaml"
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        echo "  • Standards:            .copilot/standards/"
    fi
    
    echo ""
    echo "Next steps:"
    echo "  1. Review .copilot/instructions.md and add project-specific rules"
    echo "  2. Use the @smart agent in GitHub Copilot to start planning"
    echo "  3. Run 'Setup Project' handoff to auto-analyze and document your project"
    echo "  4. The agent will populate docs/ with comprehensive documentation"
    echo ""
    print_info "Note: .copilot/ contents are gitignored by default"
    print_info "Tip: Use the 'Setup Project' handoff button to auto-configure!"
    echo ""
}

# Run main function
main "$@"
