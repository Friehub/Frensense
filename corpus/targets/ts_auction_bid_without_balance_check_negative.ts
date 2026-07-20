// SAFE: Checks the user's available balance before accepting the bid

export async function placeBid(userId: string, auctionId: string, amount: number, env: Env) {
  const auction = await env.DB.prepare(
    'SELECT * FROM auctions WHERE id = ? AND end_time > ? AND status = ?'
  ).bind(auctionId, Date.now(), 'ACTIVE').first();

  if (!auction) throw new Error('Auction not found or ended');

  // SAFE: check user balance
  const user = await env.DB.prepare(
    'SELECT balance FROM users WHERE id = ?'
  ).bind(userId).first();

  if (!user || Number(user.balance) < amount) {
    throw new Error('Insufficient balance to place this bid');
  }

  // SAFE: place hold on funds
  await env.DB.prepare(
    'UPDATE users SET balance = balance - ?, held_balance = held_balance + ? WHERE id = ? AND balance >= ?'
  ).bind(amount, amount, userId, amount).run();

  await env.DB.prepare(
    'INSERT INTO bids (user_id, auction_id, amount, created_at) VALUES (?, ?, ?, ?)'
  ).bind(userId, auctionId, amount, Date.now()).run();

  // Release previous bidder's hold
  if (auction.bidder_id) {
    await env.DB.prepare(
      'UPDATE users SET balance = balance + ?, held_balance = held_balance - ? WHERE id = ?'
    ).bind(auction.current_bid, auction.current_bid, auction.bidder_id).run();
  }

  await env.DB.prepare(
    'UPDATE auctions SET current_bid = ?, bidder_id = ? WHERE id = ? AND current_bid < ?'
  ).bind(amount, userId, auctionId, amount).run();

  return { bidPlaced: true };
}
