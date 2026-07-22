#!/usr/bin/env node
// Unified entry point for the Agent9527 CLI.

import { spawn } from "node:child_process";
import { existsSync, realpathSync } from "fs";
import { createRequire } from "node:module";
import path from "path";
import { fileURLToPath } from "url";

import { prepareThirdPartyApiLaunch } from "../lib/product-policy.js";

// __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const require = createRequire(import.meta.url);
const agent9527PackageRoot = realpathSync(path.join(__dirname, ".."));

const PLATFORM_PACKAGE_BY_TARGET = {
  "x86_64-unknown-linux-musl": "@tkpdx01/agent9527-linux-x64",
  "aarch64-unknown-linux-musl": "@tkpdx01/agent9527-linux-arm64",
  "x86_64-apple-darwin": "@tkpdx01/agent9527-darwin-x64",
  "aarch64-apple-darwin": "@tkpdx01/agent9527-darwin-arm64",
  "x86_64-pc-windows-msvc": "@tkpdx01/agent9527-win32-x64",
  "aarch64-pc-windows-msvc": "@tkpdx01/agent9527-win32-arm64",
};

const { platform, arch } = process;

let targetTriple = null;
switch (platform) {
  case "linux":
  case "android":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-unknown-linux-musl";
        break;
      case "arm64":
        targetTriple = "aarch64-unknown-linux-musl";
        break;
      default:
        break;
    }
    break;
  case "darwin":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-apple-darwin";
        break;
      case "arm64":
        targetTriple = "aarch64-apple-darwin";
        break;
      default:
        break;
    }
    break;
  case "win32":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-pc-windows-msvc";
        break;
      case "arm64":
        targetTriple = "aarch64-pc-windows-msvc";
        break;
      default:
        break;
    }
    break;
  default:
    break;
}

if (!targetTriple) {
  throw new Error(`Unsupported platform: ${platform} (${arch})`);
}

const platformPackage = PLATFORM_PACKAGE_BY_TARGET[targetTriple];
if (!platformPackage) {
  throw new Error(`Unsupported target triple: ${targetTriple}`);
}

function findAgent9527Executable() {
  let vendorRoot;
  try {
    const packageJsonPath = require.resolve(`${platformPackage}/package.json`);
    vendorRoot = path.join(path.dirname(packageJsonPath), "vendor");
  } catch {
    vendorRoot = path.join(__dirname, "..", "vendor");
  }

  const agent9527Executable = path.join(
    vendorRoot,
    targetTriple,
    "bin",
    process.platform === "win32" ? "agent9527.exe" : "agent9527",
  );
  if (existsSync(agent9527Executable)) {
    return agent9527Executable;
  }

  const packageManager = detectPackageManager();
  const updateCommand =
    packageManager === "bun"
      ? "bun install -g @tkpdx01/agent9527@latest"
      : packageManager === "pnpm"
        ? "pnpm add -g @tkpdx01/agent9527@latest"
        : "npm install -g @tkpdx01/agent9527@latest";
  throw new Error(
    `Missing optional dependency ${platformPackage}. Reinstall Agent9527: ${updateCommand}`,
  );
}

const binaryPath = findAgent9527Executable();

// Use an asynchronous spawn instead of spawnSync so that Node is able to
// respond to signals (e.g. Ctrl-C / SIGINT) while the native binary is
// executing. This allows us to forward those signals to the child process
// and guarantees that when either the child terminates or the parent
// receives a fatal signal, both processes exit in a predictable manner.

function isPnpmOwnedAgent9527Install(nodeModulesDir) {
  if (!existsSync(path.join(nodeModulesDir, ".modules.yaml"))) {
    return false;
  }

  try {
    return (
      realpathSync(path.join(nodeModulesDir, "@tkpdx01", "agent9527")) ===
      agent9527PackageRoot
    );
  } catch {
    return false;
  }
}

/**
 * Use heuristics to detect the package manager that was used to install Agent9527
 * in order to give the user a hint about how to update it.
 */
