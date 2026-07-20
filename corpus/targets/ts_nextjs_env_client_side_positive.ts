// [frensense]
// observation: A secret key is exposed as a NEXT_PUBLIC_ environment variable, making it available in the client-side bundle.
// impact: Anyone who views the page source or JavaScript bundle can read the secret, compromising API authentication.
// improvement: Never prefix secret keys with NEXT_PUBLIC_. Only non-sensitive public values should use NEXT_PUBLIC_.

export async function getServerSideProps() {
  return {
    props: {
      apiKey: process.env.NEXT_PUBLIC_INTERNAL_API_KEY
    }
  };
}
