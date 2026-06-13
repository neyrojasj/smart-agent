#!/usr/bin/env bash
# PreToolUse hook (matcher: Read|Grep|Glob|Bash).
#
# When the active model is Sonnet or Opus, blocks direct investigation tool
# calls (Read/Grep/Glob, plus `grep` invoked via Bash) and instructs the model
# to delegate the research to a Haiku subagent via the Agent tool instead.
# Haiku (and any other) models pass through.
set -euo pipefail

input="$(cat)"
tool_name="$(jq -r '.tool_name' <<< "$input")"

case "$tool_name" in
  Read|Grep|Glob) ;;
  Bash)
    command_str="$(jq -r '.tool_input.command // ""' <<< "$input")"
    # Only gate Bash invocations that run `grep` (as its own command, e.g.
    # `grep ...`, `... | grep ...`, `cmd && grep ...`). Other Bash commands
    # (ls, find, cat, etc.) pass through untouched.
    if ! grep -qE '(^|[|;&]|\s)grep(\s|$)' <<< "$command_str"; then
      exit 0
    fi
    ;;
  *) exit 0 ;;
esac

transcript="$(jq -r '.transcript_path' <<< "$input")"

model=""
if [[ -f "$transcript" ]]; then
  model="$(tail -n 300 "$transcript" \
    | jq -rs '[.[] | select(.type == "assistant") | .message.model // empty] | last // ""' \
    2>/dev/null || true)"
fi

case "$model" in
  *opus*|*sonnet*)
    reason=$(cat <<'EOF'
Direct Read/Grep/Glob calls (and `grep` run via Bash) are disabled for Sonnet/Opus on this project. Delegate this investigation to a subagent running on Haiku instead.

Call the Agent tool now with model: "haiku" (subagent_type: "Explore" for pure search/navigation, or "general-purpose" for broader research), passing a SELF-CONTAINED prompt that includes:

1. GOAL — the exact question to answer or fact to locate.
2. SCOPE — exact paths, directories, file globs, or grep patterns to search. Be specific; do not make Haiku guess where to look.
3. OUTPUT FORMAT — the precise shape of the answer you need back (e.g. "a list of file:line matches with one line of context each", "the current value of X", "yes/no plus the deciding evidence").
4. CONTEXT — naming conventions, what to ignore, edge cases, and anything else needed to interpret results correctly without asking follow-up questions.
5. EXHAUSTIVENESS — tell it to be thorough within scope, then stop and report — not to broaden the search on its own.

Write the prompt as if briefing a competent but unfamiliar colleague who cannot ask follow-up questions: a vague brief produces a vague result. If you need several independent pieces of information, prefer one subagent call per coherent topic over many tiny ones.
EOF
    )
    jq -n --arg reason "$reason" '{
      hookSpecificOutput: {
        hookEventName: "PreToolUse",
        permissionDecision: "deny",
        permissionDecisionReason: $reason
      }
    }'
    ;;
  *)
    exit 0
    ;;
esac
