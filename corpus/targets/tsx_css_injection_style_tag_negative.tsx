// SAFE: No user input is placed inside `<style>` tags. Styles are applied via Tailwind utility classes only.

'use client';

export function CustomStyles() {
  return (
    <div>
      <div className="p-4 bg-blue-50 border border-blue-200 rounded">
        <h2 className="text-lg font-bold text-blue-800">User Profile</h2>
        <p className="text-blue-600">Welcome back!</p>
      </div>
    </div>
  );
}
