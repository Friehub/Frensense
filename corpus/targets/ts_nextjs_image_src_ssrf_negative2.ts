// SAFE: Image URLs are proxied through a server-side endpoint that validates and sanitizes the target

import Image from 'next/image';

export default function UserAvatar({ params }: { params: { imageUrl: string } }) {
  const proxiedSrc = `/api/image-proxy?url=${encodeURIComponent(params.imageUrl)}`;
  return (
    <Image
      src={proxiedSrc}
      alt="User avatar"
      width={200}
      height={200}
    />
  );
}
