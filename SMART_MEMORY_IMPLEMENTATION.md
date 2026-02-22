# Smart Memory Implementation Guide

## Purpose

`smart-memory` is an optional Python package + CLI that adds active, project-scoped memory to Smart Agent.

It should:
- Persist structured memory in JSON.
- Answer interactive questions about a project.
- Search the project when memory is missing.
- Use Copilot SDK to synthesize high-quality answers.
- Keep memory isolated per project folder.

---

## Goals

1. Provide a reusable memory service for day-to-day development support.
2. Allow fast Q&A for architecture, testing, build, and conventions.
3. Generate daily/monthly summaries of changes and learning.
4. Keep installation optional from the main smart-agent installer.
5. Use Copilot SDK as the core reasoning/synthesis engine.

---

## Non-Goals

- Replacing `.copilot/docs/` documentation generation.
- Acting as a full replacement for code search tools.
- Modifying project source by default.

---

## Package Layout

Suggested package layout:

```text
memory/
  pyproject.toml
  README.md
  src/
    smart_memory/
      __init__.py
      cli/
        __init__.py    # click/typer group setup + global flags
        ask.py         # ask command
        inspect.py     # inspect subcommandsmk
        serve.py       # serve command
        cache.py       # cache subcommands
        record.py      # record-change command
        config_cmd.py  # config view/set/get commands
        memory_cmd.py  # remember, forget, edit, search, list
      config.py
      server.py
      copilot_client.py
      memory_store.py
      project_index.py
      query_engine.py
      summarizer.py
      detectors/
        __init__.py
        base.py        # detector protocol + registry
        build.py       # build/compile detection
        test.py        # test framework detection
        run.py         # dev/start/run detection
        ci.py          # CI pipeline detection
        api.py         # API style detection
        conventions.py # lint/format conventions
      plugins/         # user-provided custom detectors
      schemas.py
  tests/
    test_config.py
    test_memory_store.py
    test_project_index.py
    test_detectors.py
    test_cli.py
```

Splitting `cli.py` into a package prevents a single file from growing unmanageable as commands multiply. Each command module registers itself with the CLI group in `cli/__init__.py`.

The `detectors/` package replaces the monolithic `detectors.py`, allowing each detection domain (build, test, CI, etc.) to be developed and tested independently. Users can drop custom detectors into `plugins/` for project-specific detection logic.

---

## Runtime Model

### 1. Project Scope

`smart-memory` always runs with a project root scope:
- `--project /path/to/project`
- default: current working directory (auto-detects git root by walking up from cwd)

All indexing, searching, and persisted knowledge are scoped to that root. The `--project` flag is only needed when targeting a directory other than cwd.

### 2. Active Memory Flow

When asked a question:
1. Normalize question and project scope.
2. Try to answer from stored memory.
3. If low confidence or no answer:
   - search codebase and project metadata
   - gather evidence snippets
   - ask Copilot SDK with evidence
4. Return answer with sources/confidence.
5. Persist new answer and evidence summary.

### 3. Change Summary Ingestion (AI-Routed)

When a user provides a summary of changes (example: "I changed ZXF code, summary: ..."):
1. Capture summary payload and optional metadata (files, feature, date).
2. Ask Copilot SDK to classify where this information belongs.
3. Persist into one or more memory buckets (knowledge, session log, monthly summary, decisions).
4. Store routing rationale and confidence for traceability.

Routing targets:
- `knowledge.json` for stable project facts and behavior explanations.
- `sessions/YYYY-MM-DD.json` for day-specific work context.
- `summaries/YYYY-MM.json` for period summaries and highlights.
- `decisions.json` (optional) for architectural choices and rationale.

If routing confidence is low, write to `sessions/YYYY-MM-DD.json` first and mark as `needs_review: true`.

---

## Environment Variables

Required or recommended env vars:

- `SMART_AGENT_MEMORY_DIR`
  - Base storage location for memory JSON.
  - Example: `~/.smart-agent/memory`

- `SMART_MEMORY_MODEL`
  - Copilot model name for sessions.
  - Example: `gpt-4`

