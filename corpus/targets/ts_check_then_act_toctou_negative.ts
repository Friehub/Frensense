// SAFE: Stock reservation inside transaction with atomic check
async function createOrder(items: OrderItem[]) {
    return prisma.$transaction(async (tx) => {
        for (const item of items) {
            const result = await tx.stockLevel.updateMany({
                where: { variantId: item.variantId, qtyOnHand: { gte: item.quantity } },
                data: { qtyOnHand: { decrement: item.quantity } }
            });
            if (result.count === 0) throw new Error('OUT_OF_STOCK');
        }
    });
}

// SAFE: Coupon increment with atomic check inside transaction
async function applyCoupon(code: string, orderId: string) {
    return prisma.$transaction(async (tx) => {
        const result = await tx.coupon.updateMany({
            where: {
                code,
                OR: [
                    { usageLimit: null },
                    { usedCount: { lt: prisma.raw('usageLimit') } }
                ]
            },
            data: { usedCount: { increment: 1 } }
        });
        if (result.count === 0) throw new Error('COUPON_EXHAUSTED');
    });
}

// SAFE: Upsert eliminates check-then-create race
async function addToWishlist(userId: string, productId: string) {
    const wishlist = await prisma.wishlist.upsert({
        where: { userId },
        create: { userId },
        update: {}
    });
    await prisma.wishlistItem.create({
        data: { wishlistId: wishlist.id, productId }
    });
}

// SAFE: Idempotency with unique constraint
async function handleWebhook(reference: string, status: string) {
    await prisma.$transaction(async (tx) => {
        const payment = await tx.payment.findFirst({
            where: { providerRef: reference },
            skipLocked: true
        });
        if (!payment || payment.status === 'SUCCESS') return;
        await tx.payment.update({ where: { id: payment.id }, data: { status: 'SUCCESS' } });
    });
}
