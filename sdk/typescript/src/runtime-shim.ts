import { SkillBundle } from "./tool";
import { runContractTest, Transcript } from "./contract-test";

// Executes transcript against in-memory bundle by calling handlers.
export async function runTranscriptWithBundle(bundle: SkillBundle, transcript: Transcript) {
  return runContractTest(bundle, transcript);
}
