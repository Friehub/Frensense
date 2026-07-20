// SAFE: The URL param value is validated against the allowed options list before passing to Select, rejecting invalid values with a fallback

'use client';

import { useSearchParams, useRouter } from 'next/navigation';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

const ROLES = ['user', 'admin', 'moderator'] as const;

type Role = typeof ROLES[number];

function isValidRole(value: string): value is Role {
  return ROLES.includes(value as Role);
}

export function RoleSelector() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const rawRole = searchParams.get('role') ?? 'user';
  const currentRole = isValidRole(rawRole) ? rawRole : 'user';

  const handleChange = (value: string) => {
    if (!isValidRole(value)) return;
    const params = new URLSearchParams(searchParams.toString());
    params.set('role', value);
    router.push(`?${params.toString()}`);
  };

  return (
    <Select value={currentRole} onValueChange={handleChange}>
      <SelectTrigger className="w-48">
        <SelectValue placeholder="Select role" />
      </SelectTrigger>
      <SelectContent>
        {ROLES.map((role) => (
          <SelectItem key={role} value={role}>{role}</SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
