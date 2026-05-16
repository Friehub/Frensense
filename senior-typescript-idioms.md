# Senior TypeScript Idioms — A Complete Reference for LLMs

> This document teaches how experienced TypeScript engineers think and write code.
> Every rule has a "why" — senior engineers don't follow rules because a linter
> told them to. They follow them because they have seen what happens when you don't.
> When generating TypeScript, apply every principle here by default unless the
> context explicitly overrides it.

---

## 0. The Mental Model

Before any specific rule: understand how a senior TypeScript engineer reads code.

They ask four questions in order:

1. **What does the type say this can be?** — Is `null` possible here? Is this `any`? Does the type tell the truth?
2. **What happens when this fails?** — Is the error caught? Is the promise awaited? Is the rejection path handled?
3. **Where does the data come from?** — Is this user input? Is it validated before use? Could it be malicious?
4. **What does this cost at runtime?** — Does this block the event loop? Does this re-render unnecessarily? Does this fetch N+1 times?

Code that makes these four questions easy to answer is considered good TypeScript.
Code that makes them hard — regardless of whether it compiles — is considered unidiomatic.

---

## 1. The Type System Is Your Contract — Use It Fully

### The Rule

TypeScript's type system exists to make impossible states unrepresentable.
Every `any`, every `!` non-null assertion, every type cast with `as` is a place
where you are telling the compiler to stop helping you.
Senior engineers treat the type system as a tool, not an obstacle.

### Why Seniors Care

`any` does not mean "I don't know the type." It means "disable all type checking here."
A codebase that uses `any` freely is a codebase where TypeScript provides no safety guarantee.
The entire value proposition of TypeScript over JavaScript is the type checker —
disabling it defeats the purpose.

### What LLMs Typically Write (Wrong)

```typescript
// any spreads like a virus — once a value is any, everything that touches it is any
function processData(data: any): any {
  return data.user.profile.name; // no error even if this crashes at runtime
}

async function fetchUser(id: string): Promise<any> {
  const res = await fetch(`/api/users/${id}`);
  return res.json(); // returns any — all type safety lost
}

// Non-null assertion without justification
const name = user!.profile!.name!; // three potential runtime crashes
```

### What a Senior Writes

```typescript
// Define the shape explicitly
interface UserProfile {
  name: string;
  email: string;
  avatarUrl: string | null; // explicit about nullability
}

interface User {
  id: string;
  profile: UserProfile;
  createdAt: Date;
}

// Parse at the boundary, use strong types everywhere inside
async function fetchUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  if (!res.ok) {
    throw new ApiError(`Failed to fetch user ${id}`, res.status);
  }
  // Validate the response shape at the boundary
  const data = await res.json();
  return parseUser(data); // throws if shape is wrong, never returns bad data
}

// Optional chaining for nullable access — no assertion
const name = user?.profile?.name ?? "Anonymous";
```

### `unknown` Instead of `any` for Untrusted Data

```typescript
// Wrong — any completely disables type checking
function handleWebhook(payload: any) {
  processEvent(payload.type, payload.data); // no safety
}

// Right — unknown forces you to narrow before use
function handleWebhook(payload: unknown) {
  if (!isWebhookPayload(payload)) {
    throw new Error("Invalid webhook payload shape");
  }
  processEvent(payload.type, payload.data); // now typed
}

function isWebhookPayload(value: unknown): value is WebhookPayload {
  return (
    typeof value === "object" &&
    value !== null &&
    "type" in value &&
    "data" in value
  );
}
```

---

## 2. Discriminated Unions — The TypeScript Superpower

### The Rule

When a value can be one of several distinct shapes, use a discriminated union
with a literal `type` or `kind` field. This gives the compiler full narrowing
capability and makes exhaustive handling checkable at compile time.

### Why Seniors Care

The alternative — optional fields that may or may not be present — forces callers
to check for `undefined` everywhere and provides no guarantee that combinations
of fields are actually valid. Discriminated unions make invalid combinations
unrepresentable.

### What LLMs Typically Write (Wrong)

```typescript
interface ApiResponse {
  data?: User;       // only present on success
  error?: string;    // only present on failure
  statusCode: number;
  // Is it possible to have both data AND error? The type doesn't say.
  // Is it possible to have neither? The type doesn't say.
}

function handleResponse(res: ApiResponse) {
  if (res.data) {
    // Is res.error also set? We don't know from the type alone
    display(res.data);
  }
}
```

### What a Senior Writes

```typescript
type ApiResponse<T> =
  | { status: "success"; data: T; }
  | { status: "error"; error: ApiError; code: number; }
  | { status: "loading"; };

function handleResponse<T>(res: ApiResponse<T>) {
  switch (res.status) {
    case "success":
      display(res.data); // TypeScript knows data exists here
      break;
    case "error":
      showError(res.error, res.code); // TypeScript knows error and code exist here
      break;
    case "loading":
      showSpinner();
      break;
    default:
      // Exhaustiveness check — compile error if a new variant is added without handling it
      const _exhaustive: never = res;
  }
}
```

