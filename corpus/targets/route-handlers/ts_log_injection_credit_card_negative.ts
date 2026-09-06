// SAFE: Masked credit card numbers before logging, showing only the last 4 digits.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function maskCardNumber(card: string): string {
    const cleaned = card.replace(/\D/g, "");
    if (cleaned.length >= 4) {
        return "****-****-****-" + cleaned.slice(-4);
    }
    return "****";
}

function processPayment(req: Request, res: Response) {
    const creditCard = maskCardNumber(req.body.creditCard);
    logger.info(`Payment of ${req.body.amount} from card ${creditCard} processed`);
    res.json({ success: true });
}

function logPaymentDetails(req: Request, res: Response) {
    logger.info("Card payment processed", {
        lastFour: req.body.cardNumber.slice(-4),
        expiry: "**/**",
        name: req.body.name,
    });
    res.json({ success: true });
}