- `SMART_MEMORY_MAX_CONTEXT_FILES`
  - Max number of files to include as evidence per query.
  - Example: `20`

- `SMART_MEMORY_LOG_LEVEL`
  - `debug|info|warning|error`

- `SMART_MEMORY_CACHE_DIR`
  - Optional cache path; defaults under project memory folder.

- `SMART_MEMORY_CACHE_TTL_SECONDS`
  - Default TTL for answer cache entries.

- `SMART_MEMORY_CACHE_MAX_ITEMS`
  - Maximum number of cached query/answer items per project.

- `NO_COLOR`
  - When set, disables colored terminal output (standard convention).

Defaults:
- `SMART_AGENT_MEMORY_DIR=~/.smart-agent/memory`
- `SMART_MEMORY_MODEL=gpt-4`
- `SMART_MEMORY_MAX_CONTEXT_FILES=20`
- `SMART_MEMORY_LOG_LEVEL=info`
- `SMART_MEMORY_CACHE_TTL_SECONDS=3600`
- `SMART_MEMORY_CACHE_MAX_ITEMS=1000`

### Project Config File (`.smart-memory.toml`)

As an alternative to environment variables, projects can define a `.smart-memory.toml` at the project root. Environment variables always override file values.

```toml
[memory]
model = "gpt-4o"
max_context_files = 30
cache_ttl_seconds = 7200
log_level = "debug"

[redaction]
# Additional regex patterns for secret redaction (beyond built-in defaults)
custom_patterns = [
  "INTERNAL_KEY_[A-Za-z0-9]+",
  "my-org-secret-[a-f0-9]{32}",
]
```

Manage via CLI:

```bash
smart-memory config list                 # show effective config
smart-memory config set model gpt-4o     # persist to .smart-memory.toml
smart-memory config get cache-ttl        # print effective value
```

---

## Storage Design

Project-specific storage under memory dir:

```text
$SMART_AGENT_MEMORY_DIR/
  projects/
    <project_id>/
      project.json
      knowledge.json
      decisions.json
      sessions/
        2026-02-22.json
      summaries/
        2026-02.json
      index/
        files.json
        fingerprints.json
      cache/
        answers.json
        queries.json
        evidence.json
        stats.json
```

### `project.json`

```json
{
  "schema_version": 1,
  "project_id": "3d4f...",
  "project_root": "/abs/path",
  "created_at": "2026-02-22T12:00:00Z",
  "last_updated": "2026-02-22T12:05:00Z"
}
```

### `knowledge.json`

```json
{
  "schema_version": 1,
  "entries": [
    {
      "id": "k-001",
      "question": "What does XYZ feature do?",
      "answer": "...",
      "confidence": 0.82,
      "sources": ["src/xyz/service.ts:42"],
      "updated_at": "2026-02-22T12:07:00Z"
    }
  ]
}
```

### `sessions/YYYY-MM-DD.json`

Stores interactive queries/responses and diagnostics.

Also stores ingested user summaries with AI routing metadata.

### `summaries/YYYY-MM.json`

Stores month-level summaries and notable changes.

### `cache/*`

Stores cache layers for faster retrieval:
- `queries.json`: normalized query fingerprints.
- `answers.json`: cached final answers + sources + TTL.
- `evidence.json`: reusable snippet bundles by topic.
- `stats.json`: hit rates and invalidation counters.

---

## Copilot SDK Integration

`smart-memory` must use `github-copilot-sdk`.

High-level integration:
1. Start `CopilotClient({"auto_start": true})`.
2. Create session with selected model.
3. Send prompt containing:
   - user question
   - project scope
   - evidence snippets
   - expected output format
4. Handle streaming events and final answer.
5. Destroy session and stop client cleanly.

For change-summary ingestion, send a structured prompt requiring JSON output:
- `category`
- `target_files`
- `importance`
- `routing_targets`
- `reasoning`
- `confidence`

Error handling:
- Timeout handling for stalled sessions (see `E1005`).
- Retry with smaller context when prompt too large.
- Fallback to evidence-only response when SDK unavailable (exit code `3`, see `E1004`).
- Surface authentication failures immediately with recovery steps (see `E1001`, `E1002`).
- Handle rate limits with backoff and user notification (see `E1006`).

