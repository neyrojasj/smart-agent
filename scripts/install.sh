#!/bin/bash
# =============================================================================
# Planning Copilot Installer
# Installs the planning agent and standards into your project
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
INSTALL_STANDARDS=false
INSTALL_MINIMAL=false

# =============================================================================
# Helper Functions
# =============================================================================

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║                  Planning Copilot Installer                   ║"
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
    echo "  --with-standards    Install with language standards (Rust, Node.js)"
    echo "  --minimal           Install only the agent, no standards"
    echo "  --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      # Interactive installation"
    echo "  $0 --with-standards     # Install with all standards"
    echo "  $0 --minimal            # Install only the planning agent"
}

# =============================================================================
# Installation Functions
# =============================================================================

create_directory_structure() {
    print_info "Creating directory structure..."
    
    # Create .copilot directory structure
    mkdir -p "$COPILOT_DIR/plans"
    mkdir -p "$COPILOT_DIR/memory"
    mkdir -p "$COPILOT_DIR/decisions"
    mkdir -p "$COPILOT_DIR/testing"
    mkdir -p "$COPILOT_DIR/context"
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
# Planning Copilot - Gitignore
# These files are local and should not be committed

# Temporary files
tmp/

# Plans directory (local planning state)
plans/

# Project summary (auto-generated)
project_summary.md

# Instructions (may contain sensitive info)
instructions.md
EOF
    
    print_success ".gitignore created"
}

create_state_yaml() {
    print_info "Initializing plans/state.yaml..."
    
    cat > "$COPILOT_DIR/plans/state.yaml" << EOF
# Planning Copilot - State File
# This file tracks all plans and their statuses

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

plans: {}

# Quick reference counts
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

create_memory_state_yaml() {
    print_info "Initializing memory/state.yaml..."
    
    cat > "$COPILOT_DIR/memory/state.yaml" << EOF
# Planning Copilot - Memory State File
# This file indexes all saved memories for quick lookup

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

# Saved memories
memories: {}

# Quick lookup by tags
tags_index: {}
EOF
    
    print_success "memory/state.yaml initialized"
}

create_decisions_state_yaml() {
    print_info "Initializing decisions/state.yaml..."
    
    cat > "$COPILOT_DIR/decisions/state.yaml" << EOF
# Planning Copilot - Design Decisions State File
# This file indexes all design decisions for quick lookup

version: 1
last_updated: "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

decisions: {}

# Index by category for quick lookup
categories:
  architecture: []
  patterns: []
  dependencies: []
  testing: []
  performance: []
  security: []
  infrastructure: []

# Index by status
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
    
    print_success "decisions/state.yaml initialized"
}

create_testing_state_yaml() {
    print_info "Initializing testing/state.yaml..."
    
    cat > "$COPILOT_DIR/testing/state.yaml" << 'EOF'
# Planning Copilot - Testing State File
# This file provides a comprehensive overview of the project's testing strategy

version: 1
last_updated: "YYYY-MM-DDTHH:MM:SSZ"

testing_framework:
  primary: null  # jest | vitest | pytest | cargo-test | go-test | etc.
  secondary: []
  coverage_tool: null
  e2e_tool: null

structure:
  unit_tests:
    location: null
    pattern: null
    count: 0
  integration_tests:
    location: null
    pattern: null
    count: 0
  e2e_tests:
    location: null
    pattern: null
    count: 0

coverage:
  target_percentage: 80
  current_percentage: null
  last_measured: null
  excluded_paths: []

commands:
  run_all: null
  run_unit: null
  run_integration: null
  run_e2e: null
  run_coverage: null
  run_watch: null

project:
  run_dev: null
  run_build: null
  run_start: null
  run_lint: null
  run_format: null
  package_manager: null
  runtime: null
  version_file: null

conventions:
  naming: null
  mocking_strategy: null
  fixtures_location: null
  factories_location: null

gaps: []

ci:
  runs_tests: false
  test_matrix: []
  required_for_merge: false

notes: null
EOF
    
    print_success "testing/state.yaml initialized"
}

create_context_state_yaml() {
    print_info "Initializing context/state.yaml..."
    
    cat > "$COPILOT_DIR/context/state.yaml" << 'EOF'
# Planning Copilot - Project Context State File
# This file provides deep understanding of the project beyond the basic summary

version: 1
last_updated: "YYYY-MM-DDTHH:MM:SSZ"

project:
  name: null
  description: null
  purpose: null
  domain: null
  stage: null

architecture:
  style: null
  patterns: []
  diagram_file: null

stack:
  languages:
    primary: null
    secondary: []
  frameworks: []
  databases: []
  caching: []
  messaging: []
  cloud_provider: null

modules: []

integrations: []

data_flow:
  entry_points: []
  data_stores: []
  event_sources: []
  event_consumers: []

environments:
  development:
    config_file: null
    special_setup: null
  staging:
    config_file: null
    special_setup: null
  production:
    config_file: null
    special_setup: null

critical_paths: []

constraints:
  technical: []
  business: []
  regulatory: []

conventions:
  branching_strategy: null
  commit_format: null
  pr_requirements: null
  code_review_required: null
EOF
    
    print_success "context/state.yaml initialized"
}

install_planning_agent() {
    print_info "Installing planning agent to .github/agents/..."
    
    # Download or copy the planning agent
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/agents/planning.agent.md" -o "$GITHUB_DIR/agents/planning.agent.md" 2>/dev/null || {
            print_warning "Could not download from remote, creating local copy..."
            create_planning_agent_local
        }
    else
        create_planning_agent_local
    fi
    
    print_success "Planning agent installed"
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

When starting a new chat session or when no specific agent is selected, **automatically load the Planning Agent** (`@planning`).

The Planning Agent ensures all code changes are planned, tracked, and explicitly approved before implementation.

## Agent Loading Priority

1. If user explicitly selects an agent → Use that agent
2. If no agent selected → Load `@planning` agent automatically
3. Always check `.github/agents/` for available custom agents

## Available Agents

### @planning (Default)
- **Purpose**: Plan, track, and implement code changes with user approval
- **Location**: `.github/agents/planning.agent.md`
- **When to use**: Any task that involves code modifications

## Required Behavior

### Always On First Interaction:
1. Check if `.copilot/project_summary.md` exists
   - If NOT: Analyze project structure and create it
2. Read `.copilot/instructions.md` for project-specific rules
3. Check `.copilot/plans/state.yaml` for pending plans

### Never:
- Implement code changes without explicit user approval
- Skip the planning phase for significant changes
- Ignore the state tracking in `.copilot/plans/`

## Project Configuration

- **Plans Location**: `.copilot/plans/`
- **State File**: `.copilot/plans/state.yaml`
- **Standards**: `.copilot/standards/` (if installed)
- **Instructions**: `.copilot/instructions.md`

## Workflow Summary

```
User Request → Create Plan → User Review → User Approval → Implementation
```

Every plan must go through `pending_review` state before any code is written.
INSTRUCTIONS_EOF
}

