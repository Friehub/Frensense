// SAFE: User data is HTML-escaped before being inserted into the email template
import nodemailer from "nodemailer";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

const transport = nodemailer.createTransport({ /* config */ });

export async function sendWelcomeEmail(user: { name: string; email: string }) {
    await transport.sendMail({
        from: "noreply@example.com",
        to: user.email,
        subject: "Welcome!",
        html: `<h1>Welcome, ${escapeHtml(user.name)}!</h1><p>Thanks for joining.</p>`,
    });
}

export async function sendNotification(to: string, subject: string, message: string) {
    await transport.sendMail({
        from: "noreply@example.com",
        to,
        subject: escapeHtml(subject),
        html: `<div><h2>${escapeHtml(subject)}</h2><p>${escapeHtml(message)}</p></div>`,
    });
}
