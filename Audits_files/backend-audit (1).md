# Backend Services Audit Report
**Date:** May 19, 2026  
**Scope:** All tRPC routers and services under `packages/api/modules/`

---

## Summary

| Severity | Count |
|----------|-------|
| 🔴 Critical | 8 |
| 🟠 High | 14 |
| 🟡 Medium | 11 |
| 🟢 Low / Design | 7 |

---

## 1. IAM — `iam/services/user-service.ts` · `iam/router/index.ts`

### 🔴 CRITICAL — `registerSchema` allows `role: 'SELLER'` to be client-supplied
Any visitor calling `api.iam.register` can pass `role: "SELLER"` in the request body and self-promote to a seller without going through onboarding or KYC. The `role` field in `registerSchema` is `z.enum(['BUYER', 'SELLER']).default('BUYER')` — the enum accepts `SELLER` from untrusted input.  
**Fix:** Remove `role` from `registerSchema`. Always hardcode `role: 'BUYER'` in `userService.register()`. Sellers should only be promoted via `iam.onboardSeller`.

### 🔴 CRITICAL — `forgotPassword` and `resetPassword` are `publicProcedure` with no rate limiting
Both endpoints are rate-limitable attack vectors. A bot can enumerate valid email addresses by timing responses (if not careful), and can spam reset emails to any address. Neither endpoint has a rate-limit guard — only `register` and `onboardSeller` use `rateLimitProcedure`.  
**Fix:** Change both to `rateLimitProcedure`.

### 🔴 CRITICAL — `twoFactorEnabled` toggle stores no TOTP secret — 2FA is cosmetic
`toggleTwoFactor` flips a boolean but no TOTP secret is ever generated, stored, or validated. The admin login page has a step-2 MFA code field — but `handleFinalLogin` ignores `mfaCode` entirely and calls `signIn('credentials', ...)` with only email+password. Enabling "2FA" in settings does nothing to actual login security.  
**Fix:** Integrate a TOTP library (e.g. `otplib`). On enable: generate a secret, return a QR code, verify the first code before storing the secret. On login: if `twoFactorEnabled`, require a valid TOTP code before completing auth.

### 🟠 HIGH — `resetToken` stored in plain text in the database
`userService.requestPasswordReset()` stores the reset token as a raw string in `User.resetToken`. If the database is breached, all active reset tokens are immediately usable. Industry standard is to store a bcrypt/SHA-256 hash of the token and compare on verification.  
**Fix:** Hash the token before storing: `const storedToken = crypto.createHash('sha256').update(token).digest('hex')`. Compare the hash in `resetPassword`.

### 🟠 HIGH — No `emailVerified` field or email verification flow
Users can register with any email address and immediately transact. There is no email ownership verification step. A buyer can register with another person's email, receive that person's order confirmations, and potentially cause disputes. No `emailVerified` column exists in the schema.  
**Fix:** Add `emailVerified Boolean @default(false)` to the User model. Send a verification email on registration. Block checkout or key actions until verified (or at least warn the user).

### 🟠 HIGH — JWT role is never refreshed from DB — stale role persists until session expires
When NextAuth issues a JWT, it embeds `role` from the session at login time. If an admin changes a user's role in the admin panel, the old JWT remains valid and the user keeps their old role until their session expires naturally. There is no mechanism to invalidate or refresh the token.  
**Fix:** In the `jwt` callback in `auth.config.ts`, periodically re-fetch the user's role from DB (e.g. check on each request if the token was issued more than 5 minutes ago, or use a Redis blacklist for forcible invalidation).

### 🟠 HIGH — `callbackUrl` in login form is not validated — open redirect
`LoginForm.tsx` reads `callbackUrl` from `?callbackUrl=` and calls `router.push(callbackUrl)` directly. An attacker can craft a phishing URL: `https://yourapp.com/login?callbackUrl=https://evil.com`. After login, the user is redirected to the attacker's site.  
**Fix:** Validate that `callbackUrl` is a relative path (starts with `/`) before using it. NextAuth's own `callbackUrl` handling does this — make sure `redirect: true` paths go through NextAuth, not a manual `router.push`.

