---
name: planning
description: Create implementation plans, architectural options, and phased execution strategy. All plans are written to disk as markdown files.
version: "2.4"
---

# Planning Skill

## Identity

- **Name**: planning
- **Version**: 2.4
- **Description**: Creates implementation plans, architectural decisions, and strategic approaches for complex tasks. All plans — regardless of size — are persisted to disk as `.md` files in `.github/copilot/plans/`.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: plan, design, architect, approach | "Create a plan for user auth" |
| "how should I..." | "How should I structure the API?" |
| "design...for" | "Design a solution for caching" |
| Large scope tasks | "Add a complete payment system" |

---

## Capabilities

What this skill can do:

- ✅ Analyze request complexity (small/medium/big changes)
- ✅ Ask clarifying questions before planning (with domain-specific templates)
- ✅ Create detailed implementation plans with phases
- ✅ Identify files and components affected
- ✅ Assess risks and propose mitigations
- ✅ Provide rollback strategies
- ✅ Make architectural decisions (ADRs)
- ✅ Capture Non-Functional Requirements (performance, security, accessibility, compliance)
- ✅ Evaluate and recommend tech stacks when user is "open to suggestions"
- ✅ Identify MVP/primitive-whole slice vs full feature scope
- ✅ Produce Definition of Done per phase
- ✅ Plan third-party integrations and external dependencies explicitly
- ✅ Address domain-specific concerns (landing pages, full-stack apps, CLI tools, APIs)

---

## Dependencies

- `context.md` - For session context and project identity
- `.github/copilot/docs/` - For existing architecture understanding
- `.github/copilot/standards/` - For coding standards to plan against
- `coding.skill` - Chains to for implementation

---

## Workflow

### Step 1: Load Project Context

```
Read from context.md:
- Project type and stack
- Existing decisions
- User preferences
```

### Step 2: Analyze Change Size

**SMALL** (<100 lines)
- Brief PLAN-XXX.md with summary, scope, and tasks
- Still written to disk (all plans go to disk)

**MEDIUM** (100-500 lines)
- Standard PLAN-XXX.md with phases and risks
- 2-3 phases max

**BIG** (>500 lines)
- Full PLAN-XXX.md document
- Multiple phases with milestones
- Risk assessment + rollback strategy

ALL sizes → Written to `.github/copilot/plans/PLAN-XXX.md`

### Step 3: Ask Clarifying Questions (MANDATORY)

Before creating ANY plan, ensure you understand:

```markdown
🤔 **Before I create a plan, I have some questions:**

1. **[Scope]**: [Clarify boundaries]
2. **[Behavior]**: [Clarify expected outcomes]
3. **[Constraints]**: [Identify limitations]
4. **[Integration]**: [Understand dependencies]

Please answer these so I can create an accurate plan.
```

**Priority Distinction — Ask All Must-Have Questions First:**

| Priority | Description | Action |
|----------|-------------|--------|
| **Must-Have** | Without this info, the plan would be fundamentally incorrect | Block plan creation; ask the user |
| **Nice-to-Have** | Affects quality but plan can proceed with a documented assumption | Note assumption in plan; can clarify later |

> If >3 critical (Must-Have) gaps remain unanswered, recommend a discovery session before committing to a plan.

**Core Question Categories:**

| Category | Example Questions |
|----------|-------------------|
| **Scope** | "Should this include X? What about Y?" |
| **Behavior** | "What should happen when Z occurs?" |
| **Constraints** | "Are there performance/security requirements?" |
| **Integration** | "How should this interact with existing feature A?" |
| **Edge Cases** | "What if the user does X? What about empty input?" |
| **Priority** | "Which aspects are must-have vs nice-to-have?" |
| **Queue/List systems** | "What when queue is full? Fairness algorithm: FIFO, party-size weighted, or staff override?" |

---

**Extended Clarification Areas (Add Relevant Sections Per Request):**

