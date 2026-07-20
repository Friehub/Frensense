// [frensense]
// observation: A parent container has the `group` class and a child element uses `group-hover:` to reveal sensitive information (e.g., API keys, auth tokens, database credentials) on hover over the parent. This exposes sensitive data when the user merely mouses over the parent area, without deliberate action. Hover-based disclosure can also be triggered programmatically or via CSS injection.
// impact: An attacker can exfiltrate sensitive data by tricking a user into hovering over a seemingly innocuous element. Data like API keys, internal IDs, or user tokens become visible in the DOM and can be captured via screenshot, screen recording, or social engineering.
// improvement: Never display sensitive data on hover alone. Require explicit user action (click, toggle) to reveal secrets, and apply proper authorization checks before rendering sensitive content.

'use client';

export function ApiKeyCard({ apiKey }: { apiKey: string }) {
  return (
    <div className="group p-4 border rounded hover:bg-gray-50">
      <h2>API Settings</h2>
      <p className="text-sm text-gray-500">Hover to see your API key</p>
      <div className="hidden group-hover:block mt-2 p-2 bg-yellow-50 border border-yellow-200 rounded">
        <code className="text-sm">{apiKey}</code>
      </div>
    </div>
  );
}