### 🟠 HIGH — No `updateAddress` endpoint — addresses can only be added or deleted, never edited
The `iam` router has `addAddress` and `deleteAddress` but no `updateAddress`. The address page renders an "Edit" button per address, but it opens the Add form (which creates a new address instead of updating). Users who mistype a street or city must delete and re-add the entire address, losing default status in the process.

### 🟡 MEDIUM — Phone number in `registerSchema` and `addressSchema` has no format validation
Both schemas accept `phone: z.string().min(1)`. A single character passes validation. Nigerian phone numbers follow a specific format (e.g. `080XXXXXXXX`, 11 digits). No regex validation is applied at any layer.  
**Fix:** `z.string().regex(/^(\+?234|0)[789][01]\d{8}$/, 'Invalid Nigerian phone number')`.

### 🟡 MEDIUM — `addAddress` uses `as any` to bypass type safety
`tx.userAddress.create({ data: { ...data, userId } as any })` casts the full create payload to `any`. This suppresses TypeScript's check that `country` and other required fields are present in the `data` object. If the schema changes, this will silently produce a runtime error.

---

## 2. Cart — `cart/services/cart-service.ts` · `cart/router/index.ts`

### 🟠 HIGH — Cart merge on login is never triggered
`cartService.mergeCart()` is implemented and correct, but it is never called anywhere in the application. When a guest adds items and then logs in, their guest cart is abandoned — the user's cart starts empty. The merge function just sits unused.  
**Fix:** Call `mergeCart(guestSessionId, userId)` in the NextAuth `signIn` callback or in a post-login effect in `CartContext`.

### 🟡 MEDIUM — Flash sale price snapshot is refreshed on every `addItem` call
When a buyer adds an item already in the cart, `addItem` does an upsert that refreshes `priceSnapshot` to the current price. If a flash sale ends between the first and second add, the snapshot price changes mid-session. The buyer may have added items at sale price, but by the time they check out, the snapshot has been refreshed to full price.  
**Fix:** Only set `priceSnapshot` on `create`, not on `update`. Keep the price the buyer first saw.

### 🟡 MEDIUM — `cart.get` and `cart.add` use `publicProcedure` — `ctx.sessionId` can be `null`
Both procedures call `ctx.sessionId!` with a non-null assertion. If the session cookie is missing or stripped by a proxy, `sessionId` is `null` and `cartService.getCart` throws `MISSING_CART_IDENTIFIER`. The `!` assertion bypasses TypeScript's null check and produces an unhandled runtime error.  
**Fix:** Add an explicit guard: `if (!ctx.sessionId) throw new TRPCError({ code: 'BAD_REQUEST', message: 'Session required' })`.

### 🟡 MEDIUM — No cart item limit — a single cart can hold unbounded items
A buyer can add thousands of distinct variants to a cart with no cap. Each `createFromCart` call then tries to reserve stock for all of them inside a transaction, which can time out on a large cart and leave the order in a broken state.  
**Fix:** Add a max items check in `addItem`: `if (cart.items.length >= 100) throw new Error('CART_LIMIT_REACHED')`.

---

## 3. Catalog — `catalog/services/`

### 🔴 CRITICAL — `createProduct` throws `NO_WAREHOUSE_CONFIGURED` on a fresh production DB
`catalogService.createProduct()` calls `warehouseService.findFirst({ orderBy: { name: 'asc' } })`. The production migration creates the Warehouse table but inserts zero rows. No warehouse admin API exists. Every product listing attempt fails until a warehouse is manually inserted via SQL.  
**Fix (immediate):** Add an `INSERT INTO "Warehouse"` to the migration SQL. **Fix (proper):** Add `admin.createWarehouse` and `admin.listWarehouses` tRPC procedures and a warehouse management UI in the admin inventory page.

