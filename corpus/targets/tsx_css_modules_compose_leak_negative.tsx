// SAFE: CSS module composition is performed locally in the CSS file itself, not dynamically via user input. No user-controlled values are passed to `composes` paths.

import styles from './ThemedWidget.module.css';

export function ThemedWidget() {
  return (
    <div className={styles.widget}>
      <div className={styles.composed}>Widget Content</div>
    </div>
  );
}
