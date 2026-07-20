// [frensense]
// observation: `fetch()` is called with `cache: 'no-cache'` for static reference data that rarely changes (e.g., country list, currency codes)
// impact: every request bypasses the RSC cache and hits the origin server, wasting bandwidth and increasing latency for data that could be cached indefinitely
// improvement: remove `cache: 'no-cache'` or use `force-cache` for static reference data

export default async function CountrySelect() {
  const countries = await fetch('https://api.example.com/reference/countries', {
    cache: 'no-cache',
  }).then((r) => r.json())

  return (
    <select>
      {countries.map((c: { code: string; name: string }) => (
        <option key={c.code} value={c.code}>{c.name}</option>
      ))}
    </select>
  )
}
