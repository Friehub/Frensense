// [frensense]
// observation: A checkbox uses Tailwind's `peer` variant to style a label based on `checked` state (e.g., `peer/agree:checked:block`). This CSS-only validation simulates form validation visually but does not enforce actual constraint validation. An attacker can toggle the checkbox state in DevTools or submit the form with JavaScript to bypass the appearance of validation.
// impact: An attacker can bypass client-side validation that relies solely on peer-checked styling. The visual feedback suggests the form is validated, but the underlying input may not actually be checked, leading to submission of invalid or incomplete data. This can result in business logic bypass, unauthorized actions, or data corruption.
// improvement: Use JavaScript-based form validation or the HTML5 `required` attribute on checkboxes to enforce actual validation. Peer-based styling should only be used for visual polish, never as the sole validation mechanism.

'use client';

export function AgreementForm() {
  return (
    <form>
      <div className="flex items-center gap-2">
        <input type="checkbox" id="agree" className="peer/agree" />
        <label htmlFor="agree">I agree to the terms and conditions</label>
      </div>
      <button
        type="submit"
        className="peer/agree:checked:bg-blue-500 peer/agree:checked:text-white bg-gray-300 text-gray-500 px-4 py-2 rounded mt-4"
      >
        Submit
      </button>
    </form>
  );
}
