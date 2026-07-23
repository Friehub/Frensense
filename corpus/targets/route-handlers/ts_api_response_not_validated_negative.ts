// SAFE: validate external API response with zod
import { z } from 'zod';

const WeatherResponse = z.object({
  main: z.object({ temp: z.number() }),
  weather: z.array(z.object({ description: z.string() })).min(1),
});

app.get('/api/weather', async (req, res) => {
  const response = await fetch(`https://api.weather.com/v1/current?city=${req.query.city}`);
  const data = await response.json();

  const result = WeatherResponse.safeParse(data);
  if (!result.success) {
    logger.error({ data, errors: result.error }, 'Invalid weather API response');
    return res.status(502).json({ error: 'Weather service returned unexpected data' });
  }

  res.json({ temperature: result.data.main.temp, conditions: result.data.weather[0].description });
});

const RateResponse = z.object({
  rates: z.record(z.string(), z.number()),
});

app.get('/api/exchange-rate', async (req, res) => {
  const response = await fetch('https://api.exchangerate-api.com/v4/latest/USD');
  const data = await response.json();

  const result = RateResponse.safeParse(data);
  if (!result.success) return res.status(502).json({ error: 'Exchange rate API error' });

  const rate = result.data.rates[req.query.currency as string];
  if (rate === undefined) return res.status(400).json({ error: 'Unknown currency' });
  res.json({ rate });
});
