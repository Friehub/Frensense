// SAFE: Uses watch() with a selector function to only subscribe to specific fields
import { useForm } from "react-hook-form";

interface LargeForm {
  field1: string;
  field2: string;
  field3: string;
}

export function LargeForm() {
  const { register, handleSubmit, watch } = useForm<LargeForm>();
  const field1Value = watch("field1");

  const onSubmit = (data: LargeForm) => {
    console.log(data);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register("field1")} />
      <input {...register("field2")} />
      <input {...register("field3")} />
      <p>Field 1: {field1Value}</p>
      <button type="submit">Submit</button>
    </form>
  );
}
