// SAFE: uses Next.js built-in server action which includes CSRF protection by default

'use server'

import { redirect } from 'next/navigation'

export async function transfer(data: FormData) {
  const amount = data.get('amount')
  const toAccount = data.get('toAccount')
  // CSRF is automatically enforced by Next.js server actions
  redirect('/success')
}
