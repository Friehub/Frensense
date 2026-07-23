// SAFE: Stripped newline and carriage return characters from user input before passing to the logger.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function sanitizeLogInput(input: string): string {
    return input.replace(/[\n\r]/g, "_");
}

function handleLogin(req: Request, res: Response) {
    const username = sanitizeLogInput(req.body.username);
    logger.info(`Login attempt from user: ${username}`);
    res.json({ success: true });
}

function handlePayment(req: Request, res: Response) {
    const note = sanitizeLogInput(req.body.note);
    logger.info(`Payment note: ${note}`);
    res.json({ success: true });
}
