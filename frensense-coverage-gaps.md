# Vulnerability Classes Frensense Cannot Currently Catch

Gaps are divided into three tiers. **Tier 1** is architecturally out of scope for the current taint model — catching these would require a fundamentally new analysis mode, not just new spec entries. **Tier 2** requires engine-level primitives that do not exist yet but are buildable on top of the current architecture. **Tier 3** is purely a `frensense-lang` gap — the engine already supports it, but the spec tables are missing or wrong.

---

## Tier 1 — Architecturally Out of Scope

These classes require analysis models the engine does not have and cannot grow into through incremental spec or engine changes alone.

### 1.1 Second-Order / Stored Injection

**What it is:** User input is written to a persistence layer (database, file, cache) in one request and read back in a later, separate request where it is used in a sink.

```python
# Request 1 — store
db.execute("INSERT INTO comments VALUES (?)", [request.form["comment"]])

# Request 2 — exploit (different function, different HTTP request)
row = db.fetchone("SELECT comment FROM comments WHERE id = ?", [id])
template.render(row["comment"])   # XSS — taint started in a previous request
```

**Why the engine can't catch it:** The taint engine traces data flow within a single analysis session (single file or call graph). It has no model of persistence as a taint propagator across requests. A database write does not produce a tainted return value; a database read is not marked as a source.

**What would be required:** A cross-request taint model: designate all database/file/cache reads as tainted sources in a second analysis pass, regardless of whether the written value was tainted in the first pass. This requires a two-pass architecture and understanding which APIs are persistence reads vs persistence writes.

---

### 1.2 Business Logic Flaws — IDOR, Authorization Bypass

**What it is:** User input is an ID or key used to access a resource, and the code never checks that the authenticated user is permitted to access that resource.

```javascript
app.get('/invoice/:id', (req, res) => {
  const invoice = db.find({ _id: req.params.id });  // no ownership check
  res.json(invoice);
});
```

**Why the engine can't catch it:** Taint flows correctly from `req.params.id` to `db.find`. There is no injection — the query is structurally correct. What is missing is a check that `invoice.userId === req.session.userId`. Detecting this requires the engine to know that "any function serving a resource by user-controlled ID should verify ownership" — which is a business rule, not a data flow property.

**What would be required:** A separate ownership-check analysis layer that models authorization patterns. This is a substantially different problem from taint analysis.

---

### 1.3 Race Conditions and TOCTOU

**What it is:** Check-then-act sequences where the resource state can change between the check and the use.

```python
if os.path.exists(filename):           # check
    with open(filename, 'r') as f:     # act — attacker replaces with symlink here
        content = f.read()
```

**Why the engine can't catch it:** TOCTOU detection requires identifying temporally separated operations on the same resource and knowing they are not atomic. The temporal analysis module handles event ordering within a single execution path, not concurrent interleaving.

---

### 1.4 ReDoS — Catastrophic Regex Backtracking

**What it is:** User input is passed to a regex engine with an exponential-time pattern, causing a denial of service.

```javascript
const re = /^(a+)+$/;
re.test(req.query.input);   // "aaaaaaaab" hangs for seconds
```

**Why the engine can't catch it:** This requires two independent analyses: (a) taint flow from `req.query.input` to a regex evaluation call, and (b) static analysis of the regex pattern itself to detect catastrophic backtracking (nested quantifiers, ambiguous paths). The engine has the taint flow half but has no regex pattern analyzer. Corpus-based fingerprinting is also ineffective here because the pattern is in the regex string literal, not the code structure.

---

### 1.5 Timing Side-Channel — Non-Constant-Time Comparison

**What it is:** Using `==` or `str.compare` to compare a secret value against user input allows an attacker to measure response time and recover the secret byte by byte.

```python
if user_token == stored_token:   # leaks secret via timing
if hmac.compare_digest(user_token, stored_token):  # safe
```