---

## CLI Commands

### Global Flags

All commands accept these global flags:

- `--project <path>` — Project root path. **Defaults to cwd.** Auto-detects git root by walking up from cwd when not specified. Only required when targeting a different directory.
- `--format <human|json|table>` — Output format. Default: `human` (colored terminal output). Use `json` for scripting and pipelines, `table` for columnar display.
- `--quiet` / `-q` — Suppress metadata; print only the core answer or result.
- `--verbose` / `-v` — Show reasoning chain: cache layer hit, files searched, SDK latency, routing decisions.
- `--no-persist` — Run the command without writing to memory stores. Useful for dry-runs and scripting.
- `--no-color` — Disable colored terminal output (also honored via `NO_COLOR` env var).

### Command Aliases

Frequently used commands have short aliases for power users:

| Full Command     | Alias |
|------------------|-------|
| `ask`            | `a`   |
| `record-change`  | `r`   |
| `inspect`        | `i`   |
| `summarize`      | `s`   |
| `remember`       | `rem` |
| `search`         | `q`   |
| `status`         | `st`  |

### Exit Codes

All commands use consistent exit codes for scripting:

| Code | Meaning                                  |
|------|------------------------------------------|
| `0`  | Success, answer or result returned       |
| `1`  | General error                            |
| `2`  | No answer found / below confidence threshold |
| `3`  | SDK unavailable, fallback used           |

### Expected Errors

The CLI must handle these error scenarios gracefully with clear messages and actionable recovery hints.

#### Authentication & SDK Errors

| Error Code | Message | Cause | Recovery |
|------------|---------|-------|----------|
| `E1001` | `Copilot SDK not authenticated. Run 'gh auth login' or check your GitHub credentials.` | No valid Copilot/GitHub authentication token found. | Run `gh auth login --scopes copilot` and ensure the account has Copilot access. |
| `E1002` | `Copilot SDK session expired. Re-authenticate to continue.` | Token expired mid-session. | Re-run `gh auth login` or refresh the token. |
| `E1003` | `Copilot SDK not installed or not found in PATH.` | `github-copilot-sdk` package missing or not importable. | Run `pip install github-copilot-sdk` or check venv activation. |
| `E1004` | `Copilot SDK connection refused. Is the Copilot service running?` | SDK client cannot reach the Copilot backend. | Check network connectivity and firewall rules. Retry after a moment. |
| `E1005` | `Copilot SDK request timed out after {timeout}s.` | SDK call exceeded the configured timeout. | Retry with `--verbose` to inspect latency. Reduce context with fewer evidence files. |
| `E1006` | `Copilot SDK rate limit exceeded. Try again in {wait}s.` | Too many SDK requests in a short window. | Wait for the indicated cooldown or use cached answers. |
| `E1007` | `Copilot SDK returned an invalid response. Falling back to evidence-only answer.` | SDK response could not be parsed (malformed JSON, unexpected schema). | Check `--verbose` output. If persistent, report as a bug. |
| `E1008` | `Model '{model}' is not available. Available models: {list}.` | `SMART_MEMORY_MODEL` or config specifies a model not supported by the SDK. | Update to a supported model via `smart-memory config set model <name>`. |

#### Project & Storage Errors

| Error Code | Message | Cause | Recovery |
|------------|---------|-------|----------|
| `E2001` | `No project detected. Run 'smart-memory init' or specify --project.` | No `project.json` found and auto-detection failed (no `.git` root). | Run `smart-memory init` in the project directory. |
| `E2002` | `Memory directory not writable: {path}` | Filesystem permission issue on the memory storage path. | Check permissions on `$SMART_AGENT_MEMORY_DIR` or set a writable path. |
| `E2003` | `Corrupted memory file: {file}. Invalid JSON.` | A stored JSON file has been manually edited or truncated and cannot be parsed. | Restore from backup with `smart-memory import`, or delete the corrupted file and re-run `smart-memory init`. |
| `E2004` | `Schema version mismatch in {file}. Expected v{expected}, found v{found}.` | Memory files were created by a newer/older version of `smart-memory`. | Run `smart-memory migrate` (future) or re-initialize the project. |
| `E2005` | `Disk space insufficient for memory storage.` | Storage volume is full or near quota. | Free disk space or relocate `SMART_AGENT_MEMORY_DIR` to a volume with space. |
| `E2006` | `Memory entry not found: {id}` | `forget`, `edit`, or reference to a non-existent entry ID. | Run `smart-memory list` to see available entry IDs. |