#### 🎯 Target Audience & Personas
- Who are the primary users? (role, technical comfort, device)
- Mobile vs desktop usage split?
- Key user pain points / jobs-to-be-done?
- Any accessibility needs for specific user groups?

#### 🎨 Design & UX Context
*(For any UI-facing work)*
- Do you have an existing brand: colors, logo, font?
- Design inspiration or reference sites?
- Does a design file (Figma, Adobe XD) already exist?
- Design system or component library to use (Tailwind, Material, custom)?
- Is a designer involved, or should the plan include design decisions?

#### 📝 Content Strategy
*(For content-rich pages, marketing sites, documentation)*
- Is copy/content already written and approved?
- Who provides: testimonials, images, service descriptions, pricing?
- What languages / locales? (e.g., Spanish Latin America — which country, currency, phone format?)
- Should the plan include a content-gathering pre-phase?

#### ⚡ Non-Functional Requirements (NFR)
*(MANDATORY for any production system)*
- **Performance**: Target load time / response latency? (e.g., "< 3s on mobile 4G")
- **Scale / Concurrency**: Expected concurrent users at peak?
- **Availability / Uptime**: SLA target? (e.g., 99.5% during business hours)
- **Browser / Platform**: Which OS/browser versions must be supported?
- **Accessibility**: WCAG level required (A, AA, AAA)?
- **Offline / Resilience**: Must the app function without internet?
- **For real-time systems**: What is the network reliability at the deployment site? Should the app fall back to polling if WebSocket fails? Reconnection strategy on disconnect (exponential backoff)? What happens to queued updates during a network partition?

#### 🌐 Internationalization & Localization (i18n)
*(For any content served to multiple languages or locales)*
- Which specific locales are required? (language code + country, e.g., `es-MX`, `es-AR`)
- Multi-language from launch, or single language initially?
- String management: hardcoded, JSON resource files, CMS, or external i18n service?
- URL strategy: path prefix (`/es/`), query param (`?lang=es`), subdomain, or content negotiation?
- Locale auto-detection: user preference, geolocation, Accept-Language header, or cookie?
- Should automated tests run against all locales?
- Country-specific requirements: phone formats, currency symbols, date/number formats, legal disclaimers?

#### 🏪 Small Business (SMB) / Pyme Context
*(For small businesses with limited staff and budget)*
- Hosting budget constraints? (affects choice between managed SaaS vs self-hosted)
- Staff technical literacy — can they manage infrastructure or need a turnkey solution?
- How quickly must staff be able to learn/operate the system? (target: < X minutes)
- Support model: self-service docs, vendor support, or IT person on staff?
- How critical is uptime to business cash flow? (e.g., restaurant queue down = lost revenue)
- **Defaults for SMB unless stated**: prefer managed/hosted solutions over self-hosted; simplicity over feature density; turnkey deploy; minimal maintenance burden.


#### 🔧 Third-Party Integrations & External Dependencies
- Which external services are required? (payment, SMS, email, maps, analytics, CRM)
- Are API keys / credentials already available?
- What is the fallback if an external service is unavailable?
- Cost constraints on third-party usage?
- **For forms / contact pages**: Where should form submissions be processed?
  - (a) Email service (SendGrid, Mailgun, Brevo)  
  - (b) Managed form handler (Netlify Forms, Formspree, Basin)  
  - (c) Custom serverless function or backend endpoint  
  - (d) CRM integration (HubSpot, Pipedrive)  
  → Choice affects hosting requirements, cost, and Phase 1 scope.

#### 🔐 Authentication & Authorization
*(For multi-user or access-controlled systems)*
- Are there multiple user roles? List each role and its permissions.
- How does authentication work? (login form, SSO, social auth, API key, no auth)
- Do customers/public users need accounts, or is public access sufficient?
- Are there admin-only or staff-only sections?

