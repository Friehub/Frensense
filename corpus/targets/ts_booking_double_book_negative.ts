// SAFE: Uses an atomic INSERT with an existence check and a unique constraint

export async function bookSlot(userId: string, resourceId: string, slotStart: string, slotEnd: string, env: Env) {
  const existing = await env.DB.prepare(
    'SELECT id FROM bookings WHERE resource_id = ? AND slot_start = ? AND status = ?'
  ).bind(resourceId, slotStart, 'CONFIRMED').first();

  if (existing) {
    throw new Error('This time slot is already booked');
  }

  // SAFE: try to insert — unique constraint catches races
  try {
    await env.DB.prepare(
      'INSERT INTO bookings (user_id, resource_id, slot_start, slot_end, status) VALUES (?, ?, ?, ?, ?)'
    ).bind(userId, resourceId, slotStart, slotEnd, 'CONFIRMED').run();
  } catch (e) {
    throw new Error('Slot was just taken by another user');
  }

  return { booked: true };
}
