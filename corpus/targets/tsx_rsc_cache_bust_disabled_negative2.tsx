// SAFE: explicitly sets `cache: 'force-cache'` for static reference data and adds revalidation every 24 hours

export default async function CountrySelect() {
  const countries = await fetch('https://api.example.com/reference/countries', {
    cache: 'force-cache',
    next: { revalidate: 86400 },
  }).then((r) => r.json())

  return (
    <select>
      {countries.map((c: { code: string; name: string }) => (
        <option key={c.code} value={c.code}>{c.name}</option>
      ))}
    </select>
  )
}
