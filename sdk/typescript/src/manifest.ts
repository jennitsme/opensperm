import { z } from "zod";

export const CapabilitySchema = z.object({
  network: z
    .object({ egress: z.array(z.string().url()).optional() })
    .optional(),
  filesystem: z
    .object({ paths: z.array(z.string()), mode: z.enum(["ro", "rw"]).optional() })
    .optional(),
  secrets: z.array(z.string()).optional(),
});

export const ManifestSchema = z.object({
  name: z.string().min(1),
  description: z.string().optional(),
  version: z.string().regex(/\d+\.\d+\.\d+(?:-[A-Za-z0-9.-]+)?/),
  language: z.enum(["typescript", "rust"]),
  entry: z.string(),
  inputs: z.record(z.string(), z.object({
    type: z.string(),
    description: z.string().optional(),
    required: z.boolean().default(true),
  })),
  outputs: z.record(z.string(), z.object({
    type: z.string(),
    description: z.string().optional(),
  })),
  capabilities: CapabilitySchema.default({}),
  policyScopes: z.array(z.string()).optional(),
});

export type Manifest = z.infer<typeof ManifestSchema>;
