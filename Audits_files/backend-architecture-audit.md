# Production-Grade Hardening Audit: Backend & Database Relational Architecture

**Scope:** Advanced logic verification, financial transaction safety, relational schema design, concurrency, and authorization boundaries.  
**Date:** May 19, 2026  
**Target Base:** `jumia-clone/packages/` (API and DB packages)

---

## Executive Summary

This report acts as a supplementary architectural and logic audit of the ecommerce backend and database schemas. The focus is placed on vulnerabilities, race conditions, relational integrity gaps, and financial validation bugs that must be resolved to transition the codebase from a working prototype to a production-grade ecommerce platform. 

This audit does not duplicate any of the 40 issues identified in the baseline `backend-audit.md` report. Instead, it uncovers deeper transactional, boundary, and referential vulnerabilities across the platform modules.

| Severity | Count | Focus Areas |
|----------|-------|-------------|
| CRITICAL | 4     | Financial leaks, double-spending, escrow leaks, and cross-talk vulnerabilities. |
| HIGH     | 3     | Authorization bypasses, review system manipulation, and database locks. |
| MEDIUM   | 3     | Stock pollution, missing support admin tools, and rating aggregation errors. |

---

## 1. Financial & Settlement Logic Vulnerabilities

### [CRITICAL] Seller Coupon Over-Scoping (Systemic Platform Financial Drain)
*   **Location:** `packages/api/modules/promo/services/promo-service.ts` & `packages/api/modules/order/services/order-service.ts`
*   **Vulnerability:** 
    In `promo-service.ts` (`validateCoupon`), a coupon issued by a specific seller is validated by checking if *any* cart item belongs to that seller:
    ```typescript
    const isSellerApplicable = cart.items.some(item => item.sellerId === promo.sellerId);
    ```
    However, in `order-service.ts` (`createFromCart`), if the coupon is validated, the discount is calculated against the **entire order subtotal** (across all sellers) rather than only the subtotal of the items belonging to that specific seller:
    ```typescript
    const discount = coupon.type === 'PERCENTAGE' 
      ? subtotal.mul(coupon.value).div(100) 
      : coupon.value;
    ```
*   **Impact:** 
    A buyer can exploit this by adding a low-cost item (e.g., ₦100) from Seller A (who offers a 50% discount coupon) and a high-cost item (e.g., ₦1,000,000) from Seller B to their cart. The validation passes because Seller A's item is in the cart. The system then applies the 50% discount to the entire ₦1,000,100 subtotal, resulting in a ₦500,050 discount. This causes an immediate, massive financial drain on the platform.
*   **Remediation:** 
    Modify the discount calculation in `order-service.ts` to calculate the subtotal of only the items belonging to the coupon's issuing seller (`coupon.sellerId`), and apply the percentage or fixed discount exclusively to that seller-specific subtotal.

---

### [CRITICAL] Unreconciled Return Refunds (Seller Double-Payout Escrow Leak)
*   **Location:** `packages/api/modules/return/services/return-service.ts` & `packages/api/modules/revenue/services/ledger-service.ts`
*   **Vulnerability:** 
    When an administrator approves a return via `returnService.approveReturn()`, the buyer is refunded the item price back to their wallet:
    ```typescript
    const refundAmount = request.orderLine.unitPrice.mul(request.orderLine.quantity);
    await paymentService.fundWallet(request.orderLine.package.order.userId, refundAmount.toNumber(), tx);
    ```
    However, the ledger service is never updated. While the `LedgerEntryType` enum in `seller.prisma` defines a `REFUND` type, there is no code in the entire backend service that actually invokes or creates a `REFUND` ledger entry.
*   **Impact:** 
    The seller's ledger entry for the original sale remains active. Because the system does not record a matching debit entry of type `REFUND` against the seller, the seller will still receive the full payout for the returned item. The platform suffers a double loss: paying out the customer for the refund, and paying out the seller for a returned product.