### Result Type Pattern for Fallible Operations

```typescript
// Instead of throw/catch scattered everywhere, make failure explicit in the type
type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E };

async function parseConfig(path: string): Promise<Result<Config, ConfigError>> {
  try {
    const content = await fs.readFile(path, "utf-8");
    const parsed = JSON.parse(content);
    return { ok: true, value: validateConfig(parsed) };
  } catch (err) {
    return { ok: false, error: new ConfigError(path, err) };
  }
}

// Call site — failure is visible and handled
const result = await parseConfig("./config.json");
if (!result.ok) {
  logger.error("Config failed to load", { error: result.error });
  process.exit(1);
}
// result.value is now safely typed as Config
```

---

## 3. Async/Await — Every Promise Must Be Accounted For

### The Rule

Every `Promise` must be either `await`ed, `return`ed, or explicitly handled with
`.catch()`. An unhandled promise is a silent failure — errors disappear with no
trace, and execution order becomes non-deterministic.

### Why Seniors Care

Node.js and browsers emit `UnhandledPromiseRejection` warnings that are easy to miss
in logs. In Node.js 15+, unhandled rejections crash the process. More subtly, a
floating promise means code continues executing before the async work completes —
causing race conditions that are extraordinarily hard to debug.

### What LLMs Typically Write (Wrong)

```typescript
// Floating promise — the error disappears silently
function saveUser(user: User) {
  db.users.create(user); // not awaited, not returned — fire and forget
  return { success: true }; // returned before the DB write completes
}

// async callback in forEach — forEach doesn't await
async function processAll(items: Item[]) {
  items.forEach(async (item) => {
    await processItem(item); // each iteration runs independently, errors are lost
  });
  // This line runs before any processItem completes
  logger.info("All items processed"); // lie
}

// Promise created but never awaited
function sendNotifications(users: User[]) {
  const promises = users.map(u => sendEmail(u.email));
  // promises array is created but never awaited — all emails may fail silently
}
```

### What a Senior Writes

```typescript
// Always await database operations in non-void functions
async function saveUser(user: User): Promise<SaveResult> {
  const saved = await db.users.create(user);
  return { success: true, id: saved.id };
}

// Use for...of to process items sequentially with proper error handling
async function processAllSequential(items: Item[]): Promise<void> {
  for (const item of items) {
    await processItem(item); // each item awaited before next starts
  }
  logger.info("All items processed"); // true
}

// Use Promise.all for parallel execution with unified error handling
async function processAllParallel(items: Item[]): Promise<void> {
  await Promise.all(items.map(item => processItem(item)));
  logger.info("All items processed"); // true
}

// Promise.allSettled when you want all results regardless of individual failures
async function sendNotifications(users: User[]): Promise<void> {
  const results = await Promise.allSettled(
    users.map(u => sendEmail(u.email))
  );
  const failed = results.filter(r => r.status === "rejected");
  if (failed.length > 0) {
    logger.warn(`${failed.length} notifications failed`, { failed });
  }
}
```

### The `void` Operator for Intentional Fire-and-Forget

When you genuinely want to fire-and-forget (rare, but legitimate), be explicit:

```typescript
// Implicit floating promise — ambiguous intent
sendAnalyticsEvent("page_view");

// Explicit fire-and-forget — intent is clear, linters won't complain
void sendAnalyticsEvent("page_view");
// Still add error handling inside the function so failures aren't silent
```

---

## 4. Error Handling — Errors Are Information, Not Surprises

### The Rule

Catch blocks must do something meaningful. An empty catch block or a catch block
that only logs and swallows the error is almost always wrong.
Error types should carry enough information to debug the failure without a debugger.

### Why Seniors Care

The most common production bugs are not logic errors — they are failures that were
caught, silently discarded, and then caused mysterious downstream failures.
An empty `catch` block is a lie to the next developer: "nothing can go wrong here."

### What LLMs Typically Write (Wrong)

```typescript
// Silent failure — the error is completely lost
try {
  await processPayment(order);
} catch (e) {
  // swallowed
}

// Log-and-continue — better, but still wrong if the operation was critical
try {
  await syncDatabase();
} catch (error) {
  console.log(error); // console.log, not an error logger
  // execution continues as if the sync succeeded
}

// Error with no useful information
throw new Error("Something went wrong");
```

### What a Senior Writes

```typescript
// Custom error classes carry structured information
class PaymentError extends Error {
  constructor(
    message: string,
    public readonly orderId: string,
    public readonly code: PaymentErrorCode,
    public readonly originalError?: unknown,
  ) {
    super(message);
    this.name = "PaymentError";
    // Maintain prototype chain in compiled output
    Object.setPrototypeOf(this, PaymentError.prototype);
  }
}

// Errors are caught, enriched, and re-thrown or handled decisively
async function processOrder(order: Order): Promise<OrderResult> {
  try {
    const payment = await processPayment(order);
    return { success: true, paymentId: payment.id };
  } catch (err) {
    // Classify the error
    if (err instanceof PaymentDeclinedError) {
      // Expected failure — communicate it cleanly to the caller
      return { success: false, reason: "payment_declined", code: err.code };
    }
    // Unexpected failure — wrap with context and re-throw
    throw new PaymentError(
      `Order ${order.id} payment failed unexpectedly`,
      order.id,
      PaymentErrorCode.Unknown,
      err,
    );
  }
}
```

