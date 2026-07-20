// SAFE: Dynamic CSS module composition paths are validated against a predefined allowlist of known module paths.

const ALLOWED_COMPOSE_SOURCES = new Set([
  './styles/base.module.css',
  './styles/theme.module.css',
  './styles/variants.module.css',
]);

function isValidComposeSource(path: string): boolean {
  return ALLOWED_COMPOSE_SOURCES.has(path);
}

import styles from './ThemedWidget.module.css';

export function ThemedWidget({ composeFrom }: { composeFrom: string }) {
  const safeSource = isValidComposeSource(composeFrom)
    ? composeFrom
    : './styles/base.module.css';

  return (
    <div className={styles.widget}>
      <style>{`.composed { composes: base from "${safeSource}"; }`}</style>
      <div className="composed">Widget Content</div>
    </div>
  );
}
