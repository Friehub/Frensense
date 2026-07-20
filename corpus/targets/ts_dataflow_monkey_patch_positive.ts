// [frensense]
// observation: The code modifies built-in prototypes or global objects at runtime, affecting the behavior of all usages of that type across the entire application.
// impact: Monkey-patching native objects can break third-party libraries that rely on original behavior, cause subtle bugs in unrelated code, and make the application unpredictable. It violates encapsulation and is a common source of hard-to-find bugs.
// improvement: Use wrapper functions, utility modules, or extension methods instead of modifying built-in prototypes.

String.prototype.capitalize = function () {
  return this.charAt(0).toUpperCase() + this.slice(1);
};

Array.prototype.first = function <T>(): T | undefined {
  return this[0];
};

export function formatUserName(firstName: string, lastName: string) {
  return `${firstName.capitalize()} ${lastName.capitalize()}`;
}

export function getFirstItem<T>(items: T[]): T | undefined {
  return items.first();
}
