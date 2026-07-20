// [frensense]
// observation: Formik initialValues populated directly from API response without filtering sensitive fields, exposing PII in form state.
// impact: Sensitive fields from API response (email, phone, internal notes) are exposed in the DOM, visible to browser extensions and other scripts.
// improvement: Selectively map only needed fields from the API response into initialValues, never spreading the entire response.

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
  return (
    <Formik initialValues={user} onSubmit={(values) => console.log(values)}>
      <Form>
        <Field name="name" />
        <Field name="email" />
        <button type="submit">Save</button>
      </Form>
    </Formik>
  );
}
