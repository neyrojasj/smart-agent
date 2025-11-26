# Planning Copilot

A structured planning agent for GitHub Copilot that helps you plan, track, and implement code changes with explicit user approval at every stage.

## Features

- 🎯 **Structured Planning Workflow** - Never implement changes without explicit approval
- 📋 **Plan Tracking** - Track plans through their lifecycle (draft → pending_review → approved → in_progress → completed → archived)
- 📁 **Organized Storage** - All plans stored in `.copilot/plans/` with state tracking
- 🛡️ **Best Practices** - Language-specific standards for Rust and Node.js
- 🔍 **Project Analysis** - Auto-generates project summary on first run
- 📝 **GitHub Integration** - Reads existing `.github/` configurations

## Quick Start

### Installation

Run one of these commands in your project root:

```bash
# Interactive installation (prompts for options)
curl -sSL https://raw.githubusercontent.com/neyrojasj/planning-copilot/main/scripts/install.sh | bash

# Install with language standards (Rust, Node.js)
curl -sSL https://raw.githubusercontent.com/neyrojasj/planning-copilot/main/scripts/install.sh | bash -s -- --with-standards

# Minimal installation (agent only, no standards)
curl -sSL https://raw.githubusercontent.com/neyrojasj/planning-copilot/main/scripts/install.sh | bash -s -- --minimal
```

### What Gets Installed

```
your-project/
├── .github/
│   ├── copilot-instructions.md  # Auto-loads planning agent
│   └── agents/
│       └── planning.agent.md    # The planning agent
└── .copilot/
    ├── .gitignore           # Excludes temp files from git
    ├── instructions.md      # Project-specific instructions
    ├── project_summary.md   # Auto-generated project analysis
    ├── standards/           # (Optional) Language best practices
    │   ├── rust.md
    │   └── nodejs.md
    ├── plans/
    │   ├── state.yaml       # Tracks all plans
    │   └── PLAN-XXX.md      # Individual plan files
    └── tmp/                 # Temporary files
```

## How It Works

### Auto-Loading the Planning Agent

The `copilot-instructions.md` file ensures that when no specific agent is selected, the **Planning Agent is automatically loaded**. This guarantees that all code changes go through the planning workflow.

### 1. First Run Initialization

When you first use the planning agent, it will:

1. ✅ Check if `.copilot/project_summary.md` exists
2. 📊 If not, analyze your project structure and create it
3. 📖 Read `.copilot/instructions.md` for custom rules
4. 🔍 Scan `.github/` folder for existing Copilot configurations
5. 📝 Document findings in `.copilot/instructions.md`

### 2. Planning Workflow

```
User Request → Create Plan (draft)
                    ↓
              Complete Plan → Move to (pending_review)
                    ↓
              Present to User for Approval
                    ↓
         ┌─────────┴─────────┐
         ↓                   ↓
    User Approves       User Rejects
         ↓                   ↓
   (approved)           (rejected)
         ↓
   Begin Implementation → (in_progress)
         ↓
   Complete Work → (completed)
         ↓
   Archive → (archived)
```

### 3. User Approval Required

The agent will **NEVER** implement changes without explicit approval:

```
📋 **Plan Ready for Review**

I've created a plan for [description]. 

**Summary:**
[Brief summary of changes]

**Files affected:**
- file1.rs (create)
- file2.ts (modify)

⚠️ **Please review the full plan at:** `.copilot/plans/PLAN-001.md`

**Do you approve this plan?** Reply with:
- ✅ "approve" or "yes" to proceed
- ❌ "reject" or "no" to cancel
- 📝 "revise" with your feedback
```

## Commands

Use these commands when chatting with the `@planning` agent:

| Command | Description |
|---------|-------------|
| `plan new <description>` | Create a new plan |
| `plan list` | Show all plans and their statuses |
| `plan show <ID>` | Display a specific plan |
| `plan approve <ID>` | Approve a plan for implementation |
| `plan reject <ID>` | Reject a plan |
| `plan implement <ID>` | Start implementing an approved plan |
| `plan archive <ID>` | Archive a completed plan |
| `plan status` | Show summary of all plans by status |

