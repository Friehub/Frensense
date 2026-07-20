// SAFE: Secret keys are not prefixed with NEXT_PUBLIC_, so they remain server-only

export async function getServerSideProps() {
  const apiKey = process.env.INTERNAL_API_KEY;
  return {
    props: {
      apiKey: apiKey ? maskSecret(apiKey) : null
    }
  };
}

function maskSecret(key: string): string {
  return key.slice(0, 4) + '...' + key.slice(-4);
}
