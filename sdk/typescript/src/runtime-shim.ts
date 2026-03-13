import { SkillBundle } from "./tool";
import { runContractTest, Transcript } from "./contract-test";

// Placeholder shim: directly uses contract test runner for now.
export async function runTranscriptWithBundle(bundle: SkillBundle, transcript: Transcript) {
  return runContractTest(bundle, transcript);
}
