// SAFE alternative: use fast-xml-parser with XXE protection
import { XMLParser } from 'fast-xml-parser';

const parser = new XMLParser({
  ignoreAttributes: false,
  processEntities: false,
  htmlEntities: false,
});

app.post('/api/upload-xml', (req, res) => {
  let xml = '';
  req.on('data', chunk => xml += chunk);
  req.on('end', () => {
    try {
      const result = parser.parse(xml);
      res.json(result);
    } catch {
      res.status(400).json({ error: 'Invalid XML' });
    }
  });
});
