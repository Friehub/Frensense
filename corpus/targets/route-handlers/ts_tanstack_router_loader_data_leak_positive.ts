// [frensense]
// observation: a route loader returns the full API response including internal/sensitive fields, and all route components receive the entire dataset
// impact: sensitive data leak — components that only need a subset of the data receive internal fields (e.g., `ssn`, `internalNotes`) that can be accidentally rendered or passed to analytics
// improvement: define a specific `loaderDeps` or transform the loader data to only return the fields needed by the route
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Route } from '@tanstack/react-router'

interface UserData {
  id: string
  name: string
  email: string
  ssn: string
  internalNotes: string
}

const userRoute = new Route({
  path: '/users/$userId',
  loader: async ({ params }): Promise<UserData> => {
    const res = await fetch(`/api/users/${params.userId}`)
    return res.json()
  },
  component: ({ useLoader }) => {
    const data = useLoader()
    return <div>{data.name} — {data.email}</div>
  },
})