**Why the engine can't catch it:** The vulnerability is not in the data flow — the comparison operands are correctly obtained. It is in the comparison *operator* being used in a security context. Determining whether a comparison is a security check (and therefore requires constant-time semantics) requires semantic intent inference that goes beyond what taint or pattern matching can express.

---

### 1.6 DOM-Based XSS

**What it is:** Untrusted data from the browser's own environment (`location.hash`, `document.cookie`, `postMessage`) is written into the DOM on the client side.

```javascript
document.getElementById("result").innerHTML = location.search.split("q=")[1];
```

**Why the engine can't catch it:** The engine analyzes server-side code only. DOM XSS is entirely client-side — there is no server to analyze. A separate browser-context analysis would be needed, with different sources (`location.*`, `document.referrer`, `postMessage`) and different sinks (`innerHTML`, `outerHTML`, `document.write`, `eval`).

---

### 1.7 Mass Assignment / Parameter Binding

**What it is:** A user-controlled map (query params, JSON body) is bound directly to a model/struct, allowing the user to set fields the developer did not intend (e.g., `isAdmin = true`).

```python
user = User(**request.json)   # user can set role, isAdmin, etc.
db.session.add(user)
```

**Why the engine can't catch it:** Detecting this requires knowing the schema of `User` — which fields are sensitive and should not be user-settable — and comparing it against what the binding allows. This is a data model awareness problem; the engine has no model of ORM schemas or struct definitions.

---

## Tier 2 — Engine Gaps (Architecture Supports It, Primitives Are Missing)

These classes are within the architecture's reach but require new analysis primitives in the engine that do not exist today.

### 2.1 String Concatenation as Taint Propagation

**Affects:** SQL injection (Java `+`), LDAP injection, XPath injection, HTML injection, JavaScript template literals, Python `%` formatting, Python f-strings.

**What's missing:** The taint engine handles named function calls via `PropagatorRule` but has no handler for binary operators (`+`, `%`) or interpolation nodes (`template_literal`, `formatted_string_expression`). When a tainted string is concatenated into a query string using `+` or embedded in an f-string, the result is not marked as tainted.

```java
String query = "SELECT * WHERE id = " + userId;   // userId tainted → query not tainted
stmt.execute(query);                               // → missed injection
```

```python
query = f"SELECT * WHERE name = '{username}'"      # username tainted → query not tainted
cursor.execute(query)                              # → missed injection
```

**Engine change needed:** `data_flow/engine.rs` needs a handler for `binary_operator` nodes with `+` or `%` operators that marks the result as tainted if either operand is tainted. Template literal nodes (`template_string`, `formatted_string_expression`) need similar handling: if any interpolation expression is tainted, the entire literal result is tainted.

---

### 2.2 Member Access Taint Propagation

**Affects:** Any language where user input is a property of a parameter object: `req.body.username`, `req.params.id`, `ctx.Value(key)`, `context.WithValue`.

**What's missing:** The engine seeds taint on parameter names (via `classify_param_taint`) but does not propagate it through property/field access chains. `req` is tainted; `req.body` is not automatically tainted even though it should be.

**Engine change needed:** `data_flow/engine.rs` needs a `NodeRole::MemberAccess` handler: if the receiver object is tainted, the result of the member access is also tainted. The spec can opt in per-language by returning `NodeRole::MemberAccess` from `classify()` for `member_expression` (JS), `attribute` (Python), `field_access` (Java), `selector_expression` (Go).

---

### 2.3 Global Taint Seeding (Flask `request`, Django `request`)

**Affects:** Python Flask and Django handlers where the `request` object is an imported module-level global, not a function parameter.

**What's missing:** `classify_param_taint` only fires on function parameters. Flask view functions have no `request` parameter — they import the global `request` object. No parameter to classify means no taint seeded.

