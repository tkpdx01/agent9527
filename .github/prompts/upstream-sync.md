# Agent9527 upstream integration task

Integrate the fetched `upstream/main` OpenAI Codex changes into the current Agent9527 worktree.
The workflow has already attempted the Git merge, so the worktree may contain unresolved conflicts or
a clean but not-yet-committed merge.

Required outcome:

1. Preserve every applicable upstream behavior and bug fix.
2. Resolve all merge conflicts and leave no unmerged paths.
3. Keep Agent9527 branding, executable names, crate names, environment variables, package names, and
   filesystem paths. New upstream `codex-*` product identifiers normally need the corresponding
   `agent9527-*` adaptation.
4. Preserve intentional references to the real upstream project, including
   `https://github.com/openai/codex`, upstream release downloads, and the `@openai/codex` package used
   to install the sync agent. Do not rewrite those references to nonexistent Agent9527 resources.
5. Preserve the `@tkpdx01/agent9527` npm packaging and OpenAI-compatible third-party API support.
6. Follow `AGENTS.md`, including formatting, Rust lint conventions, module size guidance, and tests.
7. Run proportionate checks and fix failures. At minimum, make the Agent9527 CLI compile and keep the
   repository formatting clean. When changes touch platform-specific code, compile-check the affected
   target or binary so Linux-only validation does not hide macOS or Windows failures.

Safety and workflow invariants:

- Do not read or print secrets or credentials.
- Do not push, tag, create releases, or publish packages. The outer workflow owns those operations.
- Do not modify `.github/workflows/upstream-sync.yml`,
  `.github/workflows/agent9527-publish.yml`, `scripts/finalize_upstream_sync.py`, or this prompt.
- Do not change the release version or `.github/upstream.json`; the outer workflow does that after
  your integration succeeds.
- Do not abort the merge or reset away either side's changes.
- Finish with the desired changes in the worktree and index, without creating a commit.

## Fork automation policy

- Keep .github/dependabot.yaml absent in Agent9527. Dependency updates arrive through upstream sync; running Dependabot in this fork creates duplicate pull requests.
