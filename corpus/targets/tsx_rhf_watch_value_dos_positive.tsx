// [frensense]
// observation: React Hook Form watch() called on a large form with hundreds of fields, triggering excessive re-renders on every keystroke.
// impact: Performance denial of service — each watched field change causes full re-render of the entire form, making the app unresponsive for users with low-end devices.
// improvement: Use useWatch for individual field subscriptions or debounce watch() with a selector.
// cwe: CWE-400
// cvss: 7.5
// owasp: 
// severity: High

import { useForm } from "react-hook-form";

interface LargeForm {
  field1: string;
  field2: string;
  field3: string;
}

export function LargeForm() {
  const { register, handleSubmit, watch } = useForm<LargeForm>();
  const allValues = watch();

  const onSubmit = (data: LargeForm) => {
    console.log(data);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register("field1")} />
      <input {...register("field2")} />
      <input {...register("field3")} />
      <pre>{JSON.stringify(allValues)}</pre>
      <button type="submit">Submit</button>
    </form>
  );
}