### 🟠 HIGH — `deleteProduct` has no ownership check — any seller can delete another seller's product
`catalog.deleteProduct` is a `sellerProcedure` but the service uses `prisma.product.delete({ where: { id: productId } })` without scoping to the caller's `sellerId`. A seller who knows another seller's product ID can delete it.  
**Fix:** `prisma.product.delete({ where: { id: productId, sellerId: seller.id } })`.

### 🟠 HIGH — `updateProduct` allows a seller to change `sellerId` and transfer products to another seller
`catalogService.updateProduct()` accepts `Partial<ProductInput>` merged into the update `data`. If `sellerId` is in `ProductInput`, a malicious seller can pass a different `sellerId` and re-assign the product. The schema should strip `sellerId` from updates.

### 🟡 MEDIUM — `getProductBySlug` runs 4 separate queries sequentially (N+1 for recommendations)
The server component awaits: (1) `catalogQueryService.getProductBySlug`, (2) `RustClient.recommendations.forProduct`, (3) `productRelationService.findMany`, (4) Redis cache wrapper. All are sequential. On a cold cache this adds 4× network round-trip latency to every product page load.  
**Fix:** Run recommendations and related products concurrently via `Promise.all`.

### 🟡 MEDIUM — Bulk import `catalog-import-service.ts` has no per-row error isolation
If a CSV row fails to insert (e.g. duplicate SKU), the entire bulk import transaction rolls back. A single malformed row in a 500-row import file fails the entire batch silently and the seller sees a generic error.  
**Fix:** Process rows in individual `try/catch` blocks and return a per-row result summary.

---

## 4. Order — `order/services/order-service.ts`

### 🔴 CRITICAL — `WALLET` payment method is not handled — order stays `PENDING_PAYMENT` then auto-cancels
`createFromCart` accepts `paymentMethod: 'WALLET'` but `updateStatus` only has a special path for `POD`/`PAY_ON_DELIVERY`. A wallet order is created as `PENDING_PAYMENT` and never transitions. The 30-minute SLA timeout then cancels it. The buyer's wallet is never debited but they see a success screen.  
**Fix:** After `createFromCart` returns, if `paymentMethod === 'WALLET'`, immediately call `paymentService.payWithWallet(orderId, userId, amount)` and then transition the order to `PAID`.

### 🟠 HIGH — `cancelOrder` does not release coupon usage — coupons are permanently consumed on cancellation
`cancelOrder` calls `updateStatus(orderId, 'CANCELLED')` which releases stock, but there is no call to release the coupon. `markCouponUsed` increments `usedCount` and creates a `CouponRedemption` record at order creation. Neither is reversed on cancellation.  
**Fix:** In the `CANCELLED` branch of `updateStatus`, call `promoService.releaseCoupon(orderId)` which should decrement `usedCount` and delete the `CouponRedemption` record.

### 🟠 HIGH — `publishEvent` called inside DB transaction — ghost events on rollback
In `updateStatus`, `publishEvent('order.status_updated', ...)` is called while `db` may be a transaction client. If the outer transaction rolls back after this point, the event has already been published but the DB state has not changed. Downstream consumers will act on a state that was never committed.  
**Fix:** Move all `publishEvent` calls to after the transaction commits, or use the transactional outbox pattern (already used in `payWithWallet` — apply the same pattern here).

### 🟡 MEDIUM — `listUserOrders` and `listSellerPackages` accept `limit`/`offset` but the router doesn't expose pagination controls
Both service methods support pagination via `limit` and `offset` parameters, but the `order` router procedures call them without passing these values — they use the default `limit: 20`. The frontend also passes no pagination params. The API is wired for pagination but never used.

### 🟡 MEDIUM — Order `track` procedure is `publicProcedure` with no ownership or token check
`api.order.track` is public and accepts an `orderId`. Anyone who obtains an order UUID (from a URL, email, or screenshot) can retrieve full order details including the buyer's delivery address. Order IDs appear in browser URLs on the success page.  
**Fix:** Require auth and verify `order.userId === ctx.session.user.id`, or implement a separate signed tracking token that doesn't expose the internal order ID.

---

