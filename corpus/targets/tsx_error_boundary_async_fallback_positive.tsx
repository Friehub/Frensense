// [frensense]
// observation: the error boundary fallback renders user-controlled input (e.g., from error.message or URL params) without sanitization
// impact: XSS — an attacker can craft an error message or URL parameter that executes arbitrary JavaScript in the fallback UI
// improvement: sanitize any user-influenced text in the error boundary fallback before rendering
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

'use client'

import { Component, type ReactNode } from 'react'

interface Props {
  children: ReactNode
}

interface State {
  error: Error | null
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { error: null }

  static getDerivedStateFromError(error: Error): State {
    return { error }
  }

  render() {
    if (this.state.error) {
      return (
        <div role="alert">
          <h2>Something went wrong</h2>
          <p>{this.state.error.message}</p>
        </div>
      )
    }
    return this.props.children
  }
}

export default function App() {
  return (
    <ErrorBoundary>
      <main>App content</main>
    </ErrorBoundary>
  )
}
