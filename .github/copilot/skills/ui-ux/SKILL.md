---
name: ui-ux
description: Generate a project design system and produce a persistent ui-style skill for all future UI coding. Runs once per project unless the user requests a style change.
version: "1.0"
---

# UI/UX Skill

## Identity

- **Name**: ui-ux
- **Version**: 1.0
- **Description**: Generates a complete design system (style, colors, typography, layout patterns, UX rules) for the user's project and persists it as a `ui-style` skill that all subsequent UI coding must follow.

---

## Triggers

When to activate this skill:

| Trigger Pattern | Example Request |
|-----------------|-----------------|
| Keywords: web page, UI, design, style, frontend, layout | "Create a web page for my app" |
| "build.*page" | "Build a landing page" |
| "design.*app" | "Design the app UI" |
| "create.*component" | "Create a pricing card" |
| Keywords: redesign, restyle, change style | "Change the application style" |
| "style.*application" | "Define the style for the app" |

---

## Capabilities

- ✅ Generate a complete tailored design system from the user's product type
- ✅ Leverage `ui-ux-pro-max` external library (with user consent)
- ✅ Fall back to AI knowledge if external library is not authorized
- ✅ Produce a persistent `ui-style` project skill that encodes the chosen design system
- ✅ Skip re-generation if a style already exists (idempotency gate)
- ✅ Update the style when the user explicitly requests a change

---

## Dependencies

- `.github/copilot/context.md` — for project identity, stack, and prior authorization decisions
- `.github/copilot/external/ui-ux-pro-max-skill/` — external design intelligence library (optional, requires user approval)
- `.github/copilot/skills/ui-style/SKILL.md` — **output** of this skill; used by `coding` skill for all UI work

---

## Workflow

### Step 0: Idempotency Gate (MANDATORY FIRST CHECK)

Before doing anything, check if a style has already been defined:

```
1. Check if .github/copilot/skills/ui-style/SKILL.md exists
2. If YES and the user did NOT explicitly ask to "change", "update", or "redesign" the style:
   → STOP. Inform the user:

   "A design system already exists for this project (.github/copilot/skills/ui-style/SKILL.md).
    I'll apply it to your request. If you want to change the style, say 'change the application style'."

   → Then route to coding skill with ui-style as a dependency.

3. If YES and the user DID ask to change the style → continue with workflow (overwrite at the end).
4. If NO → continue with workflow.
```

---

### Step 1: Gather Project Information

Collect the following from the user's request and `context.md`:

| Information | Where to Look | If Missing |
|-------------|--------------|------------|
| Product type | User request, context.md | Ask: "What type of product is this? (e.g., SaaS, portfolio, e-commerce, healthcare app)" |
| Target audience | User request, context.md | Infer from product type if possible |
| Tech stack | context.md stack field | Ask: "Which frontend stack are you using? (e.g., React, Next.js, Vue, HTML+Tailwind)" |
| Style preference | User request | Infer from product type if not stated |

---

### Step 2: External Library Authorization

> ⚠️ This step determines whether to use the `ui-ux-pro-max` external library.

#### Step 2a: Check Prior Decision

```
1. Read context.md → look for "ui-ux-pro-max authorization" under User Preferences or Key Decisions.
2. If a prior decision exists:
   - "approved"  → skip to Step 3 (use library)
   - "denied"    → skip to Step 4 (AI knowledge only)
3. If no prior decision → continue to Step 2b.
```

#### Step 2b: Ask for Authorization

Present this message to the user:

```markdown
⚠️ **External Library Authorization Required**

To generate the best design system for your project, I can leverage **UI/UX Pro Max**
(https://github.com/nextlevelbuilder/ui-ux-pro-max-skill) — a design intelligence library
with 161 industry-specific design rules, 67 UI styles, 161 color palettes, 57 font pairings,
and 99 UX guidelines.

This library is already cloned locally at `.github/copilot/external/ui-ux-pro-max-skill/`.
No internet access is required. Only local Python scripts will run.

**Do you approve using this library to generate your design system?**

Reply with:
- ✅ **yes** — Use the library (recommended, best results)
- ❌ **no** — I'll use my built-in knowledge instead
```

