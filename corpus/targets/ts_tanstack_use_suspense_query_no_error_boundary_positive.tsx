// [frensense]
// observation: `useSuspenseQuery` is used without a `<Suspense>` or `<ErrorBoundary>` parent, so when the query is pending or fails, React throws the promise/error unhandled
// impact: the entire component tree unmounts or the app crashes with an unhandled error when the query is loading or fails, because no Suspense boundary catches the thrown promise
// improvement: wrap the component using `useSuspenseQuery` in a `<Suspense fallback={...}>` and optionally an `<ErrorBoundary>` to handle loading and error states gracefully
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

import { useSuspenseQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
}

export function UserProfile({ userId }: { userId: string }) {
  const { data } = useSuspenseQuery<User>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  return <div>{data.name}</div>
}

export default function App() {
  return <UserProfile userId="123" />
}
