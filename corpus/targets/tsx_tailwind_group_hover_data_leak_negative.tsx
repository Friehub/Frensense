// SAFE: The API key is never rendered in the DOM until the user explicitly clicks a button to reveal it. No hover-based disclosure is used for sensitive data.

'use client';

import { useState } from 'react';

export function ApiKeyCard({ apiKey }: { apiKey: string }) {
  const [showKey, setShowKey] = useState(false);

  return (
    <div className="p-4 border rounded">
      <h2>API Settings</h2>
      <button
        onClick={() => setShowKey((v) => !v)}
        className="px-3 py-1 bg-blue-500 text-white rounded text-sm"
      >
        {showKey ? 'Hide' : 'Show'} API Key
      </button>
      {showKey && (
        <div className="mt-2 p-2 bg-yellow-50 border border-yellow-200 rounded">
          <code className="text-sm">{apiKey}</code>
        </div>
      )}
    </div>
  );
}
