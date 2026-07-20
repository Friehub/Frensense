// SAFE: Sort column is validated using a zod enum schema, and the sort logic is decoupled from user input via a lookup table

'use client';

import { z } from 'zod';
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

const sortColumnSchema = z.enum(['name', 'email', 'role']);
const SORT_COLUMNS = sortColumnSchema.options;

type SortConfig = {
  column: z.infer<typeof sortColumnSchema>;
  direction: 'asc' | 'desc';
};

export function UsersTable({ users }: { users: User[] }) {
  const searchParams = useSearchParams();
  const router = useRouter();
  const rawSort = searchParams.get('sort') ?? 'name';
  const sortColumn = sortColumnSchema.safeParse(rawSort).data ?? 'name';

  const sorted = useMemo(() => {
    return [...users].sort((a, b) => a[sortColumn].localeCompare(b[sortColumn]));
  }, [users, sortColumn]);

  const toggleSort = (column: string) => {
    const parsed = sortColumnSchema.safeParse(column);
    if (!parsed.success) return;
    const params = new URLSearchParams(searchParams.toString());
    params.set('sort', parsed.data);
    router.push(`?${params.toString()}`);
  };

  return (
    <Table>
      <TableHeader>
        <TableRow>
          {SORT_COLUMNS.map((col) => (
            <TableHead key={col} className="cursor-pointer" onClick={() => toggleSort(col)}>
              <span className="flex items-center gap-1">
                {col.charAt(0).toUpperCase() + col.slice(1)}
                {sortColumn === col && <ArrowUpDown className="h-4 w-4" />}
              </span>
            </TableHead>
          ))}
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
