# Generate Project Skills

Execute this prompt to scan the current project and automatically generate custom skills based on detected patterns, technologies, and workflows.

## ⚠️ CRITICAL: Skill Location

**ALL skills MUST be created in `.github/skills/`**

Skills are the agent's specialized capabilities - each one handles a specific domain.

---

## Instructions

Perform ALL of the following steps in order. Do not skip any steps.

---

## Step 1: Load Current State

Read existing configuration:

```
1. Read .copilot/context.md for project identity
2. Read .github/skills/index.yaml for existing skills
3. Note which skills are already defined (avoid duplicates)
```

---

## Step 2: Deep Project Scan

Analyze the entire project for patterns that warrant custom skills:

### 2.0 Proposal-First Rule (NEW)

Before creating any skill files, generate a proposal list from scan results:

| Skill | Evidence | Confidence | Create Now | Defer |
|-------|----------|------------|------------|-------|
| `[name]/SKILL.md` | `[paths]` | [high/med/low] | [yes/no] | [reason] |

Rules:
- `high` confidence + clear evidence: create now.
- `medium/low` confidence: defer and record as skill gap.
- Do not create speculative skills without concrete evidence.

### 2.1 Technology Detection

| Pattern to Detect | Files/Directories | Suggested Skill |
|-------------------|-------------------|-----------------|
| GraphQL | `*.graphql`, `schema.graphql`, `/graphql/` | `graphql/SKILL.md` |
| Database/ORM | `migrations/`, `prisma/`, `*.entity.ts`, `models/` | `database/SKILL.md` |
| CI/CD | `.github/workflows/`, `.gitlab-ci.yml`, `Jenkinsfile` | `devops/SKILL.md` |
| Kubernetes | `k8s/`, `*.yaml` with `apiVersion`, `helm/` | `kubernetes/SKILL.md` |
| Terraform/IaC | `*.tf`, `terraform/`, `pulumi/` | `infrastructure/SKILL.md` |
| i18n/l10n | `locales/`, `i18n/`, `*.po`, `*.xliff` | `localization/SKILL.md` |
| Authentication | `auth/`, `passport`, `jwt`, `oauth` | `auth/SKILL.md` |
| API Gateway | `gateway/`, `api-gateway`, `kong`, `nginx` | `gateway/SKILL.md` |
| Message Queue | `queue/`, `workers/`, `rabbitmq`, `kafka`, `bull` | `messaging/SKILL.md` |
| Caching | `redis`, `cache/`, `memcached` | `caching/SKILL.md` |
| Search | `elasticsearch`, `algolia`, `meilisearch` | `search/SKILL.md` |
| File Storage | `s3`, `storage/`, `uploads/`, `minio` | `storage/SKILL.md` |
| Email | `email/`, `mailer/`, `sendgrid`, `ses` | `email/SKILL.md` |
| Payments | `payments/`, `stripe`, `paypal`, `billing/` | `payments/SKILL.md` |
| Analytics | `analytics/`, `tracking/`, `segment`, `mixpanel` | `analytics/SKILL.md` |
| Logging | `logging/`, `winston`, `pino`, `sentry` | `observability/SKILL.md` |
| Mobile | `ios/`, `android/`, `react-native`, `flutter` | `mobile/SKILL.md` |
| Monorepo | `packages/`, `apps/`, `lerna.json`, `turbo.json` | `monorepo/SKILL.md` |
| Microservices | Multiple `service-*/` dirs, `docker-compose` | `microservices/SKILL.md` |
| WebSocket | `socket.io`, `ws`, `websocket/` | `realtime/SKILL.md` |
| Background Jobs | `jobs/`, `workers/`, `cron/`, `agenda` | `jobs/SKILL.md` |
| Feature Flags | `flags/`, `launchdarkly`, `unleash` | `feature-flags/SKILL.md` |

### 2.2 Framework-Specific Patterns

| Framework | Pattern | Suggested Skill |
|-----------|---------|-----------------|
| Next.js | `pages/`, `app/`, `next.config.js` | `nextjs/SKILL.md` |
| NestJS | `*.module.ts`, `*.controller.ts` | `nestjs/SKILL.md` |
| Django | `manage.py`, `settings.py`, `urls.py` | `django/SKILL.md` |
| FastAPI | `main.py` with FastAPI, `routers/` | `fastapi/SKILL.md` |
| Rails | `Gemfile` with rails, `app/controllers/` | `rails/SKILL.md` |
| Spring | `pom.xml` with spring, `@RestController` | `spring/SKILL.md` |
| Actix/Axum | `Cargo.toml` with actix/axum | `rust-web/SKILL.md` |

### 2.3 Domain-Specific Patterns

Analyze business logic for domain skills:

| Domain Indicator | Suggested Skill |
|------------------|-----------------|
| E-commerce (cart, checkout, products) | `ecommerce/SKILL.md` |
| CMS (content, posts, pages) | `cms/SKILL.md` |
| SaaS (tenants, subscriptions, billing) | `saas/SKILL.md` |
| Social (users, posts, feeds, follows) | `social/SKILL.md` |
| Marketplace (listings, orders, sellers) | `marketplace/SKILL.md` |

