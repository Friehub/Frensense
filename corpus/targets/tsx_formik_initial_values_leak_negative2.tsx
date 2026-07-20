// SAFE: Uses a typed DTO to extract only displayable fields from API response
import { Formik, Form, Field } from "formik";

interface UserProfile {
  id: number;
  name: string;
  email: string;
  phone: string;
  ssn: string;
  internalNotes: string;
}

interface FormDTO {
  name: string;
  email: string;
}

function toFormDTO(user: UserProfile): FormDTO {
  return {
    name: user.name,
    email: user.email,
  };
}

interface FormProps {
  user: UserProfile;
}

export function ProfileEditForm({ user }: FormProps) {
  return (
    <Formik initialValues={toFormDTO(user)} onSubmit={(values) => console.log(values)}>
      <Form>
        <Field name="name" />
        <Field name="email" />
        <button type="submit">Save</button>
      </Form>
    </Formik>
  );
}
