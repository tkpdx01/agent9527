import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalAgent9527Home = process.env.AGENT9527_HOME;
let currentAgent9527Home: string | undefined;

beforeEach(async () => {
  currentAgent9527Home = await fs.mkdtemp(path.join(os.tmpdir(), "agent9527-sdk-test-"));
  process.env.AGENT9527_HOME = currentAgent9527Home;
});

afterEach(async () => {
  const agent9527HomeToDelete = currentAgent9527Home;
  currentAgent9527Home = undefined;

  if (originalAgent9527Home === undefined) {
    delete process.env.AGENT9527_HOME;
  } else {
    process.env.AGENT9527_HOME = originalAgent9527Home;
  }

  if (agent9527HomeToDelete) {
    await fs.rm(agent9527HomeToDelete, { recursive: true, force: true });
  }
});