## 5. Payment — `payment/services/`

### 🔴 CRITICAL — Checkout success page never calls `verifyPayment` — payment can be bypassed
After Paystack/Flutterwave redirects back to `/checkout/success?orderId=...&reference=...`, the success page only reads `orderId` and calls `api.order.get`. It never reads the `reference` param or calls `api.payment.verifyPayment`. A buyer can skip payment, paste the success URL, see a confirmation screen, and the order stays `PENDING_PAYMENT` waiting for a webhook that may never come.  
**Fix:** On success page mount, call `api.payment.verifyPayment({ reference })` using `searchParams.get('reference')`. Show a loading state and display a clear failure message if not confirmed.

### 🟠 HIGH — `initializePaystack` (legacy) and `initializePayment` (new) both create a `Payment` record for the same order
Both procedures exist simultaneously. If anything calls both (or if a retry hits the legacy endpoint), two `Payment` records are created for the same `orderId` with different `providerRef` values. The webhook handler resolves the matching record and marks it `SUCCESS`, leaving the other permanently `PENDING`. This corrupts payment state for the order.  
**Fix:** Remove `initializePaystack` from the router entirely. Ensure all callers use `initializePayment`.

### 🟠 HIGH — Monnify adapter silently falls back to `'mock_token'` on auth failure
`MonnifyAdapter.getAuthToken()` catches errors and returns `'mock_token'`. Every subsequent Monnify API call will receive a 401, which is thrown as `MONNIFY_INIT_FAILED`. The real cause — wrong credentials — is hidden. In production with a misconfigured Monnify key, all Monnify payments fail silently.  
**Fix:** Rethrow the auth error with a descriptive message: `throw new Error('MONNIFY_AUTH_FAILED: Check MONNIFY_API_KEY and MONNIFY_SECRET_KEY')`.

### 🟠 HIGH — Flutterwave webhook uses plain string comparison — trivially forgeable
`FlutterwaveAdapter.verifyWebhookSignature()` returns `signature === FLW_WEBHOOK_SECRET` — a static secret comparison, not an HMAC. The code comment acknowledges this: `"WARNING: This is not as secure as HMAC."` Anyone who obtains the secret can forge any payment event.  
**Fix:** Implement HMAC-SHA256 as Flutterwave's documentation describes, or at minimum verify the transaction server-side via `FlutterwaveAdapter.verifyTransaction(id)` on every webhook before processing.

### 🟠 HIGH — Fraud check always receives `ip_address: 'unknown'` — IP-based fraud signals are blind
`initializePayment` in the router calls `paymentService.initializeTransaction(...)` without extracting the caller's IP from `ctx.req`. The default parameter `ipAddress = 'unknown'` is always used. The Rust fraud service's IP velocity, geo-mismatch, and VPN detection features are completely unused.  
**Fix:** Extract IP in the router: `const ip = ctx.req?.headers?.get('x-forwarded-for') ?? ctx.req?.headers?.get('cf-connecting-ip') ?? 'unknown'` and pass it to `initializeTransaction`.

### 🟡 MEDIUM — Wallet funding callback URL is `/wallet` — the page is at `/account/wallet`
`paymentService.requestWalletFunding()` sets `callback_url: ${NEXTAUTH_URL}/wallet`. After funding, the payment provider redirects to a 404. The correct path is `/account/wallet`.

### 🟡 MEDIUM — `Paystack.verifyWebhookSignature` in the adapter uses `===` not `timingSafeEqual`
The adapter method does `return hash === signature`. String comparison in JS is not constant-time. The main `paymentService.verifyWebhookSignature` (used by the webhook route) correctly uses `crypto.timingSafeEqual` — but the adapter's method is used by `api.payment.verifyPayment` and is vulnerable to timing side-channels.

---

## 6. Inventory — `inventory/services/inventory-service.ts`