#### Configuration Errors

| Error Code | Message | Cause | Recovery |
|------------|---------|-------|----------|
| `E3001` | `Invalid config file: {path}. TOML parse error at line {line}.` | `.smart-memory.toml` has a syntax error. | Fix the TOML syntax at the indicated line. |
| `E3002` | `Unknown config key: '{key}'. Run 'smart-memory config list' for valid keys.` | `config set` or `config get` used with an unrecognized key. | Check available keys with `smart-memory config list`. |
| `E3003` | `Invalid value for '{key}': expected {type}, got '{value}'.` | Config value does not match expected type (e.g., non-numeric TTL). | Provide a value of the correct type. |

#### CLI Usage Errors

| Error Code | Message | Cause | Recovery |
|------------|---------|-------|----------|
| `E4001` | `No question provided. Pass a question as argument or use --pipe for stdin.` | `ask` called without a question argument and not piped. | Provide a question: `smart-memory ask "..."` or pipe via `--pipe`. |
| `E4002` | `File not found: {path}` | `--summary-file`, `--batch`, `--redact-patterns`, or `import` references a missing file. | Check the file path exists and is readable. |
| `E4003` | `Import file schema not recognized. Expected smart-memory export format.` | `import` given a file that doesn't match the expected export schema. | Use a file produced by `smart-memory export`. |
| `E4004` | `Port {port} already in use.` | `serve --mode http` cannot bind to the requested port. | Choose a different port with `--port` or stop the process using that port. |
| `E4005` | `Cannot read from stdin. No input detected for --pipe/--summary-stdin.` | Stdin flag used but no data piped in. | Pipe data: `echo "..." \| smart-memory ask --pipe`. |

#### Error Output Format

All errors follow a consistent structure in both human and JSON output:

**Human output (stderr):**
```
Error [E1001]: Copilot SDK not authenticated.
  Run 'gh auth login' or check your GitHub credentials.
  Hint: Ensure your GitHub account has Copilot access enabled.
```

**JSON output (`--format json`):**
```json
{
  "error": {
    "code": "E1001",
    "message": "Copilot SDK not authenticated.",
    "recovery": "Run 'gh auth login' or check your GitHub credentials.",
    "hint": "Ensure your GitHub account has Copilot access enabled."
  }
}
```

When the SDK is unavailable and a fallback is possible, the CLI should:
1. Print a warning (not an error) to stderr: `Warning [E1004]: SDK unavailable. Returning evidence-only answer.`
2. Return exit code `3` (not `1`).
3. Include `"fallback": true` in JSON output.

### Primary Commands

- `smart-memory init`
  - Explicit project initialization. Creates `project.json`, sets up directory structure, and optionally runs `inspect project` for first-time detection.

- `smart-memory status`
  - One-shot health check showing: project detected, memory dir, entry count, cache size, index freshness, SDK connectivity, and storage usage.

- `smart-memory serve`
  - Starts interactive server loop for current project.

- `smart-memory ask "What does XYZ feature do?"`
  - One-shot Q&A.
  - `--threshold <0.0-1.0>` — Only return answers above this confidence; trigger fresh SDK call otherwise.
  - `--pipe` — Read question from stdin instead of argument.
  - `--batch <file>` — Process multiple questions from a file, one per line.

- `smart-memory summarize --period today`
  - Summarizes work for current day.

- `smart-memory summarize --period month --month 2026-02`
  - Summarizes monthly activity.

