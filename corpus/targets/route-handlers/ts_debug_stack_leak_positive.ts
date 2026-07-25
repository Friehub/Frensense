// [frensense]
// observation: Caught Error objects are serialized directly into API responses.
// impact: Stack traces and internal variables are leaked to clients, providing attackers with detailed execution context.
// improvement: Return a generic error message and log the full error internally.
// cwe: CWE-489
// cvss: 5.3
// owasp: A05:2021
// severity: Medium

async function handleError(e: Error, req: Request) {
  // VULNERABLE: stack trace leak
  return Response.json({
    status: 'error',
    message: e.message,
    stack: e.stack,
    details: e
  }, { status: 500 });
}

app.use((err, req, res, next) => {
  // VULNERABLE: sending raw error to client
  res.status(500).json({ error: err });
});
