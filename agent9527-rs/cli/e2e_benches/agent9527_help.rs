#![allow(clippy::expect_used)]

use std::process::Command;

use divan::Bencher;

fn main() {
    divan::main();
}

/// Exercises the Bazel-backed end-to-end benchmark path with a cheap,
/// deterministic Agent9527 invocation. Richer scenarios can add separate
/// benchmark binaries without making the shared harness depend on them.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn agent9527_help(bencher: Bencher) {
    let agent9527 = agent9527_utils_cargo_bin::cargo_bin("agent9527")
        .expect("agent9527 binary should be available through Bazel runfiles");

    bencher.bench_local(move || {
        let output = Command::new(&agent9527)
            .arg("--help")
            .output()
            .expect("agent9527 --help should run");
        assert!(output.status.success(), "agent9527 --help should succeed");
    });
}
