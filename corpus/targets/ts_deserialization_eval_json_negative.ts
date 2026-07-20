// SAFE: JSON.parse for deserialization
import express from 'express';

const app = express();
app.use(express.json());

app.post('/api/data', (req, res) => {
  res.json(req.body);
});

// SAFE: explicit JSON.parse with error handling
function parseJsonData(data: string): any {
  try {
    return JSON.parse(data);
  } catch {
    throw new Error('Invalid JSON');
  }
}
