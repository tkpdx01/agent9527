#!/usr/bin/env -S NODE_NO_WARNINGS=1 pnpm ts-node-esm --files

import { Agent9527 } from "@tkpdx01/agent9527-sdk";

import { agent9527PathOverride } from "./helpers.ts";

const agent9527 = new Agent9527({ agent9527PathOverride: agent9527PathOverride() });

const thread = agent9527.startThread();

const schema = {
  type: "object",
  properties: {
    summary: { type: "string" },
    status: { type: "string", enum: ["ok", "action_required"] },
  },
  required: ["summary", "status"],
  additionalProperties: false,
} as const;

const turn = await thread.run("Summarize repository status", { outputSchema: schema });
console.log(turn.finalResponse);
