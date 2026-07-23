// [frensense]
// observation: User input placed directly in email To/From/Subject fields without sanitization of newlines, allowing email header injection.
// impact: An attacker can inject additional email headers (CC, BCC, Reply-To) or even entirely new email bodies by inserting CRLF sequences. This enables spamming, phishing, and spoofing through the application's email infrastructure.
// improvement: Sanitize user input by removing or encoding CRLF characters (\\r\\n, \\n) before including in email headers. Use typed email libraries that handle encoding.

import nodemailer from 'nodemailer';

const transporter = nodemailer.createTransport({ /* config */ });

app.post('/api/contact', async (req, res) => {
  // VULNERABLE: user input in subject line
  const { name, email, message } = req.body;

  await transporter.sendMail({
    from: '"Contact Form" <noreply@example.com>',
    to: 'support@example.com',
    subject: `New message from ${name}`,
    text: message,
    replyTo: email,
  });

  res.json({ status: 'sent' });
});

app.post('/api/invite', async (req, res) => {
  // VULNERABLE: user input in subject
  const { inviteeEmail, inviterName } = req.body;
  await transporter.sendMail({
    to: inviteeEmail,
    subject: `${inviterName} invited you!`,
    // ...
  });
  res.json({ status: 'sent' });
});
