// [frensense]
// observation: No locking mechanism prevents two users from booking the same time slot simultaneously, allowing double-booking.
// impact: Two customers can both book and pay for the same appointment slot; at least one will be disappointed and require a refund.
// improvement: Use a pessimistic lock or an atomic INSERT with a unique constraint on (resource_id, slot_start).

export async function bookSlot(userId: string, resourceId: string, slotStart: string, slotEnd: string, env: Env) {
  // VULNERABLE: no lock — two concurrent requests can both insert
  await env.DB.prepare(
    'INSERT INTO bookings (user_id, resource_id, slot_start, slot_end, status) VALUES (?, ?, ?, ?, ?)'
  ).bind(userId, resourceId, slotStart, slotEnd, 'CONFIRMED').run();

  return { booked: true };
}