### Handling `catch (err)` — `err` Is `unknown` in Strict Mode

```typescript
// Wrong — err is unknown, this crashes if err is not an Error
} catch (err) {
  logger.error(err.message); // TypeScript error: 'err' is of type 'unknown'
}

// Right — narrow before use
} catch (err) {
  if (err instanceof Error) {
    logger.error("Operation failed", { message: err.message, stack: err.stack });
  } else {
    logger.error("Operation failed with non-Error value", { err });
  }
}

// Utility that centralises this pattern
function toError(value: unknown): Error {
  if (value instanceof Error) return value;
  return new Error(String(value));
}

} catch (err) {
  logger.error("Operation failed", { error: toError(err) });
}
```

---

## 5. Null and Undefined — Be Deliberate About Absence

### The Rule

`null` and `undefined` are different things. `null` means "intentionally absent."
`undefined` means "not set." Pick one convention per codebase and stick to it.
Use optional chaining (`?.`) and nullish coalescing (`??`) — never assume presence
without checking.

### Why Seniors Care

`Cannot read property 'x' of undefined` is the most common runtime error in
JavaScript. TypeScript's `strictNullChecks` makes these compile errors instead —
but only if you don't use `!` to silence the checks.

### What LLMs Typically Write (Wrong)

```typescript
// Non-null assertion without proof
const userId = req.headers["x-user-id"]!; // crashes if header is absent

// Mixing null and undefined returns
function findUser(id: string): User | undefined | null {
  if (cache.has(id)) return cache.get(id); // returns undefined if not in cache
  return null; // returns null otherwise — inconsistent
}

// Type cast instead of narrowing
const element = document.getElementById("root") as HTMLElement; // crashes if null
element.innerHTML = "Hello";
```

### What a Senior Writes

```typescript
// Explicit absence check with informative error
const rawUserId = req.headers["x-user-id"];
if (!rawUserId || Array.isArray(rawUserId)) {
  throw new AuthenticationError("Missing or malformed x-user-id header");
}
const userId: string = rawUserId; // now narrowed to string

// Consistent return type — pick undefined, not null, for "not found"
function findUser(id: string): User | undefined {
  return cache.get(id); // cache.get returns T | undefined natively
}

// Or use Result for richer error information
function findUser(id: string): Result<User, "not_found"> {
  const user = cache.get(id);
  if (!user) return { ok: false, error: "not_found" };
  return { ok: true, value: user };
}

// Null check before use
const element = document.getElementById("root");
if (!element) {
  throw new Error("Root element #root not found in DOM");
}
element.innerHTML = "Hello"; // narrowed to HTMLElement
```

### Nullish Coalescing vs OR — Know the Difference

```typescript
const value = 0;

// Wrong use of || — 0 is falsy, so this uses the fallback even though 0 is valid
const count = value || 10; // count is 10, but 0 was a valid value

// Right — ?? only falls back on null/undefined, not other falsy values
const count = value ?? 10; // count is 0, correctly preserving the value
```

---

## 6. Functions — Small, Typed, Named for Their Intent

### The Rule

Functions do one thing. Their name is a verb phrase describing exactly what that
one thing is. Parameters and return types are always explicitly annotated on
public or exported functions. Arrow functions for callbacks, named functions
for module-level declarations.

### Why Seniors Care

An unnamed arrow function assigned to a variable has no name in stack traces.
When a production error occurs, the stack trace shows `<anonymous>` instead of
`validateUserInput`, making debugging significantly harder.
Well-named functions are documentation — `getUserById` tells you more than
`get` or `fetch`.

### What LLMs Typically Write (Wrong)

```typescript
// No return type — the return type is inferred as any from the fetch
export const getUser = async (id) => {
  const res = await fetch(`/users/${id}`);
  return res.json();
};

// Too many responsibilities in one function
async function handleRequest(req: Request, res: Response) {
  // authentication
  const token = req.headers.authorization?.split(" ")[1];
  const payload = jwt.verify(token, process.env.JWT_SECRET);
  // database
  const user = await db.users.findOne({ id: payload.sub });
  // business logic
  if (user.role !== "admin") throw new Error("Forbidden");
  const data = await db.orders.findMany({ userId: user.id });
  // serialization
  res.json(data.map(d => ({ id: d.id, amount: d.amount })));
}
```

### What a Senior Writes

