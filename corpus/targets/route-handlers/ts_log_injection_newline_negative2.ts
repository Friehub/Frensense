// SAFE: Used structured JSON logging where user input is passed as a separate field, preventing log line injection.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function handleLogin(req: Request, res: Response) {
    logger.info("Login attempt", { username: req.body.username, ip: req.ip });
    res.json({ success: true });
}

function handlePayment(req: Request, res: Response) {
    logger.info("Payment note added", { note: req.body.note, userId: req.user.id });
    res.json({ success: true });
}