---

## Step 3: Generate Skill Files

For each detected pattern marked `Create Now` that doesn't already have a skill, create a skill file.

### Skill File Template

`.github/skills/[name]/SKILL.md`:

```markdown
---
name: [skill-name]
description: [What this skill does and when to use it in this project]
---

# [Skill Name] Skill

## Identity

- **Name**: [skill_name]
- **Version**: 1.0
- **Description**: [What this skill handles in THIS project]
- **Auto-generated**: true
- **Detected from**: [What pattern triggered this]

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| [keyword 1] | "[example request 1]" |
| [keyword 2] | "[example request 2]" |
| [pattern] | "[example request]" |

---

## Capabilities

What this skill can do in this project:

- ✅ [Capability 1 specific to detected pattern]
- ✅ [Capability 2 specific to detected pattern]
- ✅ [Capability 3 specific to detected pattern]

---

## Project Context

### Detected Patterns

- **Location**: [paths where pattern was found]
- **Technology**: [specific tech/library used]
- **Configuration**: [config files found]

### Key Files

| File | Purpose |
|------|---------|
| `[path]` | [what it does] |
| `[path]` | [what it does] |

### Existing Conventions

[Note any patterns/conventions found in the existing code]

---

## Workflow

### Step 1: [First Step Name]

[Description of what to do first]

### Step 2: [Second Step Name]

[Description of what to do second]

### Step 3: [Third Step Name]

[Description of what to do third]

---

## Templates

### [Template Name 1]

\`\`\`[language]
[Template code specific to this project's patterns]
\`\`\`

### [Template Name 2]

\`\`\`[language]
[Template code specific to this project's patterns]
\`\`\`

---

## Integration Points

How this skill connects with others:

| Skill | Relationship |
|-------|--------------|
| coding | [how they interact] |
| testing | [how they interact] |
| [other] | [how they interact] |

---

## Checklist

Before completing work with this skill:

- [ ] [Check 1 relevant to this domain]
- [ ] [Check 2 relevant to this domain]
- [ ] [Check 3 relevant to this domain]

---

## Never Do

- ❌ [Domain-specific anti-pattern 1]
- ❌ [Domain-specific anti-pattern 2]
- ❌ [Domain-specific anti-pattern 3]

---

*Auto-generated by setup.skill | Detected from: [pattern] | Generated: [DATE]*
```

---

## Step 4: Update Skill Registry

Add ALL new skills to `.github/skills/index.yaml`:

```yaml
skills:
  # ... existing core skills ...

  # === AUTO-GENERATED SKILLS ===
  
  [skill_name]:
    file: "[name]/SKILL.md"
    name: "[Display Name]"
    description: "[Brief description]"
    priority: [next available number, starting from 10]
    triggers:
      keywords:
        - [keyword1]
        - [keyword2]
      patterns:
        - "[pattern1]"
        - "[pattern2]"
    requires_approval: [true for destructive operations, false otherwise]
    auto_generated: true
    detected_from: "[pattern that triggered generation]"
    can_chain_to:
      - [related skills]
```

### 4.1 Record Deferred Skill Gaps (NEW)

Update `.copilot/docs/skills-opportunities.md` with deferred entries:

```markdown
## Deferred Skill Gaps

- `[name]/SKILL.md`: [reason], trigger phrases: [keywords/patterns]
```

### Priority Guidelines

| Priority Range | Skill Type |
|----------------|------------|
| 1-9 | Core skills (planning, coding, analysis, etc.) |
| 10-19 | Technology skills (database, graphql, etc.) |
| 20-29 | Framework skills (nextjs, nestjs, etc.) |
| 30-39 | Domain skills (ecommerce, saas, etc.) |
| 40+ | Custom/project-specific skills |

---

## Step 5: Update Context Memory

Update `.copilot/context.md` with generated skills:

```markdown
## Learned Context

### Available Skills

| Skill | Type | Description |
|-------|------|-------------|
| planning | core | Plans and architecture |
| coding | core | Code generation |
| analysis | core | Code review and debugging |
| documentation | core | Docs management |
| testing | core | Test creation |
| setup | core | Project initialization |
| [new_skill] | auto-generated | [description] |
| [new_skill] | auto-generated | [description] |

### Project-Specific Rules

- [Rule derived from skill generation]
- [Rule derived from skill generation]
```

---

## Step 6: Validate Skills

For each generated skill:

```
1. Verify file was created correctly
2. Check skill is registered in index.yaml
3. Confirm triggers don't overlap with existing skills
4. Ensure templates match project patterns
```

---

## Step 7: Report Summary

