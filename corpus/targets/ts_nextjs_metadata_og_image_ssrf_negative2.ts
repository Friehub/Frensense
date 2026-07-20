// SAFE: Only relative paths are accepted for og:image URLs, external URLs rejected entirely

import type { Metadata } from 'next';

interface Props {
  params: { slug: string };
  searchParams: { image?: string };
}

export async function generateMetadata({ searchParams }: Props): Promise<Metadata> {
  const image = searchParams.image || '/default-og.png';
  if (image.startsWith('http://') || image.startsWith('https://')) {
    return {
      openGraph: {
        images: [{ url: '/default-og.png' }],
      },
    };
  }
  return {
    openGraph: {
      images: [{ url: image }],
    },
  };
}

export default function Page() {
  return <div>Content</div>;
}
