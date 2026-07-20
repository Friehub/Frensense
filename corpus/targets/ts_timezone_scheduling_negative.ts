// SAFE: Stores all times in UTC and normalizes user-provided times to UTC

export async function createBookingSlot(resourceId: string, startTime: string, endTime: string, timezone: string, env: Env) {
  // SAFE: convert to UTC before storing
  const startUtc = new Date(startTime).toISOString();
  const endUtc = new Date(endTime).toISOString();

  await env.DB.prepare(
    'INSERT INTO booking_slots (resource_id, start_time_utc, end_time_utc, timezone) VALUES (?, ?, ?, ?)'
  ).bind(resourceId, startUtc, endUtc, timezone).run();

  return { created: true };
}

export async function getAvailableSlots(resourceId: string, date: string, userTimezone: string, env: Env) {
  const dateStart = new Date(`${date}T00:00:00Z`).toISOString();
  const dateEnd = new Date(`${date}T23:59:59Z`).toISOString();

  const slots = await env.DB.prepare(
    'SELECT * FROM booking_slots WHERE resource_id = ? AND start_time_utc >= ? AND start_time_utc <= ?'
  ).bind(resourceId, dateStart, dateEnd).all();

  // Convert to user's timezone for display
  return slots.map((slot) => ({
    ...slot,
    start_time_local: new Date(slot.start_time_utc).toLocaleString('en-US', { timeZone: userTimezone }),
    end_time_local: new Date(slot.end_time_utc).toLocaleString('en-US', { timeZone: userTimezone }),
  }));
}
