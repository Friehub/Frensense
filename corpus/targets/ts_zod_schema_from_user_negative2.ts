// SAFE: user input is validated against an enum-constrained schema before mapping to fixed types

import { z } from 'zod';

const AllowedFieldType = z.enum(['text', 'number', 'email']);
const UserFieldConfigSchema = z.object({
  key: z.string(),
  type: AllowedFieldType,
});

function buildFieldSchema(config: z.infer<typeof UserFieldConfigSchema>) {
  switch (config.type) {
    case 'text': return z.object({ [config.key]: z.string() });
    case 'number': return z.object({ [config.key]: z.number() });
    case 'email': return z.object({ [config.key]: z.string().email() });
  }
}

function handleRequest(userConfig: string) {
  const config = UserFieldConfigSchema.parse(JSON.parse(userConfig));
  return buildFieldSchema(config);
}
