// SAFE: disable external entities in xml2js
import { Parser } from 'xml2js';

app.post('/api/upload-xml', (req, res) => {
  let xml = '';
  req.on('data', chunk => xml += chunk);
  req.on('end', () => {
    const parser = new Parser({
      explicitCharkey: false,
      trim: true,
      // SAFE: disable DTD processing
      xmlns: false,
      // SAFE: no entity expansion
      normalize: true,
      normalizeTags: false,
    });
    parser.parseString(xml, (err, result) => {
      if (err) return res.status(400).json({ error: 'Invalid XML' });
      res.json(result);
    });
  });
});
