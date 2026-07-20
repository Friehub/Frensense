// SAFE: Rejects non-relative URLs and validates the path format
import { Link } from "@remix-run/react";

interface UserLinkProps {
  userUrl: string;
  label: string;
}

function isValidPath(path: string): boolean {
  if (path.startsWith("javascript:") || path.startsWith("data:")) {
    return false;
  }
  try {
    const url = new URL(path, "http://localhost");
    return url.origin === "http://localhost";
  } catch {
    return false;
  }
}

export function UserLink({ userUrl, label }: UserLinkProps) {
  const safeUrl = isValidPath(userUrl) ? userUrl : "/";
  return <Link to={safeUrl}>{label}</Link>;
}
