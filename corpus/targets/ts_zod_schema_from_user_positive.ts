// [frensense]
// observation: A Zod schema is constructed dynamically using user-provided input, allowing the user to influence the validation rules.
// impact: An attacker can inject arbitrary schema definitions through user input, potentially bypassing validation, causing Denial of Service via complex schemas, or leaking schema structure information.
// improvement: Use fixed, pre-defined schemas that reference a user-provided config object for parameters, rather than building schemas from raw user input.

import { z } from 'zod';

function buildSchemaFromConfig(config: Record<string, unknown>) {
  const shape: Record<string, z.ZodTypeAny> = {};
  for (const [key, type] of Object.entries(config)) {
    if (type === 'string') shape[key] = z.string();
    else if (type === 'number') shape[key] = z.number();
    else shape[key] = z.any();
  }
  return z.object(shape);
}

function handleRequest(userConfig: string) {
  const parsed = JSON.parse(userConfig);
  const schema = buildSchemaFromConfig(parsed);
  return schema;
}
