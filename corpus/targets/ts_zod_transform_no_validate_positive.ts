// [frensense]
// observation: A Zod `.transform()` is chained without output validation, so the transformed type is inferred but not checked at runtime.
// impact: If the transformation does not produce a valid value (returns null, throws, or produces wrong shape), the downstream code receives malformed data without any safety net.
// improvement: Chain `.pipe()` with an output schema, or add `.refine()` after `.transform()` to validate the result.

import { z } from 'zod';

const StringToNumber = z.string().transform((val) => {
  const parsed = parseInt(val, 10);
  return isNaN(parsed) ? 0 : parsed;
});

function process(input: unknown) {
  const value = StringToNumber.parse(input);
  return value * 2;
}