- `smart-memory remember --title "Auth Decision" --content "Use JWT for API"`
  - Adds a manual memory entry.

- `smart-memory record-change --feature "ZXF" --summary "..."`
  - Ingests a user-provided change summary and uses Copilot SDK to decide placement.
  - `--dry-run` — Show where AI would route the summary without persisting.
  - `--summary-stdin` — Read summary content from stdin for pipeline usage.

- `smart-memory record-change --feature "ZXF" --summary-file ./summary.md`
  - Same as above, reading content from file.

- `smart-memory forget <id>`
  - Delete or archive a specific knowledge entry by ID. Supports `--archive` to move to an archive file instead of permanent deletion.

- `smart-memory edit <id>`
  - Update an existing memory entry interactively. Opens entry content in `$EDITOR` or accepts `--content` / `--answer` flags for non-interactive use.

- `smart-memory search <query>`
  - Search across all memory entries (knowledge, sessions, summaries, decisions) by keyword or regex pattern. Supports `--scope knowledge|sessions|summaries|decisions|all`.

- `smart-memory list`
  - Lists memory entries. Supports `--scope` and `--since <date>` filters.

- `smart-memory export --format md|json`
  - Export project memory as a single portable file for sharing or backup.

- `smart-memory import <file>`
  - Import memory from an exported file. Supports `--merge` (combine with existing) and `--replace` (overwrite).

- `smart-memory path`
  - Prints effective memory paths.

- `smart-memory gc`
  - Garbage collect expired cache entries, orphaned sessions, and low-confidence entries older than `--max-age` days (default: 90).

- `smart-memory audit`
  - Scan stored memory for accidentally persisted secrets (API keys, tokens, passwords) using built-in and custom redaction patterns. See Security section.

- `smart-memory debug <query>`
  - Like `ask` but shows the full reasoning chain: cache lookup result, files searched, evidence gathered, prompt sent to SDK, SDK response, and routing decision.

### Cache Commands

- `smart-memory cache stats`
  - Shows cache hit/miss rates and current cache sizes.

- `smart-memory cache clear [--scope answers|queries|evidence|all]`
  - Clears selected cache scope.

### Config Commands

- `smart-memory config list`
  - Show effective configuration (env vars + project config file merged).

- `smart-memory config set <key> <value>`
  - Persist a setting to project-level `.smart-memory.toml`.

- `smart-memory config get <key>`
  - Print the effective value of a specific config key.

### Stdin / Pipeline Support

Commands that accept content support stdin for composability:

```bash
# Pipe a question
echo "What changed in auth?" | smart-memory ask --pipe

# Pipe git diff as change summary
git diff --stat | smart-memory record-change --feature auto --summary-stdin

# Batch questions
smart-memory ask --batch questions.txt --format json > answers.json
```

---

## Discovery Commands (Project Intelligence)

These commands improve project understanding by extracting build/test/run details.

All `inspect` results are automatically persisted to `knowledge.json` so that `ask` can reuse them without re-running detection. Entries are tagged with `source: "inspect"` and updated on subsequent runs.

- `smart-memory inspect all`
  - Run all inspect sub-commands at once and produce a unified project profile. Outputs a combined report instead of requiring 10 separate calls.

- `smart-memory inspect project`
  - Detect language(s), frameworks, package managers, repo type.

- `smart-memory inspect build`
  - Find compile/build commands from project files.

- `smart-memory inspect test`
  - Find how tests are executed and where tests live.

- `smart-memory inspect test-types`
  - Detect unit/integration/e2e test presence.

- `smart-memory inspect conventions`
  - Infer conventions from lint/format configs and docs.

- `smart-memory inspect api`
  - Detect API style and likely endpoint locations.

- `smart-memory inspect architecture`
  - Infer module boundaries and data flow hints.

- `smart-memory inspect dependencies`
  - Show top dependencies and dev tooling.

- `smart-memory inspect scripts`
  - Enumerate available scripts/targets from package files.

- `smart-memory inspect ci`
  - Parse CI files and test/build pipelines.

- `smart-memory inspect health`
  - Check for common project issues: missing README, no CI config, no tests found, outdated lock files, missing license. Outputs actionable recommendations.

