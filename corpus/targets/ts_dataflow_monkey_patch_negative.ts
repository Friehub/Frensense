// SAFE: utility functions replace prototype extensions, avoiding global mutation

function capitalize(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function first<T>(items: T[]): T | undefined {
  return items[0];
}

export function formatUserName(firstName: string, lastName: string) {
  return `${capitalize(firstName)} ${capitalize(lastName)}`;
}

export function getFirstItem<T>(items: T[]): T | undefined {
  return first(items);
}
