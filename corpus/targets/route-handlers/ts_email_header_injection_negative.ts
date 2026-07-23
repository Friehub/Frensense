// SAFE: sanitize user input in email headers
function sanitizeHeader(value: string): string {
  return value.replace(/[\r\n]/g, '').trim();
}

app.post('/api/contact', async (req, res) => {
  const { name, email, message } = req.body;

  await transporter.sendMail({
    from: '"Contact Form" <noreply@example.com>',
    to: 'support@example.com',
    subject: `New message from ${sanitizeHeader(name)}`,
    text: message,
    replyTo: sanitizeHeader(email),
  });

  res.json({ status: 'sent' });
});

app.post('/api/invite', async (req, res) => {
  const { inviteeEmail, inviterName } = req.body;
  await transporter.sendMail({
    to: inviteeEmail,
    subject: `${sanitizeHeader(inviterName)} invited you!`,
  });
  res.json({ status: 'sent' });
});
