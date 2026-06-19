#!/usr/bin/env bash
#
# afk-copilot.sh — GitHub Copilot AFK loop (installed as afk-ralph.sh).
#
# For each issue file under .scratch/*/issues/ with Status: ready-for-agent (in ascending
# path order), it spins up a FRESH headless `claude` session that implements exactly that
# one slice with TDD and lands a single commit. Continuity between sessions comes from the
# last 5 commits + the issue file + the working tree (the Ralph pattern).
#
# Usage (after weft init installs it as afk-ralph.sh):
#   .github/scripts/afk-ralph.sh [FEATURE]         # FEATURE: .scratch/<feature>/, PRD.md path, or slug
#   .github/scripts/afk-ralph.sh weft --dry-run    # print the prompt for the next slice, don't run
#
# Env knobs:
#   COPILOT_MAX      safety cap on iterations        (default: 50)
#
# Model is hardcoded to claude-sonnet-4-6.
#
set -euo pipefail

FEATURE_ARG="${1:-}"
DRY_RUN=false
[[ "${2:-}" == "--dry-run" ]] && DRY_RUN=true

MAX_ITERS="${COPILOT_MAX:-50}"
MODEL="claude-sonnet-4-6"
SCRATCH_DIR=".scratch"

# ---- preflight ----------------------------------------------------------------
command -v jq >/dev/null     || { echo "✗ jq not found"; exit 1; }
command -v claude >/dev/null || { echo "✗ claude CLI not found"; exit 1; }
git rev-parse --is-inside-work-tree >/dev/null 2>&1 || { echo "✗ not a git repo"; exit 1; }
[[ -d "$SCRATCH_DIR" ]] || { echo "✗ $SCRATCH_DIR/ not found — no local issues"; exit 1; }

# Resolve the feature folder and PRD path from the argument.
# Accepts: a folder (.scratch/feat-foo/ or feat-foo), a PRD.md path, or nothing (all features).
FEATURE_DIR=""
if [[ -n "$FEATURE_ARG" ]]; then
  arg="$FEATURE_ARG"
  # Strip trailing slash
  arg="${arg%/}"
  if [[ -d "$arg" ]]; then
    # Direct folder path
    FEATURE_DIR="$arg"
  elif [[ -d "$SCRATCH_DIR/$arg" ]]; then
    # Bare slug
    FEATURE_DIR="$SCRATCH_DIR/$arg"
  elif [[ -f "$arg" && "$arg" == *.md ]]; then
    # PRD.md path — feature folder is its parent
    FEATURE_DIR="$(dirname "$arg")"
  else
    echo "✗ Cannot resolve feature folder from: $FEATURE_ARG"; exit 1
  fi
  prd_path="$FEATURE_DIR/PRD.md"
  [[ -f "$prd_path" ]] || { echo "✗ PRD not found: $prd_path"; exit 1; }
  echo "→ Feature: $FEATURE_DIR"
  echo "→ PRD: $prd_path"
fi

# Work on the CURRENT branch — never switch or create. But if that branch is the
# default branch, the loop is about to commit autonomously straight onto it, so
# make the human confirm first.
current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$current_branch" == "main" || "$current_branch" == "master" ]]; then
  echo "⚠ You are on '$current_branch'. This loop will commit autonomously onto it."
  read -r -p "  Continue on '$current_branch'? [y/N] " reply </dev/tty
  case "$reply" in
    [yY]|[yY][eE][sS]) echo "→ continuing on '$current_branch'." ;;
    *) echo "✗ Aborted. Switch to a working branch and re-run."; exit 1 ;;
  esac
fi

# ---- issue helpers ------------------------------------------------------------
find_next_issue() {
  local search_root="${FEATURE_DIR:-$SCRATCH_DIR}"
  while IFS= read -r f; do
    local status
    status="$(grep -m1 '^Status:' "$f" 2>/dev/null | sed 's/^Status:[[:space:]]*//' | tr -d '[:space:]')"
    if [[ "$status" == "ready-for-agent" ]]; then
      echo "$f"
      return 0
    fi
  done < <(find "$search_root" -path "*/issues/*.md" | sort)
}

issue_title() {
  local f="$1"
  grep -m1 '^# ' "$f" 2>/dev/null | sed 's/^# //' || basename "$f" .md
}

mark_done() {
  local f="$1"
  sed -i '' 's/^Status: ready-for-agent/Status: done/' "$f"
}

