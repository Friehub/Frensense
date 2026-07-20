// SAFE: Uses zod validation schema to sanitize and validate form inputs before submission
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";

const profileSchema = z.object({
  username: z.string().min(3).max(20).regex(/^[a-zA-Z0-9_]+$/),
  bio: z.string().max(500),
});

type FormData = z.infer<typeof profileSchema>;

export function ProfileForm() {
  const { register, handleSubmit } = useForm<FormData>({
    resolver: zodResolver(profileSchema),
  });

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
