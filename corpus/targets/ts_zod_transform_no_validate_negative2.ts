// SAFE: After transform, a refinement validates the output is within expected bounds

import { z } from 'zod';

const StringToNumber = z.string().transform((val) => {
  const parsed = parseInt(val, 10);
  if (isNaN(parsed)) throw new Error('Not a number');
  return parsed;
}).refine((n) => n >= 0 && Number.isInteger(n), {
  message: 'Value must be a non-negative integer',
});

function process(input: unknown) {
  const value = StringToNumber.parse(input);
  return value * 2;
}
