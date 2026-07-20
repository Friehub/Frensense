// SAFE: Uses a zod schema to parse and validate the role value from URL params, rejecting invalid input with a safe default

'use client';

import { z } from 'zod';
import { useSearchParams, useRouter } from 'next/navigation';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

const roleSchema = z.enum(['user', 'admin', 'moderator']);

export function RoleSelector() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const rawRole = searchParams.get('role') ?? 'user';
  const currentRole = roleSchema.safeParse(rawRole).data ?? 'user';

  const handleChange = (value: string) => {
    const parsed = roleSchema.safeParse(value);
    if (!parsed.success) return;
    const params = new URLSearchParams(searchParams.toString());
    params.set('role', parsed.data);
    router.push(`?${params.toString()}`);
  };

  return (
    <Select value={currentRole} onValueChange={handleChange}>
      <SelectTrigger className="w-48">
        <SelectValue placeholder="Select role" />
      </SelectTrigger>
      <SelectContent>
        {roleSchema.options.map((role) => (
          <SelectItem key={role} value={role}>{role}</SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
