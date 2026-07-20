// SAFE: uses default cache behavior (force-cache) for static reference data

export default async function CountrySelect() {
  const countries = await fetch('https://api.example.com/reference/countries').then((r) => r.json())

  return (
    <select>
      {countries.map((c: { code: string; name: string }) => (
        <option key={c.code} value={c.code}>{c.name}</option>
      ))}
    </select>
  )
}
