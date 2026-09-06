// SAFE: Manual validation is performed in the onSubmit handler before any data is sent to the server

'use client';

import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { Form, FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

interface ProfileFormValues {
  email: string;
  age: number;
}

function validate(data: ProfileFormValues): Record<string, string> {
  const errors: Record<string, string> = {};
  if (!data.email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(data.email)) errors.email = 'Invalid email';
  if (!data.age || data.age < 1 || data.age > 150) errors.age = 'Age must be 1-150';
  return errors;
}

export function ProfileForm({ onSubmit }: { onSubmit: (data: ProfileFormValues) => Promise<void> }) {
  const [serverError, setServerError] = useState<string | null>(null);
  const form = useForm<ProfileFormValues>({
    defaultValues: { email: '', age: 0 },
  });

  const handleSubmit = form.handleSubmit(async (data) => {
    const errors = validate(data);
    if (Object.keys(errors).length > 0) {
      Object.entries(errors).forEach(([field, msg]) => form.setError(field as keyof ProfileFormValues, { message: msg }));
      return;
    }
    await onSubmit(data);
  });

  return (
    <Form {...form}>
      <form onSubmit={handleSubmit}>
        {serverError && <p style={{ color: 'red' }}>{serverError}</p>}
        <FormField name="email" render={({ field }) => (
          <FormItem>
            <FormLabel>Email</FormLabel>
            <FormControl><Input {...field} /></FormControl>
            <FormMessage />
          </FormItem>
        )} />
        <FormField name="age" render={({ field }) => (
          <FormItem>
            <FormLabel>Age</FormLabel>
            <FormControl><Input type="number" {...field} /></FormControl>
            <FormMessage />
          </FormItem>
        )} />
        <Button type="submit">Save</Button>
      </form>
    </Form>
  );
}
