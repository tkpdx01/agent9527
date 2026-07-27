# agent9527-protocol

This crate defines the "types" for the protocol used by Agent9527 CLI, which includes both "internal types" for communication between `agent9527-core` and `agent9527-tui`, as well as "external types" used with `agent9527 app-server`.

This crate should have minimal dependencies.

Ideally, we should avoid "material business logic" in this crate, as we can always introduce `Ext`-style traits to add functionality to types in other crates.
