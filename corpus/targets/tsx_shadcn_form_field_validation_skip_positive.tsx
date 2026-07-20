// [frensense]
// observation: A shadcn/ui Form component is rendered with `form.handleSubmit` but the form is created without a Zod resolver validation schema, so required fields, type constraints, and custom validations are never checked before submission.
// impact: Invalid or malicious data can be submitted — missing required fields, wrong types, or values exceeding allowed ranges — bypassing client-side validation entirely. If server-side validation is also weak, this leads to data corruption or injection attacks.
// improvement: Pass a zod resolver (or any validation schema) to `useForm` via the `resolver` option to enforce field-level validation before submit.

'use client';

import { useForm } from 'react-hook-form';
import { Form, FormField, FormItem, FormLabel, FormControl } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

interface ProfileFormValues {
  email: string;
  age: number;
  website: string;
}

export function ProfileForm({ onSubmit }: { onSubmit: (data: ProfileFormValues) => Promise<void> }) {
  const form = useForm<ProfileFormValues>({
    defaultValues: { email: '', age: 0, website: '' },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
        <FormField name="email" render={({ field }) => (
          <FormItem>
            <FormLabel>Email</FormLabel>
            <FormControl><Input {...field} /></FormControl>
          </FormItem>
        )} />
        <FormField name="age" render={({ field }) => (
          <FormItem>
            <FormLabel>Age</FormLabel>
            <FormControl><Input type="number" {...field} /></FormControl>
          </FormItem>
        )} />
        <FormField name="website" render={({ field }) => (
          <FormItem>
            <FormLabel>Website</FormLabel>
            <FormControl><Input {...field} /></FormControl>
          </FormItem>
        )} />
        <Button type="submit">Save</Button>
      </form>
    </Form>
  );
}
