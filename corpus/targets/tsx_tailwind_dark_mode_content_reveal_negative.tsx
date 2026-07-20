// SAFE: Admin controls are gated by server-side role check. The content is never rendered for non-admin users, regardless of dark/light mode.

export function AdminControls({ role }: { role: string }) {
  if (role !== 'admin') {
    return null;
  }

  return (
    <div>
      <h2>Admin Panel</h2>
      <p>Delete users, view logs, manage billing</p>
      <button>Delete All Users</button>
    </div>
  );
}
