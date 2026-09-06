// SAFE: Avoid logging credit card details entirely; log only a transaction reference.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function processPayment(req: Request, res: Response) {
    const transactionId = `txn_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`;
    logger.info("Payment processed", {
        transactionId,
        amount: req.body.amount,
        userId: req.user.id,
    });
    res.json({ transactionId, success: true });
}

function logPaymentDetails(req: Request, res: Response) {
    logger.info("Card payment details logged", {
        userId: req.user.id,
        amount: req.body.amount,
    });
    res.json({ success: true });
}
