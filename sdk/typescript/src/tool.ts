import { z } from "zod";
import { Manifest, ManifestSchema } from "./manifest";

export type ToolHandler<I, O> = (input: I) => Promise<O>;

export interface ToolDefinition<I, O> {
  name: string;
  inputSchema: z.ZodType<I>;
  outputSchema: z.ZodType<O>;
  handler: ToolHandler<I, O>;
}

export interface SkillBundle {
  manifest: Manifest;
  tools: ToolDefinition<unknown, unknown>[];
}

export function defineSkill(bundle: SkillBundle) {
  ManifestSchema.parse(bundle.manifest);
  // TODO: validate tool schemas align with manifest inputs/outputs
  return bundle;
}
