// [frensense]
// observation: API client errors are silently swallowed using `.catch(() => null)` or similar empty catch blocks, discarding error information.
// impact: Network failures, server errors, and unexpected response codes are hidden from the application. Downstream code receives null or undefined without context, leading to confusing behavior, data inconsistency, and silent data loss.
// improvement: Log the error, provide a meaningful fallback, or re-throw with context instead of swallowing.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

async function loadUserProfile(userId: string) {
  const response = await fetch(`/api/users/${userId}`)
    .catch(() => null);
  if (!response) return { name: 'Unknown' };
  return response.json();
}
