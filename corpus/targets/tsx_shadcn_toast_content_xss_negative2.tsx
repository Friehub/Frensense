// SAFE: User content is sanitized with DOMPurify before being passed to innerHTML, preventing XSS

'use client';

import DOMPurify from 'dompurify';
import { toast } from '@/components/ui/use-toast';

export function showRichNotification(title: string, htmlContent: string) {
  const sanitized = DOMPurify.sanitize(htmlContent, { ALLOWED_TAGS: ['b', 'i', 'em', 'strong', 'a'], ALLOWED_ATTR: ['href'] });
  toast({
    title,
    description: <div dangerouslySetInnerHTML={{ __html: sanitized }} />,
  });
}

export function showNotification(title: string, message: string) {
  toast({
    title,
    description: message,
  });
}