### 🔴 CRITICAL — `inventory.reserve` is `publicProcedure` — unauthenticated users can deplete stock
Any anonymous caller who knows a `variantId` can fire repeated calls to `inventory.reserve` and drain stock for any product without placing an order. This is a denial-of-stock attack surface that would make all items appear out of stock.  
**Fix:** Change to `protectedProcedure`. Better: remove it as a standalone exposed endpoint and only call `inventoryService.reserveStock()` internally from `order.createFromCart`.

### 🟠 HIGH — Redis-to-DB split write can drift on process crash (reservation without DB record)
In the Redis fallback path: the Lua script atomically decrements the Redis key (step 2), then writes a `StockReservation` to Postgres (step 3) in a separate operation. If the process crashes between these two, Redis shows stock as reserved but no reservation record exists. The nightly `syncAllStock` cron corrects this, but any orders placed in that window against the drifted stock are at risk.  
**Fix:** Use Redis as cache only for reads. Write the DB reservation first, then update Redis. Or use a Redis transaction + DB outbox approach.

### 🟠 HIGH — `updateStock` does not verify the `warehouseId` belongs to the calling seller
`inventory.updateStock` accepts any `warehouseId` from the seller. Since `Warehouse` has no `sellerId` field, the service cannot verify the warehouse belongs to the caller. A seller can update stock in another seller's warehouse location.  
**Fix:** Add `sellerId` to the `Warehouse` model, or scope the `warehouseId` lookup to `Warehouse.where({ id: warehouseId, sellerId: seller.id })`.

### 🟡 MEDIUM — `StockLevel.sellerId` has no FK constraint — referential integrity is not enforced
The `StockLevel` model has `sellerId String` with no `@relation` to `Seller`. Prisma enforces no foreign key. If a seller is deleted, their `StockLevel` rows remain and can be queried with a dangling `sellerId`.

---

## 7. Logistics — `logistics/services/logistics-service.ts` · `logistics/router/index.ts`

### 🔴 CRITICAL — Return window uses `order.updatedAt` instead of actual delivery timestamp
`returnService.initiateReturn()` calculates the return window as `(Date.now() - line.package.order.updatedAt) / ms_per_day`. `updatedAt` changes every time the order is touched — by admin edits, status updates, payment confirmation. An order edited yesterday has a 1-day-old window regardless of when it was actually delivered.  
**Fix:** Add `deliveredAt DateTime?` to the `Order` model. Set it in `orderService.updateStatus()` when status becomes `DELIVERED`. Use it exclusively in the return window calculation.

### 🟠 HIGH — `registerAsAgent` has no admin approval — any user can self-promote to delivery agent
`logistics.registerAsAgent` is a `protectedProcedure`. Any logged-in buyer calls it, gets a `DeliveryAgent` record created, and has their role changed to `AGENT` — all without any admin review. A malicious user becomes an agent instantly and can mark shipments as delivered to trigger escrow release.  
**Fix:** Change the agent creation to set `status: 'PENDING'`. Add an admin approval step before the role is changed to `AGENT`.

### 🟠 HIGH — `updateShipmentStatus` has no ownership check — any agent can update any shipment
`logistics.updateShipmentStatus` is an `agentProcedure` but passes `input.shipmentId` directly to `logisticsService.updateStatus()` without verifying the shipment is assigned to the calling agent. Agent A can mark Agent B's shipment as delivered, triggering a premature escrow release for an order they have nothing to do with.  
**Fix:** Add `where: { id: shipmentId, agentId: agent.id }` to the shipment lookup in `updateStatus`.

### 🟠 HIGH — Failed delivery (`FAILED` status) immediately cancels the package — no retry possible
In `logisticsService.updateStatus()`, when status is `FAILED`, it calls `packageService.updateStatus(packageId, 'CANCELLED')`. A first failed delivery attempt (customer not home, wrong address) permanently cancels the package. The shipment state machine allows retrying from `FAILED`, but the parent package is already cancelled.  
**Fix:** Map `FAILED` to `RETURN_TO_SELLER` or `PENDING` on the package, not `CANCELLED`. Only cancel if the seller explicitly requests cancellation or after N failed attempts.

