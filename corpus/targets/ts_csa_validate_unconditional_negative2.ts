// SAFE: Uses zod schema validation which naturally returns errors for invalid fields
import { z } from "zod";

const CredentialsSchema = z.object({
  username: z.string().min(3, "Username must be at least 3 characters"),
  password: z.string().min(8, "Password must be at least 8 characters").regex(/[A-Z]/, "Must contain uppercase").regex(/[0-9]/, "Must contain number"),
  mfaCode: z.string().length(6).optional(),
});

function validateCredentials(input: unknown) {
  const result = CredentialsSchema.safeParse(input);
  return { valid: result.success, errors: result.success ? [] : result.error.issues.map(i => i.message) };
}

function validateApiKey(key: string): boolean {
  const ApiKeySchema = z.string().min(20).regex(/^(sk|pk)_live_/);
  return ApiKeySchema.safeParse(key).success;
}