function detectPackageManager() {
  // pnpm's owning node_modules directory can be several parents above the
  // package in isolated global layouts. Search ancestors of both the canonical
  // package root and lexical entrypoint because pnpm may link either path.
  const entrypointDir = path.dirname(path.resolve(process.argv[1]));
  for (const startDir of new Set([agent9527PackageRoot, entrypointDir])) {
    const filesystemRoot = path.parse(startDir).root;
    for (
      let currentDir = startDir;
      currentDir !== filesystemRoot;
      currentDir = path.dirname(currentDir)
    ) {
      if (isPnpmOwnedAgent9527Install(path.join(currentDir, "node_modules"))) {
        return "pnpm";
      }
    }

    if (isPnpmOwnedAgent9527Install(path.join(filesystemRoot, "node_modules"))) {
      return "pnpm";
    }
  }

  const userAgent = process.env.npm_config_user_agent || "";
  if (/\bbun\//.test(userAgent)) {
    return "bun";
  }

  const execPath = process.env.npm_execpath || "";
  if (execPath.includes("bun")) {
    return "bun";
  }

  if (
    __dirname.includes(".bun/install/global") ||
    __dirname.includes(".bun\\install\\global")
  ) {
    return "bun";
  }

  return userAgent ? "npm" : null;
}

const packageManager = detectPackageManager();
const packageManagerEnvVar =
  packageManager === "bun"
    ? "AGENT9527_MANAGED_BY_BUN"
    : packageManager === "pnpm"
      ? "AGENT9527_MANAGED_BY_PNPM"
      : "AGENT9527_MANAGED_BY_NPM";
const env = {
  ...process.env,
  AGENT9527_MANAGED_PACKAGE_ROOT: agent9527PackageRoot,
};
const bundledLanguagePackRoot = path.join(agent9527PackageRoot, "languages");
if (!env.AGENT9527_LANGUAGE_PACK_ROOT && existsSync(bundledLanguagePackRoot)) {
  env.AGENT9527_LANGUAGE_PACK_ROOT = bundledLanguagePackRoot;
}
if (!env.AGENT9527_SYSTEM_LOCALE) {
  try {
    env.AGENT9527_SYSTEM_LOCALE = Intl.DateTimeFormat().resolvedOptions().locale;
  } catch {
    // The Rust runtime still checks LC_ALL, LC_MESSAGES, and LANG.
  }
}
delete env.AGENT9527_MANAGED_BY_NPM;
delete env.AGENT9527_MANAGED_BY_BUN;
delete env.AGENT9527_MANAGED_BY_PNPM;
env[packageManagerEnvVar] = "1";

let launch;
try {
  launch = prepareThirdPartyApiLaunch(process.argv.slice(2), env);
} catch (error) {
  // eslint-disable-next-line no-console
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(2);
}

const child = spawn(binaryPath, launch.args, {
  stdio: "inherit",
  env: launch.env,
});

child.on("error", (err) => {
  // Typically triggered when the binary is missing or not executable.
  // Re-throwing here will terminate the parent with a non-zero exit code
  // while still printing a helpful stack trace.
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});

// Forward common termination signals to the child so that it shuts down
// gracefully. In the handler we temporarily disable the default behavior of
// exiting immediately; once the child has been signaled we simply wait for
// its exit event which will in turn terminate the parent (see below).
const forwardSignal = (signal) => {
  if (child.killed) {
    return;
  }
  try {
    child.kill(signal);
  } catch {
    /* ignore */
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((sig) => {
  process.on(sig, () => forwardSignal(sig));
});

// When the child exits, mirror its termination reason in the parent so that
// shell scripts and other tooling observe the correct exit status.
// Wrap the lifetime of the child process in a Promise so that we can await
// its termination in a structured way. The Promise resolves with an object
// describing how the child exited: either via exit code or due to a signal.
const childResult = await new Promise((resolve) => {
  child.on("exit", (code, signal) => {
    if (signal) {
      resolve({ type: "signal", signal });
    } else {
      resolve({ type: "code", exitCode: code ?? 1 });
    }
  });
});

if (childResult.type === "signal") {
  // Re-emit the same signal so that the parent terminates with the expected
  // semantics (this also sets the correct exit code of 128 + n).
  process.kill(process.pid, childResult.signal);
} else {
  process.exit(childResult.exitCode);
}