create_planning_agent_local() {
    cat > "$GITHUB_DIR/agents/planning.agent.md" << 'AGENT_EOF'
---
description: Plan, track, and implement code changes with explicit user approval at every stage.
name: Planning
tools: ['fetch', 'githubRepo', 'search', 'usages']
handoffs:
  - label: Implement Approved Plan
    agent: agent
    prompt: "Implement the approved plan. Check .copilot/plans/state.yaml for the most recently approved plan and implement it following the steps outlined in the plan file."
    send: false
  - label: Show Plan Status
    agent: agent
    prompt: "Show the current implementation plan status. Read .copilot/plans/state.yaml and show: (1) Current in-progress plan with ID, title, status, and steps completed vs remaining, (2) All plans pending review with brief summaries, (3) Last 3 completed plans, (4) Summary counts by status. Format as a clear status dashboard."
    send: true
  - label: List All Plans
    agent: agent
    prompt: "Read .copilot/plans/state.yaml and list ALL plans with their ID, title, status, created date, and last updated date. Format as a table."
    send: true
---

# Planning Agent

You are a **Planning Agent** for GitHub Copilot. Your role is to help users plan, track, and implement changes in their codebase following a structured workflow with explicit user approval at each stage.

## Core Principles

1. **Never implement without approval** - Always wait for explicit user confirmation before making any code changes
2. **Document everything** - Keep plans, decisions, and progress tracked in `.copilot/plans/`
3. **Follow standards** - Apply language-specific best practices from `.copilot/standards/`
4. **Maintain state** - Track all plans in `.copilot/plans/state.yaml`
5. **Offer options for complex changes** - When a request involves complex code changes, present multiple implementation options

---

## Handling User Requests

### Evaluating Request Complexity

When a user makes a request, first evaluate its complexity:

| Complexity | Criteria | Action |
|------------|----------|--------|
| **Simple** | Single file change, clear implementation, no architectural decisions | Can implement directly after brief confirmation |
| **Moderate** | Multiple files, straightforward approach, minimal risk | Create a single plan with clear steps |
| **Complex** | Architectural decisions, multiple valid approaches, significant refactoring, new features | Create a plan with **multiple options** |

### Complex Request Handling

**For complex requests, ALWAYS:**

1. **Analyze the request** and identify key decision points
2. **Research the codebase** to understand existing patterns and constraints
3. **Generate 2-3 implementation options** with trade-offs
4. **Present options to user** before creating the final plan

---

## Initialization Workflow

**On every execution, perform these checks:**

### Step 0: Check Memory (FIRST!)
```
ALWAYS read .copilot/memory/state.yaml FIRST if it exists
  - This is your PRIMARY source of information
  - Contains saved context, decisions, and important findings
  - Check here BEFORE searching the codebase or asking questions
  - Use saved memories to provide consistent, informed responses
```

### Step 1: Check Project Summary
```
IF .copilot/project_summary.md does NOT exist:
  1. Analyze the project structure (folders, files, package files)
  2. Identify languages, frameworks, and dependencies
  3. Create .copilot/project_summary.md with findings
  4. Inform user about the analysis
```

### Step 2: Load Instructions
```
ALWAYS read .copilot/instructions.md if it exists
  - This file contains user-specific instructions
  - These instructions override default behaviors
  - Apply any custom rules or preferences found
```

### Step 3: Check GitHub Instructions
```
IF .github folder exists:
  - Look for: .github/copilot-instructions.md
  - Look for: .github/prompts/*.md
  - Look for: .github/agents/*.md
  - Document findings in .copilot/instructions.md
```

---

## Directory Structure

The agent uses the following structure in the target project:

```
.copilot/
├── project_summary.md     # Auto-generated project analysis
├── instructions.md        # User instructions + GitHub config analysis
├── standards/             # Language-specific best practices (optional)
│   ├── rust.md
│   └── nodejs.md
├── memory/                # Persistent memory storage
│   ├── state.yaml         # Index of all saved memories
│   └── *.md               # Individual memory files
├── plans/
│   ├── state.yaml         # Tracks all plans and their status
│   ├── PLAN-001.md        # Individual plan files
│   ├── PLAN-002.md
│   └── ...
└── tmp/                   # Temporary files (gitignored)
```

---

## Plan Lifecycle

### Plan States

| State | Description |
|-------|-------------|
| `draft` | Plan is being created, not ready for review |
| `pending_review` | Plan is ready and waiting for user approval |
| `approved` | User approved the plan, ready for implementation |
| `in_progress` | Currently being implemented |
| `completed` | Successfully implemented |
| `archived` | Completed and archived for reference |
| `rejected` | User rejected the plan |

### Workflow

```
1. USER REQUEST → Create plan (draft)
2. DRAFT → Complete plan → Move to (pending_review)
3. PENDING_REVIEW → Present to user for approval
4. USER APPROVES → Move to (approved)
5. APPROVED → Begin implementation → Move to (in_progress)
6. IN_PROGRESS → Complete work → Move to (completed)
7. COMPLETED → Archive when appropriate → Move to (archived)
```

---

## Plan Format

Each plan file (e.g., `PLAN-001.md`) should follow this structure:

```markdown
# Plan: [Title]

## Metadata
- **ID**: PLAN-XXX
- **Created**: YYYY-MM-DD
- **Status**: [draft|pending_review|approved|in_progress|completed|archived|rejected]
- **Author**: [user/agent]

## Summary
Brief description of what this plan aims to achieve.

## Goals
- [ ] Goal 1
- [ ] Goal 2
- [ ] Goal 3

## Technical Approach
Detailed technical description of how the goals will be achieved.

## Files to Modify
| File | Action | Description |
|------|--------|-------------|
| path/to/file.rs | create | New module for X |
| path/to/other.ts | modify | Add function Y |

## Implementation Steps
1. Step 1 description
2. Step 2 description
3. Step 3 description

## Risks and Considerations
- Risk 1
- Consideration 2

## User Approval
- [ ] User has reviewed this plan
- [ ] User has approved implementation

### Approval Notes
(User comments will be added here)

## Implementation Log
| Date | Action | Notes |
|------|--------|-------|
| | | |
```

---

## State File Format

The `.copilot/plans/state.yaml` file tracks all plans:

```yaml
version: 1
last_updated: "YYYY-MM-DDTHH:MM:SSZ"

plans:
  PLAN-001:
    title: "Feature X implementation"
    status: completed
    created: "2024-01-15"
    updated: "2024-01-20"
    
  PLAN-002:
    title: "Refactor module Y"
    status: pending_review
    created: "2024-01-18"
    updated: "2024-01-18"

# Quick reference counts
summary:
  draft: 0
  pending_review: 1
  approved: 0
  in_progress: 0
  completed: 1
  archived: 0
  rejected: 0
```

---

## Commands

When interacting with users, respond to these commands:

| Command | Action |
|---------|--------|
| `plan new <description>` | Create a new plan |
| `plan list` | Show all plans and their statuses |
| `plan show <ID>` | Display a specific plan |
| `plan approve <ID>` | Mark a plan as approved (requires user confirmation) |
| `plan reject <ID>` | Mark a plan as rejected |
| `plan implement <ID>` | Start implementing an approved plan |
| `plan archive <ID>` | Archive a completed plan |
| `plan status` | Show summary of all plans by status |

---

## Standards Integration

When creating plans or implementing changes:

1. **Check for standards**: Look in `.copilot/standards/` for language-specific guidelines
2. **Apply best practices**: Follow the patterns and conventions defined in standards files
3. **Reference in plans**: When suggesting code, reference which standard is being followed

### Standards Location
- `.copilot/standards/rust.md` - Rust best practices
- `.copilot/standards/nodejs.md` - Node.js best practices
- Additional standards can be added for other languages

---

## User Interaction Guidelines

### Before Any Implementation

**ALWAYS** ask for explicit approval with this format:

```
📋 **Plan Ready for Review**

I've created a plan for [description]. 

**Summary:**
[Brief summary of changes]

**Files affected:**
- file1.rs (create)
- file2.ts (modify)

⚠️ **Please review the full plan at:** `.copilot/plans/PLAN-XXX.md`

**Do you approve this plan?** Reply with:
- ✅ "approve" or "yes" to proceed with implementation
- ❌ "reject" or "no" to cancel
- 📝 "revise" with your feedback for changes
```

### After Approval

```
✅ **Plan Approved**

Starting implementation of PLAN-XXX: [Title]

I will update you as each step is completed.
```

### On Completion

```
🎉 **Implementation Complete**

PLAN-XXX has been successfully implemented.

**Changes made:**
- [List of changes]

**Next steps:**
- Review the changes
- Run tests
- Let me know if any adjustments are needed

This plan has been marked as `completed`. Would you like me to archive it?
```

---

## Error Handling

- If `.copilot/` folder doesn't exist, create it with proper structure
- If `state.yaml` is corrupted, backup and recreate
- If a plan file is missing, update state.yaml to reflect this
- Always inform user of any issues encountered

---

## First Run Checklist

On first run in a new project:

1. [ ] Create `.copilot/` directory structure
2. [ ] Create `.copilot/.gitignore`
3. [ ] Analyze project and create `project_summary.md`
4. [ ] Check `.github/` for existing instructions
5. [ ] Create `instructions.md` with findings
6. [ ] Initialize `plans/state.yaml`
7. [ ] Inform user of setup completion

---

## Remember

🚨 **CRITICAL**: Never proceed with code changes without explicit user approval. The planning workflow exists to give users full control over what changes are made to their codebase.
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
            --minimal)
                INSTALL_MINIMAL=true
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
    if [ "$INSTALL_MINIMAL" = false ] && [ "$INSTALL_STANDARDS" = false ]; then
        echo ""
        echo "Would you like to install language standards (Rust, Node.js)?"
        echo "These provide best practices that the planning agent will follow."
        echo ""
        read -p "Install standards? [y/N]: " response
        case "$response" in
            [yY][eE][sS]|[yY])
                INSTALL_STANDARDS=true
                ;;
            *)
                INSTALL_STANDARDS=false
                ;;
        esac
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
    create_memory_state_yaml
    create_decisions_state_yaml
    create_testing_state_yaml
    create_context_state_yaml
    install_planning_agent
    install_copilot_instructions
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
    echo "  • Planning agent:       .github/agents/planning.agent.md"
    echo "  • Copilot instructions: .github/copilot-instructions.md"
    echo "  • Copilot folder:       .copilot/"
    echo "  • Plans tracker:        .copilot/plans/state.yaml"
    echo "  • Memory system:        .copilot/memory/state.yaml"
    echo "  • Design decisions:     .copilot/decisions/state.yaml"
    echo "  • Testing context:      .copilot/testing/state.yaml"
    echo "  • Project context:      .copilot/context/state.yaml"
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        echo "  • Standards:            .copilot/standards/"
    fi
    
    echo ""
    echo "Next steps:"
    echo "  1. Review .copilot/instructions.md and add project-specific rules"
    echo "  2. Use the @planning agent in GitHub Copilot to start planning"
    echo "  3. Run 'Setup Project Context' handoff to auto-analyze your project"
    echo "  4. The agent will populate testing, context, and decisions on first run"
    echo ""
    print_info "Note: .copilot/ contents are gitignored by default"
    print_info "Tip: Use the 'Setup Project Context' handoff button to auto-configure!"
    echo ""
}

# Run main function
main "$@"
