// [frensense]
// observation: Credit card numbers from user input are directly logged without masking, exposing sensitive financial information in log files.
// impact: An attacker with access to log files can extract credit card numbers, violating PCI DSS compliance and enabling financial fraud.
// improvement: Mask or truncate credit card numbers before logging, or avoid logging them entirely.
// cwe: CWE-117
// cvss: 5.3
// owasp: A09:2021
// severity: Medium

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function processPayment(req: Request, res: Response) {
    const creditCard = req.body.creditCard;
    const amount = req.body.amount;
    logger.info(`Payment of ${amount} from card ${creditCard} processed`);
    res.json({ success: true });
}

function logPaymentDetails(req: Request, res: Response) {
    const { cardNumber, cvv, expiry, name } = req.body;
    logger.info(`Card payment: ${cardNumber}, CVV: ${cvv}, Expiry: ${expiry}, Name: ${name}`);
    res.json({ success: true });
}
