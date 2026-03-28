<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# Development Guide

## Prerequisites

| Requirement | Version | Check Command |
|-------------|---------|---------------|
| [Node.js] | [>= 20.x] | `node --version` |
| [npm] | [>= 10.x] | `npm --version` |
| [Docker] | [optional] | `docker --version` |

---

## Initial Setup

### 1. Clone Repository

```bash
git clone [repository-url]
cd [project-name]
```

### 2. Install Dependencies

```bash
[npm install]
# or
[yarn install]
# or
[pnpm install]
```

### 3. Configure Environment

```bash
# Copy example environment file
cp .env.example .env

# Edit with your values
[vim / code] .env
```

### 4. Setup Database (if applicable)

```bash
# Start database
[docker-compose up -d db]

# Run migrations
[npm run db:migrate]

# Seed data (optional)
[npm run db:seed]
```

### 5. Verify Setup

```bash
# Run tests
[npm test]

# Start development server
[npm run dev]
```

---

## Environment Variables

### Required

| Variable | Description | Example |
|----------|-------------|---------|
| `[DATABASE_URL]` | Database connection string | `postgres://user:pass@localhost:5432/db` |
| `[JWT_SECRET]` | Secret for JWT signing | `your-secret-key` |

### Optional

| Variable | Description | Default |
|----------|-------------|---------|
| `[PORT]` | Server port | `3000` |
| `[LOG_LEVEL]` | Logging verbosity | `info` |
| `[NODE_ENV]` | Environment mode | `development` |

### Environment Files

| File | Purpose | Git |
|------|---------|-----|
| `.env` | Local development | ❌ Ignored |
| `.env.example` | Template with all vars | ✅ Tracked |
| `.env.test` | Test environment | ❌ Ignored |
| `.env.production` | Production (CI only) | ❌ Ignored |

---

## Scripts Reference

### Development

| Command | Description |
|---------|-------------|
| `[npm run dev]` | Start development server with hot reload |
| `[npm run build]` | Build for production |
| `[npm start]` | Start production server |
| `[npm run watch]` | Watch and rebuild on changes |

### Testing

| Command | Description |
|---------|-------------|
| `[npm test]` | Run all tests |
| `[npm run test:watch]` | Run tests in watch mode |
| `[npm run test:coverage]` | Generate coverage report |

### Code Quality

| Command | Description |
|---------|-------------|
| `[npm run lint]` | Check for linting errors |
| `[npm run lint:fix]` | Auto-fix linting errors |
| `[npm run format]` | Format code with Prettier |
| `[npm run typecheck]` | Run TypeScript checks |

### Database

| Command | Description |
|---------|-------------|
| `[npm run db:migrate]` | Run pending migrations |
| `[npm run db:rollback]` | Rollback last migration |
| `[npm run db:seed]` | Seed database with data |
| `[npm run db:reset]` | Reset and re-seed database |

### Utilities

| Command | Description |
|---------|-------------|
| `[npm run clean]` | Remove build artifacts |
| `[npm run docs]` | Generate API documentation |

---

## Development Workflow

### Starting Work

```bash
# 1. Update main branch
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Start development server
[npm run dev]
```

### Making Changes

```bash
# 1. Make your changes

# 2. Run tests
[npm test]

# 3. Check linting
[npm run lint]

# 4. Commit changes
git add .
git commit -m "feat: description"
```

### Submitting PR

```bash
# 1. Push branch
git push origin feature/my-feature

# 2. Create PR on GitHub/GitLab
# 3. Wait for CI checks
# 4. Request review
```

---

## Debugging

### VS Code

Launch configurations in `.vscode/launch.json`:

```json
{
  "configurations": [
    {
      "name": "Debug Server",
      "type": "node",
      "request": "launch",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"]
    }
  ]
}
```

### Logging

```typescript
// Use structured logging
import { logger } from './logger';

logger.info('Operation completed', { userId, action });
logger.error('Operation failed', { error, context });
```

### Common Issues

| Issue | Solution |
|-------|----------|
| Port in use | `lsof -i :3000` then `kill -9 <PID>` |
| Node modules issues | `rm -rf node_modules && npm install` |
| Type errors | `npm run typecheck` to see details |

---

## Docker (if applicable)

### Development

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f [service]

# Stop all services
docker-compose down
```

### Building

```bash
# Build image
docker build -t [image-name] .

# Run container
docker run -p 3000:3000 [image-name]
```

---
*Last updated: [DATE]*
