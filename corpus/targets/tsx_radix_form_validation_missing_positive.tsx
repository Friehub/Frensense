// [frensense]
// observation: A Radix Form is rendered without `onSubmit` event handler and without client-side validation, allowing the user to submit invalid data directly to the server without any feedback.
// impact: Invalid data (empty required fields, malformed emails, out-of-range values) is sent to the server, causing unnecessary server-side errors, potential data corruption, or wasted API calls. The user receives no inline feedback and may be confused by generic error responses.
// improvement: Always implement `onSubmit` with proper validation and provide user feedback for invalid fields.

import * as Form from '@radix-ui/react-form';
import { Button } from '@/components/ui/button';

export function SignupForm() {
  return (
    <Form.Root>
      <Form.Field name="email">
        <Form.Label>Email</Form.Label>
        <Form.Control asChild>
          <input type="email" placeholder="you@example.com" />
        </Form.Control>
      </Form.Field>
      <Form.Field name="password">
        <Form.Label>Password</Form.Label>
        <Form.Control asChild>
          <input type="password" placeholder="Min 8 characters" />
        </Form.Control>
      </Form.Field>
      <Form.Submit asChild>
        <Button type="submit">Sign Up</Button>
      </Form.Submit>
    </Form.Root>
  );
}
