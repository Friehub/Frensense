// SAFE: Uses controlled inputs with per-field validation state before allowing submission.

import { useState, FormEvent } from 'react';

export function ContactForm() {
  const [email, setEmail] = useState('');
  const [age, setAge] = useState('');
  const [message, setMessage] = useState('');
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const errs: Record<string, string> = {};
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) errs.email = 'Invalid email';
    const ageNum = parseInt(age, 10);
    if (isNaN(ageNum) || ageNum < 1 || ageNum > 150) errs.age = 'Age must be 1-150';
    if (!message.trim() || message.length > 1000) errs.message = 'Message must be 1-1000 chars';
    setErrors(errs);
    return Object.keys(errs).length === 0;
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!validate()) return;
    fetch('/api/contact', {
      method: 'POST',
      body: JSON.stringify({ email, age: parseInt(age, 10), message }),
      headers: { 'Content-Type': 'application/json' },
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <input name="email" value={email} onChange={(e) => setEmail(e.target.value)} />
      {errors.email && <span>{errors.email}</span>}
      <input name="age" value={age} onChange={(e) => setAge(e.target.value)} />
      {errors.age && <span>{errors.age}</span>}
      <textarea name="message" value={message} onChange={(e) => setMessage(e.target.value)} />
      {errors.message && <span>{errors.message}</span>}
      <button type="submit">Send</button>
    </form>
  );
}
