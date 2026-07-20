// SAFE: Validates user input server-side before passing to meta, using zod schema
import type { MetaFunction } from "@remix-run/node";
import { z } from "zod";

const metaSchema = z.object({
  title: z.string().max(100),
  description: z.string().max(300),
});

export const meta: MetaFunction = ({ data }) => {
  const parsed = metaSchema.safeParse(data);
  if (!parsed.success) {
    return [{ title: "Default Title" }];
  }
  return [
    { title: parsed.data.title },
    { name: "description", content: parsed.data.description },
  ];
};
