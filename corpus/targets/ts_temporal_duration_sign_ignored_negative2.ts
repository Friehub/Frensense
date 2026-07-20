// SAFE: Use Duration.with() to handle sign explicitly rather than discarding it with abs().

import { Temporal } from '@js-temporal/polyfill';

function adjustDeadline(instant: Temporal.Instant, deltaDays: number): Temporal.Instant {
  const duration = Temporal.Duration.from({ days: Math.abs(deltaDays) });
  if (deltaDays < 0) {
    return instant.subtract(duration);
  }
  return instant.add(duration);
}
