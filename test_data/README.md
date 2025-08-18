# Test Data

This directory contains sample JSONL files for testing the OpenAI Batch API with yara-x related questions.

## Files

### `yara_x_questions.jsonl`
A comprehensive batch request file containing 10 questions about yara-x:
- Basic differences between YARA and yara-x
- Syntax and performance improvements
- Module and integration capabilities
- Advanced features like regex, string matching, and conditions
- Best practices for rule creation and metadata

### `simple_batch.jsonl`
A minimal batch request file with 3 simple questions, useful for testing and demonstration.

## Usage

These files can be uploaded to OpenAI's Files API and used as input for batch processing:

```bash
# Upload the file
curl -X POST https://api.openai.com/v1/files \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -F purpose="batch" \
  -F file="@test_data/yara_x_questions.jsonl"

# Create a batch job
curl -X POST https://api.openai.com/v1/batches \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "input_file_id": "file-abc123",
    "endpoint": "/v1/chat/completions",
    "completion_window": "24h"
  }'
```

## File Format

Each line in the JSONL files represents a single request with:
- `custom_id`: Unique identifier for the request
- `method`: HTTP method (always "POST" for chat completions)
- `url`: API endpoint ("/v1/chat/completions")
- `body`: Request body containing model, messages, and parameters

## Expected Response Format

When the batch completes, responses will be in JSONL format with:
- `custom_id`: Matching the request identifier
- `response`: Contains status_code, request_id, and response body
- `error`: Present if the request failed