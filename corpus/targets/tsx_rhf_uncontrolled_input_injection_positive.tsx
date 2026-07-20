// [frensense]
// observation: React Hook Form register() used without validation schema, allowing raw unvalidated input to be submitted directly.
// impact: Unvalidated form input can contain SQL injection strings, XSS payloads, or command injection sequences passed to backend.
// improvement: Add a validation schema (zod, yup) to register() calls or use the `validate` option with sanitization rules.

import { useForm } from "react-hook-form";

interface FormData {
  username: string;
  bio: string;
}

export function ProfileForm() {
  const { register, handleSubmit } = useForm<FormData>();

  const onSubmit = (data: FormData) => {
    fetch("/api/profile", {
      method: "POST",
      body: JSON.stringify(data),
    });
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register("username")} />
      <textarea {...register("bio")} />
      <button type="submit">Save</button>
    </form>
  );
}
