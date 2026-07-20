// SAFE: Image URL is validated against an allowlist of permitted domains and protocols.

const ALLOWED_IMAGE_DOMAINS = [
  'storage.example.com',
  'cdn.example.com',
  'secure.gravatar.com',
];

function isValidImageUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return parsed.protocol === 'https:' &&
      ALLOWED_IMAGE_DOMAINS.includes(parsed.hostname);
  } catch {
    return false;
  }
}

export function UserAvatar({ imageUrl }: { imageUrl: string }) {
  const safeUrl = isValidImageUrl(imageUrl) ? imageUrl : '/default-avatar.png';
  return <img src={safeUrl} alt="User avatar" className="avatar" />;
}

export function ProfileCover({ coverUrl }: { coverUrl: string | undefined }) {
  const safeUrl = coverUrl && isValidImageUrl(coverUrl) ? coverUrl : '/default-cover.jpg';
  return (
    <div className="cover-photo">
      <img src={safeUrl} alt="Cover" />
    </div>
  );
}