**Engine change needed:** A new mechanism for file-level taint seed injection. If the file's import list contains Flask's `request` (detectable via `extract_imports`), the name `"request"` should be pre-seeded as `TaintOrigin::UserInput` in the function's initial taint context before analysis begins.

---

### 2.4 Argument-Value-Conditional Sinks (Weak Crypto)

**Affects:** Java `MessageDigest.getInstance("MD5")`, `Cipher.getInstance("DES/ECB")`, Python `hashlib.new("md5")`, JS `crypto.createHash("md5")`.

**What's missing:** The current sink model matches on method name only. `MessageDigest.getInstance("SHA-256")` is safe; `MessageDigest.getInstance("MD5")` is a vulnerability. The same method name, different string argument. There is no mechanism to express "this call is a sink only when argument 0 is one of these specific string values."

**Engine change needed:** A new `SinkRule` variant in the spec trait:
```rust
enum SinkKind {
    Always,
    WhenArgMatches { arg_index: usize, values: &'static [&'static str] },
}
```
The engine checks the argument's literal value at the call site when this variant is active.

---

### 2.5 Absence-of-Guard Sinks (XXE, CSRF, Security Headers)

**Affects:** Java `DocumentBuilderFactory` without `setFeature(FEATURE_SECURE_PROCESSING, true)`, Spring controllers without `@CsrfToken`, Express responses without `helmet()`.

**What's missing:** These vulnerabilities are defined by what is **not** present. The engine detects positive signals (calls that happen) but cannot currently detect negative signals (configuration calls that should happen but don't).

```java
DocumentBuilderFactory dbf = DocumentBuilderFactory.newInstance();
// missing: dbf.setFeature(XMLConstants.FEATURE_SECURE_PROCESSING, true)
DocumentBuilder db = dbf.newDocumentBuilder();
db.parse(userInput);   // XXE — but only because the guard above is absent
```

**Engine change needed:** A "guard-required" sink type: a sink call is reported only if a specific suppressing call was not observed earlier in the same scope. The scope tracking (same function, same factory object) requires object identity tracking that the current alias tracker approximates but doesn't fully model.

---

### 2.6 Constructor-as-Sink

**Affects:** Java `new Random()` (weak randomness), `new File(userInput)` (path traversal), Python `subprocess.Popen(userInput)`.

**What's missing:** The taint engine processes `method_invocation` (method calls on receivers) but `object_creation_expression` (`new X(args)`) is not currently classified as a potential sink. Constructors that wrap user input into a dangerous type are therefore invisible.

**Engine change needed:** `object_creation_expression` nodes in the AST need to be checked against the sink list in the same way method calls are. The "callee" in this context is the constructor type name (`Random`, `File`, `ProcessBuilder`).

---

### 2.7 Object Spread / Destructuring Propagation (Prototype Pollution)

**Affects:** JavaScript `{ ...req.body }`, `Object.assign(target, req.body)` where target is an object with a prototype chain.

**What's missing:** `{ ...userInput }` (object spread) in the AST is a `spread_element` inside an `object`. The spread element's result is not marked as tainted even if its source is tainted. The `__proto__` key attack vector requires the engine to understand that when a tainted object is spread into another object, the keys of the tainted object (including `__proto__`) are effectively set on the target.

**Engine change needed:** Spread operator handling in the propagation loop: if a `spread_element`'s expression is tainted, the containing object literal is tainted. This combines with the member access gap — `req.body.__proto__` would need to propagate taint through to the mutation target.

---

### 2.8 CRLF / Response Splitting

**Affects:** HTTP response headers containing user input with unfiltered `\r\n` sequences.

```javascript
res.setHeader("Location", req.query.url);   // if url contains \r\n, injects headers
```

**What's missing:** The engine detects `setHeader` as a `HeaderInjection` sink when the value is tainted. But the specific subclass of header injection caused by CRLF is not distinguished. More importantly, the current taint model has no way to express "this is a sink only for strings that could contain newlines" — which is a content-property concern, not a flow concern. CRLF detection as a standalone class requires checking whether the tainted string could ever contain `\r\n`, which requires value analysis beyond what the taint model supports.

