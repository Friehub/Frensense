// SAFE: Restricts glob to a specific directory with known-safe public components
const modules = import.meta.glob('/src/widgets/*.tsx');

export function WidgetList() {
  return (
    <ul>
      {Object.keys(modules).map((path) => (
        <li key={path}>{path.replace('/src/widgets/', '')}</li>
      ))}
    </ul>
  );
}
