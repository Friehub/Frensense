# Jumia Clone — Comprehensive Audit Report
> Audited: 2026-05-02 | Status: Post-Phase 5

---

## BUGS

### B1 — `FlashSales.tsx` — Not calling the promo endpoint
**File**: `apps/web/src/components/home/FlashSales.tsx`
**Problem**: Component calls `api.catalog.listProducts.useQuery({})` (a generic product list) and slices the first 6 items as "flash sales". The dedicated `api.promo.getFlashSales` procedure exists but is never used. The timer shows a hardcoded static string `"08h : 22m : 45s"`.
**Fix**: Call `api.promo.getFlashSales`, compute a live countdown from `flashSale.endTime`.

---

### B2 — `FlashSalesPage` (`/flash-sales`) — Fully static shell
**File**: `apps/web/src/app/(buyer)/flash-sales/page.tsx`
**Problem**: Renders a hardcoded "No Flash Sales active right now" state with a static `00:00:00` timer and never queries `api.promo.getFlashSales`. Backend data is ignored.
**Fix**: Wire to `api.promo.getFlashSales`, display real products in a grid, and drive a real countdown timer.

---

### B3 — `SavedItemsPage` — Wishlist backend wired but UI is a static stub
**File**: `apps/web/src/app/(buyer)/account/saved/page.tsx`
**Problem**: The page always renders "You haven't saved any items yet" regardless of the user's actual wishlist. The `catalog.getWishlist` tRPC procedure was implemented in Phase 5 but is never called here.
**Fix**: Query `api.catalog.getWishlist`, render wishlist items with remove and add-to-cart CTAs.

---

### B4 — `ProductActions.tsx` — "ADD TO WISHLIST" button is a no-op
**File**: `apps/web/src/components/products/ProductActions.tsx:91`
**Problem**: The heart button has no `onClick` handler and no tRPC mutation. Clicking it does nothing. The `catalog.addToWishlist` procedure exists on the backend.
**Fix**: Add an `api.catalog.addToWishlist.useMutation()` call; toggle heart icon state on success.

---

### B5 — `HomePage` — "Top Categories" is placeholder grid
**File**: `apps/web/src/app/page.tsx`
**Problem**: The "Top Categories" section renders `[...Array(6)]` grey circles with hardcoded labels "Category 1…6". The `catalog.getCategoryTree` procedure exists but is not called.
**Fix**: Fetch and render real categories with icons and links to `/category/[slug]`.

---

### B6 — `inventoryService.releaseStock` called with `orderId`, not `reservationId`
**File**: `packages/api/modules/order/services/order-service.ts:143`
**Problem**: `await inventoryService.releaseStock(orderId)` is called when an order is CANCELLED. The `releaseStock` function signature expects a `reservationId` (a `StockReservation.id`), not an `orderId`. This results in `null` lookups and stock never being released.
**Fix**: Look up all active `StockReservation` records for the order and release each by its own ID, or add an `releaseStockByOrderId` method.

---

### B7 — `payment.handleWebhook` — Does not trigger order state machine events
**File**: `packages/api/modules/payment/services/payment-service.ts:129`
**Problem**: On webhook success, the code directly calls `tx.order.update({ data: { status: 'PAID' } })`. This bypasses `orderService.updateStatus()`, meaning the side effects of the PAID transition (inventory confirmation, ledger entry, event publishing, package notifications) are **never triggered**.
**Fix**: Replace the direct `tx.order.update` call with `await orderService.updateStatus(payment.orderId, 'PAID')`.

---

### B8 — `payment.payWithWallet` — Same bypass as B7
**File**: `packages/api/modules/payment/services/payment-service.ts:166`
**Problem**: `tx.order.update({ data: { status: 'PAID' } })` is used directly, bypassing the state machine and all PAID side effects.
**Fix**: Call `orderService.updateStatus` after the wallet transaction completes.

---

### B9 — Search page filter inputs are non-functional
**File**: `apps/web/src/app/(buyer)/search/page.tsx`
**Problem**: The Price Min/Max inputs and Brand checkboxes in the sidebar filter UI are not wired to state. Changing them has no effect on the results query.
**Fix**: Add `useState` for `minPrice`, `maxPrice`, `brands`; pass them into `api.catalog.listProducts.useQuery`.

