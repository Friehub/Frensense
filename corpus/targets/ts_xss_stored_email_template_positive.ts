// [frensense]
// observation: User-controlled data (recipient name, subject, or body) is embedded in an HTML email template without sanitization, enabling XSS in email clients that render HTML.
// impact: A recipient's email client (e.g., Gmail, Outlook) may execute JavaScript embedded in the email, or the email content may be used to phish the recipient.
// improvement: Sanitize all user data before inserting into email templates, or use a plain-text alternative.

import nodemailer from "nodemailer";

const transport = nodemailer.createTransport({ /* config */ });

export async function sendWelcomeEmail(user: { name: string; email: string }) {
    await transport.sendMail({
        from: "noreply@example.com",
        to: user.email,
        subject: "Welcome!",
        html: `<h1>Welcome, ${user.name}!</h1><p>Thanks for joining.</p>`,
    });
}

export async function sendNotification(to: string, subject: string, message: string) {
    await transport.sendMail({
        from: "noreply@example.com",
        to,
        subject,
        html: `<div><h2>${subject}</h2><p>${message}</p></div>`,
    });
}
