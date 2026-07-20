// [frensense]
// observation: Open Graph metadata uses a user-controlled URL for og:image, which the server fetches or the client loads on share.
// impact: An attacker can provide an image URL that points to internal services (metadata endpoint, internal APIs) causing SSRF or information disclosure.
// improvement: Validate and restrict image URLs to an allowlist of trusted origins.

import type { Metadata } from 'next';

interface Props {
  params: { slug: string };
  searchParams: { image?: string };
}

export async function generateMetadata({ searchParams }: Props): Promise<Metadata> {
  return {
    openGraph: {
      images: [{ url: searchParams.image || '/default-og.png' }],
    },
  };
}

export default function Page() {
  return <div>Content</div>;
}
