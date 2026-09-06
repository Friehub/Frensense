// [frensense]
// observation: A form's onSubmit handler directly accesses form values and passes them to an API or state update without any validation or sanitization.
// impact: Malformed, malicious, or unexpected data is accepted and processed, potentially leading to XSS, data corruption, or backend injection attacks.
// improvement: Validate all form fields against a schema before processing the submission.

import { FormEvent } from 'react';

interface FormData {
  email: string;
  age: string;
  message: string;
}

export function ContactForm() {
  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const data: FormData = {
      email: (form.elements.namedItem('email') as HTMLInputElement).value,
      age: (form.elements.namedItem('age') as HTMLInputElement).value,
      message: (form.elements.namedItem('message') as HTMLTextAreaElement).value,
    };
    submitToApi(data);
  };

  const submitToApi = (data: FormData) => {
    fetch('/api/contact', {
      method: 'POST',
      body: JSON.stringify(data),
      headers: { 'Content-Type': 'application/json' },
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <input name="email" type="text" />
      <input name="age" type="text" />
      <textarea name="message" />
      <button type="submit">Send</button>
    </form>
  );
}
