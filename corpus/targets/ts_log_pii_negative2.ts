// SAFE alternative: structured logging with PII redaction middleware
import { createLogger, format } from 'winston';

const piiRedaction = format(info => {
  if (info.email) info.email = info.email.replace(/(.{3}).+(.{2}@)/, '$1***$2');
  if (info.phone) info.phone = info.phone.replace(/(\d{3})\d{4}(\d{3})/, '$1****$2');
  if (info.ssn) info.ssn = '***-**-****';
  return info;
});

const logger = createLogger({ format: format.combine(piiRedaction(), format.json()) });
