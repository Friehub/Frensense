// [frensense]
// observation: External API response used directly without schema validation, assuming the structure is correct.
// impact: If the external API changes its response format, returns an error, or is compromised, the application may crash, expose sensitive data, or execute unexpected code paths. SSRF via malicious API responses.
// improvement: Validate external API responses against a schema (zod, ajv) or type guard before using the data.

app.get('/api/weather', async (req, res) => {
  // VULNERABLE: response not validated
  const response = await fetch(`https://api.weather.com/v1/current?city=${req.query.city}`);
  const data = await response.json();

  // If API changes response format, this crashes
  res.json({ temperature: data.main.temp, conditions: data.weather[0].description });
});

app.get('/api/exchange-rate', async (req, res) => {
  // VULNERABLE: no validation of external data
  const response = await fetch('https://api.exchangerate-api.com/v4/latest/USD');
  const data = await response.json();

  // If API returns error, rates might be undefined
  const rate = data.rates[req.query.currency as string];
  res.json({ rate });
});