```typescript
// Explicit types, single responsibility, clear name
async function fetchUserById(id: string): Promise<User> {
  const res = await fetch(`/users/${id}`);
  if (!res.ok) {
    throw new ApiError(`User ${id} not found`, res.status);
  }
  return parseUser(await res.json());
}

// Decomposed — each function has one job and a name that proves it
async function handleGetOrders(req: Request, res: Response): Promise<void> {
  const user = await authenticateRequest(req);     // throws on invalid auth
  await assertUserRole(user, "admin");              // throws on wrong role
  const orders = await fetchOrdersForUser(user.id);
  res.json(serializeOrders(orders));
}

function serializeOrders(orders: Order[]): SerializedOrder[] {
  return orders.map(({ id, amount }) => ({ id, amount }));
}
```

### Default Parameters vs Optional Parameters

```typescript
// Optional parameter — caller must handle undefined
function createUser(name: string, role?: string) {
  const userRole = role ?? "viewer"; // must default inside
}

// Default parameter — cleaner, the default is part of the signature
function createUser(name: string, role: string = "viewer") {
  // role is always a string here, no defaulting needed
}

// Options object for many optional parameters
interface CreateUserOptions {
  role?: string;
  sendWelcomeEmail?: boolean;
  orgId?: string;
}

function createUser(name: string, options: CreateUserOptions = {}): Promise<User> {
  const { role = "viewer", sendWelcomeEmail = true, orgId } = options;
  ...
}
```

---

## 7. Interfaces vs Types — Know When to Use Each

### The Rule

Use `interface` for object shapes that represent a domain concept or will be extended.
Use `type` for unions, intersections, primitives, and computed shapes.
Don't mix them arbitrarily — be consistent within a codebase.

### Why Seniors Care

`interface` declarations merge — if you declare the same interface twice in different
files, TypeScript merges them. This is useful for module augmentation but surprising
if unintentional. `type` aliases cannot be re-declared, which prevents accidental merging.
For domain objects, `interface` is preferred because it produces better error messages
and works more naturally with `implements`.

### What LLMs Typically Write (Wrong)

```typescript
// Using type for everything indiscriminately
type User = { id: string; name: string; };
type UserWithEmail = User & { email: string }; // extends via intersection
type UserOrAdmin = User | Admin; // this is correct use of type

// Using interface for unions (doesn't work)
interface Status = "active" | "inactive"; // syntax error
```

### What a Senior Writes

```typescript
// Interfaces for domain shapes — extensible, mergeable, clear implements
interface User {
  id: string;
  name: string;
  email: string;
  createdAt: Date;
}

interface AdminUser extends User {
  permissions: Permission[];
  lastLoginAt: Date;
}

// Types for unions, computed shapes, and utility types
type UserId = string; // newtype alias
type UserStatus = "active" | "inactive" | "suspended";
type UserOrAdmin = User | AdminUser;
type PartialUser = Partial<User>;
type UserKeys = keyof User;

// Types for function signatures
type RequestHandler = (req: Request, res: Response) => Promise<void>;
type Middleware = (req: Request, res: Response, next: NextFunction) => void;
```

---

## 8. Generics — Write Once, Type Safely

### The Rule

When a function works the same way for multiple types, make it generic.
Generic type parameters should be constrained to the minimum required shape.
A generic named `T` is fine for simple cases; more complex generics deserve
meaningful names.

### Why Seniors Care

Copy-pasting the same function with different types is how type safety breaks down.
Generics let you write the logic once and have the compiler verify it's correct
for every type it's used with. Over-constraining generics (`T extends object`)
makes them less useful; under-constraining them loses type safety.

### What LLMs Typically Write (Wrong)

```typescript
// Duplicate functions for different types
async function fetchUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  return res.json();
}
async function fetchProduct(id: string): Promise<Product> {
  const res = await fetch(`/api/products/${id}`);
  return res.json();
}

// Generic that's too loose — T could be anything
function first<T>(arr: T[]): T {
  return arr[0]; // returns undefined if array is empty — T doesn't say that
}
```

### What a Senior Writes

```typescript
// Single generic fetch function — works for any endpoint
async function fetchResource<T>(url: string, parser: (raw: unknown) => T): Promise<T> {
  const res = await fetch(url);
  if (!res.ok) {
    throw new ApiError(`Request to ${url} failed`, res.status);
  }
  return parser(await res.json()); // parser handles validation at the boundary
}

// Used as:
const user = await fetchResource(`/api/users/${id}`, parseUser);
const product = await fetchResource(`/api/products/${id}`, parseProduct);

// Generic with honest return type
function first<T>(arr: readonly T[]): T | undefined {
  return arr[0]; // undefined is in the type — callers must handle it
}

// Constrained generic — T must have an id field
function deduplicateById<T extends { id: string }>(items: T[]): T[] {
  const seen = new Set<string>();
  return items.filter(item => {
    if (seen.has(item.id)) return false;
    seen.add(item.id);
    return true;
  });
}
```

### Utility Types — Use What TypeScript Gives You

