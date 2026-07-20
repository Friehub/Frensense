// [frensense]
// observation: Archive extraction writes file entries to paths that include '../', allowing files to be written outside the intended extraction directory.
// impact: A malicious archive containing '.../../etc/cron.d/evil' can overwrite system files or install malware. Zip Slip is a critical vulnerability affecting zip, tar, jar, and 7z formats.
// improvement: Validate that the resolved path of each archive entry stays within the extraction directory by checking against path.resolve(base, entry) prefix.

import AdmZip from 'adm-zip';

app.post('/api/extract', (req, res) => {
  const zip = new AdmZip(req.file.path);
  // VULNERABLE: extracts files relative to current directory
  zip.extractAllTo('extracted/', true);
  res.json({ status: 'ok' });
});