### 🟠 HIGH — Shipping fee uses state-level multiplier only — no weight, distance, or LGA
`logisticsService.calculateShipping()` multiplies a base rate by 1.5 for 6 northern states and 1.0 for everything else. There is no weight tier calculation, no LGA routing, no distance logic, no pickup station discount. A 10 kg shipment Lagos→Abuja costs the same as a 1 kg envelope.

### 🟡 MEDIUM — `agentId: 'system-unassigned'` is hardcoded as a string — not a valid FK
When a shipment is auto-created by the logistics worker, it sets `agentId: 'system-unassigned'`. If the `Shipment.agentId` column has a foreign key constraint to `DeliveryAgent`, this insert fails. If it doesn't, joins on `agent` return null for all unassigned shipments.  
**Fix:** Make `agentId` nullable. Set it to `null` for unassigned shipments. Add an admin alert for shipments with `agentId IS NULL`.

---

## 8. Affiliate — `affiliate/services/affiliate-service.ts`

### 🔴 CRITICAL — Referral cookie is never read at checkout — zero commissions are ever attributed
`ReferralTracker` sets a `referralLinkId` cookie when a buyer lands via `?ref=slug`. But `checkout.tsx` never reads this cookie. `createOrder.mutate()` is called without `referralLinkId`. The event consumer's affiliate attribution branch never runs. Every affiliate who has ever sent traffic has earned zero commission.  
**Fix:** In `checkout.tsx`, before calling `createOrder.mutate`, read the cookie: `const ref = document.cookie.match(/referralLinkId=([^;]+)/)?.[1]` and include it in the mutation input.

### 🔴 CRITICAL — `confirmMatureCommissions` re-funds wallets for all previously `CONFIRMED` commissions on every cron run
Step 1 bulk-updates `PENDING → CONFIRMED`. Step 2 fetches all commissions where `status = 'CONFIRMED'` — which includes commissions confirmed on previous cron runs that have not yet been updated to `PAID`. Each of those agents gets a duplicate wallet credit on every subsequent cron execution.  
**Fix:** Capture the IDs returned by the `updateMany` in step 1 and filter step 2 to only those IDs: `commissionService.findMany({ where: { id: { in: updatedIds } } })`.

### 🟠 HIGH — `getAgentStats` double-counts: `CONFIRMED` and `PAID` commissions are summed together
`stats.find(s => s.status === 'CONFIRMED' || s.status === 'PAID')` returns a single sum for both statuses. A commission moves from `CONFIRMED → PAID` after the wallet is funded. But it remains in the "confirmed" sum — so agents see their lifetime confirmed earnings as `confirmed_amount + paid_amount` instead of just `confirmed_amount`.  
**Fix:** Use two separate `.find()` calls: one for `CONFIRMED` (awaiting payout) and one for `PAID` (historical earnings). Display them separately in the dashboard.

### 🟠 HIGH — Any authenticated user can self-register as an affiliate with no vetting
`affiliate.register` is a `protectedProcedure`. Any buyer, seller, or admin can call it and receive a 5% commission rate immediately. There is no admin approval, no KYC requirement, no minimum account age, and no fraud screening before the first commission is earned.

### 🟡 MEDIUM — Commission rate is hardcoded at 5% — no per-agent or per-campaign rate negotiation
`registerAgent()` always creates agents with `commissionRate: 5.0`. There is no admin UI to adjust rates per agent, and no rate tiers for performance-based commission structures. The field exists on the model but is never written to after creation.

---

## 9. Dispute — `dispute/services/dispute-service.ts`

### 🟡 MEDIUM — Dispute can be opened on any order regardless of delivery status
`disputeService.openDispute()` verifies the order belongs to the buyer but does not check order status. A buyer can open a dispute immediately after placing an order — before it's even shipped — with no grace period. This creates noise in the dispute queue for cases that haven't actually failed yet.  
**Fix:** Require `order.status` to be `SHIPPED`, `DELIVERED`, or `COMPLETED` before a dispute can be opened.

