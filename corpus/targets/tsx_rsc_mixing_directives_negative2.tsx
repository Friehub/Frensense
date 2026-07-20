// SAFE: uses only 'use server' in a separate action file

'use server'

export async function submitForm(data: FormData) {
  const name = data.get('name')
  await fetch('/api/submit', { method: 'POST', body: data })
}