*   **Remediation:** 
    Implement a `recordRefund` function in `ledger-service.ts` that writes a negative `LedgerEntryType.REFUND` entry against the seller's ledger. Invoke this within the `approveReturn` database transaction block to ensure atomic reversal of the escrowed funds.

---

### [CRITICAL] Missing Discount Ledger Apportionment (Accounting Discrepancy)
*   **Location:** `packages/api/modules/revenue/services/ledger-service.ts` (`recordSale`)
*   **Vulnerability:** 
    `ledgerService.recordSale` aggregates the seller ledger entry using the original `unitPrice` and `quantity` from `OrderLine`:
    ```typescript
    const grossAmount = line.unitPrice.mul(line.quantity);
    ```
    This calculation completely ignores any applied promotions or coupon discounts. There is no mechanism inside the ledger service or the database schema to track or apportion discounts per line item or order package.
*   **Impact:** 
    Sellers are paid out the full original amount of their items even when a discount has been applied. If a coupon reduces an item's price by 20%, the customer pays 80%, but the platform records a sale of 100% gross to the seller's account. This produces an accounting mismatch between actual received customer funds and expected payouts.
*   **Remediation:** 
    Add a `discountApplied` field to the `OrderLine` and `OrderPackage` schemas. When creating an order, distribute the coupon discount across applicable lines (pro-rata based on item value). Update `recordSale` to calculate `grossAmount = (unitPrice * quantity) - discountApplied`.

---

## 2. Concurrency & Identity Gaps

### [CRITICAL] Webhook Idempotency Race Condition (Wallet Funding Double-Spend)
*   **Location:** `packages/api/modules/payment/services/payment-service.ts` (`handleWebhook`)
*   **Vulnerability:** 
    The payment webhook handler checks if the payment record status is already `SUCCESS` before initiating the database transaction:
    ```typescript
    if (payment.status === 'SUCCESS') {
      return;
    }
    ```
    If the status is `PENDING`, it enters the transaction block:
    ```typescript
    await prisma.$transaction(async (tx) => {
      await tx.payment.update({
        where: { id: payment.id },
        data: { status: 'SUCCESS' }
      });
      if (payment.orderId === 'WALLET_FUND') {
        await this.fundWallet(payment.userId, payment.amount.toNumber(), tx);
      }
    ...
    ```
    This status check is done outside the database transaction without any row lock or isolation. Additionally, the transaction's update query on the `Payment` table uses `{ id: payment.id }` which is a valid database UPDATE even if the status is already `SUCCESS`.
*   **Impact:** 
    If an attacker issues multiple concurrent requests to the webhook endpoint (or if the payment provider retries the webhook concurrently), both requests can read `payment.status === 'PENDING'` simultaneously. Both will enter the transaction block, execute the update successfully, and call `fundWallet` multiple times. This allows a user to multiply their wallet funds through a basic concurrency exploit.
*   **Remediation:** 
    Incorporate the status condition directly into the update filter inside the transaction:
    ```typescript
    await tx.payment.update({
      where: { id: payment.id, status: 'PENDING' },
      data: { status: 'SUCCESS' }
    });
    ```
    If a concurrent request tries to update the payment after the first has completed, the query will throw a "Record not found" error due to the status mismatch, rolling back the transaction and preventing the duplicate wallet credit.

---

### [HIGH] Unverified Seller Profile Status Bypass
*   **Location:** `packages/api/trpc.ts` (`sellerProcedure`) & `packages/api/modules/seller/router/index.ts`
*   **Vulnerability:** 
    During onboarding (`sellerService.onboard`), the user's role is updated to `'SELLER'` immediately, even though their seller profile status is set to `'PENDING_VERIFICATION'`.
    The `sellerProcedure` middleware in `trpc.ts` only validates that the user's role is `'SELLER'`:
    ```typescript
    export const sellerProcedure = protectedProcedure.use(({ ctx, next }) => {
      if (ctx.session.user.role !== "SELLER" && ctx.session.user.role !== "ADMIN") {
        throw new TRPCError({ code: "FORBIDDEN" });
      }
      return next();
    });
    ```
    None of the procedures in `_sellerRouter` (e.g., `listMyProducts`, `createCoupon`, `bulkActivateProducts`, `getDashboardMetrics`) fetch the seller profile to verify its status.
