import sys
from pathlib import Path

_EXAMPLES_ROOT = Path(__file__).resolve().parents[1]
if str(_EXAMPLES_ROOT) not in sys.path:
    sys.path.insert(0, str(_EXAMPLES_ROOT))

from _bootstrap import ensure_local_sdk_src, runtime_config

ensure_local_sdk_src()

from openai_agent9527 import Agent9527

with Agent9527(config=runtime_config()) as agent9527:
    # Browser login returns a live handle. Open `auth_url` and call `wait()`
    # in a real app; this example cancels immediately so it stays non-blocking.
    login = agent9527.login_chatgpt()
    canceled = login.cancel()
    completed = login.wait()
    account = agent9527.account()

    print("login.id:", login.login_id)
    print("login.auth_url:", login.auth_url)
    print("login.cancel.status:", canceled.status)
    print("login.completed.success:", completed.success)
    print("account.requires_openai_auth:", account.requires_openai_auth)
