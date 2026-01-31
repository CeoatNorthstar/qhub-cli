# API Testing Examples

Quick reference for testing the QHub API endpoints.

## Setup

```bash
# Set the base URL
export API_URL="http://localhost:8787"  # Development
# or
export API_URL="https://qhub-api.workers.dev"  # Production
```

## Authentication Endpoints

### Register a New User

```bash
curl -X POST $API_URL/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securePassword123",
    "username": "johndoe"
  }'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "username": "johndoe",
    "tier": "free"
  },
  "expires_at": 1704067200
}
```

### Login

```bash
curl -X POST $API_URL/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securePassword123"
  }'
```

### Verify Token

```bash
export TOKEN="your_jwt_token_here"

curl -X GET $API_URL/auth/verify \
  -H "Authorization: Bearer $TOKEN"
```

### List Active Sessions

```bash
curl -X GET $API_URL/auth/sessions \
  -H "Authorization: Bearer $TOKEN"
```

### Logout

```bash
curl -X POST $API_URL/auth/logout \
  -H "Authorization: Bearer $TOKEN"
```

### Logout from All Devices

```bash
curl -X POST $API_URL/auth/logout-all \
  -H "Authorization: Bearer $TOKEN"
```

## AI Chat Endpoints

### Send a Chat Message

```bash
curl -X POST $API_URL/ai/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "What is quantum computing?"
  }'
```

Response:
```json
{
  "response": "Quantum computing is a type of computation that harnesses...",
  "conversation_id": "conv-123abc",
  "tokens_used": 256
}
```

### Continue a Conversation

```bash
curl -X POST $API_URL/ai/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "Can you explain more about qubits?",
    "conversation_id": "conv-123abc"
  }'
```

### List All Conversations

```bash
curl -X GET $API_URL/ai/conversations \
  -H "Authorization: Bearer $TOKEN"
```

### Get a Specific Conversation

```bash
curl -X GET $API_URL/ai/conversations/conv-123abc \
  -H "Authorization: Bearer $TOKEN"
```

### Delete a Conversation

```bash
curl -X DELETE $API_URL/ai/conversations/conv-123abc \
  -H "Authorization: Bearer $TOKEN"
```

### Check AI Usage

```bash
curl -X GET $API_URL/ai/usage \
  -H "Authorization: Bearer $TOKEN"
```

Response:
```json
{
  "usage": {
    "today": 5,
    "limit": 10,
    "total_conversations": 12,
    "total_messages": 45,
    "tier": "free"
  }
}
```

## Quantum Job Endpoints

### Submit a Quantum Circuit Job

```bash
curl -X POST $API_URL/quantum/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "circuit_code": "from qiskit import QuantumCircuit\nqc = QuantumCircuit(2)\nqc.h(0)\nqc.cx(0, 1)",
    "backend": "qiskit_aer_simulator",
    "name": "Bell State Test"
  }'
```

Response:
```json
{
  "job_id": "job-456def",
  "status": "pending",
  "created_at": 1704067200
}
```

### List All Jobs

```bash
# All jobs
curl -X GET $API_URL/quantum/jobs \
  -H "Authorization: Bearer $TOKEN"

# Filter by status
curl -X GET "$API_URL/quantum/jobs?status=completed" \
  -H "Authorization: Bearer $TOKEN"

# With pagination
curl -X GET "$API_URL/quantum/jobs?limit=20&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

### Get Job Details

```bash
curl -X GET $API_URL/quantum/jobs/job-456def \
  -H "Authorization: Bearer $TOKEN"
```

### Cancel a Job

```bash
curl -X DELETE $API_URL/quantum/jobs/job-456def \
  -H "Authorization: Bearer $TOKEN"
```

### Get Job Statistics

```bash
curl -X GET $API_URL/quantum/stats \
  -H "Authorization: Bearer $TOKEN"
```

Response:
```json
{
  "stats": {
    "total": 25,
    "by_status": {
      "pending": 2,
      "running": 1,
      "completed": 20,
      "failed": 2,
      "cancelled": 0
    },
    "active": 3,
    "limit": 3,
    "recent_job": {
      "id": "job-456def",
      "name": "Bell State Test",
      "status": "completed",
      "created_at": 1704067200
    },
    "tier": "free"
  }
}
```

### Rerun a Job

```bash
curl -X POST $API_URL/quantum/jobs/job-456def/rerun \
  -H "Authorization: Bearer $TOKEN"
```

## Health & Info Endpoints

### Health Check

```bash
curl -X GET $API_URL/health
```

### API Info

```bash
curl -X GET $API_URL/
```

Response:
```json
{
  "name": "QHub API",
  "version": "1.0.0",
  "status": "operational",
  "environment": "production",
  "endpoints": {
    "auth": "/auth",
    "ai": "/ai",
    "quantum": "/quantum"
  }
}
```

## Complete Workflow Example

```bash
#!/bin/bash

# 1. Register a new user
echo "Registering user..."
REGISTER_RESPONSE=$(curl -s -X POST http://localhost:8787/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "TestPass123",
    "username": "testuser"
  }')

echo $REGISTER_RESPONSE

# 2. Extract token
TOKEN=$(echo $REGISTER_RESPONSE | jq -r '.token')
echo "Token: $TOKEN"

# 3. Send AI chat message
echo -e "\n\nSending AI chat message..."
CHAT_RESPONSE=$(curl -s -X POST http://localhost:8787/ai/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "message": "Hello, explain quantum superposition in simple terms."
  }')

echo $CHAT_RESPONSE | jq '.'

# 4. Submit quantum job
echo -e "\n\nSubmitting quantum job..."
JOB_RESPONSE=$(curl -s -X POST http://localhost:8787/quantum/submit \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "circuit_code": "qc = QuantumCircuit(2)\nqc.h(0)\nqc.cx(0,1)",
    "name": "Bell State"
  }')

echo $JOB_RESPONSE | jq '.'

JOB_ID=$(echo $JOB_RESPONSE | jq -r '.job_id')

# 5. Check job status
echo -e "\n\nChecking job status..."
curl -s -X GET "http://localhost:8787/quantum/jobs/$JOB_ID" \
  -H "Authorization: Bearer $TOKEN" | jq '.'

# 6. Check usage stats
echo -e "\n\nChecking usage stats..."
curl -s -X GET http://localhost:8787/ai/usage \
  -H "Authorization: Bearer $TOKEN" | jq '.'

echo -e "\n\nDone!"
```

## Error Response Examples

### 400 Bad Request
```json
{
  "error": "Message is required"
}
```

### 401 Unauthorized
```json
{
  "error": "Invalid or expired token"
}
```

### 404 Not Found
```json
{
  "error": "Job not found"
}
```

### 429 Too Many Requests
```json
{
  "error": "Daily usage limit reached (10 messages). Upgrade your plan for more.",
  "usage_limit": 10,
  "current_usage": 10
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error"
}
```

## Tips

1. **Save your token**: After registering or logging in, save the token to an environment variable for easier testing.

2. **Use jq for pretty output**: Pipe responses through `jq '.'` for formatted JSON output.

3. **Test pagination**: Try different `limit` and `offset` values to test pagination.

4. **Test error cases**: Try invalid tokens, missing fields, etc. to ensure error handling works.

5. **Monitor usage**: Regularly check `/ai/usage` and `/quantum/stats` to track your usage.

6. **Session management**: Test creating multiple sessions and logging out from specific devices.
