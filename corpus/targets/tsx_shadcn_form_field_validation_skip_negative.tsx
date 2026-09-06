// SAFE: The form uses a zod resolver to validate all fields before submission, preventing invalid data

'use client';

import { z } from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { Form, FormField, FormItem, FormLabel, FormControl, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

const profileSchema = z.object({
  email: z.string().email('Invalid email address'),
  age: z.coerce.number().int().min(1, 'Age must be positive').max(150, 'Age must be under 150'),
  website: z.string().url('Must be a valid URL').optional().or(z.literal('')),
});

type ProfileFormValues = z.infer<typeof profileSchema>;

export function ProfileForm({ onSubmit }: { onSubmit: (data: ProfileFormValues) => Promise<void> }) {
  const form = useForm<ProfileFormValues>({
    resolver: zodResolver(profileSchema),
    defaultValues: { email: '', age: 0, website: '' },
  });

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)}>
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
        <FormField name="website" render={({ field }) => (
          <FormItem>
            <FormLabel>Website</FormLabel>
            <FormControl><Input {...field} /></FormControl>
            <FormMessage />
          </FormItem>
        )} />
        <Button type="submit">Save</Button>
      </form>
    </Form>
  );
}
