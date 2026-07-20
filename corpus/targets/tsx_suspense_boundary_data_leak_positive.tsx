// [frensense]
// observation: an error boundary wraps a Suspense fallback or async component that throws during fetch — the error boundary catches the rejection and renders error details including the stack trace
// impact: internal fetch URLs, query parameters, and server error messages are exposed to users through the error boundary UI
// improvement: sanitize the error before rendering; never show error.message or error.stack to users

'use client'

import { Component, ReactNode } from 'react'

interface Props {
  children: ReactNode
}

interface State {
  error: Error | null
}

export class DataErrorBoundary extends Component<Props, State> {
  state: State = { error: null }

  static getDerivedStateFromError(error: Error): State {
    return { error }
  }

  render() {
    if (this.state.error) {
      return (
        <div role="alert">
          <h2>Something went wrong</h2>
          <pre>{this.state.error.stack}</pre>
        </div>
      )
    }
    return this.props.children
  }
}
