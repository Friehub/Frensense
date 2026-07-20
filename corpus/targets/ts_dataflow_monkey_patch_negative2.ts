// SAFE: a service class encapsulates the extended behavior without touching prototypes

class StringFormatter {
  static capitalize(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
  }
}

class ArrayHelper {
  static first<T>(items: T[]): T | undefined {
    return items[0];
  }
}

export function formatUserName(firstName: string, lastName: string) {
  return `${StringFormatter.capitalize(firstName)} ${StringFormatter.capitalize(lastName)}`;
}

export function getFirstItem<T>(items: T[]): T | undefined {
  return ArrayHelper.first(items);
}
