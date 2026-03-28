# Markdown Best Practices & Standards

This document contains best practices for Markdown formatting that should be followed when creating or reviewing any `.md` documents.

> **📌 Important**: These standards must be read before writing any Markdown document. Following these guidelines ensures consistency, readability, and proper rendering across different Markdown parsers.

---

## Core Principles

### 1. Consistency First

Choose a style and stick to it throughout the document. Inconsistent formatting creates confusion and makes documents harder to maintain.

### 2. Readability Over Brevity

Markdown source should be readable even without rendering. Good formatting in the source file translates to good formatting in the output.

### 3. Parser Compatibility

Not all Markdown parsers are created equal. Following these standards ensures your documents render correctly across different platforms (GitHub, VS Code, kramdown, CommonMark, etc.).

### 4. Documentation Quality

- Keep docs task-oriented: explain what to do, why, and expected outcomes
- Keep code snippets accurate and synchronized with current implementation
- Update related docs in the same change when behavior, APIs, or config change
- Prefer short sections with clear headings over long narrative blocks
- Remove stale references, dead links, and outdated examples promptly

---

## Headings

### ATX-Style Headings (Required)

**Always use ATX-style headings** (hash symbols) instead of Setext-style (underlines).

```markdown
❌ Bad - Setext style
Heading 1
=========

Heading 2
---------

✅ Good - ATX style
# Heading 1

## Heading 2

### Heading 3
```

### Space After Hash

**Always include a space** between the hash symbol(s) and the heading text.

```markdown
❌ Bad - No space
#Heading Text
##Another Heading

✅ Good - Space after hash
# Heading Text
## Another Heading
```

### Heading Hierarchy

Maintain a logical hierarchy:

- Use only **one `H1`** per document (typically the document title)
- Don't skip levels (e.g., don't jump from `##` to `####`)
- Limit depth to 3-4 levels for clarity

```markdown
❌ Bad - Skipped level
# Main Title
### Subsection (skipped H2!)

✅ Good - Proper hierarchy
# Main Title
## Section
### Subsection
```

### Blank Lines Around Headings

**Surround headings with blank lines** for better readability and consistent rendering.

```markdown
❌ Bad - No blank lines
Some paragraph text.
## New Section
More text here.

✅ Good - Blank lines around heading
Some paragraph text.

## New Section

More text here.
```

---

## Lists

### 🚨 CRITICAL: Blank Line Before Lists

**A bullet list must always start with a blank line.** This is essential for proper rendering across all parsers.

```markdown
❌ Bad - No blank line before list
Here is my list:
- Item 1
- Item 2
- Item 3

✅ Good - Blank line before list
Here is my list:

- Item 1
- Item 2
- Item 3
```

> **⚠️ Warning**: Some parsers (like kramdown) may fail to render lists correctly without a preceding blank line. This rule ensures maximum compatibility.

### Consistent List Markers

Use hyphens (`-`) for unordered lists. Avoid mixing markers.

```markdown
❌ Bad - Mixed markers
* Item 1
- Item 2
+ Item 3

✅ Good - Consistent hyphens
- Item 1
- Item 2
- Item 3
```

### Ordered Lists

Use actual sequential numbers for ordered lists to aid editing and navigation.

```markdown
❌ Bad - All same numbers
1. First item
1. Second item
1. Third item

✅ Good - Sequential numbers
1. First item
2. Second item
3. Third item
```

### Nested List Indentation

Use **2 or 4 spaces** consistently for nested items. Two spaces are recommended for better mobile display.

```markdown
✅ Good - Consistent 2-space indent
- Parent item
  - Child item
  - Another child
    - Grandchild item

✅ Also good - Consistent 4-space indent
- Parent item
    - Child item
    - Another child
        - Grandchild item
```

### Blank Lines in Lists

- **Between list items**: Only add blank lines if items contain multiple paragraphs
- **After the list**: Always include a blank line after the list ends

```markdown
✅ Good - Simple list (no blank lines between items)
- Item 1
- Item 2
- Item 3

Some paragraph after the list.

✅ Good - Complex list items (blank lines between)
- First item with a longer explanation that might need
  more context and details.

- Second item that also needs
  extended explanation.
```

---

## Emphasis

### Consistent Syntax for Bold and Italic

Use **asterisks** for emphasis, not underscores. This avoids conflicts with technical terms that contain underscores.

```markdown
❌ Bad - Using underscores
_italic text_
__bold text__

✅ Good - Using asterisks
*italic text*
**bold text**
***bold and italic***
```

### Strikethrough

Use double tildes for strikethrough.

```markdown
~~deleted text~~
```

---

## Code

### Inline Code

Use single backticks for inline code, commands, file names, and technical terms.

```markdown
Use the `npm install` command to install dependencies.
Edit the `config.yaml` file to change settings.
```

### Fenced Code Blocks

**Always specify the language** for syntax highlighting.

````markdown
❌ Bad - No language specified
```
function hello() {
    return "world";
}
```

✅ Good - Language specified
```javascript
function hello() {
    return "world";
}
```
````

### Supported Languages

Common language identifiers:

| Language | Identifier |
|----------|------------|
| JavaScript | `javascript` or `js` |
| TypeScript | `typescript` or `ts` |
| Python | `python` or `py` |
| Bash/Shell | `bash` or `shell` |
| JSON | `json` |
| YAML | `yaml` |
| Markdown | `markdown` or `md` |
| C | `c` |
| C++ | `cpp` |
| Go | `go` |
| Rust | `rust` |

