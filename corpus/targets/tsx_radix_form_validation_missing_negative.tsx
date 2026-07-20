// SAFE: onSubmit handler validates all fields before submission and prevents submission of invalid data

import * as Form from '@radix-ui/react-form';
import { Button } from '@/components/ui/button';

interface FormData {
  email: string;
  password: string;
}

function validate(data: FormData): Record<string, string> {
  const errors: Record<string, string> = {};
  if (!data.email.includes('@')) errors.email = 'Invalid email';
  if (data.password.length < 8) errors.password = 'Password must be at least 8 characters';
  return errors;
}

export function SignupForm() {
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const data: FormData = { email: form.email.value, password: form.password.value };
    const errors = validate(data);
    if (Object.keys(errors).length > 0) {
      alert(Object.values(errors).join('\n'));
      return;
    }
    fetch('/api/signup', { method: 'POST', body: JSON.stringify(data) });
  };

  return (
    <Form.Root onSubmit={handleSubmit}>
      <Form.Field name="email">
        <Form.Label>Email</Form.Label>
        <Form.Control asChild>
          <input type="email" placeholder="you@example.com" required />
        </Form.Control>
        {({ validity }) => validity.validityBadInput && <Form.Message>Invalid email</Form.Message>}
      </Form.Field>
      <Form.Field name="password">
        <Form.Label>Password</Form.Label>
        <Form.Control asChild>
          <input type="password" placeholder="Min 8 characters" required minLength={8} />
        </Form.Control>
        {({ validity }) => validity.valueMissing && <Form.Message>Required</Form.Message>}
      </Form.Field>
      <Form.Submit asChild>
        <Button type="submit">Sign Up</Button>
      </Form.Submit>
    </Form.Root>
  );
}