```markdown
🎯 **Skills Generated Successfully**

## Scan Results

- **Patterns detected**: [count]
- **Skills generated**: [count]
- **Skills skipped (already exist)**: [count]
- **Skills deferred as gaps**: [count]

## New Skills

| Skill | Detected From | Triggers |
|-------|---------------|----------|
| `[name]/SKILL.md` | [pattern] | [keywords] |

## Deferred Gaps

| Skill | Why Deferred | Trigger to Generate |
|-------|--------------|---------------------|
| `[name]/SKILL.md` | [reason] | [keywords/patterns] |
| `[name]/SKILL.md` | [pattern] | [keywords] |

## Updated Files

- `.github/skills/index.yaml` - Added [N] skills
- `.copilot/context.md` - Updated available skills
- `.github/skills/[name]/SKILL.md` - Created for each skill

## Routing Preview

Based on detected patterns, here's how requests will be routed:

| Example Request | Routed To |
|-----------------|-----------|
| "[example 1]" | [skill_name] |
| "[example 2]" | [skill_name] |

## Next Steps

1. Review generated skills in `.github/skills/`
2. Customize templates and workflows as needed
3. Add project-specific rules to skills
4. Test routing with example requests

---

*Generated: [DATE]*
```

---

## Skill Update Mode

If skills already exist and you want to UPDATE them:

### Update Workflow

```
1. Read existing skill file
2. Rescan project for new patterns
3. MERGE new capabilities (don't overwrite custom changes)
4. Update templates if project patterns changed
5. Update registry if triggers need adjustment
```

### Preserve Custom Changes

When updating, preserve:
- Custom workflow steps added by user
- Additional triggers added manually
- Project-specific rules and checks
- Custom templates

Only update:
- `Detected Patterns` section
- `Key Files` section
- Generic templates (if patterns changed)

---

## Important Notes

1. **No duplicate skills** - Check index.yaml before creating
2. **Project-specific** - Templates must match THIS project's patterns
3. **Preserve conventions** - Match existing code style in templates
4. **Accurate triggers** - Keywords must be specific enough to avoid false matches
5. **Chain correctly** - Skills should chain to related skills appropriately
6. **Test routing** - Verify triggers work as expected
7. **Enable on-demand creation** - Deferred gaps must include trigger hints for future change requests

---

## Runtime Gap Policy (NEW)

When a user asks for a code change and no existing skill matches:

1. Check deferred gaps in `.copilot/docs/skills-opportunities.md`
2. If a gap matches, generate and register that skill first
3. If no gap matches, synthesize a new project-specific skill from request + codebase evidence
4. Re-run routing and execute through the new skill

Never execute the change request directly without first establishing a suitable skill.

---

## Example Generated Skills

### Example: Database Skill (Prisma detected)

```markdown
---
name: database
description: Manage Prisma schema, migrations, and database operations.
---

# Database Skill

## Identity

- **Name**: database
- **Version**: 1.0
- **Description**: Manages Prisma schema, migrations, and database operations
- **Auto-generated**: true
- **Detected from**: `prisma/schema.prisma`, `prisma/migrations/`

## Triggers

| Pattern | Example |
|---------|---------|
| migration, prisma, database, model | "Create a new migration for users" |
| "add.*field", "modify.*table" | "Add email field to User model" |

## Project Context

### Detected Patterns
- **Location**: `prisma/`
- **Technology**: Prisma ORM
- **Configuration**: `prisma/schema.prisma`

### Key Files
| File | Purpose |
|------|---------|
| `prisma/schema.prisma` | Database schema |
| `prisma/migrations/` | Migration history |

## Workflow

1. Read current schema.prisma
2. Understand existing models
3. Generate schema changes
4. Create migration
5. Update related types

## Templates

### New Model
\`\`\`prisma
model [Name] {
  id        String   @id @default(cuid())
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  // fields
}
\`\`\`
```

### Example: NextJS Skill (Next.js detected)

```markdown
---
name: nextjs
description: Manage Next.js pages, routes, and components for this project.
---

# NextJS Skill

## Identity

- **Name**: nextjs
- **Version**: 1.0
- **Description**: Manages Next.js pages, API routes, and components
- **Auto-generated**: true
- **Detected from**: `next.config.js`, `app/` directory

## Triggers

| Pattern | Example |
|---------|---------|
| page, route, component, api | "Create a new page for dashboard" |
| "add.*api", "create.*route" | "Add an API route for users" |

## Project Context

### Detected Patterns
- **Location**: `app/`, `components/`
- **Technology**: Next.js 14 (App Router)
- **Configuration**: `next.config.js`

### Key Files
| File | Purpose |
|------|---------|
| `app/layout.tsx` | Root layout |
| `app/page.tsx` | Home page |
| `components/` | Shared components |

## Templates

### New Page
\`\`\`tsx
export default function [Name]Page() {
  return (
    <main>
      <h1>[Name]</h1>
    </main>
  );
}
\`\`\`

### API Route
\`\`\`tsx
import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({ });
}
\`\`\`
```

---

*Execute this prompt to generate project-specific skills automatically.*
