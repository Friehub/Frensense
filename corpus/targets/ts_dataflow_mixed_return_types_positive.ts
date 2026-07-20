// [frensense]
// observation: A function returns either a value of type T or an Error instance, but callers only handle the success case, ignoring the error branch.
// impact: Errors are silently ignored because the caller treats the return value as type T without checking for Error. This can lead to cascading failures, data corruption, or security bypasses when error conditions go unnoticed.
// improvement: Use a discriminated union (Result type), throw exceptions for errors, or ensure all callers check for Error before using the result.

async function fetchUserData(userId: string): Promise<{ name: string } | Error> {
  const response = await fetch(`/api/users/${userId}`);
  if (!response.ok) return new Error('Failed to fetch');
  return response.json();
}

async function displayUser(userId: string) {
  const user = await fetchUserData(userId);
  return { name: user.name.toUpperCase() };
}
