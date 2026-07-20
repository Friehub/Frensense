// SAFE: Toast content is rendered as plain text using JSX interpolation, which React escapes automatically

'use client';

import { toast } from '@/components/ui/use-toast';

export function showNotification(title: string, message: string) {
  toast({
    title,
    description: <span>{message}</span>,
  });
}