---

### B10 — Category page filter inputs non-functional (same as B9)
**File**: `apps/web/src/app/(buyer)/category/[slug]/page.tsx`
**Problem**: Same pattern — uncontrolled filter inputs with no state management.
**Fix**: Same approach as B9.

---

### B11 — `promoService.validateCoupon` — Missing expiry and min-order checks
**File**: `packages/api/modules/promo/services/promo-service.ts`
**Problem**: The service only checks `usageLimit`. It does not check: (1) whether the promotion is still within its date window (`startDate`/`endDate`), (2) whether the `orderTotal` meets a minimum order value, or (3) whether a per-user usage limit has been hit.
**Fix**: Add date range and minimum order amount validation before returning the promotion.

---

### B12 — `affiliateService.confirmMatureCommissions` — Wrong status filter
**File**: `packages/api/modules/affiliate/services/affiliate-service.ts:88`
**Problem**: The cron queries orders with `status: 'DELIVERED'` and `updatedAt: { lte: yesterday }`. It should be querying for orders with status `COMPLETED` (the final terminal state after the escrow window), not `DELIVERED`. Commissions confirmed prematurely can lead to disputes if the buyer triggers a return.
**Fix**: Change the filter to `status: 'COMPLETED'`.

---

### B13 — `return-service.ts` — Does not fire a `refund.processed` event
**File**: `packages/api/modules/return/services/return-service.ts:40`
**Problem**: `approveReturn` processes the wallet refund but never calls `publishEvent('refund.processed', ...)`. The notification worker has a handler for this event but it is never triggered.
**Fix**: Add `await publishEvent('refund.processed', { userId: ..., orderId: ..., amount: ... })` after the transaction commits.

---

### B14 — `sellerDashboardService` — Revenue double-counts all ledger entries (DEBIT and CREDIT mixed)
**File**: `packages/api/modules/seller/services/seller-dashboard-service.ts:24`
**Problem**: `ledgerEntries.reduce((acc, entry) => acc.add(entry.amount), ...)` sums ALL entries including PENALTY and WITHDRAWAL debit entries, inflating or deflating the revenue figure.
**Fix**: Filter for `type: 'SALE'` or `type: 'COMMISSION'` credit entries only.

---

### B15 — `catalog.syncToSearch` — Hardcoded `rating: 4.5` and `review_count: 10`
**File**: `packages/api/modules/catalog/services/catalog-service.ts`
**Problem**: The search index is populated with mocked rating values. The real `product.averageRating` and `product.reviewCount` fields exist and are populated, but are not used in the search sync.
**Fix**: Pass `variant.product.averageRating?.toNumber() || 0` and `variant.product.reviewCount || 0`.

---

## INCOMPLETE INTEGRATIONS

### I1 — Coupon code UI missing from Checkout
**File**: `apps/web/src/app/(buyer)/checkout/page.tsx`
**Problem**: The `promo.validateCoupon` endpoint exists and the `Coupon` schema is defined, but there is no coupon code input field in the checkout UI. The discount value is never applied to the order total.
**Action**: Add a coupon input field and wire it to `api.promo.validateCoupon`; apply the discount to the displayed total.

---

### I2 — Wallet funding UI missing
**File**: Frontend
**Problem**: `paymentService.fundWallet` and `payment.payWithWallet` procedures exist. There is no UI for a buyer to fund their wallet (e.g., via Paystack top-up). The wallet balance is only shown in a limited context.
**Action**: Add a "Fund Wallet" section to the Account page or a dedicated `/account/wallet` page with a top-up flow.

---

### I3 — Return initiation UI missing from Order Detail page
**File**: `apps/web/src/app/(buyer)/account/orders/[id]/page.tsx`
**Problem**: The `return.initiate` tRPC procedure is implemented, but the order detail page has no "Request Return" button for eligible DELIVERED items.
**Action**: Add a "Return Item" button per order line (visible when `status === 'DELIVERED'`), wired to the `return.initiate` mutation.

---

### I4 — Admin return management UI missing
**File**: Frontend
**Problem**: `return.listPending` and `return.approve` admin procedures exist but there is no admin page to view and act on return requests.
**Action**: Create `apps/web/src/app/(admin)/returns/page.tsx`.

