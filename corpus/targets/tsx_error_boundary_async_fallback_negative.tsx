// SAFE: error messages are sanitized by using textContent via a ref instead of dangerouslySetInnerHTML or raw interpolation

'use client'

import { Component, createRef, type ReactNode } from 'react'

interface Props {
  children: ReactNode
}

interface State {
  error: Error | null
}

export class ErrorBoundary extends Component<Props, State> {
  state: State = { error: null }
  private errorRef = createRef<HTMLParagraphElement>()

  static getDerivedStateFromError(error: Error): State {
    return { error }
  }

  componentDidUpdate() {
    if (this.state.error && this.errorRef.current) {
      this.errorRef.current.textContent = this.state.error.message
    }
  }

  render() {
    if (this.state.error) {
      return (
        <div role="alert">
          <h2>Something went wrong</h2>
          <p ref={this.errorRef} />
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
