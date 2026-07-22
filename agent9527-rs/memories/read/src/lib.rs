//! Read-path helpers for Agent9527 memories.
//!
//! This crate owns memory injection, memory citation parsing, and telemetry
//! classification for read access to the memory folder. It intentionally does
//! not depend on the memory write pipeline.

pub mod citations;
mod metrics;
pub mod usage;

use agent9527_utils_absolute_path::AbsolutePathBuf;

pub fn memory_root(agent9527_home: &AbsolutePathBuf) -> AbsolutePathBuf {
    agent9527_home.join("memories")
}