### Plugin Detectors

Users can add custom detectors without modifying core code by placing Python modules in the project's `.smart-memory/detectors/` directory:

```python
# .smart-memory/detectors/my_detector.py
from smart_memory.detectors.base import Detector

class MyDetector(Detector):
    name = "custom-check"

    def detect(self, project_root: Path) -> dict:
        # Return detection results
        return {"found": True, "details": "..."}
```

Custom detectors are automatically discovered and available via `smart-memory inspect custom-check`.

---

## Example Q&A Prompts

Interactive usage examples:

- `smart-memory ask "How do I compile this project?"`
- `smart-memory ask "How are tests constructed in this repo?"`
- `smart-memory ask "Do we have integration tests?"`
- `smart-memory ask "Where is auth implemented?"`
- `smart-memory ask "How do I run only e2e tests?"`
- `smart-memory ask "What changed today in payment flow?"`
- `smart-memory ask "Summarize last month changes related to CI"`
- `smart-memory record-change --feature "ZXF" --summary "Refactored parser and updated tests"`

Expected answer shape:
- short answer
- confidence score (color-coded: green > 0.8, yellow 0.5–0.8, red < 0.5)
- source references
- staleness indicator (when answered from cache: age + whether dependent files changed)
- optional next commands

---

## Detection Rules (Build/Test/Run)

`inspect` should use deterministic detectors before asking SDK.

### Build/Compile detection

Check in order:
- Node: `package.json` scripts (`build`, `compile`, `dev`)
- Python: `pyproject.toml`, `tox.ini`, `noxfile.py`, `Makefile`
- Rust: `Cargo.toml` (`cargo build`)
- Go: `go.mod` (`go build`, `go test`)
- Java: `pom.xml`, `build.gradle`
- C/C++: `CMakeLists.txt`, `Makefile`

### Test detection

Check:
- Script targets (`test`, `test:unit`, `test:e2e`, etc.)
- Framework config files:
  - JS: `jest.config.*`, `vitest.config.*`, `playwright.config.*`, `cypress.*`
  - Python: `pytest.ini`, `pyproject.toml` pytest sections
  - Rust: `Cargo.toml` + `tests/`
  - Go: `*_test.go`
- Folder patterns:
  - `tests/unit`, `tests/integration`, `tests/e2e`, `spec`, `__tests__`

### Run/start detection

Check:
- `dev/start/run` scripts
- container files (`Dockerfile`, `compose`) for service startup hints
- README quickstart snippets

---

## JSON Output Schemas

### Summary output

```json
{
  "schema_version": 1,
  "type": "summary",
  "period": "today",
  "project_id": "3d4f...",
  "generated_at": "2026-02-22T12:15:00Z",
  "model": "gpt-4",
  "summary": {
    "highlights": ["..."],
    "files_touched": ["src/a.ts"],
    "notable_decisions": ["..."],
    "risks": ["..."]
  },
  "sources": ["git", "workspace scan"]
}
```

### Answer output

```json
{
  "schema_version": 1,
  "type": "answer",
  "project_id": "3d4f...",
  "question": "How do I run tests?",
  "answer": "Run `npm test` ...",
  "confidence": 0.9,
  "sources": [
    "package.json:scripts.test",
    "README.md:42"
  ],
  "generated_at": "2026-02-22T12:16:00Z"
}
```

### Change record output

```json
{
  "schema_version": 1,
  "type": "change_record",
  "project_id": "3d4f...",
  "feature": "ZXF",
  "summary": "Refactored parser and updated tests",
  "routing": {
    "targets": ["knowledge", "sessions", "summaries"],
    "reasoning": "Behavior-level change plus day/month relevance",
    "confidence": 0.84
  },
  "stored_at": [
    "knowledge.json:k-021",
    "sessions/2026-02-22.json:s-145",
    "summaries/2026-02.json:m-033"
  ],
  "generated_at": "2026-02-22T13:02:00Z"
}
```

### Cache answer entry

