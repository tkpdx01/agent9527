# ThreadManager Sample

Small one-shot binary that starts a Agent9527 thread with `ThreadManager` from
`agent9527-core-api`, submits a single user turn, and prints the final assistant
message.

```sh
cargo run -p agent9527-thread-manager-sample -- "Say hello"
```

Use `--model` to override the configured default model:

```sh
cargo run -p agent9527-thread-manager-sample -- --model gpt-5.2 "Say hello"
```

The prompt can also be piped through stdin:

```sh
printf 'Say hello\n' | cargo run -p agent9527-thread-manager-sample
```