# ---- prompt builder -----------------------------------------------------------
build_prompt() {
  local issue_path="$1" title="$2" recent="$3"
  cat <<EOF
Implement EXACTLY one vertical slice this session, then stop.

## This session: $title

Read the full scope, requirements table, and copy-paste-correct trace annotations:
  $issue_path

## Recent commits (continuity with prior sessions)
$recent

## How to work
1. Read CONTEXT.md and docs/adr/ first — follow the domain language and existing decisions.
2. Run /tdd — use strict red → green → refactor for every change.
3. Stay within the scope of the issue above. Do not touch unrelated files or start other slices.
4. Add @implements and @verifies trace annotations to every file you change.
5. Run the full test suite — it must be green before you commit.

## Finishing (required)
- ONE commit; subject must start with: slice: $title
- Trailer: Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
- Do NOT push. Do NOT open a PR.
- Update Status: done in $issue_path after the commit.
- If blocked, commit nothing and explain what stopped you.
EOF
}

# ---- the loop -----------------------------------------------------------------
last_path=""
no_progress=0
iter=0

while (( iter++ < MAX_ITERS )); do
  issue_path="$(find_next_issue)"
  if [[ -z "$issue_path" ]]; then
    echo "✓ No ready-for-agent issues remain — all slices done."
    exit 0
  fi

  title="$(issue_title "$issue_path")"
  recent="$(git log -5 --pretty=format:'- %h %s' 2>/dev/null || echo '(no commits yet)')"

  echo "──────────────────────────────────────────────────────────────"
  echo "→ $issue_path: $title"
  prompt="$(build_prompt "$issue_path" "$title" "$recent")"

  if $DRY_RUN; then
    echo "── DRY RUN: prompt for $issue_path ──"
    printf '%s\n' "$prompt"
    exit 0
  fi

  head_before="$(git rev-parse HEAD)"
  ts="$(date +%Y%m%d-%H%M%S)"
  slug="$(basename "$issue_path" .md)"
  LOG_DIR="$(dirname "$(dirname "$issue_path")")/logs"
  mkdir -p "$LOG_DIR"
  log_file="$LOG_DIR/${slug}-${ts}.log"
  raw_file="$LOG_DIR/${slug}-${ts}.jsonl"

  echo "  running claude — streaming steps below (raw events → $raw_file)"
  echo "  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄"
  # Fresh headless session, AFK (no permission prompts), one slice.
  # stream-json + --verbose surfaces each assistant message / tool call live;
  # the jq filter renders those events human-readably while tee keeps full logs.
  printf '%s' "$prompt" | claude -p \
      --dangerously-skip-permissions \
      --verbose --output-format stream-json \
      --model "$MODEL" \
    | tee "$raw_file" \
    | jq --unbuffered -r '
        if   .type=="system" and .subtype=="init" then "⚙️  session start (model \(.model // "?"))"
        elif .type=="assistant" then
          ( .message.content[]?
            | if   .type=="text"     then .text
              elif .type=="tool_use" then "🔧 \(.name): \(.input|tostring|.[0:200])"
              else empty end )
        elif .type=="user" then
          ( .message.content[]?
            | select(.type=="tool_result")
            | (.content // "")
            | (if type=="array" then (map(.text // "")|join(" ")) else tostring end)
            | "   ↳ \(.[0:200])" )
        elif .type=="result" then "✅ \(.subtype // "done")  (cost $\(.total_cost_usd // 0))"
        else empty end' 2>/dev/null \
    | tee "$log_file" || true
  echo "  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄"

  head_after="$(git rev-parse HEAD)"

  if [[ "$head_before" != "$head_after" ]]; then
    echo "✓ $title committed: $(git log -1 --pretty=format:'%h %s')"
    mark_done "$issue_path"
    no_progress=0
  else
    echo "⚠ $issue_path produced no commit (see $log_file)."
    if [[ "$issue_path" == "$last_path" ]]; then
      (( no_progress++ ))
    else
      no_progress=1
    fi
    if (( no_progress >= 2 )); then
      echo "✗ $issue_path made no progress twice in a row — stopping for human review."
      exit 1
    fi
    echo "  Retrying once before giving up…"
  fi

  last_path="$issue_path"
done

echo "✗ Hit iteration cap ($MAX_ITERS) — stopping."
exit 1
