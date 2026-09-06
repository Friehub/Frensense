// [frensense]
// observation: A shadcn/ui Select component accepts its value directly from URL search params without validating against the list of allowed options.
// impact: An attacker can set an arbitrary select value via URL parameters (e.g., `?role=superadmin`), bypassing the UI constraints. This can lead to privilege escalation, injection attacks, or unexpected application behavior if the value is used in queries or state.
// improvement: Validate the incoming value against the known set of options before passing it to the Select component, using a type-safe enum or option list.

'use client';

import { useSearchParams, useRouter } from 'next/navigation';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';

const ROLES = ['user', 'admin', 'moderator'] as const;

export function RoleSelector() {
  const searchParams = useSearchParams();
  const router = useRouter();
  const currentRole = searchParams.get('role') ?? 'user';

  const handleChange = (value: string) => {
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