#### Step 2c: Record Decision

After the user replies:

```
1. Update context.md → Key Decisions table:
   | ui-ux-pro-max authorization | [approved/denied] | ui-ux | [today's date] |

2. If approved → continue to Step 3
3. If denied → skip to Step 4
```

---

### Step 3: Generate Design System (External Library Path)

Run the design system generator using the cloned library:

```bash
python3 .github/copilot/external/ui-ux-pro-max-skill/src/ui-ux-pro-max/scripts/search.py \
  "<product_type> <industry> <style_keywords>" \
  --design-system \
  -p "<Project Name>"
```

Populate `<product_type>`, `<industry>`, and `<style_keywords>` from Step 1.

**Example** for a SaaS productivity tool:
```bash
python3 .github/copilot/external/ui-ux-pro-max-skill/src/ui-ux-pro-max/scripts/search.py \
  "saas productivity tool b2b" \
  --design-system \
  -p "MyApp"
```

If a specific stack was identified, also run a stack search:
```bash
python3 .github/copilot/external/ui-ux-pro-max-skill/src/ui-ux-pro-max/scripts/search.py \
  "<stack_keyword>" \
  --stack <stack>
```

Available stacks: `react`, `nextjs`, `vue`, `nuxtjs`, `html-tailwind`, `svelte`, `astro`, `angular`, `laravel`, `react-native`, `flutter`, `swiftui`, `jetpack-compose`, `shadcn`, `nuxt-ui`

Capture the full output — it includes: pattern, style, colors, typography, key effects, anti-patterns, and pre-delivery checklist.

→ Continue to Step 5 with this output.

---

### Step 4: Generate Design System (AI Knowledge Path)

When the external library is not authorized, use built-in reasoning to define the design system.

Determine the following based on the product type and context:

| Element | Determine |
|---------|-----------|
| **Layout pattern** | Hero + features? Dashboard-first? Content-heavy? |
| **UI style** | Minimal clean, glassmorphism, dark mode, soft UI, etc. |
| **Primary color** | Industry-appropriate (e.g., blue for SaaS, green for health) |
| **Secondary color** | Complementary or accent |
| **Typography** | Heading font + body font + mood |
| **Spacing system** | 4px / 8px grid |
| **Component style** | Rounded vs sharp, subtle shadows vs flat |
| **Anti-patterns** | What NOT to use for this product type |
| **Accessibility baseline** | Contrast ratio, touch targets, focus states |

Format the output to match the same structure as Step 3 output (pattern, style, colors, typography, effects, anti-patterns, checklist).

→ Continue to Step 5 with this output.

---

### Step 5: Present Design System to User

Show the generated design system as a summary and ask for approval:

```markdown
📋 **Proposed Design System for [Project Name]**

**Pattern**: [layout structure]
**Style**: [UI style name]
**Primary Color**: [hex] — [mood/name]
**Secondary Color**: [hex] — [mood/name]
**CTA Color**: [hex]
**Background**: [hex]
**Typography**: [Heading Font] / [Body Font] — [mood]
**Key Effects**: [effects]

**Sections / Layout Order**:
1. [Section 1]
2. [Section 2]
...

**Anti-patterns to avoid**:
- [item]
- [item]

**Stack**: [stack-specific notes if available]

Reply with: ✅ approve | 📝 revise [feedback]
```

If the user revises:
- Apply their feedback
- Re-present the updated design system
- Do NOT re-run the external library script unless the product type fundamentally changed
- Repeat until approved

---

### Step 6: Produce the `ui-style` Project Skill

Once the design system is approved, create (or overwrite) the project-specific style skill:

**File**: `.github/copilot/skills/ui-style/SKILL.md`

