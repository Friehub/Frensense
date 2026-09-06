// SAFE: The sort column is validated against an allowlist of known column keys before being used in sorting

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

const SORTABLE_COLUMNS = ['name', 'email', 'role'] as const;
type SortColumn = typeof SORTABLE_COLUMNS[number];

function isValidSortColumn(value: string): value is SortColumn {
  return SORTABLE_COLUMNS.includes(value as SortColumn);
}

export function UsersTable({ users }: { users: User[] }) {
  const searchParams = useSearchParams();
  const router = useRouter();
  const rawSort = searchParams.get('sort') ?? 'name';
  const sortBy: SortColumn = isValidSortColumn(rawSort) ? rawSort : 'name';

  const sorted = useMemo(() => {
    return [...users].sort((a, b) => {
      const aVal = a[sortBy];
      const bVal = b[sortBy];
      return aVal.localeCompare(bVal);
    });
  }, [users, sortBy]);

  const toggleSort = (column: string) => {
    if (!isValidSortColumn(column)) return;
    const params = new URLSearchParams(searchParams.toString());
    params.set('sort', column);
    router.push(`?${params.toString()}`);
  };

  return (
    <Table>
      <TableHeader>
        <TableRow>
          {SORTABLE_COLUMNS.map((col) => (
            <TableHead key={col} onClick={() => toggleSort(col)}>
              {col.charAt(0).toUpperCase() + col.slice(1)}
              {sortBy === col && <ArrowUpDown className="inline h-4 w-4 ml-1" />}
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
