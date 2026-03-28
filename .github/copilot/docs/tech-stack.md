<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# Technology Stack

## Runtime Environment

| Component | Version | Purpose |
|-----------|---------|---------|
| [Node.js / Python / Rust / Go] | [version] | Primary runtime |
| [Docker] | [version] | Containerization (if used) |

## Languages

| Language | Usage | File Extensions | Config |
|----------|-------|-----------------|--------|
| [TypeScript] | Primary | `.ts`, `.tsx` | `tsconfig.json` |
| [JavaScript] | Secondary | `.js`, `.jsx` | - |

## Package Management

| Manager | Version | Lock File |
|---------|---------|-----------|
| [npm / yarn / pnpm / cargo / pip] | [version] | `[lock file name]` |

---

## Frameworks & Libraries

### Core Framework

| Name | Version | Purpose | Docs |
|------|---------|---------|------|
| [Express / Fastify / etc.] | [version] | Web framework | [link] |

### Application Libraries

| Name | Version | Purpose |
|------|---------|---------|
| [Library 1] | [version] | [what it does] |
| [Library 2] | [version] | [what it does] |

### Development Dependencies

| Name | Version | Purpose |
|------|---------|---------|
| [TypeScript] | [version] | Type checking |
| [ESLint] | [version] | Linting |
| [Prettier] | [version] | Formatting |
| [Jest / Vitest] | [version] | Testing |

---

## Data Storage

### Databases

| Type | Technology | Version | Purpose |
|------|------------|---------|---------|
| Primary | [PostgreSQL / MySQL / MongoDB] | [version] | Main data store |
| Cache | [Redis / Memcached] | [version] | Caching layer |

### File Storage

| Type | Service | Purpose |
|------|---------|---------|
| [Local / S3 / GCS] | [service name] | [what's stored] |

---

## External Services

### Required Services

| Service | Purpose | Fallback |
|---------|---------|----------|
| [Service 1] | [why needed] | [what happens if unavailable] |

### Optional Services

| Service | Purpose | Feature Flag |
|---------|---------|--------------|
| [Service 1] | [enhancement it provides] | [how to disable] |

---

## Build & Bundling

| Tool | Version | Config File |
|------|---------|-------------|
| [Vite / Webpack / esbuild / Cargo] | [version] | `[config file]` |

---

## Dependency Management

### Adding Dependencies

```bash
# Production dependency
[npm install / cargo add] [package]

# Development dependency
[npm install -D / cargo add --dev] [package]
```

### Updating Dependencies

```bash
# Check for updates
[npm outdated / cargo outdated]

# Update all
[npm update / cargo update]
```

### Security

```bash
# Audit for vulnerabilities
[npm audit / cargo audit]
```

---
*Last updated: [DATE]*
