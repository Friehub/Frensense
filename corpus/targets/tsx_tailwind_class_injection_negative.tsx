// SAFE: User input is mapped through an allowlist of permitted Tailwind classes, preventing arbitrary class injection

'use client';

import { useSearchParams } from 'next/navigation';

const ALLOWED_VARIANTS: Record<string, string> = {
  default: 'border-gray-200 bg-white',
  warning: 'border-yellow-400 bg-yellow-50',
  error: 'border-red-400 bg-red-50',
  success: 'border-green-400 bg-green-50',
  info: 'border-blue-400 bg-blue-50',
};

type Variant = keyof typeof ALLOWED_VARIANTS;

function isValidVariant(value: string): value is Variant {
  return value in ALLOWED_VARIANTS;
}

export function UserProfileCard({ name }: { name: string }) {
  const searchParams = useSearchParams();
  const rawVariant = searchParams.get('variant') ?? 'default';
  const variant = isValidVariant(rawVariant) ? rawVariant : 'default';
  const variantClasses = ALLOWED_VARIANTS[variant];

  return (
    <div className={`p-4 border rounded shadow ${variantClasses}`}>
      <h2 className="text-lg font-bold">{name}</h2>
      <p>Welcome to your profile</p>
    </div>
  );
}
