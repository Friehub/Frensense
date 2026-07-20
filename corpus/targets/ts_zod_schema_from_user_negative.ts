// SAFE: schema is pre-defined; user config only provides allowed parameter values

import { z } from 'zod';

const FieldConfigSchema = z.object({
  label: z.string(),
  type: z.enum(['string', 'number', 'email']),
  required: z.boolean().optional(),
});

const FormSchema = z.object({
  fields: z.array(FieldConfigSchema),
});

function handleRequest(userConfig: string) {
  const config = FormSchema.parse(JSON.parse(userConfig));
  return config;
}
