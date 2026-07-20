// [frensense]
// observation: SMS OTP send endpoint has no rate limiting, allowing attackers to flood a phone number with messages.
// impact: Thousands of SMS messages sent to a single number causes financial cost to the application (SMS is $0.01-$0.10 per message), phone spam complaints, and potential carrier blacklisting of the sender number.
// improvement: Rate limit OTP requests per phone number (e.g., max 3 per 10 minutes) and per IP address.

app.post('/api/send-otp', async (req, res) => {
  // VULNERABLE: no rate limiting on OTP
  const { phoneNumber } = req.body;
  const otp = generateOTP();

  await db.storeOTP(phoneNumber, otp);
  await smsClient.send(phoneNumber, `Your code is: ${otp}`);
  res.json({ status: 'sent' });
});

app.post('/api/verify-otp', async (req, res) => {
  const { phoneNumber, otp } = req.body;
  const stored = await db.getOTP(phoneNumber);
  if (stored === otp) {
    return res.json({ verified: true });
  }
  res.json({ verified: false });
});