### 🟡 MEDIUM — No dispute limit per order — the same order can generate unlimited disputes
There is no check for existing disputes before `disputeService.create()`. A buyer can open the same dispute 100 times on the same order, creating 100 threads in the moderator queue.  
**Fix:** Add `const existing = await disputeService.findFirst({ where: { orderId, buyerId, status: { not: 'RESOLVED' } } })` and throw `DISPUTE_ALREADY_OPEN` if found.

---

## 10. Revenue / Ledger — `revenue/services/ledger-service.ts`

### 🟠 HIGH — `withdrawFunds` writes the ledger debit BEFORE checking balance — negative balance window
Inside the transaction: step 1 creates a negative ledger entry (the withdrawal debit). Step 2 re-calculates available balance. If step 2 finds the balance is negative, it throws and the transaction rolls back. This is correct — but between the two steps, the balance is temporarily negative in the same transaction. If the transaction isolation level is `READ COMMITTED`, a concurrent read could see the negative state.  
**Fix:** Check the available balance BEFORE creating the withdrawal entry. Throw early if insufficient. Then create the entry as step 2.

### 🟡 MEDIUM — `generateStatement` fetches all ledger entries for the period into memory — no pagination
For a high-volume seller, `sellerLedgerEntryService.findMany` for a month-long period could return tens of thousands of rows. All are loaded into memory to compute `gross`, `commission`, and `net`. This will OOM the API server for active sellers.  
**Fix:** Replace with `sellerLedgerEntryService.groupBy` or `aggregate` queries scoped by `type` and `createdAt` range.

### 🟡 MEDIUM — `exportLedger` fetches the entire ledger history with no limit — unbounded response
`exportLedger` calls `findMany` with no `take` or pagination. A seller with 10,000+ entries downloads the entire table. This will timeout and exhaust memory.  
**Fix:** Add `take: 5000` as a hard limit and document the cap, or implement cursor-based streaming export.

---

## 11. Review — `review/services/review-service.ts`

### 🟡 MEDIUM — Rating aggregation runs two additional DB queries after every review — synchronously in the request
After creating a review, `reviewService.createReview()` immediately runs `review.aggregate` on all reviews for the product and `review.aggregate` on all reviews for the seller. For a seller with thousands of products and hundreds of thousands of reviews, these aggregations block the HTTP response.  
**Fix:** Move rating recalculation to a background job (BullMQ) published via `publishEvent('review.created', ...)`. Return the review to the user immediately.

### 🟡 MEDIUM — Reviews are permanently `PENDING` until manually moderated — no admin moderation UI
`createReview` always creates reviews with `status: 'PENDING'`. `getProductReviews` only returns `APPROVED` reviews. But there is no admin or moderator UI to review pending submissions. The moderator page only handles disputes. Reviews accumulate invisibly and are never shown.  
**Fix:** Add a review moderation section to the admin panel or moderator page with `review.listPending` and `review.moderate` tRPC procedures exposed to `moderatorProcedure`.

---

## 12. Advertising — `advertising/services/advertising-service.ts`

### 🟡 MEDIUM — Ad spend is debited from the seller's ledger with no minimum balance check
`advertisingService.recordClick()` creates a `sellerLedgerEntry` of type `AD_SPEND` with a negative amount. There is no check that the seller's available balance is sufficient before debiting. A seller with ₦0 balance can run active campaigns and accumulate negative ledger entries indefinitely.  
**Fix:** In `recordClick`, before creating the `AD_SPEND` entry, call `ledgerService.getSellerBalance(sellerId, 'AVAILABLE')` and pause the campaign if the balance is below the bid amount.

### 🟡 MEDIUM — `addAdGroup` does not verify the `productId` belongs to the seller's catalogue
`advertisingService.addAdGroup()` accepts any `productId`. A seller can create an ad group promoting a competitor's product. The ownership check only verifies campaign ownership, not product ownership.  
**Fix:** Add `prisma.product.findFirst({ where: { id: productId, sellerId: seller.id } })` before creating the ad group.

---

