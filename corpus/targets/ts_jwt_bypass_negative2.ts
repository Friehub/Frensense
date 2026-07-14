// [frensense]
// observation: jwt.decode used for non-auth purposes — extracting claims for logging or display only.
// impact: None — decode without verification is acceptable when the result is not used for access control.
// improvement: N/A — this is the correct pattern for non-auth inspection.

import jwt from "jsonwebtoken";

function extractAuditMetadata(token: string): { sub?: string; iat?: number } | null {
    // Decode only for logging — not used for authentication
    const payload = jwt.decode(token);
    if (!payload || typeof payload === "string") return null;
    return { sub: payload.sub as string | undefined, iat: payload.iat };
}

function getTokenExpiry(token: string): Date | null {
    // Display purposes only — shows the expiry date to the user
    const decoded = jwt.decode(token, { complete: true });
    if (!decoded || typeof decoded.payload === "string") return null;
    const exp = (decoded.payload as { exp?: number }).exp;
    return exp ? new Date(exp * 1000) : null;
}

function logTokenSubject(token: string, logger: any): void {
    // Extract subject for logging correlation — not for authorization
    const claims = jwt.decode(token);
    if (claims && typeof claims === "object") {
        logger.info({ sub: claims.sub }, "processing request");
    }
}
