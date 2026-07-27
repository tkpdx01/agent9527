# OpenAI Agent9527 Python SDK

Build Python applications that start Agent9527 threads, run turns, stream progress,
and control workspace access.

## Install

Install the SDK:

```bash
pip install openai-agent9527
```

## Quickstart

The SDK reuses your existing Agent9527 authentication when one is already
available:

```python
from openai_agent9527 import Agent9527

with Agent9527() as agent9527:
    thread = agent9527.thread_start()
    result = thread.run("Explain this repository in three bullets.")
    print(result.final_response)
```

`thread.run(...)` returns a `TurnResult` containing the final response,
collected items, and token usage.

## Authentication

Existing Agent9527 authentication is reused automatically. To start ChatGPT
browser login explicitly:

```python
from openai_agent9527 import Agent9527

with Agent9527() as agent9527:
    login = agent9527.login_chatgpt()
    print(login.auth_url)
    print(login.wait().success)
```

For device-code login:

```python
with Agent9527() as agent9527:
    login = agent9527.login_chatgpt_device_code()
    print(login.verification_url, login.user_code)
    login.wait()
```

For API-key login:

```python
with Agent9527() as agent9527:
    agent9527.login_api_key("sk-...")
```

## Built-In Help

Use Python's standard `help(openai_agent9527)`, `help(Agent9527)`, or
`python -m pydoc openai_agent9527` documentation tools.

## Documentation

- [Getting started](https://github.com/openai/agent9527/blob/main/sdk/python/docs/getting-started.md)
- [API reference](https://github.com/openai/agent9527/blob/main/sdk/python/docs/api-reference.md)
- [FAQ](https://github.com/openai/agent9527/blob/main/sdk/python/docs/faq.md)
- [Examples](https://github.com/openai/agent9527/blob/main/sdk/python/examples/README.md)

The package is licensed under the
[repository Apache License 2.0](https://github.com/openai/agent9527/blob/main/LICENSE).