## 13. Ops / Admin — `ops/services/ops-service.ts` · `admin/services/admin-service.ts`

### 🟠 HIGH — No `updateUserRole` procedure — the `MODERATOR` role can never be assigned from the UI
The `UserRole` enum includes `MODERATOR`, `moderatorProcedure` is correctly defined, and the layout guard works. But the admin users page only exposes `api.admin.updateUserStatus`. There is no `api.admin.updateUserRole` procedure. The only way to assign the MODERATOR role is a direct SQL `UPDATE` on the `User` table.  
**Fix:** Add `updateUserRole: adminProcedure.input(z.object({ userId: z.string(), role: z.enum(['BUYER', 'SELLER', 'MODERATOR', 'AGENT']) }))` — deliberately excluding `ADMIN` from the enum so admins cannot self-promote others to admin.

### 🟠 HIGH — System health check hardcodes `inventory` and `fraud` Rust services as `UP` without testing them
`opsService.getSystemHealth()` only tests the `search` Rust service via `RustClient.search.health()`. For `inventory` and `fraud`, it sets `health.rustServices[s] = 'UP'` unconditionally inside the try block without making any actual call. The admin health dashboard permanently shows these services as healthy regardless of their real state.

### 🟡 MEDIUM — `getMetrics` and `getGlobalMetrics` are duplicate procedures calling the same service method
Both `ops.getMetrics` and `ops.getGlobalMetrics` call `opsService.getGlobalMetrics()` and return identical data. One of them is dead code. This causes confusion about which endpoint to use and doubles the surface area for maintenance.  
**Fix:** Remove `getMetrics`. Update any callers to use `getGlobalMetrics`.

### 🟡 MEDIUM — 22 `console.log("DEBUG: [N/N]...")` lines in the API server boot sequence ship to production
Every process start prints step-by-step boot diagnostics to stdout, polluting PM2 and cloud log aggregators with noise that makes real errors harder to find.  
**Fix:** Gate all debug boot logs behind `if (process.env.NODE_ENV !== 'production')`.

---

## 14. Notification — `notification/services/notification-service.ts`

### 🟠 HIGH — SMS (Termii) and Push (FCM) are stubs — transactional messages are never delivered
The SMS path logs `[STUB/TEST] Sending SMS` instead of calling Termii. The FCM path logs `[STUB/FCM] Push` and never sends a real notification. Order confirmations, dispute updates, OTP codes for phone verification, and delivery alerts are only delivered via email (if Resend is configured).

### 🟡 MEDIUM — Email `from` address is hardcoded as `Friehub Jumia <...>` — exposes internal project name
The Resend API call uses `from: \`Friehub Jumia <${RESEND_FROM_EMAIL}>\``. Every email sent to buyers and sellers shows the internal dev project name in the sender field. Should use the real brand name from an environment variable.

---

## What Is Well Built ✓

| Area | Strength |
|------|----------|
| Order state machine | Enforced transitions — invalid status jumps throw `INVALID_TRANSITION` |
| Webhook idempotency | Duplicate payment webhooks detected and skipped |
| Wallet payment | Single-transaction atomic debit with transactional outbox pattern |
| Stock reservation | Redis Lua script for atomic decrement — safe under concurrency |
| Payment signature | `crypto.timingSafeEqual` on Paystack webhook validation |
| SLA timeout | 30-minute BullMQ job cancels unpaid orders automatically |
| Escrow release | 7-day delayed job triggered on delivery, blocked if dispute is open |
| Multi-seller splitting | Orders split into per-seller packages at creation |
| Commission deduction | Automatically calculated and recorded at sale time |
| Coupon validation | Per-user limits, expiry, min order value, and seller scoping all enforced |
| Circuit breakers | All Rust service calls wrapped — system degrades gracefully |
| Ad click fraud | Self-click detection and Redis deduplication per IP/user per hour |
| Ledger idempotency | Sale recording is idempotent — duplicate events don't double-credit sellers |
| Prometheus metrics | `prom-client` with HTTP duration histograms wired on the API server |