---

### 2.9 ZIP Slip / Archive Path Traversal

**Affects:** Extracting ZIP/TAR/JAR archives where entry names contain `../` sequences.

```python
with zipfile.ZipFile(upload) as z:
    for name in z.namelist():
        z.extract(name, "/var/www/html")   # name could be "../../etc/cron.d/shell"
```

**What's missing:** The source is `z.namelist()` (archive entry names from a user-uploaded file), and the sink is `z.extract(name, path)`. Neither is in any source/sink table. The source requires understanding that a user-uploaded file's archive entries are attacker-controlled data — a semantic property of the upload, not a named taint source.

**Engine + lang change needed:** New taint source category `ArchiveEntryName` and corresponding source patterns (`namelist()`, `getEntries()`, `ZipEntry.getName()`) plus new sink patterns for extraction calls (`extract()`, `extractfile()`, `ZipFile.extract`).

---

## Tier 3 — `frensense-lang` Gaps Only (Engine Supports It)

These classes are fully supported by the engine's taint model. The spec tables just need new or corrected entries.

### 3.1 LDAP Injection (JS and Python)

**Missing in:** `providers/javascript.rs`, `providers/python.rs` — `known_sink_names`

LDAP filter strings built by concatenating user input are injection sinks. No JS or Python LDAP sink entries exist. Missing:

```
// JavaScript / Node
("ldap.search",       "LdapInjection"),
("client.search",     "LdapInjection"),
("ldapjs.search",     "LdapInjection"),

// Python
("connection.search", "LdapInjection"),
("search_s",          "LdapInjection"),
("search_ext",        "LdapInjection"),
("search_st",         "LdapInjection"),
```

---

### 3.2 XPath Injection (JS and Python)

**Missing in:** `providers/javascript.rs`, `providers/python.rs` — `known_sink_names`

XPath expressions built from user input are injection sinks. Missing:

```
// JavaScript
("evaluate",          "XpathInjection"),
("select",            "XpathInjection"),   // xpath.select(expr, doc)
("selectNodes",       "XpathInjection"),

// Python
("xpath",             "XpathInjection"),   // lxml tree.xpath(user_input)
("find",              "XpathInjection"),   // ET.find(user_input) — only on ElementTree
```

Note: Python's `ET.find()` is only an XPath sink on `ElementTree` instances, not `dict.find()`. Without type inference, this will generate false positives; it should be filtered by receiver type when the HIR is available.

---

### 3.3 Insecure Cookie Configuration

**Missing in:** `providers/javascript.rs`, `providers/python.rs` — no model for configuration-level cookie flags.

Cookies set without `HttpOnly`, `Secure`, or `SameSite=Strict` flags are a vulnerability class (OWASP A2, Benchmark `securecookie` category). The sink is `res.cookie(name, value, options)` where `options` omits required flags. This is a **sink-with-options-inspection** pattern — similar to the argument-value-conditional sink in Tier 2. Short-term, the call itself can be listed as a sink to flag all cookie-setting for manual review:

```
("res.cookie",        "InsecureCookie"),
("response.set_cookie", "InsecureCookie"),
("addCookie",         "InsecureCookie"),
```

Long-term, the sink should only fire when the options object omits `httpOnly: true` or `secure: true` — which requires the absence-of-property check from Tier 2 (§2.5).

---

### 3.4 Hardcoded Credentials / Secret Scanning

**Missing everywhere:** No provider has any secret scanning capability.

Hardcoded secrets (API keys, passwords, private keys, JWT secrets, database connection strings) in source code are a distinct vulnerability class. They do not involve taint flow — the literal string value in the source code is itself the secret. Detection requires:

