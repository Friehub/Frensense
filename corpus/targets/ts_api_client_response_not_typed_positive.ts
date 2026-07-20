// [frensense]
// observation: The response from an API call is cast or used as `any` without validating the shape against a schema or TypeScript type.
// impact: Downstream code assumes a specific data shape that may not match the actual response. A changed or malicious API response can cause runtime type errors, undefined property access, or injection of unexpected data into the application.
// improvement: Validate the API response against a Zod schema or use a typed API client that checks the response structure.

async function getUser(userId: string) {
  const response = await fetch(`/api/users/${userId}`);
  const data: any = await response.json();
  return { id: data.id, name: data.name, email: data.email };
}
