// SAFE alternative: type guard with fallback
interface WeatherData {
  main: { temp: number };
  weather: Array<{ description: string }>;
}

function isWeatherData(data: any): data is WeatherData {
  return data && typeof data.main?.temp === 'number' && Array.isArray(data.weather) && data.weather.length > 0;
}

app.get('/api/weather', async (req, res) => {
  const response = await fetch(`https://api.weather.com/v1/current?city=${req.query.city}`);
  const data = await response.json();

  if (!isWeatherData(data)) {
    return res.status(502).json({ error: 'Unexpected weather data format' });
  }

  res.json({ temperature: data.main.temp, conditions: data.weather[0].description });
});