- Pattern matching against known secret formats (regexes for AWS key prefixes, GitHub tokens, private key headers)
- Entropy analysis on long string literals
- Context detection: `const secret = "abc123"` adjacent to `jwt.sign(...)` or `new HS256Key("...")`

This is a **static string pattern** problem, not a taint problem. The engine has a `config_literal_hashes` fingerprint dimension and the `literal_pattern_hashes` dimension, which can match corpus patterns for specific known-secret shapes. But there are no corpus patterns for this class, and the fingerprint dimension was not designed for secret scanning.

A dedicated `SecretScanner` module with entropy thresholds and format regexes would be the right addition. The frensense-lang spec could provide `secret_patterns()` returning entropy thresholds and prefix regexes per language.

---

### 3.5 `req.session` / `req.user` as Taint Sources (JS IDOR)

**Missing in:** `providers/javascript.rs` — `JS_SOURCE_PATTERNS`

As identified in the NodeGoat recall analysis: session data is attacker-controlled in many application designs. Missing source entries:

```
"req.session",
"req.session.userId",
"req.session.id",
"req.session.user",
"req.user",
"req.user.id",
"session",
"user",
```

---

### 3.6 Weak Randomness Used for Security Tokens

**Missing in:** `providers/javascript.rs` — `JS_SINK_NAMES`

`Math.random()` is not cryptographically secure. When used to generate session tokens, CSRF tokens, password reset tokens, or OTP codes, it is a vulnerability. Missing sinks:

```
("Math.random",       "WeakRandom"),
("random",            "WeakRandom"),      // JS/Python random.random()
("randint",           "WeakRandom"),      // Python random.randint()
("choice",            "WeakRandom"),      // Python random.choice() for tokens
```

The context problem (§1.5) applies here too: `Math.random()` for a game's dice roll is fine. However, listing it as a sink and relying on the corpus fingerprint match to avoid false positives (the surrounding code for a game dice roll looks nothing like a token generation function) is a workable heuristic.

---

### 3.7 GraphQL Injection

**Missing in:** `providers/javascript.rs`, `providers/python.rs` — `known_sink_names`

GraphQL queries built by string concatenation expose the same injection class as SQL, via the query string. Missing:

```
// JavaScript
("graphql",           "GraphqlInjection"),
("query",             "GraphqlInjection"),    // apollo client.query({ query: ... })
("execute",           "GraphqlInjection"),    // graphql-js execute(schema, queryString)
("buildSchema",       "GraphqlInjection"),

// Python
("graphql_sync",      "GraphqlInjection"),
("graphql",           "GraphqlInjection"),
```

The safe path (parameterized variables via GraphQL `$variable` syntax) should be classified as a sanitizer:
```
("variables",         SanitizerKind::SqlParameterize)
```

---

### 3.8 Server-Side Template Injection (SSTI) via Template Content

**Partially missing in:** `providers/javascript.rs`, `providers/python.rs`

Currently modelled only as passing the **template name** as user input (`res.render(userInput)`). The more common SSTI pattern is passing user-controlled data **into** a template that uses `eval`-style interpolation:

```python
template = Template("Hello {{ " + user_input + " }}")   # Jinja2 SSTI
result = template.render()
```

```javascript
const tmpl = Handlebars.compile("Hello {{" + userInput + "}}");  // SSTI
```

The `Template(userString)` constructor and `Handlebars.compile(userString)` are the sinks here — the user controls the template itself. Missing:

```
// Python
("Template",          "TemplateSsti"),    // Jinja2, Mako, Tornado Template()
("Environment",       "TemplateSsti"),    // Jinja2 Environment.from_string(userInput)

// JavaScript
("compile",           "TemplateSsti"),    // Handlebars.compile, pug.compile
("render",            "TemplateSsti"),    // ejs.render(userTemplate, ...)
("template",          "TemplateSsti"),    // lodash.template(userInput)
```

---

### 3.9 NoSQL Operator Injection (Object-Level)

**Partially missing in:** `providers/javascript.rs`

