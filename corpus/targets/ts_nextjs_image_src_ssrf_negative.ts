// SAFE: The image URL is validated against an allowlist of permitted image origins

import Image from 'next/image';

const ALLOWED_ORIGINS = ['https://cdn.example.com', 'https://images.example.com'];

function isSafeImageUrl(url: string): boolean {
  return ALLOWED_ORIGINS.some((origin) => url.startsWith(origin));
}

export default function UserAvatar({ params }: { params: { imageUrl: string } }) {
  const safeSrc = isSafeImageUrl(params.imageUrl) ? params.imageUrl : '/fallback-avatar.png';
  return (
    <Image
      src={safeSrc}
      alt="User avatar"
      width={200}
      height={200}
    />
  );
}
