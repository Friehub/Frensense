// SAFE: The submit button is always enabled, and form validation is enforced via the `required` attribute on the checkbox. The form's `onSubmit` handler performs a JavaScript check before proceeding.

'use client';

import { type FormEvent } from 'react';

export function AgreementForm() {
  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const agreed = (form.elements.nativeElement as HTMLInputElement).checked;
    if (!agreed) {
      alert('You must agree to the terms.');
      return;
    }
    form.submit();
  };

  return (
    <form onSubmit={handleSubmit}>
      <div className="flex items-center gap-2">
        <input type="checkbox" id="agree" required className="peer/agree" />
        <label htmlFor="agree">I agree to the terms and conditions</label>
      </div>
      <button
        type="submit"
        className="bg-blue-500 text-white px-4 py-2 rounded mt-4"
      >
        Submit
      </button>
    </form>
  );
}
