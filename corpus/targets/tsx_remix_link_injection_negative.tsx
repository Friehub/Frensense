// SAFE: Validates URL against allowlist of allowed paths before rendering the Link
import { Link } from "@remix-run/react";

const ALLOWED_PATHS = ["/profile", "/settings", "/dashboard"];

interface UserLinkProps {
  userUrl: string;
  label: string;
}

export function UserLink({ userUrl, label }: UserLinkProps) {
  const safeUrl = ALLOWED_PATHS.includes(userUrl) ? userUrl : "/";
  return <Link to={safeUrl}>{label}</Link>;
}
