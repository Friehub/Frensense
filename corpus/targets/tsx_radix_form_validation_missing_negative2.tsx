// SAFE: Uses Radix Form's built-in validation constraints (required, type, pattern) with Form.Message for inline error display

import * as Form from '@radix-ui/react-form';
import { Button } from '@/components/ui/button';

export function SignupForm() {
  return (
    <Form.Root>
      <Form.Field name="email">
        <Form.Label>Email</Form.Label>
        <Form.Control asChild>
          <input type="email" placeholder="you@example.com" required />
        </Form.Control>
        <Form.Message match="valueMissing">Email is required</Form.Message>
        <Form.Message match="typeMismatch">Please enter a valid email</Form.Message>
      </Form.Field>
      <Form.Field name="password">
        <Form.Label>Password</Form.Label>
        <Form.Control asChild>
          <input type="password" placeholder="Min 8 characters" required minLength={8} />
        </Form.Control>
        <Form.Message match="valueMissing">Password is required</Form.Message>
        <Form.Message match="tooShort">Password must be at least 8 characters</Form.Message>
      </Form.Field>
      <Form.Submit asChild>
        <Button type="submit">Sign Up</Button>
      </Form.Submit>
    </Form.Root>
  );
}
