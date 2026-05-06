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
REPO_OWNER="neyrojasj"
REPO_NAME="smart-agent"
REPO_BRANCH="main"
REPO_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/archive/refs/heads/${REPO_BRANCH}.tar.gz"
TEMP_DIR=".smart-agent-temp"
COPILOT_DIR=".github/copilot"
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
# Repository Management
# =============================================================================

download_repository() {
    print_info "Downloading Smart Agent repository..."
    
    # Check if curl is available
    if ! command -v curl &> /dev/null; then
        print_error "curl is required but not installed"
        exit 1
    fi
    
    # Check if tar is available
    if ! command -v tar &> /dev/null; then
        print_error "tar is required but not installed"
        exit 1
    fi
    
    # Create temp directory
    mkdir -p "$TEMP_DIR"
    
    # Download repository tarball
    if ! curl -sSL "$REPO_URL" -o "$TEMP_DIR/repo.tar.gz"; then
        print_error "Failed to download repository from $REPO_URL"
        rm -rf "$TEMP_DIR"
        exit 1
    fi
    
    # Extract tarball
    if ! tar -xzf "$TEMP_DIR/repo.tar.gz" -C "$TEMP_DIR"; then
        print_error "Failed to extract repository"
        rm -rf "$TEMP_DIR"
        exit 1
    fi
    
    print_success "Repository downloaded and extracted"
}

cleanup_temp() {
    if [ -d "$TEMP_DIR" ]; then
        rm -rf "$TEMP_DIR"
        print_success "Cleaned up temporary files"
    fi
}

copy_file_from_repo() {
    local source_path="$1"
    local dest_path="$2"
    local repo_dir="$TEMP_DIR/${REPO_NAME}-${REPO_BRANCH}"
    
    if [ ! -f "$repo_dir/$source_path" ]; then
        print_error "File not found in repository: $source_path"
        cleanup_temp
        exit 1
    fi
    
    # Create destination directory if needed
    mkdir -p "$(dirname "$dest_path")"
    
    cp "$repo_dir/$source_path" "$dest_path"
}

copy_file_if_missing_from_repo() {
    local source_path="$1"
    local dest_path="$2"
    local label="$3"

    # Preserve any existing destination path, including symlinks.
    if [ -e "$dest_path" ] || [ -L "$dest_path" ]; then
        print_warning "$label already exists, preserving current file"
        return
    fi

    copy_file_from_repo "$source_path" "$dest_path"
    print_success "$label initialized"
}

# =============================================================================
# Installation Functions
# =============================================================================

create_directory_structure() {
    print_info "Creating directory structure..."
    
    # Create .github directory structure
    mkdir -p "$GITHUB_DIR/agents"
    mkdir -p "$COPILOT_DIR/skills"
    mkdir -p "$COPILOT_DIR/docs"
    mkdir -p "$COPILOT_DIR/plans"
    mkdir -p "$COPILOT_DIR/tmp"
    mkdir -p ".vscode"
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        mkdir -p "$COPILOT_DIR/standards"
    fi
    
    print_success "Directory structure created"
}

