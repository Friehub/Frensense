// SAFE: Uses manual validation via register validate callback to sanitize input
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
      <input
        {...register("username", {
          required: true,
          minLength: 3,
          maxLength: 20,
          pattern: /^[a-zA-Z0-9_]+$/,
        })}
      />
      <textarea
        {...register("bio", { maxLength: 500 })}
      />
      <button type="submit">Save</button>
    </form>
  );
}
