// SAFE: disable entity loading in libxml
import * as libxml from 'libxmljs';

app.post('/api/parse-xml', (req, res) => {
  const xml = req.body.toString();
  const doc = libxml.parseXml(xml, {
    // SAFE: do not substitute entities
    noent: false,
    // SAFE: do not load external DTD
    dtdload: false,
    // SAFE: do not load external entities
    dtdattr: false,
  });
  res.json({ parsed: doc.toString() });
});
