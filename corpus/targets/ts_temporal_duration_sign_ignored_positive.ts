// [frensense]
// observation: When adding a potentially negative Temporal.Duration to an Instant, the code takes `abs().sign` of the duration or negates it unconditionally, discarding the original sign. Negative durations represent time in the past and must be preserved.
// impact: Scheduled events in the past are incorrectly shifted into the future, or refund/subtraction logic adds instead of subtracts, causing logic errors in billing, scheduling, and time-arithmetic systems.
// improvement: Preserve the sign of the Duration. Use `instant.add(duration)` directly — Temporal handles negative durations correctly.

import { Temporal } from '@js-temporal/polyfill';

function applyOffset(instant: Temporal.Instant, durationStr: string): Temporal.Instant {
  const duration = Temporal.Duration.from(durationStr);
  const absolute = duration.abs();
  return instant.add(absolute);
}
