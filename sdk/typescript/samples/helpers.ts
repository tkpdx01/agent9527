import path from "node:path";

export function agent9527PathOverride() {
  return (
    process.env.AGENT9527_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "agent9527-rs", "target", "debug", "agent9527")
  );
}