The engine detects the flow of a tainted **string** into a MongoDB `find()` call. But the more dangerous NoSQL injection pattern is passing a tainted **object** directly:

```javascript
const filter = req.body;                  // { username: { $gt: '' } }
User.find(filter);                        // operator injection — filter object is tainted
```

The taint model handles this only if `req.body` is in `JS_SOURCE_PATTERNS` (it is) and if object-level taint propagates to `find()` with the object as the argument. Currently, `find` is listed as `NoSqlInjection` with the sink checking whether the argument is a tainted string. A tainted object passed to `find` bypasses the sink check because it is not a string — it is an object whose reference is tainted.

**Lang change needed:** The sink specification needs to express "tainted argument of any type" not just tainted strings. Until the engine supports object-level taint, a short-term fix is to additionally flag `req.body` passed **directly** (without any intermediate key access) to MongoDB operations as a source-to-sink match.

---

### 3.10 Server-Side Request Forgery via JavaScript URL Schemes

**Missing in:** `providers/javascript.rs` — sanitizer list

`res.redirect("javascript:" + userInput)` is handled by the `OpenRedirect` sink, but the `javascript:` scheme bypass is not sanitized by any current sanitizer. The sanitizer list should include URL scheme validation:

```rust
("new URL",           SanitizerKind::UrlEncode),    // URL constructor throws on non-http
("url.parse",         SanitizerKind::UrlEncode),    // partial — does not validate scheme
```

Additionally, redirect targets with `data:` or `vbscript:` schemes need a dedicated `SchemeValidator` sanitizer category that the engine checks specifically for open redirect sinks.

---

## Summary Table

| Vulnerability Class | Tier | Where to Fix |
|---|---|---|
| Second-order / Stored injection | 1 | New two-pass architecture |
| Business logic / IDOR | 1 | New authorization analysis layer |
| Race conditions / TOCTOU | 1 | New concurrent model |
| ReDoS | 1 | New regex pattern analyzer |
| Timing side-channels | 1 | New semantic intent model |
| DOM-based XSS | 1 | New client-side analysis mode |
| Mass assignment | 1 | New schema / data model awareness |
| String concatenation taint (`+`, `%`, f-string) | 2 | `data_flow/engine.rs` — binary op handler |
| Member access taint (`req.body.x`) | 2 | `data_flow/engine.rs` — MemberAccess handler |
| Global taint seeding (Flask `request`) | 2 | New per-file seed mechanism |
| Argument-value-conditional sinks (MD5 vs SHA256) | 2 | New `SinkRule` variant in spec trait |
| Absence-of-guard sinks (XXE, CSRF, headers) | 2 | New guard-required sink type |
| Constructor-as-sink (`new Random()`) | 2 | `data_flow/engine.rs` — constructor sink check |
| Object spread / prototype pollution | 2 | `data_flow/engine.rs` — spread element handler |
| CRLF / response splitting | 2 | New content-property value analysis |
| ZIP slip / archive path traversal | 2 | New `ArchiveEntryName` source category + engine |
| LDAP injection (JS, Python) | 3 | Add sinks to JS/Python providers |
| XPath injection (JS, Python) | 3 | Add sinks to JS/Python providers |
| Insecure cookie flags | 3 | Add sinks; long-term needs absence-of-prop |
| Hardcoded credentials | 3 | New `SecretScanner` module + spec method |
| `req.session` as taint source | 3 | Add to `JS_SOURCE_PATTERNS` |
| Weak randomness (`Math.random`) | 3 | Add sinks to JS/Python providers |
| GraphQL injection | 3 | Add sinks to JS/Python providers |
| SSTI via template content | 3 | Add constructor sinks to JS/Python providers |
| NoSQL operator injection (object-level) | 3 | Object-level taint in sink check |
| SSRF via JavaScript/data URL schemes | 3 | Add scheme validator sanitizer to JS provider |
