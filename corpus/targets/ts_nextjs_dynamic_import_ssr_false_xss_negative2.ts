// SAFE: uses SSR with a safe rendering approach — escaped content only

const UserContent = ({ html }: { html: string }) => <div>{html}</div>

export default function Page() {
  const html = '<script>alert("xss")</script><p>hello</p>'
  return <UserContent html={html} />
}
