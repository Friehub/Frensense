// SAFE: Image URLs are proxied through a server-side endpoint that validates and sanitizes them.

function getProxyUrl(originalUrl: string): string {
  return `/api/image-proxy?url=${encodeURIComponent(originalUrl)}`;
}

export function UserAvatar({ imageUrl }: { imageUrl: string }) {
  return <img src={getProxyUrl(imageUrl)} alt="User avatar" className="avatar" />;
}

export function ProfileCover({ coverUrl }: { coverUrl: string | undefined }) {
  if (!coverUrl) {
    return <div className="cover-photo"><img src="/default-cover.jpg" alt="Cover" /></div>;
  }
  return (
    <div className="cover-photo">
      <img src={getProxyUrl(coverUrl)} alt="Cover" />
    </div>
  );
}