```json
{
  "schema_version": 1,
  "cache_key": "ask:how-do-i-run-tests:project-3d4f",
  "answer": "Run `npm test` ...",
  "sources": ["package.json:scripts.test"],
  "created_at": "2026-02-22T13:05:00Z",
  "expires_at": "2026-02-22T14:05:00Z",
  "fingerprints": {
    "project_index": "fpr-abc123",
    "files": ["pkg-ff12", "readme-d901"]
  }
}
```

---

## Server Behavior

`smart-memory serve` should:
1. Print active project and memory path.
2. Build/refresh lightweight file index.
3. Accept interactive commands:
   - `ask <question>`
   - `record-change <feature>::<summary>`
   - `inspect <topic>`
   - `summarize <period>`
   - `remember <title>::<content>`
   - `forget <id>`
   - `search <query>`
   - `cache stats`
   - `cache clear <scope>`
   - `config list`
   - `status`
   - `help`
   - `exit`
4. Persist every interaction to session file.

### Server Modes

- `smart-memory serve --mode stdio` (default)
  - Interactive readline-based terminal session with tab completion for commands, arrow-key history, and `Ctrl+R` reverse search.

- `smart-memory serve --mode http --port 8377`
  - HTTP/JSON-RPC server mode. Exposes all commands as API endpoints for editor plugins, scripts, and CI integration.
  - Endpoints mirror CLI commands: `POST /ask`, `POST /record-change`, `GET /status`, etc.
  - Returns JSON responses matching the output schemas.

### File Watching

`smart-memory serve --watch` enables filesystem notifications (using `watchdog`) to auto-reindex when source files change, rather than requiring manual index refresh. Reindex is debounced (default: 2s) to avoid thrashing during rapid saves.

Server should use cache-before-search policy:
1. Query cache lookup.
2. If miss/stale, run retrieval + SDK.
3. Save answer cache entry.
4. Update cache stats.

---

## Installation and Optional Setup

### User install flow (optional)

1. Install main Smart Agent as usual.
2. Optionally run memory installer:

```bash
./scripts/install-memory.sh
```

Installer should:
- create venv for memory package
- install package + dependencies
- print env var instructions (do not modify shell profile)

Example printed instructions:

```bash
export SMART_AGENT_MEMORY_DIR="$HOME/.smart-agent/memory"
export SMART_MEMORY_MODEL="gpt-4"
export PATH="$HOME/.smart-agent/memory/.venv/bin:$PATH"
```

---

## Testing Strategy

Test categories for `smart-memory` package:

1. Unit tests
- config/env parsing
- path resolution
- schema validation
- detector logic (build/test/ci)

2. Integration tests
- CLI command execution
- JSON write/read behavior
- project scoping correctness
- cache invalidation on file fingerprint changes
- change-summary routing target writes

3. SDK interaction tests
- mock Copilot SDK client/session
- event handling and timeout behavior
- routing response parser for `record-change`

4. Snapshot tests
- stable JSON output structures

5. Optional e2e tests
- run against a fixture project repository
- validate `inspect` and `ask` responses include expected sources
- validate repeated `ask` calls return cache hits
- validate `record-change` stores in AI-selected targets

Suggested command set:

```bash
python -m pytest
python -m pytest tests/test_detectors.py
python -m pytest -k "cli or memory_store"
```

---

## Security and Safety

- Never include secrets in stored memory.
- Redact known secret patterns before persistence.
- Keep path scope strict to selected project root.
- Limit max file size and binary ingestion.
- Store only necessary snippets, not entire large files.

### Audit Command

`smart-memory audit` scans all stored memory files for accidentally persisted secrets using:
- Built-in patterns: AWS keys, GitHub tokens, JWTs, private keys, generic API key formats.
- Custom patterns: defined in `.smart-memory.toml` under `[redaction].custom_patterns`.
- `--redact-patterns <file>` flag: load additional regex patterns from a file (one per line).

Audit outputs a report listing matches with file, entry ID, and matched pattern name. Use `--fix` to auto-redact matches in-place.

---

## Performance Guidelines

