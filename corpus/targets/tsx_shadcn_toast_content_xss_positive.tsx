// [frensense]
// observation: A shadcn/ui toast notification renders user-controlled content directly via dangerouslySetInnerHTML or unescaped JSX, creating an XSS vector.
// impact: An attacker who can control toast content (e.g., via a malicious notification title, error message, or chat preview) can inject arbitrary JavaScript into the toast, executing in the context of the application and stealing cookies, tokens, or performing actions on behalf of the user.
// improvement: Never use dangerouslySetInnerHTML or unescaped HTML with user-controlled data in toast content. Use plain text or sanitize with DOMPurify.

'use client';

import { toast } from '@/components/ui/use-toast';

export function showNotification(title: string, message: string) {
  toast({
    title,
    description: <div dangerouslySetInnerHTML={{ __html: message }} />,
  });
}
