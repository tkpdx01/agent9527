# agent9527-client

Higher-level request policy layered on `agent9527-http-client` without any Agent9527/OpenAI API awareness.

- Provides retry utilities (`RetryPolicy`, `RetryOn`, `run_with_retry`, `backoff`) that callers plug into for unary and streaming calls.
- Supplies the `sse_stream` helper to turn byte streams into raw SSE `data:` frames with idle timeouts and surfaced stream errors.
- Defines the request telemetry callback used by higher-level clients.
- Re-exports the low-level HTTP types temporarily so consumers can migrate to `agent9527-http-client` incrementally.
