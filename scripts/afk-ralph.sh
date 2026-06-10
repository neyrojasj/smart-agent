#!/usr/bin/env bash
#
# afk-ralph.sh — AFK "Ralph" loop for building a PRD one vertical slice at a time.
#
# For each open issue labelled `slice` (in ascending issue-number order), it spins
# up a FRESH headless `claude` session that implements exactly that one slice with
# TDD and lands a single commit. Continuity between sessions comes from the last 5
# commits + the issue tracker + the working tree (the Ralph pattern).
#
# Usage:
#   scripts/afk-ralph.sh [PRD_ISSUE]        # default PRD_ISSUE=1
#   scripts/afk-ralph.sh 1 --dry-run        # print the prompt for the next slice, don't run
#
# Env knobs:
#   RALPH_LABEL    issue label that marks a slice            (default: slice)
#   RALPH_MODEL    model passed to claude --model            (default: unset → CLI default)
#   RALPH_MAX      safety cap on iterations                  (default: 50)
#
set -euo pipefail

PRD_ISSUE="${1:-1}"
DRY_RUN=false
[[ "${2:-}" == "--dry-run" ]] && DRY_RUN=true

LABEL="${RALPH_LABEL:-slice}"
MAX_ITERS="${RALPH_MAX:-50}"
LOG_DIR="logs/ralph"

# ---- preflight ----------------------------------------------------------------
command -v gh >/dev/null     || { echo "✗ gh CLI not found"; exit 1; }
command -v jq >/dev/null     || { echo "✗ jq not found"; exit 1; }
command -v claude >/dev/null || { echo "✗ claude CLI not found"; exit 1; }
git rev-parse --is-inside-work-tree >/dev/null 2>&1 || { echo "✗ not a git repo"; exit 1; }
mkdir -p "$LOG_DIR"

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

PRD_BODY="$(gh issue view "$PRD_ISSUE" --json body --jq .body)"
[[ -n "$PRD_BODY" ]] || { echo "✗ PRD issue #$PRD_ISSUE has no body / not found"; exit 1; }

# ---- prompt builder -----------------------------------------------------------
build_prompt() {
  local num="$1" title="$2" body="$3" recent="$4"
  cat <<EOF
You are an autonomous engineer building the \`weft\` requirements-traceability tool.
Work on EXACTLY ONE vertical slice this session, then stop. Do not start any other slice.

## Product context — PRD #$PRD_ISSUE
$PRD_BODY

## Your slice this session — issue #$num: $title
$body

## Recent commits (last 5, for continuity with prior sessions)
$recent

## How to work
1. Read the repo's CONTEXT.md glossary and the ADRs in docs/adr/ — follow that language and those decisions.
2. Use STRICT TDD (red → green → refactor): write a failing test first and watch it fail, write the
   simplest code to make it pass, then refactor. Lean on the project's tdd skill/conventions.
3. Implement ONLY the scope of issue #$num. Touch no unrelated files. Do not start later slices.
4. Where this tool's own requirements records exist (docs/prds/), keep them traceable with the
   appropriate @implements / @verifies annotations described in the PRD.
5. Run the FULL test suite. It must be green before you commit.

## Finishing (required)
- When green, make EXACTLY ONE git commit for this slice.
- Commit subject MUST start with: \`slice #$num: \`
- End the commit message with this trailer line:
  Co-Authored-By: Claude Opus 4.8 <noreply@anthropic.com>
- Do NOT push and do NOT open a PR.
- If you cannot finish the slice, commit NOTHING and explain clearly what blocked you.
EOF
}

# ---- the loop -----------------------------------------------------------------
last_num=""
no_progress=0
iter=0

while (( iter++ < MAX_ITERS )); do
  # Filter the label CLIENT-SIDE in jq rather than via `gh --label`: when the
  # repo has been renamed, gh's server-side label filter doesn't follow the
  # redirect and silently returns nothing, whereas the unfiltered list does.
  slice_json="$(gh issue list --state open --json number,title,body,labels \
                  --jq "[.[] | select(any(.labels[]; .name==\"$LABEL\"))] | sort_by(.number) | .[0] // empty")"
  if [[ -z "$slice_json" ]]; then
    echo "✓ No open '$LABEL' issues remain — all slices done."
    exit 0
  fi

  num="$(jq -r '.number' <<<"$slice_json")"
  title="$(jq -r '.title' <<<"$slice_json")"
  body="$(jq -r '.body'  <<<"$slice_json")"
  recent="$(git log -5 --pretty=format:'- %h %s' 2>/dev/null || echo '(no commits yet)')"

  echo "──────────────────────────────────────────────────────────────"
  echo "→ Slice #$num: $title"
  prompt="$(build_prompt "$num" "$title" "$body" "$recent")"

  if $DRY_RUN; then
    echo "── DRY RUN: prompt for #$num ──"
    printf '%s\n' "$prompt"
    exit 0
  fi

  head_before="$(git rev-parse HEAD)"
  ts="$(date +%Y%m%d-%H%M%S)"
  log_file="$LOG_DIR/slice-${num}-${ts}.log"
  raw_file="$LOG_DIR/slice-${num}-${ts}.jsonl"

  echo "  running claude — streaming steps below (raw events → $raw_file)"
  echo "  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄"
  # Fresh headless session, AFK (no permission prompts), one slice.
  # stream-json + --verbose surfaces each assistant message / tool call live;
  # the jq filter renders those events human-readably while tee keeps full logs.
  printf '%s' "$prompt" | claude -p \
      --dangerously-skip-permissions \
      --verbose --output-format stream-json \
      ${RALPH_MODEL:+--model "$RALPH_MODEL"} \
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
    echo "✓ Slice #$num committed: $(git log -1 --pretty=format:'%h %s')"
    gh issue close "$num" --comment "Implemented by AFK Ralph loop: $head_after" >/dev/null || true
    no_progress=0
  else
    echo "⚠ Slice #$num produced no commit (see $log_file)."
    if [[ "$num" == "$last_num" ]]; then
      (( no_progress++ ))
    else
      no_progress=1
    fi
    if (( no_progress >= 2 )); then
      echo "✗ Slice #$num made no progress twice in a row — stopping for human review."
      exit 1
    fi
    echo "  Retrying once before giving up…"
  fi

  last_num="$num"
done

echo "✗ Hit iteration cap ($MAX_ITERS) — stopping."
exit 1
