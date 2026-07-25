// [frensense]
// observation: getServerSideProps fetches sensitive server-side data and returns it in props, exposing it to the client.
// impact: Sensitive data like API keys, internal IDs, or PII is sent to the browser and visible in the page source.
// improvement: Only return the minimal data needed for rendering in getServerSideProps, and never include secrets.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import prisma from '@/lib/prisma';

export async function getServerSideProps() {
  const user = await prisma.user.findUnique({
    where: { id: '1' }
  });
  return {
    props: { user }
  };
}
