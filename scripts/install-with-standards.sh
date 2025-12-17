#!/bin/bash
# =============================================================================
# Smart Agent Installer - With Standards
# Installs the Smart Agent WITH language standards (Rust, Node.js, C, C++, Go, Python)
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Run the main installer with standards flag
"$SCRIPT_DIR/install.sh" --with-standards