## Plan States

| State | Description |
|-------|-------------|
| `draft` | Plan is being created, not ready for review |
| `pending_review` | Plan is ready and waiting for user approval |
| `approved` | User approved the plan, ready for implementation |
| `in_progress` | Currently being implemented |
| `completed` | Successfully implemented |
| `archived` | Completed and archived for reference |
| `rejected` | User rejected the plan |

## State Tracking

All plans are tracked in `.copilot/plans/state.yaml`:

```yaml
version: 1
last_updated: "2024-01-20T10:30:00Z"

plans:
  PLAN-001:
    title: "Add user authentication"
    status: completed
    created: "2024-01-15"
    updated: "2024-01-20"
    
  PLAN-002:
    title: "Refactor database layer"
    status: pending_review
    created: "2024-01-18"
    updated: "2024-01-18"

summary:
  draft: 0
  pending_review: 1
  approved: 0
  in_progress: 0
  completed: 1
  archived: 0
  rejected: 0
```

## Language Standards

When installed with `--with-standards`, the agent will apply best practices for:

### General (`standards/general.md`) - **Core Principles**
Fundamental programming standards that apply across ALL languages:

- **No default environment variables** - Missing config must fail at startup
- **No silent error swallowing** - All errors must be handled or propagated
- **No catch-all defaults** - Pattern matching must be exhaustive
- **No unsafe unwrapping** - Always use `.expect()` with reason or handle explicitly
- **Fail fast, fail loud** - Bugs caught in development, not production

### Rust (`standards/rust.md`)
- Project structure conventions
- Error handling with `thiserror` and `anyhow`
- Memory and performance guidelines
- Testing patterns
- Clippy lints configuration

### Node.js (`standards/nodejs.md`)
- TypeScript configuration
- ESLint and Prettier setup
- Error handling patterns
- Security best practices
- Testing with Vitest/Jest

## Customization

### Project-Specific Instructions

Edit `.copilot/instructions.md` to add:

```markdown
## Custom Rules
- Always use TypeScript strict mode
- Follow existing code style
- Include tests for new features

## Preferences
- Create small, focused plans
- Reference relevant documentation
```

### Adding New Standards

Create new files in `.copilot/standards/`:

```
.copilot/standards/
├── rust.md
├── nodejs.md
├── python.md      # Add your own
└── golang.md      # Add your own
```

## Repository Structure

```
planning-copilot/
├── agents/
│   └── planning.agent.md    # Main planning agent (with YAML frontmatter)
├── standards/
│   ├── general.md           # General programming standards (all languages)
│   ├── rust.md              # Rust best practices
│   └── nodejs.md            # Node.js best practices
├── scripts/
│   ├── install.sh           # Main installer
│   ├── install-with-standards.sh
│   └── install-minimal.sh
├── templates/
│   ├── .copilot-gitignore   # Gitignore template
│   ├── copilot-instructions.md  # Default copilot instructions
│   ├── state.yaml           # State file template
│   ├── plan-template.md     # Plan file template
│   ├── project-summary-template.md
│   └── instructions-template.md
└── README.md
```

## Agent Configuration

The planning agent uses VS Code's custom agent format with YAML frontmatter:

```yaml
---
description: Plan, track, and implement code changes with explicit user approval.
name: Planning
tools: ['fetch', 'githubRepo', 'search', 'usages']
handoffs:
  - label: Implement Approved Plan
    agent: agent
    prompt: "Implement the approved plan..."
    send: false
---
```

### Handoffs

The agent supports handoffs to transition between agents:
- **Implement Approved Plan**: Hands off to the default agent to implement an approved plan

## Gitignore

By default, the `.copilot/` folder contents are gitignored to keep plans and temporary files local:

```gitignore
# .copilot/.gitignore
tmp/
plans/
```

If you want to track plans in git, remove `plans/` from the gitignore.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - See [LICENSE](LICENSE) for details.
