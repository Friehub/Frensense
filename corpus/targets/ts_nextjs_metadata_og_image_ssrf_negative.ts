// SAFE: Image URL is validated against an allowlist of trusted image hosts

import type { Metadata } from 'next';

const ALLOWED_IMAGE_HOSTS = ['images.example.com', 'cdn.example.com'];

function isValidImageUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return ALLOWED_IMAGE_HOSTS.includes(parsed.hostname);
  } catch {
    return false;
  }
}

interface Props {
  params: { slug: string };
  searchParams: { image?: string };
}

export async function generateMetadata({ searchParams }: Props): Promise<Metadata> {
  const imageUrl = searchParams.image && isValidImageUrl(searchParams.image)
    ? searchParams.image
    : '/default-og.png';
  return {
    openGraph: {
      images: [{ url: imageUrl }],
    },
  };
}

export default function Page() {
  return <div>Content</div>;
}
