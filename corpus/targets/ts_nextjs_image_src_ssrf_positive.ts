// [frensense]
// observation: Next/Image receives a user-controlled URL as its src prop, potentially fetching from internal or malicious servers.
// impact: An attacker can cause the server to fetch images from internal IPs or cloud metadata endpoints, leaking sensitive data.
// improvement: Validate image src against an allowlist or sanitize it to prevent arbitrary URL loading.

import Image from 'next/image';

export default function UserAvatar({ params }: { params: { imageUrl: string } }) {
  return (
    <Image
      src={params.imageUrl}
      alt="User avatar"
      width={200}
      height={200}
    />
  );
}
