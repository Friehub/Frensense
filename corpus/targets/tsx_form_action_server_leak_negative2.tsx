// SAFE: server action reference is passed via a server component wrapper — the action ID is never client-accessible

'use client'

export default function AdminUserRow({ userId, name, onDelete }: { userId: string; name: string; onDelete: (formData: FormData) => Promise<void> }) {
  return (
    <form action={onDelete}>
      <input type="hidden" name="userId" value={userId} />
      <span>{name}</span>
      <button type="submit">Delete</button>
    </form>
  )
}

// Server component passes the action as a prop:
// <AdminUserRow userId="u-42" name="Alice" onDelete={deleteUserAction} />
