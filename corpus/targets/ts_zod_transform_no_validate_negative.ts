// SAFE: Transform is followed by a `.pipe()` that validates the output shape

import { z } from 'zod';

const StringToNumber = z.string().transform((val) => {
  const parsed = parseInt(val, 10);
  return isNaN(parsed) ? 0 : parsed;
}).pipe(z.number().int().nonnegative());

function process(input: unknown) {
  const value = StringToNumber.parse(input);
  return value * 2;
}