- Maintain a lightweight file index per project.
- Use mtime/fingerprint to avoid full rescans.
- Cap context files per query.
- Prefer deterministic detector outputs before SDK calls.
- Use layered caches: query cache, evidence cache, and answer cache.
- Invalidate answer cache when dependent file fingerprints change.
- Keep LRU eviction for `SMART_MEMORY_CACHE_MAX_ITEMS`.
- Track hit/miss ratio and tune TTL by command type.

### Cache Strategy

Layer 1: Query normalization cache
- Maps semantically similar questions to canonical keys.

Layer 2: Evidence cache
- Reuses extracted snippets for frequent topics (build/test/auth).

Layer 3: Answer cache
- Stores final answer + sources + expiry + fingerprint dependencies.

Invalidation rules:
- Project reindex fingerprint changed.
- Any dependent file fingerprint changed.
- TTL expired.
- Manual clear command.

---

## Rollout Plan

1. Implement package scaffold, CLI group structure, and global flags.
2. Implement memory store + schemas + `init` / `status` commands.
3. Implement project detectors (`inspect` commands) with plugin support.
4. Implement Copilot SDK wrapper and `ask` flow (with `--threshold`, `--pipe`, `--batch`).
5. Implement `serve` loop with stdio and HTTP modes + file watching.
6. Implement lifecycle commands (`forget`, `edit`, `search`, `export`, `import`, `gc`).
7. Implement `config` command and `.smart-memory.toml` support.
8. Implement `audit` command and redaction pipeline.
9. Add installer script and README docs.
10. Add tests and fixtures.

---

## Future Enhancements

- `smart-memory compare --from --to` for period diff summaries.
- `smart-memory doctor` to validate project detection quality.
- Local embedding cache for faster retrieval before SDK call.
- Team-shared memory backend option.
- `smart-memory watch` standalone mode for continuous indexing outside `serve`.
- Git hook integration for auto-recording changes on commit.
- MCP (Model Context Protocol) server mode for direct editor integration.
- Multi-project memory queries (cross-project knowledge search).

---

## Quick Command Cheat Sheet

```bash
# Initialize project memory
smart-memory init

# Check project status
smart-memory status
smart-memory st                          # alias

# Start interactive memory server for current project
smart-memory serve
smart-memory serve --mode http --port 8377   # HTTP mode
smart-memory serve --watch                   # with file watching

# Ask a one-shot question
smart-memory ask "How do I compile this project?"
smart-memory a "How do I compile this project?"   # alias
smart-memory ask "..." --threshold 0.7            # confidence gate
smart-memory ask "..." --format json              # machine output
echo "What changed?" | smart-memory ask --pipe    # stdin
smart-memory ask --batch questions.txt            # batch mode

# Debug a query (full reasoning chain)
smart-memory debug "How do I compile this project?"

# Record a user-provided change summary and let AI route it
smart-memory record-change --feature "ZXF" --summary "Refactored parser and updated tests"
smart-memory r --feature "ZXF" --summary "..." --dry-run   # preview routing
git diff --stat | smart-memory r --feature auto --summary-stdin

# Discover project setup (all at once or individually)
smart-memory inspect all
smart-memory inspect test
smart-memory inspect test-types
smart-memory inspect build
smart-memory inspect scripts
smart-memory inspect health

# Daily and monthly summaries
smart-memory summarize --period today
smart-memory summarize --period month --month 2026-02

# Memory management
smart-memory remember --title "Build note" --content "Use pnpm build"
smart-memory search "auth"
smart-memory list --scope knowledge
smart-memory edit k-001
smart-memory forget k-005
smart-memory forget k-005 --archive

# Export and import
smart-memory export --format json > backup.json
smart-memory import backup.json --merge

# Show memory paths
smart-memory path

# Cache operations
smart-memory cache stats
smart-memory cache clear --scope answers

# Garbage collection
smart-memory gc
smart-memory gc --max-age 60

# Configuration
smart-memory config list
smart-memory config set model gpt-4o
smart-memory config get cache-ttl

# Security audit
smart-memory audit
smart-memory audit --fix
smart-memory audit --redact-patterns custom-patterns.txt
```
