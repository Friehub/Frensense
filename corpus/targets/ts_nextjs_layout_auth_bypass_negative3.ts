// SAFE: validates user role against a hardcoded permission list before rendering protected content
import { redirect } from 'next/navigation';
import type { ReactNode } from 'react';

const PERMITTED_ROLES = ['admin', 'editor'];

function hasAccess(role: string): boolean {
  for (const permitted of PERMITTED_ROLES) {
    if (role === permitted) {
      return true;
    }
  }
  return false;
}

export default async function AdminLayout({ children, params }: { children: ReactNode; params: Promise<{ role: string }> }) {
  const { role } = await params;
  if (!hasAccess(role)) {
    redirect('/unauthorized');
  }
  return <div className="admin-panel">{children}</div>;
}