install_skills() {
    # -------------------------------------------------------------------------
    # Core skills — always overwritten with the latest version from the repo.
    # These are maintained by the Smart Agent framework and should never be
    # edited by hand. Customise behaviour via standards or project instructions.
    # -------------------------------------------------------------------------
    print_info "Updating core skills (always overwrite)..."

    local core_skills=(
        "planning"
        "plan-reviewer"
        "coding"
        "documentation"
        "testing"
        "setup"
        "skill-generator"
        "loop"
    )

    for skill in "${core_skills[@]}"; do
        mkdir -p "$COPILOT_DIR/skills/$skill"
        copy_file_from_repo ".github/copilot/skills/$skill/SKILL.md" "$COPILOT_DIR/skills/$skill/SKILL.md"
        print_success "Core skill updated: $skill"
    done

    # -------------------------------------------------------------------------
    # Custom skills — shipped as examples/starters; preserved once installed.
    # Skills generated by the skill-generator are also kept in this category.
    # Add your own entries below to bundle additional starter skills.
    # -------------------------------------------------------------------------
    print_info "Checking custom skills (preserve if exists)..."

    local custom_skills=(
        "rust-web-app"
    )

    for skill in "${custom_skills[@]}"; do
        mkdir -p "$COPILOT_DIR/skills/$skill"
        copy_file_if_missing_from_repo ".github/copilot/skills/$skill/SKILL.md" "$COPILOT_DIR/skills/$skill/SKILL.md" "Custom skill: $skill"
    done

    # -------------------------------------------------------------------------
    # index.yaml — preserved if it already exists so that custom skill entries
    # registered by skill-generator are not lost. A fresh copy is installed
    # only on first install. After updating, run the regen prompt (shown at the
    # end of this script) to merge new core skill entries into your index.
    # -------------------------------------------------------------------------
    copy_file_if_missing_from_repo ".github/copilot/skills/index.yaml" "$COPILOT_DIR/skills/index.yaml" "Skills index (index.yaml)"
}

install_gitignore() {
    print_info "Installing .github/copilot/.gitignore..."
    copy_file_from_repo ".github/copilot/gitignore.txt" "$COPILOT_DIR/.gitignore"
    print_success ".gitignore installed"
}

install_state_yaml() {
    print_info "Initializing plans/state.yaml (preserve if exists)..."
    local dest="$COPILOT_DIR/plans/state.yaml"
    if [ -e "$dest" ] || [ -L "$dest" ]; then
        print_warning "plans/state.yaml already exists, preserving current file"
        return
    fi
    mkdir -p "$(dirname "$dest")"
    cat > "$dest" <<'EOF'
# Smart Agent - Plans State File
# Tracks all plans and their statuses. Auto-updated by the planning skill.

version: 1
last_updated: ""

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
    print_success "plans/state.yaml initialized"
}

install_smart_agent() {
    print_info "Installing smart agents to .github/agents/..."
    copy_file_from_repo ".github/agents/smart.agent.md" "$GITHUB_DIR/agents/smart.agent.md"
    print_success "Smart agent installed"
}

install_copilot_instructions() {
    print_info "Installing copilot-instructions.md..."
    copy_file_from_repo ".github/copilot-instructions.md" "$GITHUB_DIR/copilot-instructions.md"
    print_success "copilot-instructions.md installed"
}

install_session_template() {
    print_info "Initializing session.md (preserve if exists)..."
    local dest="$COPILOT_DIR/session.md"
    if [ -e "$dest" ] || [ -L "$dest" ]; then
        print_warning "session.md already exists, preserving current file"
        return
    fi
    mkdir -p "$(dirname "$dest")"
    cat > "$dest" <<'EOF'
<!-- TEMPLATE: Auto-generated and updated each conversation by Smart Orchestrator. -->

# Session State

> Last updated: ""
> Active skill: ""
> Current task: ""

## Pending Tasks

(none)

## Recent Actions (last 20)

(none)

## Skill Confidence Log

| Skill | Confidence | Reason |
|-------|-----------|--------|

---

*Auto-updated by Smart Orchestrator. Overwritten each session.*
EOF
    print_success "session.md initialized"
}

install_context_template() {
    print_info "Initializing context.md (preserve if exists)..."
    local dest="$COPILOT_DIR/context.md"
    if [ -e "$dest" ] || [ -L "$dest" ]; then
        print_warning "context.md already exists, preserving current file"
        return
    fi
    mkdir -p "$(dirname "$dest")"
    cat > "$dest" <<'EOF'
<!-- ⚠️ REQUIRED: This file drives the Smart Orchestrator. Run the Setup skill (@smart setup project) to auto-generate. -->

# Project Context

> Last updated: ""

## Project Identity

- **Name**: ""
- **Type**: ""
- **Stack**: ""
- **Stage**: ""

## User Preferences

(none yet)

## Project-Specific Rules

(none yet)

## Key Decisions

| Decision | Reason | Skill | Date |
|----------|--------|-------|------|

---

*Auto-updated by Smart Orchestrator*
EOF
    print_success "context.md initialized"
}

