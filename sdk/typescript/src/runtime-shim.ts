import { SkillBundle } from "./tool";
import { runContractTest, Transcript } from "./contract-test";

export async function runTranscriptWithBundle(bundle: SkillBundle, transcript: Transcript) {
  return runContractTest(bundle, transcript);
}