---

## Paragraphs and Line Breaks

### Paragraph Separation

Separate paragraphs with a **single blank line**. Do not use multiple blank lines.

```markdown
❌ Bad - Multiple blank lines
First paragraph.



Second paragraph (too much space above).

✅ Good - Single blank line
First paragraph.

Second paragraph.
```

### No Indentation

**Do not indent paragraphs** - this can be interpreted as a code block.

```markdown
❌ Bad - Indented paragraph
    This paragraph is indented and might render as code.

✅ Good - No indentation
This paragraph is not indented and will render correctly.
```

### Hard Line Breaks

Avoid trailing spaces for line breaks (they're invisible and confusing). Prefer using a blank line to create a new paragraph, or use `<br>` for explicit line breaks when necessary.

```markdown
❌ Bad - Trailing spaces (invisible!)
Line one with two trailing spaces  
Line two on new line.

✅ Better - Use paragraphs
Line one.

Line two as new paragraph.

✅ Or explicit break if needed
Line one<br>
Line two on new line.
```

---

## Links and Images

### Link Style

Prefer inline links for most cases. Use reference links for frequently repeated URLs or for readability in long documents.

```markdown
✅ Inline link
See the [documentation](https://example.com/docs) for more info.

✅ Reference link (for repeated use)
See the [documentation][docs] for more info.
Later, check the [API reference][docs].

[docs]: https://example.com/docs
```

### Descriptive Link Text

**Never use "click here" or "here"** as link text. Use descriptive text.

```markdown
❌ Bad - Vague link text
For more information, click [here](https://example.com).

✅ Good - Descriptive link text
For more information, see the [installation guide](https://example.com).
```

### Images

Always include alt text for accessibility.

```markdown
❌ Bad - No alt text
![](image.png)

✅ Good - Descriptive alt text
![Screenshot of the login page](image.png)
```

---

## Tables

### Table Formatting

Use consistent formatting with proper alignment.

```markdown
✅ Good - Well-formatted table
| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Value 1  | Value 2  | Value 3  |
| Value 4  | Value 5  | Value 6  |
```

### Column Alignment

Use colons to specify alignment:

```markdown
| Left     | Center   | Right    |
|:---------|:--------:|---------:|
| Left     | Center   | Right    |
```

### Blank Lines Around Tables

Always surround tables with blank lines.

```markdown
Some text before.

| Column 1 | Column 2 |
|----------|----------|
| Value 1  | Value 2  |

Some text after.
```

---

## Blockquotes

### Basic Usage

```markdown
> This is a blockquote.
> It can span multiple lines.
```

### Nested Blockquotes

```markdown
> First level quote.
>
> > Nested quote inside.
```

### GitHub-Style Alerts

Use GitHub-style alerts for special callouts:

```markdown
> [!NOTE]
> Helpful information that users should know.

> [!TIP]
> Helpful advice for doing things better.

> [!IMPORTANT]
> Key information users need to know.

> [!WARNING]
> Urgent info that needs immediate attention.

> [!CAUTION]
> Warns about risks or negative outcomes.
```

---

## Horizontal Rules

Use three hyphens with blank lines before and after.

```markdown
Content above.

---

Content below.
```

Be consistent - don't mix `---`, `***`, and `___`.

---

## File Organization

### Document Structure

Every Markdown document should follow this structure:

1. **Title** (H1) - One per document
2. **Brief description** or purpose
3. **Table of contents** (for long documents)
4. **Main content** with logical sections
5. **References/Links** (if applicable)

### File Naming

Use lowercase with hyphens for file names:

```
✅ Good
user-guide.md
api-reference.md
getting-started.md

❌ Bad
UserGuide.md
API Reference.md
getting_started.md
```

---

## Linting and Validation

### Recommended Tools

Use `markdownlint` or similar tools to enforce these standards:

**Key rules to enable:**

| Rule | Description |
|------|-------------|
| MD001 | Heading levels should increment by one |
| MD003 | Heading style (ATX) |
| MD004 | Unordered list style (consistent) |
| MD009 | Trailing spaces |
| MD010 | Hard tabs |
| MD012 | Multiple blank lines |
| MD022 | Headings should be surrounded by blank lines |
| MD023 | Headings must start at the beginning of the line |
| MD027 | Multiple spaces after blockquote symbol |
| MD031 | Fenced code blocks should be surrounded by blank lines |
| MD032 | Lists should be surrounded by blank lines |
| MD040 | Fenced code blocks should have a language specified |

---

## Summary Checklist

When writing or reviewing Markdown documents:

### Headings
- [ ] Using ATX-style (`#`) headings
- [ ] Space after hash symbols
- [ ] Single H1 per document
- [ ] Proper hierarchy (no skipped levels)
- [ ] Blank lines around headings

### Lists
- [ ] **Blank line before every list** (CRITICAL!)
- [ ] Consistent list markers (hyphens)
- [ ] Proper indentation for nested lists
- [ ] Blank line after lists

### Formatting
- [ ] Asterisks for emphasis (not underscores)
- [ ] Language specified in fenced code blocks
- [ ] Descriptive link text
- [ ] Alt text on images
- [ ] Single blank lines between paragraphs
- [ ] Blank lines around tables and blockquotes

### Document Quality
- [ ] Consistent styling throughout
- [ ] Logical heading hierarchy
- [ ] Clear and concise writing
- [ ] No trailing whitespace
- [ ] File uses lowercase with hyphens

---

*Last updated: 2024-12-23*
