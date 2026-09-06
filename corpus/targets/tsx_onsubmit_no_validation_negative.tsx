// SAFE: Form data is validated against a Zod schema before being submitted.

import { FormEvent } from 'react';
import { z } from 'zod';

const contactSchema = z.object({
  email: z.string().email(),
  age: z.coerce.number().int().min(1).max(150),
  message: z.string().min(1).max(1000),
});

export function ContactForm() {
  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const raw = {
      email: (form.elements.namedItem('email') as HTMLInputElement).value,
      age: (form.elements.namedItem('age') as HTMLInputElement).value,
      message: (form.elements.namedItem('message') as HTMLTextAreaElement).value,
    };
    const parsed = contactSchema.safeParse(raw);
    if (!parsed.success) {
      alert('Validation failed: ' + parsed.error.message);
      return;
    }
    submitToApi(parsed.data);
  };

  const submitToApi = (data: z.infer<typeof contactSchema>) => {
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
