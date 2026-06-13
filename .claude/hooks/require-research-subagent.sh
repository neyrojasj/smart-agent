#!/usr/bin/env bash
# PreToolUse hook (matcher: *, i.e. every tool call).
#
# When the active model is Opus (or a future tier above it), blocks ALL tool
# calls except a small orchestration set (Agent, AskUserQuestion, plan-mode
# and task-tracking tools). Opus job on this project is to think, plan, and
# delegate ALL execution to Sonnet 4.6 subagents via the Agent tool - not to
# call tools directly.
#
# Sonnet, Haiku, and any other models pass through untouched.
set -euo pipefail

input="$(cat)"
tool_name="$(jq -r '.tool_name' <<< "$input")"

# Tools Opus may call directly: spawning subagents, planning, task tracking,
# and user interaction. Everything else (Read, Edit, Write, Bash, Grep, Glob,
# WebFetch, MCP tools, etc.) must be delegated to a Sonnet subagent.
case "$tool_name" in
  Agent|AskUserQuestion|EnterPlanMode|ExitPlanMode|TaskCreate|TaskUpdate|TaskGet|TaskList|TaskOutput|TaskStop|ScheduleWakeup|mcp__ccd_session__mark_chapter)
    exit 0
    ;;
esac

transcript="$(jq -r '.transcript_path' <<< "$input")"

model=""
if [[ -f "$transcript" ]]; then
  model="$(tail -n 300 "$transcript" \
    | jq -rs '[.[] | select(.type == "assistant") | .message.model // empty] | last // ""' \
    2>/dev/null || true)"
fi

case "$model" in
  *opus*)
    reason="You are running as Opus on this project, and direct tool use is disabled for you. Your job here is to think and orchestrate, not to execute: read the request, plan the approach, and delegate ALL execution (file reads, edits, searches, commands, web lookups, everything) to Sonnet 4.6 subagents via the Agent tool (model: sonnet).

Call the Agent tool now with a SELF-CONTAINED prompt that includes:

1. GOAL - the exact outcome you need, stated as a result, not a restatement of the user's request.
2. SCOPE - exact paths, files, globs, commands, or APIs to touch. Be specific; do not make the subagent guess where to look or what to change.
3. CONTEXT - naming conventions, relevant prior findings, constraints, edge cases, and anything else needed to act correctly without asking follow-up questions. The subagent has no memory of this conversation.
4. OUTPUT FORMAT - the precise shape of what should come back (e.g. a list of file:line matches with context, a diff summary, pass/fail plus evidence).
5. BOUNDARIES - what the subagent should and should not change, and when to stop and report rather than improvising further.

Write the prompt as if briefing a competent but unfamiliar colleague who cannot ask follow-up questions: a vague brief produces a vague result. Break the work into coherent subagent calls, one per independent piece of work, and fire independent ones in parallel within a single message. After each subagent reports back, synthesize the results, decide the next step, and continue orchestrating until the task is complete.

You may use AskUserQuestion, EnterPlanMode/ExitPlanMode, TaskCreate/TaskUpdate/TaskGet/TaskList, and ScheduleWakeup directly for planning and coordination. Every other tool must be delegated to a Sonnet subagent."

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