*   **Impact:** 
    Sellers whose profiles are `PENDING_VERIFICATION`, `SUSPENDED`, or `REJECTED` can still successfully perform all seller activities, including listing products, creating coupons, updating inventories, and querying financial reports.
*   **Remediation:** 
    Modify the `sellerProcedure` middleware to query the database and assert that the associated `Seller` profile status is `'ACTIVE'`.
    ```typescript
    export const sellerProcedure = protectedProcedure.use(async ({ ctx, next }) => {
      const seller = await prisma.seller.findUnique({
        where: { userId: ctx.session.user.id },
        select: { status: true }
      });
      if (!seller || (seller.status !== 'ACTIVE' && ctx.session.user.role !== 'ADMIN')) {
        throw new TRPCError({ code: "FORBIDDEN", message: "Seller profile is not active" });
      }
      return next();
    });
    ```

---

### [HIGH] Competitor Stock Update & Stock Level Pollution
*   **Location:** `packages/api/modules/inventory/router/index.ts` & `packages/api/modules/inventory/services/inventory-service.ts`
*   **Vulnerability:** 
    The `inventoryRouter.updateStock` procedure calls `inventoryService.updateStockBatch()` passing the authenticated seller's ID and an array of product variant updates.
    However, `inventoryService.updateStockBatch` uses a Prisma `upsert` query without validating that the `variantId` actually belongs to a product owned by that specific seller:
    ```typescript
    const stock = await tx.stockLevel.upsert({
      where: {
        variantId_sellerId_warehouseId: {
          variantId: update.variantId,
          sellerId,
          warehouseId
        }
      },
      update: { qtyOnHand: update.quantity },
      create: {
        variantId: update.variantId,
        sellerId,
        warehouseId,
        qtyOnHand: update.quantity,
        qtyReserved: 0
      }
    });
    ```
*   **Impact:** 
    If a seller submits an update with a `variantId` belonging to a competitor's product, the upsert will succeed. It will either create a new `StockLevel` record associating the competitor's variant with the calling seller, or update an existing one, leading to stock calculation corruption and database pollution.
*   **Remediation:** 
    Before running the database upsert in `updateStockBatch`, execute a query to verify that the `variantId` is associated with a product where `sellerId` matches the caller's `sellerId`:
    ```typescript
    const variant = await tx.productVariant.findUnique({
      where: { id: update.variantId },
      include: { product: { select: { sellerId: true } } }
    });
    if (!variant || variant.product.sellerId !== sellerId) {
      throw new Error(`UNAUTHORIZED_VARIANT_UPDATE: ${update.variantId}`);
    }
    ```

---

## 3. Relational & Functional Gaps

### [HIGH] Product Deletion Cascading Constraints
*   **Location:** `packages/db/prisma/schema/catalog.prisma` & `packages/db/prisma/schema/wishlist.prisma`
*   **Vulnerability:** 
    The relations between `ProductVariant` and transactional/tracking models such as `WishlistItem` and `CartItem` do not configure cascading deletions. For instance, in `wishlist.prisma`:
    ```prisma
    model WishlistItem {
      ...
      variant  ProductVariant @relation(fields: [variantId], references: [id])
    }
    ```
*   **Impact:** 
    If a seller or administrator attempts to delete a product or variant, and any guest or registered user has that specific item in their active shopping cart or personal wishlist, the database will throw a foreign key constraint violation error and block the deletion. Product management becomes unusable for active inventory items.
*   **Remediation:** 
    Configure the relations in the database schemas to support cascading deletes:
    ```prisma
    variant  ProductVariant @relation(fields: [variantId], references: [id], onDelete: Cascade)
    ```

