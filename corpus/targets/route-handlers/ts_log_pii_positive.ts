// [frensense]
// observation: Personally Identifiable Information (email, phone number, name, SSN) logged in plain text.
// impact: Exposing PII in logs violates GDPR (Article 32), HIPAA, SOC2, and PCI-DSS regulations. Log files are often less protected than databases and may be ingested by third-party services. Fines can reach 4% of global revenue.
// improvement: Redact PII fields before logging. Use structured logging with sensitive fields masked. Never log email, phone, SSN, or full address.

import { logger } from './logger';

app.post('/api/register', async (req, res) => {
  // VULNERABLE: PII in log
  logger.info('User registered', {
    email: req.body.email,
    phone: req.body.phone,
    name: req.body.name,
    ip: req.ip,
  });
  // ...
});

app.post('/api/contact', async (req, res) => {
  // VULNERABLE: email and message logged
  console.log('Contact form submission:', req.body.email, req.body.message);
  // ...
});
