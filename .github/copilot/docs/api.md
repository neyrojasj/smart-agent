<!-- TEMPLATE: Replace this file with real project information. Run the Setup skill (@smart setup project) to auto-generate. -->

# API Documentation

> Skip this file if your project doesn't expose APIs

## Overview

| Attribute | Value |
|-----------|-------|
| Type | [REST / GraphQL / gRPC / WebSocket] |
| Base URL | `[/api/v1 or similar]` |
| Authentication | [JWT / API Key / OAuth / None] |
| Format | [JSON / XML / Protobuf] |

---

## Authentication

### Method

[Describe the authentication method used]

### Headers

```
Authorization: Bearer <token>
X-API-Key: <key>
```

### Obtaining Credentials

[How to get API credentials for development/testing]

---

## Endpoints

### [Resource 1 - e.g., Users]

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| GET | `/[resource]` | List all | [Yes/No] |
| GET | `/[resource]/:id` | Get one | [Yes/No] |
| POST | `/[resource]` | Create | [Yes/No] |
| PUT | `/[resource]/:id` | Update | [Yes/No] |
| DELETE | `/[resource]/:id` | Delete | [Yes/No] |

#### Request/Response Examples

**GET /[resource]**
```json
// Response 200
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 100
  }
}
```

**POST /[resource]**
```json
// Request
{
  "field1": "value",
  "field2": "value"
}

// Response 201
{
  "id": "...",
  "field1": "value",
  "field2": "value",
  "createdAt": "..."
}
```

---

### [Resource 2]

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| ... | ... | ... | ... |

---

## Error Handling

### Error Format

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "details": {}
  }
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Invalid input |
| `UNAUTHORIZED` | 401 | Missing/invalid auth |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Rate Limiting

| Tier | Limit | Window |
|------|-------|--------|
| [Default] | [100 requests] | [per minute] |
| [Authenticated] | [1000 requests] | [per minute] |

**Headers returned:**
- `X-RateLimit-Limit`: Max requests
- `X-RateLimit-Remaining`: Requests left
- `X-RateLimit-Reset`: Reset timestamp

---

## Webhooks (if applicable)

### Events

| Event | Trigger | Payload |
|-------|---------|---------|
| `[event.created]` | [When X happens] | [Link to schema] |

### Verification

[How to verify webhook signatures]

---

## SDKs & Clients

| Language | Package | Install |
|----------|---------|---------|
| [JavaScript] | [@org/sdk] | `npm install @org/sdk` |

---
*Last updated: [DATE]*
