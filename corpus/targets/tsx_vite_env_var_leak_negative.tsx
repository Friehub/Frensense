// SAFE: Uses server-side API route to fetch secrets, never inlining them in client bundle
import { useEffect, useState } from "react";

export function ApiKeyDisplay() {
  const [apiKey, setApiKey] = useState<string | null>(null);

  useEffect(() => {
    fetch("/api/config")
      .then((res) => res.json())
      .then((data) => setApiKey(data.publicApiKey))
      .catch(() => setApiKey(null));
  }, []);

  return (
    <div>
      Current API key: {apiKey}
    </div>
  );
}
