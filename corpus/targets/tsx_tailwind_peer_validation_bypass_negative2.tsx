// SAFE: The form uses a controlled component pattern with React state to track the checkbox value, and the submit button is disabled until the checkbox is checked. No CSS-only validation is used.

'use client';

import { useState, type FormEvent } from 'react';

export function AgreementForm() {
  const [agreed, setAgreed] = useState(false);

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!agreed) return;
    console.log('Form submitted');
  };

  return (
    <form onSubmit={handleSubmit}>
      <div className="flex items-center gap-2">
        <input
          type="checkbox"
          id="agree"
          checked={agreed}
          onChange={(e) => setAgreed(e.target.checked)}
        />
        <label htmlFor="agree">I agree to the terms and conditions</label>
      </div>
      <button
        type="submit"
        disabled={!agreed}
        className={`px-4 py-2 rounded mt-4 ${
          agreed
            ? 'bg-blue-500 text-white'
            : 'bg-gray-300 text-gray-500 cursor-not-allowed'
        }`}
      >
        Submit
      </button>
    </form>
  );
}
