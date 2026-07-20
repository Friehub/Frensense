// [frensense]
// observation: An auction bid is accepted without verifying that the user has sufficient funds in their account to cover the bid amount.
// impact: Users can place bids far beyond their financial means, win the auction, and then fail to pay, disrupting the auction process and wasting the seller's time.
// improvement: Check the user's account balance against the bid amount before accepting the bid, and place a hold on the funds.

export async function placeBid(userId: string, auctionId: string, amount: number, env: Env) {
  const auction = await env.DB.prepare(
    'SELECT * FROM auctions WHERE id = ? AND end_time > ? AND status = ?'
  ).bind(auctionId, Date.now(), 'ACTIVE').first();

  if (!auction) throw new Error('Auction not found or ended');

  // VULNERABLE: no balance check — user can bid without funds
  await env.DB.prepare(
    'INSERT INTO bids (user_id, auction_id, amount, created_at) VALUES (?, ?, ?, ?)'
  ).bind(userId, auctionId, amount, Date.now()).run();

  await env.DB.prepare(
    'UPDATE auctions SET current_bid = ?, bidder_id = ? WHERE id = ? AND current_bid < ?'
  ).bind(amount, userId, auctionId, amount).run();

  return { bidPlaced: true };
}