```markdown
---
name: ui-style
description: Project design system for [Project Name]. Applied to all UI coding in this project.
version: "1.0"
generated_by: ui-ux
generated_date: [date]
source: [ui-ux-pro-max / ai-knowledge]
---

# UI Style — [Project Name]

> ⚠️ This file is auto-generated by the ui-ux skill. Do not edit manually.
> To change the style, ask: "change the application style".

## Design System

### Pattern & Layout
[layout pattern and section order from Step 3/4]

### UI Style
**Name**: [style name]
[description]

### Colors
| Role | Hex | Usage |
|------|-----|-------|
| Primary | [hex] | Main CTAs, key UI elements |
| Secondary | [hex] | Supporting elements |
| CTA | [hex] | Buttons, links |
| Background | [hex] | Page background |
| Surface | [hex] | Cards, panels |
| Text | [hex] | Body text |
| Text Muted | [hex] | Secondary text, captions |

### Typography
| Role | Font | Weight | Size |
|------|------|--------|------|
| Heading | [font] | [weight] | [size] |
| Body | [font] | [weight] | [size] |
| Caption | [font] | [weight] | [size] |

Google Fonts import: `[import URL if applicable]`

### Spacing System
Base unit: [4px / 8px]
Scale: [spacing scale]

### Key Effects
[effects from design system: shadows, transitions, hover states, etc.]

### Stack-Specific Guidelines
**Stack**: [stack]
[stack-specific implementation notes]

## Anti-Patterns (NEVER Use)
[list of anti-patterns from design system output]

## Pre-Delivery Checklist
- [ ] Contrast ratio ≥ 4.5:1 for body text
- [ ] Hover states on all interactive elements (150–300ms transition)
- [ ] No emoji as icons (use SVG: Heroicons / Lucide)
- [ ] `cursor-pointer` on all clickable elements
- [ ] Focus states visible for keyboard navigation
- [ ] `prefers-reduced-motion` respected
- [ ] Responsive: 375px, 768px, 1024px, 1440px
- [ ] Touch targets ≥ 44×44px

## How the Coding Skill Uses This
When generating any UI code:
1. Apply the color tokens above — never use raw hex values in components
2. Use the heading/body fonts — import via the Google Fonts URL above
3. Follow the spacing system as the base grid
4. Apply effects (shadows, transitions) from Key Effects
5. Validate every UI output against the Anti-Patterns list
6. Run through the Pre-Delivery Checklist before marking work done
```

---

### Step 7: Register and Link

After creating the `ui-style` skill file:

1. **Register in `index.yaml`** — Add `ui-style` as a skill:

```yaml
  ui-style:
    file: "ui-style/SKILL.md"
    name: "UI Style"
    description: "Project-specific design system. Auto-applied to all UI coding."
    priority: 2
    triggers:
      keywords: []       # Not routed directly — injected as dependency by coding skill
      patterns: []
    requires_approval: false
    auto_inject_for:
      - coding
      - planning
    note: "Auto-generated by ui-ux skill. Overwritten when user requests style change."
```

2. **Update `context.md`** — Add to Key Decisions:

```
| Design system established | [style name] — [source] | ui-ux | [date] |
```

3. **Inform the user**:

```markdown
✅ **Design system saved** to `.github/copilot/skills/ui-style/SKILL.md`

This style will be automatically applied to all future UI coding in this project.
To change it later, say: "change the application style".

Ready to build your UI. What would you like to create first?
```

4. **Chain to coding skill** if the original user request was to build something specific (e.g., "create a web page").

---

## Chaining Rules

| Condition | Chain To |
|-----------|----------|
| User asked to build/create a page or component | `coding` (with `ui-style` as dependency) |
| User only asked to define/set the style | Stop — style is now set |
| User asked to change the style | Re-run this skill from Step 1, overwrite `ui-style/SKILL.md` |

---

## Failure Modes

| Situation | Action |
|-----------|--------|
| Python not installed (external library path) | Fall back to Step 4 (AI knowledge). Note: "Python required for full library usage." |
| `search.py` fails or returns empty | Fall back to Step 4 (AI knowledge). Log error in session.md. |
| User doesn't approve the design system after 3 revisions | Ask: "Would you like to describe your style preferences in your own words?" and build from that. |