---

### [MEDIUM] Dispute Order Line Cross-Talk & Escrow Lockout
*   **Location:** `packages/api/modules/dispute/services/dispute-service.ts` (`openDispute`)
*   **Vulnerability:** 
    When creating a dispute, the user provides both an `orderId` and an `orderLineId`. The service validates that the `orderId` belongs to the calling buyer, but does not verify that the `orderLineId` is actually a child line of that specific order:
    ```typescript
    if (orderLineId) {
      const line = await orderLineService.findUnique({
        where: { id: orderLineId },
        include: { package: true }
      });
      if (line) sellerId = line.package.sellerId;
    }
    ```
*   **Impact:** 
    A buyer can create a dispute referencing their own valid order, but supply an `orderLineId` belonging to an entirely different user's order and seller. This allows malicious buyers to target competitor sellers, creating illegitimate disputes that automatically freeze their escrow balances and trigger unwarranted auto-escalation check cron jobs.
*   **Remediation:** 
    Enforce a strict ownership check inside the dispute creation logic to ensure the target order line is a direct child of the validated order:
    ```typescript
    if (orderLineId) {
      const line = await orderLineService.findFirst({
        where: { id: orderLineId, package: { orderId: orderId } }
      });
      if (!line) throw new Error('INVALID_ORDER_LINE');
      sellerId = line.package.sellerId;
    }
    ```

---

### [MEDIUM] Review Aggregation Pollution & Stale Moderation State
*   **Location:** `packages/api/modules/review/services/review-service.ts`
*   **Vulnerability:** 
    1.  Reviews are created with a default status of `PENDING`. However, the ratings aggregation query in `createReview` runs a database aggregation over *all* reviews for the product, without filtering for status:
        ```typescript
        const productStats = await reviewService.aggregate({
          where: { productId },
          _avg: { rating: true },
          _count: { _all: true }
        });
        ```
    2.  When an administrator approves or rejects a review via `moderateReview()`, the rating status is updated, but the service does not trigger a recalculation of the product or seller average ratings.
*   **Impact:** 
    Pending and rejected reviews are immediately included in the calculated average rating of the product and the seller. If a review is later rejected due to spam or terms violations, its rating value is never removed from the average rating, leaving the metrics permanently corrupted.
*   **Remediation:** 
    1.  Update the ratings aggregation query in `createReview` to only aggregate reviews with an `APPROVED` status:
        ```typescript
        where: { productId, status: 'APPROVED' }
        ```
    2.  Add ratings recalculation logic inside the `moderateReview` function so that approval or rejection triggers a fresh, accurate calculation of the seller and product stats.

---

### [MEDIUM] Missing Support Agent Routing APIs
*   **Location:** `packages/api/modules/support/router/index.ts`
*   **Vulnerability:** 
    While the database schemas define the structures for support tickets and messages, the `_supportRouter` only exposes guest/buyer-facing endpoints: `getActiveTicket`, `getMessages`, and `sendMessage`. There are no matching tRPC endpoints or routers exposed to allow support agents or administrators to list pending tickets, assign tickets to agents, close tickets, or send replies.
*   **Impact:** 
    The support ticketing module remains a one-way system. Buyers can create tickets and write messages, but there is no interface or API surface for the customer service team to view or respond to them, rendering the feature non-functional.
*   **Remediation:** 
    Extend the support router to include agent procedures secured with `agentProcedure` or `adminProcedure`:
    ```typescript
    listTickets: agentProcedure
      .input(z.object({ status: z.nativeEnum(SupportStatus).optional() }))
      .query(async ({ input }) => { ... }),

    assignTicket: agentProcedure
      .input(z.object({ ticketId: z.string() }))
      .mutation(async ({ ctx, input }) => { ... }),

    replyAsAgent: agentProcedure
      .input(z.object({ ticketId: z.string(), content: z.string() }))
      .mutation(async ({ ctx, input }) => { ... })
    ```
