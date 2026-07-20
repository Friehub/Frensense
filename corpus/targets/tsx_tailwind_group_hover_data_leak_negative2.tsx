// SAFE: The display of the API key requires both admin authorization and explicit user click. The key is never merely hover-revealed.

'use client';

import { useState } from 'react';

export function ApiKeyCard({ apiKey, role }: { apiKey: string; role: string }) {
  const [showKey, setShowKey] = useState(false);
  const isAdmin = role === 'admin';

  if (!isAdmin) {
    return (
      <div className="p-4 border rounded">
        <h2>API Settings</h2>
        <p className="text-sm text-gray-500">
          Contact an administrator for API access.
        </p>
      </div>
    );
  }

  return (
    <div className="p-4 border rounded">
      <h2>API Settings</h2>
      <button
        onClick={() => setShowKey((v) => !v)}
        className="px-3 py-1 bg-blue-500 text-white rounded text-sm"
      >
        {showKey ? 'Hide' : 'Reveal'} API Key
      </button>
      {showKey && (
        <div className="mt-2 p-2 bg-yellow-50 border border-yellow-200 rounded">
          <code className="text-sm">{apiKey}</code>
        </div>
      )}
    </div>
  );
}