#### 🛡️ Legal & Compliance
- Is personal data collected? (forms, accounts, analytics) → requires privacy policy
- Geographic compliance scope? (GDPR for EU users, CCPA for CA users)
- Cookie consent banner required?
- Data retention policy? Where is data stored? Who has access?
- Industry-specific regulations? (HIPAA, PCI-DSS, COPPA)

#### 📊 Analytics & Success Metrics
- What does "success" look like in measurable terms? (leads/month, conversion rate, uptime %)
- What events or conversions should be tracked?
- Analytics tool? (Google Analytics, Plausible, Mixpanel, none)
- Are alerts needed when metrics drop below a threshold?

#### 🚀 Deployment & Infrastructure
- Target hosting platform? (Netlify, Vercel, AWS, VPS, on-premise, Docker)
- Environment strategy needed? (dev / staging / prod)
- CI/CD pipeline required, or manual deploy?
- Custom domain? Who controls DNS?
- How should rollbacks work in case of a bad deploy?

#### 🖥️ CLI / Developer Tool Concerns
*(For CLI tools, SDKs, npm packages)*
- Command syntax style: GNU-style (`--option value`) or subcommand style (`tool cmd --flag`)?
- Short-flag aliases? (`-o` for `--output`)
- Exit code strategy? (0 = success, 1 = generic error, specific codes for error types)
- Output verbosity levels? (`--quiet`, `--verbose`, `--json`)
- Config file strategy: which files searched, in what order, precedence over CLI args?
- Distribution: npm global only, or also standalone binary?
- Cross-platform: any OS-specific dependencies (native binaries, fonts)?
- Startup time target? (global CLI tools should start in < 500ms)
- Memory ceiling for typical operations? (e.g., < 100MB for normal use)
- Watch mode / long-running process: debounce timing, cleanup on SIGTERM?
- npm publication: who can publish, 2FA required, automated via CI on tag?

#### �️ Security
*(For any system accepting user input, especially public-facing forms)*
- Is there a public-facing form that collects contact data? → requires spam protection
- Spam / bot protection strategy: reCAPTCHA v3, hCaptcha, or honeypot fields?
- CSRF protection required? (for any non-static form handling backend)
- Input validation: client-side only, server-side, or both?
- Rate limiting on form submissions or API endpoints?
- For authentication systems: session management strategy, token expiry?- **For CLI tools with user-controlled input** (custom CSS, config files, templates):
  - Should user-supplied CSS be sandboxed or restricted to a safe property whitelist?
  - Can config files execute arbitrary code (`.js` configs with functions), or YAML/JSON only?
  - Should file path arguments be validated to prevent directory traversal?
#### 🔍 SEO & Discoverability
*(For publicly indexed pages or marketing sites)*
- Target keywords for local / organic search?
- Structured data (JSON-LD schema): `LocalBusiness`, `Service`, `Review`, `Product`?
- Open Graph tags (`og:title`, `og:image`, `og:description`) needed?
- XML sitemap and robots.txt required?
- Canonical URL strategy for any duplicate-content risk?

#### 🔌 API & Integration Design
*(For any system exposing or consuming an HTTP API)*
- REST, GraphQL, or gRPC? If no preference, recommend REST for CRUD resources.
- Response envelope format? (`{ data, error, status }` vs flat JSON)
- API versioning strategy? (path prefix `/v1/`, accept header, or stable with deprecation)
- Rate limiting / throttling requirements?
- Authentication for API clients (Bearer token, API key, session cookie)?

---

