import { strict as assert } from "node:assert";
import { ManifestSchema } from "./manifest";
import { SkillBundle } from "./tool";

export interface TranscriptStep {
  request: unknown;
  response: unknown;
}

export interface Transcript {
  name: string;
  steps: TranscriptStep[];
}

export function runContractTest(bundle: SkillBundle, transcript: Transcript) {
  // Validate manifest shape first
  ManifestSchema.parse(bundle.manifest);

  // TODO: execute against runtime shim; for now, basic shape assertion
  for (const step of transcript.steps) {
    assert.ok(step.request, "request present");
    assert.ok(step.response, "response present");
  }
}