```typescript
// Don't rebuild what's already in the standard library
type ReadonlyUser = { readonly id: string; readonly name: string }; // manual
type ReadonlyUser = Readonly<User>; // built-in

type OptionalUser = { id?: string; name?: string }; // manual
type OptionalUser = Partial<User>; // built-in

type RequiredUser = { id: string; name: string }; // manual
type RequiredUser = Required<User>; // built-in, useful when Partial was used

// Pick and Omit for derived shapes
type UserSummary = Pick<User, "id" | "name">;
type UserWithoutPassword = Omit<User, "passwordHash">;

// Record for typed dictionaries
type UserMap = Record<string, User>; // not { [key: string]: User }
type StatusMap = Record<UserStatus, number>; // all status values must be present
```

---

## 9. Immutability — Prefer `const`, `readonly`, and Frozen Structures

### The Rule

Use `const` for every variable declaration unless reassignment is required.
Mark function parameters and interface fields `readonly` when they should not
be mutated. Treat arrays and objects passed into functions as immutable by default.

### Why Seniors Care

Mutation at a distance is the primary source of subtle bugs in large codebases.
When a function receives an array and mutates it, the caller's data changes —
often unexpectedly. `readonly` in the type system makes the intent explicit and
lets the compiler prevent accidental mutation.

### What LLMs Typically Write (Wrong)

```typescript
// let used for variables that are never reassigned
let userId = getUserId(req);
let config = loadConfig();

// Mutating a parameter
function sortUsers(users: User[]): User[] {
  users.sort((a, b) => a.name.localeCompare(b.name)); // mutates the original
  return users;
}

// Spreading without readonly
function processItems(items: Item[]) {
  items.push(newItem); // caller's array is modified
}
```

### What a Senior Writes

```typescript
// const everywhere — reassignment requires justification
const userId = getUserId(req);
const config = loadConfig();

// Readonly parameter — cannot mutate the input
function sortUsers(users: readonly User[]): User[] {
  return [...users].sort((a, b) => a.name.localeCompare(b.name));
  // Creates a new sorted array, original is untouched
}

// Readonly arrays and objects for data that should never change
const SUPPORTED_CURRENCIES = ["USD", "EUR", "GBP"] as const;
type SupportedCurrency = typeof SUPPORTED_CURRENCIES[number]; // "USD" | "EUR" | "GBP"

interface Config {
  readonly apiUrl: string;
  readonly timeout: number;
  readonly retries: number;
}
```

### Immutable Updates for State

```typescript
// Wrong — direct mutation
function addPermission(user: User, permission: Permission): User {
  user.permissions.push(permission); // mutates the original user
  return user;
}

// Right — return new value
function addPermission(user: User, permission: Permission): User {
  return {
    ...user,
    permissions: [...user.permissions, permission],
  };
}
```

---

## 10. Imports and Exports — Explicit, Organised, Purposeful

### The Rule

Prefer named exports over default exports for everything except framework conventions
(React components, Next.js pages). Import only what you use. Group imports:
external packages first, then internal modules, separated by a blank line.

### Why Seniors Care

Default exports can be imported under any name — `import User from "./user"` and
`import Cat from "./user"` both compile. Named exports enforce consistent naming
across the codebase. Barrel imports (`import * as _`) prevent tree-shaking and
inflate bundle sizes in client-side code.

### What LLMs Typically Write (Wrong)

```typescript
// Default exports everywhere — imported under different names in different files
export default function processUser(user: any) { ... }

// Star import — imports everything, prevents tree-shaking
import * as lodash from "lodash";
lodash.debounce(fn, 300); // pulls entire lodash into the bundle

// Unsorted, ungrouped imports
import { useState } from "react";
import { db } from "../lib/db";
import axios from "axios";
import { User } from "./types";
import path from "path";
```

### What a Senior Writes

```typescript
// External packages
import { useState, useCallback } from "react";
import { z } from "zod";

// Internal modules — separated by blank line from externals
import { db } from "@/lib/db";
import { logger } from "@/lib/logger";
import type { User, Session } from "@/types";

// Named exports — consistent naming everywhere it's imported
export function processUser(user: User): ProcessedUser { ... }
export function validateUser(user: unknown): user is User { ... }
export type { ProcessedUser };

// Named specific imports — tree-shakeable
import { debounce, groupBy } from "lodash-es"; // es module version
```

### Type-Only Imports

```typescript
// Mark imports as type-only when only used in type position
// This ensures they're erased at compile time and not bundled
import type { User } from "./types";
import type { Request, Response } from "express";

// Mixed import when you need both value and type
import { Severity, type SeverityLevel } from "./severity";
```

---

## 11. Classes — Use Them Purposefully, Not by Default

### The Rule

Use classes when you have encapsulated state with associated behaviour, or when
implementing an interface. Do not use classes for collections of utility functions —
use plain exported functions instead. Do not use classes just because you come
from Java or C#.

### Why Seniors Care

A class full of static methods is just a namespace — and TypeScript has modules
for that. Unnecessary classes add cognitive overhead (instantiation, `this` binding,
inheritance chains) without benefit. But classes are the right tool when you have
genuine object identity and lifecycle: database connections, event emitters, caches.

### What LLMs Typically Write (Wrong)