**Domain-Specific Default Assumptions**
*(Apply when user doesn't answer — document in plan as "Assumed")*

| Domain | Default Assumption |
|--------|-------------------|
| Web app (public-facing) | WCAG AA accessibility minimum |
| Web app with forms | Spam protection (honeypot fields at minimum) + privacy policy required |
| Real-time system | WebSocket recommended over polling; latency target = <2s for UI updates |
| CLI tool (npm global) | Startup < 500ms; memory < 100MB; GNU-style flags; exit 0 = success |
| REST API | JSON envelope `{data, error}`; `/v1/` prefix; Bearer token auth |
| Landing page (multilingual) | Always ask specific country/locale — affects phone, currency, date format |

---

### Step 4: Present Multiple Solutions (When Applicable)

When multiple valid approaches exist, OR when the user says "open to suggestions" for a key technical decision:

```markdown
🔀 **Multiple Solutions Available**

---

**Option A: [Name]** ⭐ *Recommended*
- **Approach**: [Description]
- **Pros**: [Benefits]
- **Cons**: [Drawbacks]
- **Effort**: [Low/Medium/High]

---

**Option B: [Name]**
- **Approach**: [Description]
- **Pros**: [Benefits]
- **Cons**: [Drawbacks]
- **Effort**: [Low/Medium/High]

---

**My Recommendation**: Option [X] because [reasoning].

Which approach would you prefer?
```

**Tech Stack Evaluation Matrix**
*(Use when user is "open to suggestions" or the stack is undefined)*

For each critical technology choice (frontend framework, backend, database, real-time transport, etc.):

| Criterion | Option A | Option B | Option C |
|-----------|----------|----------|----------|
| Team expertise fit | — | — | — |
| Time-to-market | — | — | — |
| Performance for stated NFRs | — | — | — |
| Community/maintenance | — | — | — |
| Hosting cost | — | — | — |
| Specific capability fit | — | — | — |

> **Rule**: Always provide an explicit recommendation with reasoning. Do NOT just list options and ask the user to pick with no guidance. If the user lacks technical judgment, recommend directly and explain trade-offs.

**Dependency Selection Guidance**
*(For tools with multiple library choices — CLI frameworks, PDF generators, testing libs)*

Present 2-3 options for each major dependency, scored by:
- Bundle size / runtime overhead
- Maintenance status (last commit, open issues)
- License compatibility
- Cross-platform support
- Required peer dependencies


### Step 5: Create Plan Document

**ALL plans are written to disk** in `.github/copilot/plans/PLAN-XXX.md`. No inline-only plans.

For SMALL changes, use a simplified version (summary, scope, tasks, testing, learning checklist).
For MEDIUM and BIG changes, use the full template:

```markdown
# PLAN-XXX: [Title]

## Status: pending_review

## Summary

[2-3 sentence description]

## Background

[Context and why this change is needed]

## MVP Slice

> Identify the smallest deliverable that provides real value. Everything else is "Phase 2+".

**Must-Have (MVP)**:
- [Core feature or behavior the solution cannot ship without]

**Out of MVP (Future)**:
- [Features that are valuable but not blocking launch]

## Scope

### In Scope
- [Item 1]
- [Item 2]

### Out of Scope
- [Item 1]

## Non-Functional Requirements

| Category | Requirement | Source |
|----------|-------------|--------|
| Performance | [e.g., page load < 3s on mobile 4G] | User requirement |
| Availability | [e.g., 99.5% uptime during business hours] | Assumed / Specified |
| Concurrency | [e.g., 50+ simultaneous users] | User requirement |
| Accessibility | [e.g., WCAG AA] | Assumed / Specified |
| Browser support | [e.g., Chrome 100+, Mobile Safari 15+] | Assumed / Specified |
| Security | [e.g., input validation, CSRF protection, rate limiting] | Best practice |
| *[CLI only]* Startup time | [e.g., < 500ms cold start] | CLI default |
| *[CLI only]* Memory ceiling | [e.g., < 100MB for typical operation] | CLI default |
| *[Real-time only]* Update latency | [e.g., queue position updates < 2s across all devices] | User requirement |

## External Dependencies

| Service | Purpose | Provider | Failure Behavior |
|---------|---------|----------|------------------|
| [e.g., SMS] | [Notify customers] | [Twilio] | [Queue and retry; show in-app fallback] |

## Implementation Phases

### Phase 1: [Name]
**Estimated changes**: ~XX lines

**Definition of Done**:
- [ ] [Specific, verifiable condition that confirms this phase is complete]
- [ ] [Test or check that proves it works]

Files affected:
- `path/to/file.ts` - [create/modify/delete] - [what changes]

Tasks:
1. [Task 1]
2. [Task 2]

### Phase 2: [Name]
[Same structure]

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| [Risk] | Low/Med/High | Low/Med/High | [How to mitigate] |

## Error Handling & Resilience
*(Include for any system with external dependencies or concurrent users)*

| Failure Scenario | Handling Strategy | User-Visible? |
|-----------------|-------------------|---------------|
| [e.g., SMS delivery failure] | [Retry 3x, then mark as failed in DB] | [No — staff sees status badge] |
| [e.g., Database connection lost] | [Return 503 with retry-after header] | [Yes — error toast] |

## Rollback Strategy

[How to undo if something goes wrong]

## Deployment & Environment Plan

- **Environments**: [dev / staging / prod or "single environment"]
- **CI/CD**: [manual deploy / GitHub Actions / other]
- **Initial data / seeding**: [how to populate required initial data]
- **Rollback procedure**: [git revert + redeploy / database migration down / other]

## Testing Requirements

- [ ] Unit tests for [component]
- [ ] Integration tests for [flow]
- [ ] [For CLI tools] Exit code tests for each error type (file not found, invalid config, generation failure)
- [ ] [For CLI tools] stdout/stderr capture validation for command output
- [ ] [For CLI tools] Help text and `--version` consistency tests
- [ ] [For real-time features] Concurrent user load test: acceptance criteria = [latency < Xms under N concurrent users, error rate < Y%]
- [ ] [For UI features] Accessibility check (WCAG AA automated scan)
- [ ] [For public forms] Spam protection verification (honeypot/CAPTCHA not bypassable)

## Distribution & Publication
*(Include for libraries, CLI tools, or npm packages)*

Pre-publication checklist:
- [ ] `npm audit` passes with no critical/high vulnerabilities
- [ ] Test suite passes on all target platforms (CI matrix: macOS, Linux, Windows)
- [ ] Build output confirmed valid (correct entry points, no missing files)
- [ ] Semantic version bumped per change impact
- [ ] CHANGELOG.md updated (if maintained)
- [ ] 2FA enabled on npm account

CI/CD publishing strategy:
- [ ] Publication triggered by: [git tag push / manual approval / merge to main]
- [ ] Platform CI matrix covers: [macOS / Linux / Windows as applicable]

## Documentation Updates

- [ ] Update `architecture.md` if structure changes
- [ ] Update `api.md` if endpoints change

## Post-Execution Learning Checklist

> Reviewed by the evaluator (learning mode) after this plan completes.

- [ ] **Skills used**: [list skills invoked] — review for missing context or workflow gaps
- [ ] **Docs referenced**: [list docs read] — check if still accurate after changes
- [ ] **Context relied on**: [list context.md sections used] — verify/update if changed
- [ ] **Discoveries**: [note any new patterns, conventions, or architecture insights]
- [ ] **Skill updates needed**: [flag if any skill lacked workflow steps for this task]

---
*Created: [DATE] | Status: pending_review*
```

### Step 5.5: Generate Knowledge File

Create `.github/copilot/plans/KNOWLEDGE-XXX.md` (same XXX as the plan) immediately after the plan. This file is the **execution context cheat sheet** — everything an AI agent needs to implement the plan without re-discovering context.

```markdown
# KNOWLEDGE-XXX: [Plan Title] — Execution Context

> **Plan**: PLAN-XXX.md
> **Generated**: [date]
>
> 📖 Re-read this file whenever you lose context mid-execution.

---

## Project Context Snapshot

- **Project**: [name from context.md]
- **Stack**: [language + framework]
- **Greenfield or brownfield**: [new project / adding to existing codebase]
- **Relevant standards**: [list which standards/*.md apply]

## Architecture Context

[Summarize the relevant parts of architecture.md — only what matters for THIS plan]

- Module boundaries relevant to this change
- Data flow through affected components
- Integration points the change touches

## Key Files & Their Roles

| File | Purpose | How This Plan Affects It |
|------|---------|--------------------------|
| `path/to/file` | [what it does] | [create/modify/delete — what changes] |

## Code Patterns to Follow

[For **brownfield** projects: extract from standards and existing code — the specific patterns the agent must follow]

[For **greenfield** projects: document the architectural patterns, conventions, and reference examples the agent should establish. If no prior code exists, document community conventions and the approach chosen in Step 4 of the plan.]

- Error handling pattern: [specific to this project/language]
- Naming convention: [relevant examples from codebase or chosen convention]
- Import/module pattern: [how this project organizes imports]

## Existing Code Snippets

[For **brownfield** projects: include actual code snippets from the codebase that the agent will need to reference or extend — function signatures, type definitions, interfaces, config structures]

[For **greenfield** projects: include reference patterns from the plan's tech stack selection — starter template, boilerplate structure, or community conventions to follow]

\`\`\`[language]
// Example: Interface or pattern to establish
[relevant snippet or reference pattern]
\`\`\`

## NFR Constraints to Enforce

> Taken from the plan's Non-Functional Requirements section. Do not implement in a way that violates these.

- [e.g., Page load < 3s on mobile 4G → no unoptimized images, lazy load below fold]
- [e.g., WCAG AA → all images need alt text, color contrast ≥ 4.5:1]
- [e.g., 50 concurrent users → connection pooling required, avoid N+1 queries]

## External Service Configuration

| Service | Integration Point | Credentials Location | Failure Handling |
|---------|------------------|---------------------|-----------------|
| [e.g., Twilio SMS] | [Phase 5, notifications module] | [.env TWILIO_API_KEY] | [Retry 3x, then mark failed] |

## Constraints & Gotchas

- [Things that are easy to get wrong in this codebase]
- [Non-obvious dependencies or side effects]
- [Environment or config requirements]
- [Cross-platform concerns (if CLI tool or multi-OS app)]

## Dependencies Between Phases

[If multi-phase plan: what each phase produces that later phases need]

- Phase 1 creates: [artifact] → used by Phase 2 in [file]
- Phase 2 creates: [artifact] → used by Phase 3 in [file]

---
*Auto-generated by planning skill. Edit if context is missing or wrong.*
```

**Rules for KNOWLEDGE-XXX.md:**
- Include REAL code snippets from the codebase, not placeholders
- Include REAL file paths discovered during planning
- **Greenfield projects**: Replace "Existing Code Snippets" content with reference patterns and chosen conventions
- Keep it focused — only context relevant to THIS plan
- If the plan is SMALL, the knowledge file can be brief (Project Context + Key Files + Code Patterns)
- For MEDIUM/BIG plans, include all sections
- Always populate "NFR Constraints to Enforce" — this prevents accidental violations during implementation

### Step 5b: Revise Plan from Plan-Reviewer Feedback

When the plan-reviewer (curate mode) returns a REVISE verdict, the planning skill receives structured feedback and revises the plan:

1. Receive evaluator feedback (issues table + suggestions)
2. Read the current PLAN-XXX.md
3. Address EACH issue from the evaluator's feedback:
   - Completeness gaps → Add missing sections/detail
   - Feasibility issues → Reorder phases or adjust scope
   - Risk gaps → Add to risk assessment
   - Scope creep → Trim to what was requested
4. Update PLAN-XXX.md in place (same file, same ID)
5. Return to evaluator for re-critique

**Rules**: Preserve plan ID across revisions. Address ALL issues. Don't add scope beyond what evaluator requested. Fix issues without bloating.

### Step 6: Update State

Update `.github/copilot/plans/state.yaml`:

```yaml
plans:
  PLAN-XXX:
    title: "[Title]"
    status: pending_review
    created: "[DATE]"
    updated: "[DATE]"
    knowledge: "KNOWLEDGE-XXX.md"
```

### Step 7: Request Approval

```markdown
📋 **Plan Ready for Review**

I've created **PLAN-XXX: [Title]**

**Summary:** [Brief summary]

**Phases:** [Number of phases]
**Files affected:** [Count]
**Estimated effort:** [Small/Medium/Large]

📄 **Plan:** `.github/copilot/plans/PLAN-XXX.md`
📖 **Knowledge:** `.github/copilot/plans/KNOWLEDGE-XXX.md`

Reply with: ✅ approve | ❌ reject | 📝 revise [feedback]
```

---

## Plan States

| State | Description |
|-------|-------------|
| `draft` | Being created |
| `pending_review` | Ready for approval |
| `approved` | Ready to implement |
| `in_progress` | Being implemented |
| `completed` | Successfully done |
| `archived` | Done and archived |
| `rejected` | Not proceeding |

---

## Step 8: Capture Decisions (After Completion)

When a plan reaches `completed` status, extract key architectural or design decisions into `docs/decisions/`:

When a plan reaches `completed` status, check:

1. Did the plan choose between multiple approaches? → DEC
2. Did the plan introduce a new pattern or convention? → DEC
3. Did the plan add/replace a dependency? → DEC
4. Did the plan change the project structure? → DEC

If ANY are true → Create `docs/decisions/DEC-XXX.md`
If NONE → Skip (not every plan produces a decision)

For each captured decision:
1. Create `docs/decisions/DEC-XXX.md` using the decision template
2. Update `docs/decisions/index.yaml` with new entry
3. Update `context.md` Key Decisions table if it affects project identity

This ensures plan rationale survives in git after the `plans/` directory is cleaned up.

---

## Output Format

Return to orchestrator:

```yaml
status: success | needs_input | error
result: 
  plan_id: "PLAN-XXX"
  plan_status: "pending_review"
  files_affected: [list]
  estimated_size: "small|medium|big"
context_updates:
  active_task: "Planning: [description]"
  pending_tasks:
    - "Implement PLAN-XXX (waiting approval)"
next_skill: null  # or "coding" if auto-approved
user_message: "[Message to show user]"
```

---

## Never Do

- ❌ Create a plan without asking clarifying questions first
- ❌ Choose an approach autonomously when multiple valid options exist
- ❌ Implement anything without explicit approval
- ❌ Skip the size analysis
- ❌ Keep a plan only in the conversation — ALL plans go to disk as `.md` files
- ❌ Create a plan without the Post-Execution Learning Checklist section
- ❌ Forget to update state.yaml
- ❌ Complete a plan without checking if decisions should be captured in `docs/decisions/`
- ❌ Ignore evaluator feedback during revision — address every issue
- ❌ Skip the Non-Functional Requirements section for any production system
- ❌ Skip Design & UX Context questions for any UI-facing feature
- ❌ Skip Legal & Compliance questions when the app collects personal data
- ❌ Present tech stack options without providing an explicit recommendation and reasoning
- ❌ Omit Definition of Done for each phase in MEDIUM/BIG plans
- ❌ Leave the KNOWLEDGE file's NFR Constraints section empty
- ❌ Treat External Dependencies as implementation details — they belong in the plan upfront
- ❌ Ask 'what language' without also asking the specific country/locale (e.g., Spanish ≠ Spanish Mexico)
- ❌ Skip SMB/Pyme context questions when the user describes a small business context

---

## Standards Integration

Before planning, check if `.github/copilot/standards/` exists and read:
- `general.md` - Universal standards
- `[language].md` - Language-specific patterns

Plans should align with these standards.
