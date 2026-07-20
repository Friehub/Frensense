// SAFE alternative: use nodemailer's built-in address object
app.post('/api/contact', async (req, res) => {
  const { name, email, message } = req.body;

  await transporter.sendMail({
    from: '"Contact Form" <noreply@example.com>',
    to: 'support@example.com',
    subject: 'New contact form message',
    text: `From: ${name} (${email})\n\n${message}`,
    replyTo: email,
  });

  res.json({ status: 'sent' });
});
