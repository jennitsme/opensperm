import { strict as assert } from "node:assert";
import { ManifestSchema } from "./manifest";
import { SkillBundle } from "./tool";

export interface TranscriptStep {
  request: { tool: string; input: unknown };
  response: { output: unknown };
}

export interface Transcript {
  name: string;
  steps: TranscriptStep[];
}

export async function runContractTest(bundle: SkillBundle, transcript: Transcript) {
  ManifestSchema.parse(bundle.manifest);
  assert.ok(bundle.tools.length > 0, "no tools defined");

  const toolMap = new Map(bundle.tools.map((t) => [t.name, t]));

  for (const step of transcript.steps) {
    const tool = toolMap.get(step.request.tool);
    assert.ok(tool, `tool ${step.request.tool} not found`);
    const parsedInput = tool.inputSchema.parse(step.request.input);
    const output = await tool.handler(parsedInput as any);
    const parsedOutput = tool.outputSchema.parse(output);
    assert.deepEqual(parsedOutput, step.response.output, "output mismatch");
  }
}
