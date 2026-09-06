// [frensense]
// observation: A shadcn/ui Table's sort column name is taken directly from URL search params and used as a dynamic key to sort data, without validating that the column name exists in the dataset.
// impact: An attacker can inject arbitrary property access paths via the `?sort=` parameter (e.g., `?sort=__proto__`, `?sort=constructor`, `?sort=admin.password`). This can lead to prototype pollution, access to private object fields, or injection into backend sort queries if the column is forwarded to an API.
// improvement: Validate the sort column against an allowlist of known column keys before using it in sorting logic.

'use client';

import { useMemo } from 'react';
import { useSearchParams, useRouter } from 'next/navigation';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import { ArrowUpDown } from 'lucide-react';

interface User {
  id: string;
  name: string;
  email: string;
  role: string;
}

export function UsersTable({ users }: { users: User[] }) {
  const searchParams = useSearchParams();
  const router = useRouter();
  const sortBy = searchParams.get('sort') ?? 'name';

  const sorted = useMemo(() => {
    return [...users].sort((a, b) => {
      const aVal = a[sortBy as keyof User];
      const bVal = b[sortBy as keyof User];
      if (typeof aVal === 'string' && typeof bVal === 'string') return aVal.localeCompare(bVal);
      return String(aVal).localeCompare(String(bVal));
    });
  }, [users, sortBy]);

  const toggleSort = (column: string) => {
    const params = new URLSearchParams(searchParams.toString());
    params.set('sort', column);
    router.push(`?${params.toString()}`);
  };

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead onClick={() => toggleSort('name')}>Name <ArrowUpDown className="inline h-4 w-4" /></TableHead>
          <TableHead onClick={() => toggleSort('email')}>Email</TableHead>
          <TableHead onClick={() => toggleSort('role')}>Role</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {sorted.map((user) => (
          <TableRow key={user.id}>
            <TableCell>{user.name}</TableCell>
            <TableCell>{user.email}</TableCell>
            <TableCell>{user.role}</TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
