// [frensense]
// observation: Booking time slots are stored without normalizing the timezone, causing users in different timezones to see incorrect availability.
// impact: Users see incorrect available slots because the system stores local times without UTC conversion, leading to double-booking or missed appointments.
// improvement: Always store booking times in UTC and convert to the user's local timezone only for display.

export async function createBookingSlot(resourceId: string, startTime: string, endTime: string, env: Env) {
  // VULNERABLE: stores time as provided, without converting to UTC
  await env.DB.prepare(
    'INSERT INTO booking_slots (resource_id, start_time, end_time, timezone) VALUES (?, ?, ?, ?)'
  ).bind(resourceId, startTime, endTime, 'America/New_York').run();

  return { created: true };
}

export async function getAvailableSlots(resourceId: string, date: string, env: Env) {
  const slots = await env.DB.prepare(
    'SELECT * FROM booking_slots WHERE resource_id = ? AND DATE(start_time) = ?'
  ).bind(resourceId, date).all();

  return slots;
}
