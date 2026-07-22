"""Command-line interface for building Agent9527 package directories."""

import argparse
import tempfile
from pathlib import Path

from .archive import write_archive
from .cargo import build_source_binaries
from .layout import build_package_dir
from .layout import prepare_package_dir
from .layout import validate_package_dir
from .ripgrep import resolve_rg_bin
from .targets import PACKAGE_VARIANTS
from .targets import TARGET_SPECS
from .targets import PackageInputs
from .targets import default_target
from .targets import resolve_input_path
from .zsh import resolve_zsh_bin
from .version import read_workspace_version


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build a canonical Agent9527 package directory and optional archive.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "--target",
        default=argparse.SUPPRESS,
        choices=sorted(TARGET_SPECS),
        help=(
            "Rust target triple for the package. Defaults to the release target "
            "for this host platform."
        ),
    )
    parser.add_argument(
        "--variant",
        choices=sorted(PACKAGE_VARIANTS),
        default="agent9527",
        help="Package variant to build.",
    )
    parser.add_argument(
        "--package-dir",
        type=Path,
        default=argparse.SUPPRESS,
        help=(
            "Output directory to create as the package root. Defaults to a new temporary directory."
        ),
    )
    parser.add_argument(
        "--archive-output",
        type=Path,
        action="append",
        default=[],
        help=(
            "Optional archive output path. May be repeated. Supported suffixes: "
            ".tar.gz, .tgz, .tar.zst, .zip."
        ),
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Replace an existing package directory or archive output.",
    )
    parser.add_argument(
        "--cargo",
        default="cargo",
        help="Cargo executable to use for source-built package artifacts.",
    )
    parser.add_argument(
        "--cargo-profile",
        default="dev-small",
        help=(
            "Cargo profile for source-built package artifacts. Use release for release packages."
        ),
    )
    parser.add_argument(
        "--entrypoint-bin",
        type=Path,
        help=(
            "Optional prebuilt entrypoint executable for the selected package "
            "variant. If omitted, the entrypoint is built with Cargo."
        ),
    )
    parser.add_argument(
        "--code-mode-host-bin",
        type=Path,
        help=(
            "Optional prebuilt agent9527-code-mode-host executable. If omitted, "
            "the host is built with Cargo."
        ),
    )
    parser.add_argument(
        "--bwrap-bin",
        type=Path,
        help=(
            "Optional prebuilt Linux bwrap executable. If omitted for Linux "
            "targets, bwrap is built with Cargo."
        ),
    )
    parser.add_argument(
        "--zsh-manifest",
        type=Path,
        help=(
            "Optional DotSlash manifest for the patched zsh fork instead of "
            "scripts/agent9527_package/agent9527-zsh."
        ),
    )
    parser.add_argument(
        "--agent9527-command-runner-bin",
        type=Path,
        help=(
            "Optional prebuilt Windows agent9527-command-runner.exe executable. "
            "If omitted for Windows targets, agent9527-command-runner is built "
            "with Cargo."
        ),
    )
    parser.add_argument(
        "--agent9527-windows-sandbox-setup-bin",
        type=Path,
        help=(
            "Optional prebuilt Windows agent9527-windows-sandbox-setup.exe "
            "executable. If omitted for Windows targets, "
            "agent9527-windows-sandbox-setup is built with Cargo."
        ),
    )
    parser.add_argument(
        "--rg-bin",
        type=Path,
        help=(
            "Optional local ripgrep executable override instead of fetching from "
            "scripts/agent9527_package/rg."
        ),
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    spec = TARGET_SPECS[getattr(args, "target", None) or default_target()]
    variant = PACKAGE_VARIANTS[args.variant]
    package_dir_arg = getattr(args, "package_dir", None)
    package_dir = (
        package_dir_arg.resolve()
        if package_dir_arg is not None
        else Path(tempfile.mkdtemp(prefix="agent9527-package-")).resolve()
    )

    source_outputs = build_source_binaries(
        spec,
        variant,
        cargo=args.cargo,
        profile=args.cargo_profile,
        entrypoint_bin=resolve_optional_input_path(
            args.entrypoint_bin,
            "prebuilt entrypoint executable",
            "--entrypoint-bin",
        ),
        code_mode_host_bin=resolve_optional_input_path(
            args.code_mode_host_bin,
            "prebuilt code-mode host executable",
            "--code-mode-host-bin",
        ),
        bwrap_bin=resolve_optional_input_path(
            args.bwrap_bin,
            "prebuilt Linux bwrap executable",
            "--bwrap-bin",
        ),
        agent9527_command_runner_bin=resolve_optional_input_path(
            args.agent9527_command_runner_bin,
            "prebuilt Windows agent9527-command-runner.exe executable",
            "--agent9527-command-runner-bin",
        ),
        agent9527_windows_sandbox_setup_bin=resolve_optional_input_path(
            args.agent9527_windows_sandbox_setup_bin,
            "prebuilt Windows agent9527-windows-sandbox-setup.exe executable",
            "--agent9527-windows-sandbox-setup-bin",
        ),
    )
    version = read_workspace_version()
    inputs = PackageInputs(
        entrypoint_bin=source_outputs.entrypoint_bin,
        code_mode_host_bin=source_outputs.code_mode_host_bin,
        rg_bin=resolve_rg_bin(spec, args.rg_bin),
        zsh_bin=resolve_zsh_bin(spec, args.zsh_manifest),
        bwrap_bin=source_outputs.bwrap_bin,
        agent9527_command_runner_bin=source_outputs.agent9527_command_runner_bin,
        agent9527_windows_sandbox_setup_bin=source_outputs.agent9527_windows_sandbox_setup_bin,
    )
    prepare_package_dir(package_dir, force=args.force)
    build_package_dir(package_dir, version, variant, spec, inputs)
    validate_package_dir(
        package_dir, variant, spec, include_zsh=inputs.zsh_bin is not None
    )

    for archive_output in args.archive_output:
        archive_path = archive_output.resolve()
        write_archive(package_dir, archive_path, force=args.force)
        print(f"Built Agent9527 package archive at {archive_path}")

    print(f"Built Agent9527 package directory at {package_dir}")
    return 0


def resolve_optional_input_path(
    explicit_path: Path | None,
    description: str,
    flag_name: str,
) -> Path | None:
    if explicit_path is None:
        return None

    return resolve_input_path(explicit_path, description, flag_name)