---

### I5 — Flash sale price not applied at cart/checkout time
**File**: `packages/api/modules/cart/services/cart-service.ts`
**Problem**: `cartService.addItem` sets `priceSnapshot: variant.price` (the base price). It never checks `promoService.getFlashSaleForVariant(variantId)` to apply a flash sale discount.
**Action**: Before creating the cart item, query the active flash sale and use `flashSale.discountedPrice` as the `priceSnapshot` if one exists.

---

### I6 — Referral link click tracking not wired in frontend
**File**: Frontend
**Problem**: `affiliateService.recordClick` exists, but no frontend middleware or route handler reads the `?ref=SLUG` query parameter and records the click. Referral attribution only works if buyers land via a tracked link and cookies are manually set.
**Action**: In the Next.js root layout or middleware, detect `?ref=` in the URL, call `affiliate.recordClick`, and store the `referralLinkId` in a cookie for use at checkout.

---

### I7 — Product ratings on the PDP are hardcoded
**File**: `apps/web/src/app/(buyer)/products/[slug]/page.tsx:53`
**Problem**: The star rating and review count are rendered as hardcoded values `(124 ratings)` and 4 filled stars. The `review.getProductRatingSummary` procedure exists.
**Action**: Fetch and display live rating data from `api.review.getProductRatingSummary`.

---

### I8 — Product reviews section missing from PDP
**File**: `apps/web/src/app/(buyer)/products/[slug]/page.tsx`
**Problem**: There is no reviews section below the product detail. `review.getProductReviews` exists and `review.create` is implemented for eligible buyers.
**Action**: Add a Reviews section to the PDP showing existing reviews and a "Write a Review" form for users with a delivered order for this product.

---

### I9 — "Sell on Jumia" CTA on homepage is a dead button
**File**: `apps/web/src/app/page.tsx`
**Problem**: The "REGISTER NOW" button is a `<button>` with no `onClick` or `href`. It should link to the seller onboarding flow.
**Action**: Change to `<Link href="/seller/register">` or link to the IAM onboarding form.

---

### I10 — Navbar notification bell is missing
**File**: `apps/web/src/components/layout/Navbar.tsx`
**Problem**: The `NotificationInbox` component exists at `components/layout/NotificationInbox.tsx` but is not rendered inside `Navbar.tsx`. Buyers have no way to access the notification center from the main nav.
**Action**: Import and render `<NotificationInbox />` in the Navbar alongside the cart button.

---

### I11 — Admin dispute queue not shown on admin dashboard
**File**: `apps/web/src/app/(admin)/dashboard/page.tsx`
**Problem**: `adminRouter.getDisputeQueue` exists but is not surfaced on the admin dashboard — there is no shortcut to escalated disputes.
**Action**: Add a dispute queue summary card to the admin dashboard.

---

### I12 — `wallet.fundWallet` has no webhook callback integration
**File**: `packages/api/modules/payment/services/payment-service.ts`
**Problem**: `initializePaystack` creates an order payment but `fundWallet` has no corresponding payment provider flow. Wallet funding has to be processed manually by calling `fundWallet` directly with a pre-known amount — no payment verification step exists.
**Action**: Add a `payment.initializeWalletTopUp` procedure that creates a special Paystack transaction, and handle its webhook separately to call `fundWallet` on success.

---

## FEATURES TO ADD

### F1 — Live Flash Sale Countdown Timer
Implement a real `useCountdown(endTime)` hook on both `FlashSales.tsx` and `FlashSalesPage`. When the timer expires, invalidate the `promo.getFlashSales` query automatically.

---

### F2 — Search Filter State (Price Range & Brand)
Wire the filter sidebar controls on the Search and Category pages to `useSearchParams` or local state, and pass `minPrice`, `maxPrice`, and `brandIds[]` to the `catalog.listProducts` query.

---

### F3 — Sort-By functional wiring on Search page
The sort-by `<select>` dropdown on the search page is currently static. Wire its value to the `sortBy` query parameter (`price_asc`, `price_desc`, `newest`).

---

### F4 — "Add to Wishlist" from Product Listing Card
`ProductCard.tsx` has no wishlist action. Add a heart button that calls `catalog.addToWishlist`, toggling fill state based on whether the variant is already in the user's wishlist.

