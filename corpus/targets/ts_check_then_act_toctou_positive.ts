// TOCTOU: Stock check outside transaction, reservation inside
async function createOrder(items: OrderItem[]) {
    for (const item of items) {
        const stock = await prisma.stockLevel.findFirst({
            where: { variantId: item.variantId, qtyOnHand: { gte: item.quantity } }
        });
        if (!stock) throw new Error('OUT_OF_STOCK');
    }

    return prisma.$transaction(async (tx) => {
        for (const item of items) {
            await tx.stockLevel.update({
                where: { variantId: item.variantId },
                data: { qtyOnHand: { decrement: item.quantity } }
            });
        }
    });
}

// TOCTOU: Coupon usage check outside transaction
async function applyCoupon(code: string, orderId: string) {
    const coupon = await prisma.coupon.findUnique({ where: { code } });
    if (coupon.usageLimit && coupon.usedCount >= coupon.usageLimit) {
        throw new Error('COUPON_EXHAUSTED');
    }

    await prisma.$transaction(async (tx) => {
        await tx.coupon.update({
            where: { code },
            data: { usedCount: { increment: 1 } }
        });
    });
}

// TOCTOU: Wishlist check-then-create
async function addToWishlist(userId: string, productId: string) {
    let wishlist = await prisma.wishlist.findUnique({ where: { userId } });
    if (!wishlist) {
        wishlist = await prisma.wishlist.create({ data: { userId } });
    }
    await prisma.wishlistItem.create({
        data: { wishlistId: wishlist.id, productId }
    });
}

// TOCTOU: Payment idempotency check outside transaction
async function handleWebhook(reference: string, status: string) {
    const payment = await prisma.payment.findFirst({ where: { providerRef: reference } });
    if (payment.status === 'SUCCESS') return;

    await prisma.$transaction(async (tx) => {
        await tx.payment.update({ where: { id: payment.id }, data: { status: 'SUCCESS' } });
    });
}
