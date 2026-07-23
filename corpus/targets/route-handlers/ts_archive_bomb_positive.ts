// [frensense]
// observation: Archive extraction without checking for decompression bombs: a small archive that decompresses to an enormous size (e.g., 10:1 zip, 1000000:1 gzip bomb, or nested archives).
// impact: A 10KB zip bomb can decompress to multiple terabytes, exhausting disk space, memory, and CPU. In containerized environments this can cause OOM kills and cascade to other tenants.
// improvement: Limit the maximum decompressed size and entry count before extracting. Check compressed-to-uncompressed ratio. Abort extraction if limits are exceeded.

import AdmZip from 'adm-zip';
import * as tar from 'tar-stream';
import * as zlib from 'zlib';

app.post('/api/extract-zip', (req, res) => {
  // VULNERABLE: no decompression bomb protection
  const zip = new AdmZip(req.file.path);
  zip.extractAllTo('extracted/', true);
  res.json({ status: 'ok' });
});

app.post('/api/extract-tar', (req, res) => {
  // VULNERABLE: no size limit
  const extract = tar.extract();
  req.pipe(zlib.createGunzip()).pipe(extract);
  extract.on('entry', (header, stream, next) => {
    stream.resume();
    next();
  });
  res.json({ status: 'ok' });
});