---

### F5 — Seller KYC Document Upload UI
`sellerDashboardService.uploadDocument` and `sellerDashboardService.getKYCStatus` are implemented but there is no UI in the seller hub to upload CAC, utility bill, etc. Add a KYC section to the seller onboarding or settings page.

---

### F6 — Seller Performance Score Visibility
The performance score is calculated in `sellerDashboardService.getMetrics` but the Seller Dashboard page does not prominently display it. Surface it as a progress meter on `(seller)/seller/dashboard`.

---

### F7 — Admin — Returns Queue page
A dedicated page at `(admin)/returns` to list, review, and approve/reject pending return requests. The backend procedures exist (`return.listPending`, `return.approve`).

---

### F8 — Wallet page for Buyers
A `/account/wallet` page showing: current balance, last 10 transactions, and a "Top Up" button that initiates a Paystack payment flow.

---

### F9 — Product Search Autocomplete
The autocomplete trie is built nightly in Redis (`zadd autocomplete_trie`), but the Navbar search input has no autocomplete dropdown. Add a `catalog.autocomplete` query that reads from Redis and renders suggestions below the search bar.

---

### F10 — Seller — Order fulfillment actions
The seller orders page (`(seller)/seller/orders/page.tsx`) likely only shows order data. Add "Mark as Shipped" and "Upload Tracking Number" actions wired to `logistics.updateShipmentStatus`.

---

## SUMMARY TABLE

| ID | Category | Severity | File / Area |
|----|----------|----------|-------------|
| B1 | Bug | High | `FlashSales.tsx` — wrong endpoint, fake timer |
| B2 | Bug | High | `/flash-sales` — static shell |
| B3 | Bug | High | `/account/saved` — wishlist not queried |
| B4 | Bug | High | `ProductActions.tsx` — wishlist button no-op |
| B5 | Bug | Medium | `page.tsx` — placeholder category grid |
| B6 | Bug | High | `order-service.ts` — stock release uses wrong ID |
| B7 | Bug | Critical | `payment-service.ts` — webhook bypasses state machine |
| B8 | Bug | Critical | `payment-service.ts` — wallet bypass same |
| B9 | Bug | Medium | `/search` — filters non-functional |
| B10 | Bug | Medium | `/category/[slug]` — filters non-functional |
| B11 | Bug | Medium | `promo-service.ts` — missing expiry/min-order check |
| B12 | Bug | Medium | `affiliate-service.ts` — wrong order status filter |
| B13 | Bug | Medium | `return-service.ts` — missing `refund.processed` event |
| B14 | Bug | Medium | `seller-dashboard-service.ts` — revenue double-counts |
| B15 | Bug | Low | `catalog-service.ts` — hardcoded rating in search |
| I1 | Integration | High | Coupon UI missing from Checkout |
| I2 | Integration | Medium | Wallet funding UI missing |
| I3 | Integration | High | Return initiation UI missing from Order Detail |
| I4 | Integration | Medium | Admin return management UI missing |
| I5 | Integration | High | Flash sale price not applied to cart |
| I6 | Integration | Medium | Referral click tracking not wired in frontend |
| I7 | Integration | Medium | PDP ratings are hardcoded |
| I8 | Integration | Medium | PDP has no reviews section |
| I9 | Integration | Low | "Sell on Jumia" CTA is dead button |
| I10 | Integration | Medium | Notification bell missing from Navbar |
| I11 | Integration | Low | Admin dispute queue not on dashboard |
| I12 | Integration | Medium | Wallet top-up has no payment verification flow |
| F1 | Feature | High | Live flash sale countdown |
| F2 | Feature | High | Search price/brand filter state |
| F3 | Feature | Medium | Sort-by wiring on search |
| F4 | Feature | Medium | Add-to-wishlist from product card |
| F5 | Feature | Medium | Seller KYC document upload UI |
| F6 | Feature | Low | Seller performance score UI |
| F7 | Feature | Medium | Admin returns queue page |
| F8 | Feature | Medium | Buyer wallet page |
| F9 | Feature | Medium | Search autocomplete dropdown |
| F10 | Feature | Medium | Seller order fulfillment actions |
