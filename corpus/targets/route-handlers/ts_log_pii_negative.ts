// SAFE: redact PII before logging
function redactPII(data: any): any {
  return {
    ...data,
    email: data.email ? data.email.replace(/(.{2}).+(@.+)/, '$1***$2') : undefined,
    phone: data.phone ? data.phone.replace(/\d(?=\d{4})/g, '*') : undefined,
    name: data.name ? data.name[0] + '***' : undefined,
  };
}

app.post('/api/register', async (req, res) => {
  logger.info('User registered', redactPII({
    email: req.body.email,
    phone: req.body.phone,
    name: req.body.name,
    userId: req.body.userId,
    ip: req.ip,
  }));
});

app.post('/api/contact', async (req, res) => {
  logger.info('Contact form submission', {
    userId: req.body.userId,
    subject: req.body.subject,
  });
});
