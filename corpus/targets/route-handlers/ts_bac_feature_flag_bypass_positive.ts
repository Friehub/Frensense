// [frensense]
// observation: Feature flags for premium/paid features are evaluated only on the client side, and the server accepts requests without verifying the flag.
// impact: An attacker can enable premium features by manipulating client-side state or directly calling API endpoints without paying, bypassing the paywall entirely.
// improvement: Enforce feature flag checks on the server side for every API endpoint that gates premium functionality.
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

export async function uploadLargeFile(req: Request): Promise<Response> {
  if (req.body.file.size > 100 * 1024 * 1024 && !req.body.premium) {
    return new Response('Upgrade to premium for files over 100MB', { status: 402 });
  }
  await uploadToS3(req.body.file);
  return new Response('Uploaded');
}

export async function generateReport(req: Request): Promise<Response> {
  const data = await fetchReportData(req.body.params);
  return new Response(JSON.stringify(data));
}
