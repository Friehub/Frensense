// [frensense]
// observation: An image src attribute is set directly from user or external input without validating or sanitizing the URL.
// impact: An attacker can supply a javascript: URL, a data: URL with malicious content, or a URL that triggers SSRF by pointing to internal services.
// improvement: Validate the image URL against an allowlist of permitted origins or protocols before rendering.

export function UserAvatar({ imageUrl }: { imageUrl: string }) {
  return (
    <img
      src={imageUrl}
      alt="User avatar"
      className="avatar"
    />
  );
}

export function ProfileCover({ coverUrl }: { coverUrl: string | undefined }) {
  return (
    <div className="cover-photo">
      <img src={coverUrl ?? '/default-cover.jpg'} alt="Cover" />
    </div>
  );
}