```typescript
// Class used as a namespace — unnecessary
class UserUtils {
  static validateEmail(email: string): boolean { ... }
  static formatName(first: string, last: string): string { ... }
  static generateAvatar(name: string): string { ... }
}

// Called as:
UserUtils.validateEmail(email); // why not just validateEmail(email)?
```

### What a Senior Writes

```typescript
// Plain functions — simpler, tree-shakeable, no this binding issues
export function validateEmail(email: string): boolean { ... }
export function formatName(first: string, last: string): string { ... }
export function generateAvatar(name: string): string { ... }

// Class used correctly — encapsulated state with lifecycle
export class DatabasePool {
  private readonly pool: pg.Pool;
  private isConnected = false;

  constructor(private readonly config: DatabaseConfig) {
    this.pool = new pg.Pool(config);
  }

  async connect(): Promise<void> {
    await this.pool.connect();
    this.isConnected = true;
  }

  async query<T>(sql: string, params?: unknown[]): Promise<T[]> {
    if (!this.isConnected) throw new Error("Pool not connected");
    const result = await this.pool.query(sql, params);
    return result.rows as T[];
  }

  async disconnect(): Promise<void> {
    await this.pool.end();
    this.isConnected = false;
  }
}
```

### Access Modifiers Are Not Optional

```typescript
// Wrong — every property is implicitly public
class UserService {
  db: Database;
  cache: Cache;
  logger: Logger;

  constructor(db: Database, cache: Cache, logger: Logger) {
    this.db = db;
    this.cache = cache;
    this.logger = logger;
  }
}

// Right — explicit visibility, constructor shorthand
class UserService {
  constructor(
    private readonly db: Database,
    private readonly cache: Cache,
    private readonly logger: Logger,
  ) {}

  // Only the methods callers need are public
  async getUser(id: string): Promise<User | undefined> { ... }
}
```

---

## 12. Validation at Boundaries — Trust Nothing from Outside

### The Rule

Data from HTTP requests, file reads, environment variables, message queues,
and third-party APIs must be validated before use. Validation happens at the
entry point, once. Everything inside the boundary operates on typed, validated data.

### Why Seniors Care

TypeScript's types are compile-time only — they do not exist at runtime.
A `User` object that came from `res.json()` is `any` at runtime regardless of
what the type says. If the API returns unexpected data, your types lie and your
code crashes. Validation at the boundary makes the types truthful.

### What LLMs Typically Write (Wrong)

```typescript
// res.json() returns any — the type annotation is a lie
async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  const user = await res.json() as User; // cast, not validation — unsafe
  return user; // could be anything at runtime
}

// Environment variables are strings — this crashes if PORT isn't a number
const PORT = parseInt(process.env.PORT); // NaN if undefined
```

### What a Senior Writes

```typescript
// Using zod for runtime validation — types and validation in one place
import { z } from "zod";

const UserSchema = z.object({
  id: z.string().uuid(),
  name: z.string().min(1),
  email: z.string().email(),
  role: z.enum(["admin", "viewer", "editor"]),
  createdAt: z.coerce.date(),
});

type User = z.infer<typeof UserSchema>; // type derived from schema — always in sync

async function getUser(id: string): Promise<User> {
  const res = await fetch(`/api/users/${id}`);
  if (!res.ok) throw new ApiError("User fetch failed", res.status);
  const raw = await res.json();
  return UserSchema.parse(raw); // throws ZodError with details if shape is wrong
}

// Environment validation at startup — fail fast with a clear message
const EnvSchema = z.object({
  PORT: z.coerce.number().min(1).max(65535).default(3000),
  DATABASE_URL: z.string().url(),
  JWT_SECRET: z.string().min(32),
  NODE_ENV: z.enum(["development", "test", "production"]),
});

export const env = EnvSchema.parse(process.env);
// All downstream code uses env.PORT (number), env.DATABASE_URL (string), etc.
// If any variable is missing or malformed, the process exits at startup with a clear error.
```

---

## 13. Documentation — Comments Explain Why, Types Explain What

### The Rule

Types document *what* something is. Comments document *why* it is that way.
Every exported function, class, and type gets a JSDoc comment.
Comments on implementation detail explain non-obvious decisions, not obvious ones.

### Why Seniors Care

`/** Gets the user. */` above `function getUser()` is noise — the name already
says that. A useful comment explains the invariant, the tradeoff, or the
non-obvious edge case: *why* this function exists, *why* this approach was chosen
over the obvious alternative, *what* the caller must ensure before calling.

### What LLMs Typically Write (Wrong)

```typescript
// Restates the name — useless
/** Gets the user by ID. */
function getUserById(id: string): Promise<User> { ... }

// No doc on exported types
export interface Config { ... }

// Obvious comment on implementation
const items = []; // initialize empty array
for (const item of data) { // loop through data
  items.push(item); // add item to array
}
```

### What a Senior Writes

