// SAFE: Email uses a separate template rendered through a sanitization library; user name is escaped
import nodemailer from "nodemailer";
import { escape } from "lodash";

const transport = nodemailer.createTransport({ /* config */ });

export async function sendWelcomeEmail(user: { name: string; email: string }) {
    await transport.sendMail({
        from: "noreply@example.com",
        to: user.email,
        subject: "Welcome!",
        html: `<h1>Welcome, ${escape(user.name)}!</h1><p>Thanks for joining.</p>`,
    });
}

export async function sendNotification(to: string, subject: string, message: string) {
    await transport.sendMail({
        from: "noreply@example.com",
        to,
        subject: escape(subject),
        html: `<div><h2>${escape(subject)}</h2><p>${escape(message)}</p></div>`,
    });
}
