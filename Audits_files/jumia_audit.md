# Jumia Clone — Bug & Feature Audit
> Scanned: 2026-05-07 | Branch: staging

---

## Legend
- **[CRASH]** — Runtime error, causes visible break
- **[WRONG]** — Works but produces incorrect data
- **[STUB]** — Page exists, uses hardcoded/fake data instead of API
- **[MISSING]** — Feature wired on frontend but not guarded or implemented
- **[MINOR]** — UX issue, not a crash

---

## Confirmed Bugs (Already Fixed This Session)

| # | File | Bug | Status |
|---|------|-----|--------|
| 1 | `CartDrawer.tsx` | `cart.total.toLocaleString()` crash — `total` never returned by API | **FIXED** |
| 2 | `CartDrawer.tsx` | `item.variant.price` used for line total — should use `priceSnapshot` | **FIXED** |
| 3 | `catalog/router/index.ts` | `getWishlist` as `protectedProcedure` → 401 for guests on product pages | **FIXED** |

---

## Active Bugs — To Fix

### [CRASH] Cart Page & Checkout — `item.price` doesn't exist
**Severity: HIGH**

Both `apps/web/src/app/(buyer)/cart/page.tsx` (line 11, 83) and
`apps/web/src/app/(buyer)/checkout/page.tsx` (line 115) compute subtotal as:
```js
item.price * item.quantity  // item.price is undefined
```
The cart API returns `item.priceSnapshot`, not `item.price`. This makes the
subtotal **₦0** on both the cart and checkout pages even when items exist.

**Files:**
- `apps/web/src/app/(buyer)/cart/page.tsx` — lines 11, 83
- `apps/web/src/app/(buyer)/checkout/page.tsx` — line 115

**Fix:** Replace `item.price` with `Number(item.priceSnapshot ?? 0)` on both pages.

---

### [MISSING] Wishlist toggle — no auth guard
**Severity: MEDIUM**

`ProductActions.tsx` calls `addToWishlist.mutateAsync()` with no session
check. If a guest user clicks the heart button, the mutation fires, hits
the `protectedProcedure` endpoint, and throws a 401 TRPC error — but the
UI shows nothing. The user has no idea why it didn't work.

**File:** `apps/web/src/components/products/ProductActions.tsx` — `handleWishlistToggle`

**Fix:** Check session before calling the mutation. If not logged in, redirect
to `/login?callbackUrl=...` or show a toast: "Sign in to save items."

---

### [STUB] Reviews page — fully hardcoded, no API connected
**Severity: MEDIUM**

`apps/web/src/app/(buyer)/account/reviews/page.tsx` uses local React state
with hardcoded mock data. The review router exists (`api.review.*`) but is
never called. The "Create Review" form only updates local state — nothing
persists.

**File:** `apps/web/src/app/(buyer)/account/reviews/page.tsx`

**Fix:** Wire to `api.review.getProductReviews` (or the user's own reviews endpoint)
and `api.review.create` mutation. Need to confirm exact endpoint names in the
review router.

---

### [STUB] Best Sellers page — static placeholder
**Severity: LOW**

`apps/web/src/app/(buyer)/best-sellers/page.tsx` renders a static
"We're updating our best sellers list. Stay tuned!" message.
No API call is made.

**File:** `apps/web/src/app/(buyer)/best-sellers/page.tsx`

**Fix:** Wire to `api.catalog.listProducts` with `sortBy: 'best_selling'`
(needs backend support) or fallback to `sortBy: 'newest'` with a label.
This is a quick win — the component structure already exists on the products
listing page.

---

### [MISSING] Notification delete — button not wired
**Severity: LOW**

`apps/web/src/app/(buyer)/notifications/page.tsx` renders a `Trash2` icon
on each notification row but there is no `notification.delete` procedure
in the router (`packages/api/modules/notification/router/index.ts` only has
`list`, `getUnread`, `markAsRead`, `markAllAsRead`). The delete button either
does nothing or is not present.

**Action needed:** Either add a `delete` procedure to the notification router
and wire it, or remove the trash icon from the UI if delete is not a planned
feature.

---

### [MINOR] Checkout `cart?.totalItems` is always 0
**Severity: LOW**

`apps/web/src/app/(buyer)/checkout/page.tsx` line 233 references
`cart?.totalItems` — but the cart API response shape has no `totalItems`
field. This is the same class of error as the `cart.total` bug. The item
count in the checkout summary will always show `(0)`.

**Fix:** Compute inline: `cart?.items?.length || 0`

---

### [MISSING] Paystack webhook — only handles `charge.success`
**Severity: MEDIUM**

`apps/web/src/app/api/webhooks/paystack/route.ts` only handles the
`charge.success` event. Production Paystack integrations also fire:
- `transfer.success` / `transfer.failed` — for seller payouts
- `charge.failed` — should update order to `PAYMENT_FAILED`
- `refund.processed` — for returns

Currently a failed charge leaves the order in a pending state permanently.

**File:** `apps/web/src/app/api/webhooks/paystack/route.ts`

**Fix:** Add handlers for `charge.failed` and `transfer.success`/`transfer.failed`
at minimum.

---

### [WRONG] Checkout — `subtotal` based on `item.price` (undefined)
Already covered under the Cart Page bug above. Same root cause, same fix.

---

## Unimplemented Features (Stubs / Placeholders)

| Feature | Page/File | Status |
|---------|-----------|--------|
| Best Sellers | `(buyer)/best-sellers/page.tsx` | Placeholder text. No API call. |
| User Reviews | `(buyer)/account/reviews/page.tsx` | Hardcoded mock data, local state only |
| Official Stores | `(buyer)/official-stores/page.tsx` | Unknown — not scanned yet |
| Jumia Global | `(buyer)/jumia-global/page.tsx` | Unknown — not scanned yet |
| Affiliate Program | `app/affiliate/page.tsx` | Unknown — router exists (`affiliateRouter`) |
| Seller Disputes | `(seller)/seller/disputes/[id]/page.tsx` | Unknown — router exists |
| Seller Advertising | `(seller)/seller/advertising/create/page.tsx` | Unknown — router exists |
| Admin Fraud Queue | `(admin)/fraud/page.tsx` | Unknown — router exists |
| Admin Sellers | `(admin)/sellers/page.tsx` | Unknown — router exists |

---

## Fix Priority Order

| Priority | Issue | Effort |
|----------|-------|--------|
| 1 | Cart/Checkout `item.price` → `priceSnapshot` | XS — 2 line changes per file |
| 2 | Wishlist auth guard with redirect/toast | S — ~10 lines |
| 3 | Checkout `cart?.totalItems` → `cart?.items?.length` | XS — 1 line |
| 4 | Paystack webhook `charge.failed` handler | S — ~15 lines |
| 5 | Best Sellers — wire to products API | S — use existing ProductCard grid |
| 6 | Notification delete — decision required (add or remove) | M |
| 7 | Reviews — wire to real API | M |
