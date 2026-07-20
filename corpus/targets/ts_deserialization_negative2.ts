// SAFE: Uses zod to parse and validate untrusted JSON with a schema
import { z } from "zod";

const UserInputSchema = z.object({
  name: z.string(),
  email: z.string().email(),
  age: z.number().int().positive().optional(),
});

function parseInput(data: string) {
  const parsed = JSON.parse(data);
  return UserInputSchema.parse(parsed);
}

function parseUntrustedBody(body: string) {
  const obj = JSON.parse(body);
  return UserInputSchema.parse(obj);
}