install_standards() {
    if [ "$INSTALL_STANDARDS" = false ]; then
        return
    fi
    
    print_info "Installing language standards..."
    
    local standards=(
        "general.md:General programming standards"
        "markdown.md:Markdown standards"
        "rust.md:Rust standards"
        "nodejs.md:Node.js standards"
        "c.md:C standards"
        "cpp.md:C++ standards"
        "golang.md:Go standards"
        "python.md:Python standards"
    )
    
    for item in "${standards[@]}"; do
        IFS=':' read -r file desc <<< "$item"
        copy_file_from_repo ".github/copilot/standards/$file" "$COPILOT_DIR/standards/$file"
        print_success "$desc installed"
    done
}

install_instructions_template() {
    print_info "Installing instructions template (preserve if exists)..."

    copy_file_if_missing_from_repo ".github/copilot/instructions.md" "$COPILOT_DIR/instructions.md" "instructions.md"
}

install_vscode_settings() {
    print_info "Installing .vscode/settings.json (preserve if exists)..."
    copy_file_if_missing_from_repo ".vscode/settings.json" ".vscode/settings.json" ".vscode/settings.json"
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
        print_warning "Not in a git repository. The .github/copilot folder will still be created."
    fi
    
    # Interactive mode if no flags provided
    interactive_mode
    
    echo ""
    print_info "Starting installation..."
    echo ""
    
    # Download repository first
    download_repository
    
    # Run installation steps
    create_directory_structure
    install_gitignore
    install_state_yaml
    install_smart_agent
    install_copilot_instructions
    install_session_template
    install_context_template
    install_skills
    install_instructions_template
    install_vscode_settings
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        install_standards
    fi
    
    # Clean up downloaded repository
    cleanup_temp
    
    echo ""
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}                    Installation Complete!                      ${NC}"
    echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo "The following has been installed:"
    echo "  • Smart agent:          .github/agents/smart.agent.md"
    echo "  • Copilot instructions: .github/copilot-instructions.md"
    echo "  • Core skills (8):      .github/copilot/skills/  (always up-to-date)"
    echo "  • Custom skills:        .github/copilot/skills/  (preserved if existing)"
    echo "  • Copilot folder:       .github/copilot/"
    echo "  • Plans tracker:        .github/copilot/plans/state.yaml"
    echo "  • VS Code settings:     .vscode/settings.json"
    
    if [ "$INSTALL_STANDARDS" = true ]; then
        echo "  • Standards:            .github/copilot/standards/"
    fi
    
    echo ""
    echo "Next steps:"
    echo "  1. Use the @smart agent in GitHub Copilot Chat"
    echo "  2. Run 'Setup Project' handoff — scans your code and generates docs + instructions"
    echo "  3. The agent creates only the docs your project needs (no empty templates)"
    echo "  4. Then run 'Generate Skills' to create project-specific skills"
    echo ""
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}  Run this prompt to integrate updated core skills with your       ${NC}"
    echo -e "${YELLOW}  custom skills and regenerate the skill index:                   ${NC}"
    echo ""
    echo -e "  ${BLUE}@smart scan all skills in .github/copilot/skills/ and regenerate${NC}"
    echo -e "  ${BLUE}index.yaml, preserving all existing custom skill entries${NC}"
    echo -e "  ${BLUE}and updating core skill descriptions from their SKILL.md files${NC}"
    echo ""
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    print_info "Note: .github/copilot/ contents are gitignored by default"
    print_info "Tip: Use the 'Setup Project' handoff button to auto-configure!"
    echo ""
}

# Run main function
main "$@"