```typescript
/**
 * Fetches a user from the cache if available, falling back to the database.
 *
 * Cache TTL is 5 minutes. After expiry the database result is re-cached.
 * Returns `undefined` if no user with this ID exists in either store.
 *
 * @throws {DatabaseError} If the database is unreachable (cache miss scenario only)
 */
export async function getUserById(id: UserId): Promise<User | undefined> { ... }

/**
 * Canonical representation of a user account.
 *
 * `passwordHash` is intentionally excluded from this type — use `UserWithCredentials`
 * in authentication contexts. This type is safe to serialize to API responses.
 */
export interface User {
  id: UserId;
  name: string;
  email: string;
  role: UserRole;
  createdAt: Date;
}

// Non-obvious implementation decision — worth a comment
// We use a WeakMap here so entries are automatically garbage-collected when
// the DOM node is removed, preventing memory leaks in long-running SPAs.
const nodeMetadata = new WeakMap<Node, NodeMetadata>();
```

---

## 14. Testing — Tests Document Behaviour

### The Rule

Tests describe *behaviour*, not implementation. Test names are sentences:
"returns undefined when user is not found", not "test getUserById".
Tests cover the failure path as much as the happy path.
Mock at the boundary (HTTP, database), not in the middle of your code.

### Why Seniors Care

Tests that describe behaviour survive refactors — the behaviour doesn't change
even when the implementation does. Tests that describe implementation break on
every refactor, providing no safety guarantee. A test suite that only tests
the happy path is a test suite that does not find bugs.

### What LLMs Typically Write (Wrong)

```typescript
// Describes implementation, not behaviour
test("getUserById calls fetch", async () => {
  const mockFetch = jest.fn().mockResolvedValue({ ok: true, json: () => ({}) });
  global.fetch = mockFetch;
  await getUserById("123");
  expect(mockFetch).toHaveBeenCalled(); // tests that fetch was called, not what happens
});

// No failure cases tested
describe("UserService", () => {
  it("returns user", async () => {
    const user = await service.getUser("valid-id");
    expect(user).toBeDefined();
  });
  // What about invalid IDs? Network errors? Not-found responses?
});
```

### What a Senior Writes

```typescript
describe("getUserById", () => {
  it("returns the user when found in cache", async () => {
    cache.set("user:abc", mockUser);
    const result = await getUserById("abc");
    expect(result).toEqual(mockUser);
  });

  it("returns undefined when user does not exist", async () => {
    server.use(http.get("/api/users/:id", () => HttpResponse.json(null, { status: 404 })));
    const result = await getUserById("nonexistent");
    expect(result).toBeUndefined();
  });

  it("throws DatabaseError when the database is unreachable", async () => {
    server.use(http.get("/api/users/:id", () => HttpResponse.error()));
    await expect(getUserById("abc")).rejects.toThrow(DatabaseError);
  });

  it("re-validates the cached value against the schema on cache hit", async () => {
    // Corrupt cached data — simulates schema drift between deployments
    cache.set("user:abc", { id: "abc" }); // missing required fields
    await expect(getUserById("abc")).rejects.toThrow(ValidationError);
  });
});
```

---

## 15. Performance — Know the Event Loop

### The Rule

Never block the event loop. Synchronous heavy computation, synchronous I/O,
and large JSON parse/stringify operations freeze Node.js for every concurrent
request. CPU-bound work belongs in a worker thread.

### The Cost Table Every Senior Has Internalized

| Operation | Relative Cost | Notes |
|---|---|---|
| Property access | 1× | Effectively free |
| Function call | 1-2× | Effectively free |
| Array spread `[...arr]` | ~10× per element | Allocates new array |
| `JSON.parse(str)` | ~100× per KB | Blocks event loop |
| `JSON.stringify(obj)` | ~100× per KB | Blocks event loop |
| `fs.readFileSync()` | ~10,000× | Blocks ALL concurrent requests |
| `await fs.readFile()` | ~10,000× but async | Does not block other requests |
| Database query | ~10,000-1,000,000× | Always async |

### What LLMs Typically Write (Wrong)

```typescript
// Synchronous I/O in a request handler — blocks all concurrent requests
app.get("/config", (req, res) => {
  const config = fs.readFileSync("./config.json", "utf-8"); // blocks event loop
  res.json(JSON.parse(config));
});

// forEach with async — race condition, errors lost
app.post("/process", async (req, res) => {
  req.body.items.forEach(async (item) => {
    await processItem(item); // NOT actually awaited by forEach
  });
  res.json({ done: true }); // sent before any processing completes
});
```

### What a Senior Writes

```typescript
// Async I/O — never blocks other requests
app.get("/config", async (req, res) => {
  const raw = await fs.promises.readFile("./config.json", "utf-8");
  res.json(JSON.parse(raw));
});

// Load config once at startup, serve from memory per request
let cachedConfig: Config | null = null;

async function getConfig(): Promise<Config> {
  if (!cachedConfig) {
    const raw = await fs.promises.readFile("./config.json", "utf-8");
    cachedConfig = ConfigSchema.parse(JSON.parse(raw));
  }
  return cachedConfig;
}

// Proper parallel processing
app.post("/process", async (req, res) => {
  const results = await Promise.all(
    req.body.items.map((item) => processItem(item))
  );
  res.json({ done: true, results });
});
```

