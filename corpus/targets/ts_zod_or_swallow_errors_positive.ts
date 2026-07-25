// [frensense]
// observation: A Zod `.or()` default is used at the end of a union chain, causing any validation error to silently fall through to a permissive fallback.
// impact: Invalid or malicious input that fails all strict schemas gets accepted by the catch-all default, bypassing validation entirely.
// improvement: Use `.catch()` carefully or handle validation errors explicitly instead of chaining an overly permissive `.or()` fallback.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import { z } from 'zod';

const StatusSchema = z.enum(['active', 'inactive', 'pending']).or(z.string());

function processStatus(data: unknown) {
  const status = StatusSchema.parse(data);
  return status;
}
