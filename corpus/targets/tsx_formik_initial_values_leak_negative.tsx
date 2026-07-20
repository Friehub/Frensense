// SAFE: Selectively maps only needed fields from API response into initialValues
import { Formik, Form, Field } from "formik";

interface UserProfile {
  id: number;
  name: string;
  email: string;
  phone: string;
  ssn: string;
  internalNotes: string;
}

interface FormProps {
  user: UserProfile;
}

export function ProfileEditForm({ user }: FormProps) {
  const safeInitialValues = {
    name: user.name,
    email: user.email,
  };
  return (
    <Formik initialValues={safeInitialValues} onSubmit={(values) => console.log(values)}>
      <Form>
        <Field name="name" />
        <Field name="email" />
        <button type="submit">Save</button>
      </Form>
    </Formik>
  );
}