---

## 16. Security — Treat User Input as Hostile

### The Rule

Every value from user input, request bodies, query parameters, headers, and
external APIs is untrusted until validated. Never interpolate user input into
SQL queries, shell commands, HTML, or file paths without sanitisation.
Never log sensitive fields.

### Why Seniors Care

Injection vulnerabilities — SQL injection, XSS, path traversal — all have the
same root cause: treating user-controlled strings as trusted. TypeScript's type
system cannot protect against this because the type says `string` regardless of
whether the string is `"Alice"` or `"'; DROP TABLE users; --"`.

### What LLMs Typically Write (Wrong)

```typescript
// SQL injection — user input directly in query string
async function getUser(id: string) {
  return db.query(`SELECT * FROM users WHERE id = '${id}'`); // catastrophic
}

// Path traversal — user controls the file path
app.get("/files/:name", (req, res) => {
  const filePath = path.join(__dirname, "uploads", req.params.name);
  res.sendFile(filePath); // "../../etc/passwd" is a valid name
});

// Logging sensitive data
logger.info("User logged in", { user, password: req.body.password }); // password in logs
```

### What a Senior Writes

```typescript
// Parameterised queries — user input is always a parameter, never part of the SQL
async function getUser(id: string): Promise<User | undefined> {
  const result = await db.query(
    "SELECT * FROM users WHERE id = $1", // query template
    [id]                                  // parameters — never interpolated
  );
  return result.rows[0];
}

// Path traversal prevention
app.get("/files/:name", (req, res) => {
  const safeBase = path.resolve(__dirname, "uploads");
  const requestedPath = path.resolve(safeBase, req.params.name);

  // Ensure the resolved path is still inside the uploads directory
  if (!requestedPath.startsWith(safeBase + path.sep)) {
    res.status(403).json({ error: "Forbidden" });
    return;
  }
  res.sendFile(requestedPath);
});

// Never log sensitive fields — use a structured logger with field filtering
logger.info("User logged in", {
  userId: user.id,
  email: user.email,
  // password, token, secret are never logged
});
```

---

## Quick Reference Checklist

When reviewing or generating TypeScript, run through this list:

**Types**
- [ ] No `any` — use `unknown` for untrusted data, then narrow it
- [ ] No `!` non-null assertions without a comment proving the invariant
- [ ] No `as TypeCast` without validation preceding it
- [ ] Discriminated unions for values with multiple distinct shapes
- [ ] `readonly` on parameters and fields that should not be mutated
- [ ] `as const` for literal arrays and objects used as type sources

**Async**
- [ ] Every `Promise` is `await`ed, `return`ed, or `void`-annotated intentionally
- [ ] No `async` callback inside `.forEach()` — use `for...of` or `Promise.all`
- [ ] No synchronous I/O (`readFileSync`, `execSync`) in request handlers
- [ ] `Promise.allSettled` when partial failures are acceptable

**Errors**
- [ ] No empty `catch` blocks
- [ ] No `catch (err)` that accesses `err.message` without narrowing to `Error`
- [ ] Custom error classes extend `Error` with `Object.setPrototypeOf`
- [ ] All errors include enough context to debug without a debugger

**Null Safety**
- [ ] `??` instead of `||` when the left side can be a valid falsy value
- [ ] `?.` optional chaining instead of `&&` chains for property access
- [ ] Explicit `undefined` returns when "not found" is a valid outcome

**Structure**
- [ ] No functions with more than ~4-5 parameters — use an options object
- [ ] No class that is just a namespace of static methods — use plain functions
- [ ] Named exports over default exports (except framework conventions)
- [ ] No `import * as` for large libraries — import named exports

**Validation**
- [ ] All external data (HTTP, env vars, files) validated with a schema (Zod, etc.)
- [ ] `env` variables validated at startup, not at usage sites
- [ ] No `as Type` cast of `res.json()` — parse and validate it

**Security**
- [ ] No string interpolation into SQL — use parameterised queries
- [ ] No user-controlled paths without `path.resolve` + prefix check
- [ ] No sensitive fields in log statements

**Tests**
- [ ] Test names are behaviour descriptions, not implementation descriptions
- [ ] Failure paths are tested (404, network error, validation failure)
- [ ] Mocks are at the boundary (HTTP server, DB), not inside business logic

---

## The Single Most Important Principle

> **If it's possible at runtime, it must be possible in the type.**

The cardinal sin of TypeScript is lying with types — writing `Promise<User>` when
the function can return `undefined`, writing `string` when the value can be `null`,
writing `User` when the data hasn't been validated.

Every senior TypeScript engineer's instinct, when they see `any`, `!`, `as Type`,
or `res.json()` without validation, is: *the type is lying here.*

Types that tell the truth mean the compiler finds bugs before production does.
Types that lie mean the compiler gives false confidence and bugs reach users.
Every `any` and every cast is a bet that the data will always be what you expect.
Senior engineers don't make that bet.
