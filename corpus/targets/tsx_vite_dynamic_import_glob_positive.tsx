// [frensense]
// observation: import.meta.glob('/src/**/*') exposes full file structure pattern to client bundle at build time.
// impact: Attackers can enumerate server-side file paths, routes, and internal module structure from client-side source maps or glob patterns.
// improvement: Use explicit import paths or restrict glob patterns to specific directories with known-safe contents.

const modules = import.meta.glob('/src/**/*');

export function FileExplorer() {
  return (
    <ul>
      {Object.keys(modules).map((path) => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  );
}
