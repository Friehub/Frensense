# Frensense Runtime — Multi-Language & Multi-Framework Design

Based on full corpus audit: 5 languages (ts, tsx, rust, go, nodejs), 30+ frameworks
(Express, Fastify, Hono, NestJS, Next.js, Remix, SvelteKit, Actix-web, Axum, Warp,
Rocket, Go net/http, Gin, Echo, and more), 2000+ patterns across all of them.

---

## The Core Insight: HTTP Is the Universal Normalization Layer

This is the most important thing to understand about the multi-language problem:

```
TypeScript + Express  ─┐
Rust + Axum           ─┤   All speak HTTP   ┌─► Same probe payloads
Go + Gin              ─┤  ───────────────►  │   Same oracle logic
Python + FastAPI      ─┤                    │   Same canary server
PHP + Laravel         ─┘                    └─► Same timing analysis
```

The **oracle never changes** regardless of what language the server is written in.
`; sleep 5 #` delays the response by 5 seconds whether the handler is in
TypeScript, Rust, Go, or Python. `root:x:0:0` in the response body means
the same thing in every language. The canary callback server receives TCP
connections regardless of which HTTP client the target uses to make SSRF
requests.

What **does** change per language and framework:

| What changes | Why it changes | Where it lives |
|---|---|---|
| Route registration syntax | Each framework registers handlers differently | `RouteExtractor` |
| Injection point access syntax | `req.body.x` vs `c.Query("x")` vs `web::Json<T>` | `InjectionPointExtractor` |
| Server startup command | `ts-node`, `cargo run`, `go run`, `python -m` | `ServerProbe` |
| Framework auto-detection | package.json vs Cargo.toml vs go.mod | `ProjectDetector` |
| Non-HTTP bug probing strategy | Async bugs, hook bugs, race conditions | `NonHttpProber` |

Everything else — probes, oracles, canary server, behavioral traces, scheduling,
reporting — is **identical** across all languages.

---

## Architecture: The Language Adapter Pattern

```
┌────────────────────────────────────────────────────────────────────┐
│                     Frensense Runtime Core                          │
│                                                                     │
│   ┌──────────────┐    ┌──────────────────────────────────────────┐ │
│   │ Static       │    │           Language Adapter               │ │
│   │ Report JSON  │───►│                                          │ │
│   └──────────────┘    │  detect_framework(project_root)         │ │
│                       │       │                                  │ │
│                       │  ┌────▼──────────────────────────────┐  │ │
│                       │  │   FrameworkAdapter (trait)         │  │ │
│                       │  │   ─────────────────────────────── │  │ │
│                       │  │   fn extract_routes()             │  │ │
│                       │  │   fn extract_injection_points()   │  │ │
│                       │  │   fn startup_command()            │  │ │
│                       │  │   fn auth_convention()            │  │ │
│                       │  └───────────────────────────────────┘  │ │
│                       │       │  Implemented by:                 │ │
│                       │  ExpressAdapter  │  AxumAdapter          │ │
│                       │  FastifyAdapter  │  ActixAdapter         │ │
│                       │  NextJsAdapter   │  GinAdapter           │ │
│                       │  RemixAdapter    │  EchoAdapter          │ │
│                       │  NestJsAdapter   │  NetHttpAdapter       │ │
│                       │  HonoAdapter     │  (+ future: Flask,    │ │
│                       │  SvelteKitAdptr  │   Django, Laravel)    │ │
│                       └──────────────────────────────────────────┘ │
│                                    │                               │
│                       ┌────────────▼───────────────────────────┐  │
│                       │         HTTP Probe Layer               │  │
│                       │   (identical for every language)       │  │
│                       │   probes / oracle / canary / tracer    │  │
│                       └────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────┘
```

### The `FrameworkAdapter` trait

**New file: `frensense-runtime/src/adapters/mod.rs`**

```rust
use crate::route_extractor::{RouteBinding, InjectionPoint};

/// One implementation per framework.
/// The runtime selects the right adapter from `detect_framework()`.
pub trait FrameworkAdapter: Send + Sync {
    /// Human name, e.g. "Express", "Axum", "Go/Gin"
    fn name(&self) -> &'static str;

    /// Extract HTTP route bindings from a source file.
    /// Called once per file that appears in the static report.
    fn extract_routes(&self, file_path: &std::path::Path, source: &str) -> Vec<RouteBinding>;

    /// Extract injection points from the function body text.
    /// The `original_content` field of Advisory is the input.
    fn extract_injection_points(&self, function_body: &str) -> Vec<InjectionPoint>;

    /// Command to start the server, e.g. `["npx", "ts-node", "src/index.ts"]`.
    /// Used when --start-server is passed (optional feature).
    fn startup_command(&self, project_root: &std::path::Path) -> Option<Vec<String>>;

    /// How this framework expresses authentication in requests.
    /// Used to inject credentials from --auth-* flags.
    fn auth_convention(&self) -> AuthConvention;

    /// File extensions this framework operates on.
    fn extensions(&self) -> &'static [&'static str];
}

#[derive(Debug, Clone)]
pub enum AuthConvention {
    BearerToken,             // Authorization: Bearer <token>
    SessionCookie(String),   // Cookie: <name>=<value>
    ApiKeyHeader(String),    // X-API-Key: <key>
    BasicAuth,               // Authorization: Basic <base64>
    None,
}
```

### Framework auto-detection

**New file: `frensense-runtime/src/adapters/detector.rs`**

```rust
use std::path::Path;
use std::fs;

pub fn detect_framework(
    project_root: &Path,
    advisory_rule_id: &str,
) -> Box<dyn FrameworkAdapter> {
    // 1. Language prefix from rule_id narrows the search
    let lang_prefix = advisory_rule_id.split('_').next().unwrap_or("");

    match lang_prefix {
        "ts" | "tsx" | "nodejs" => detect_ts_framework(project_root),
        "rust"                  => detect_rust_framework(project_root),
        "go"                    => detect_go_framework(project_root),
        "py"                    => detect_python_framework(project_root),
        "php"                   => detect_php_framework(project_root),
        _                       => Box::new(UnknownAdapter),
    }
}

fn detect_ts_framework(root: &Path) -> Box<dyn FrameworkAdapter> {
    let pkg = read_package_json(root);

    // Check dependencies in order of specificity
    if has_dep(&pkg, "next")             { return Box::new(NextJsAdapter); }
    if has_dep(&pkg, "@remix-run/node")
    || has_dep(&pkg, "@remix-run/react") { return Box::new(RemixAdapter); }
    if has_dep(&pkg, "@sveltejs/kit")    { return Box::new(SvelteKitAdapter); }
    if has_dep(&pkg, "astro")            { return Box::new(AstroAdapter); }
    if has_dep(&pkg, "@nestjs/core")     { return Box::new(NestJsAdapter); }
    if has_dep(&pkg, "fastify")          { return Box::new(FastifyAdapter); }
    if has_dep(&pkg, "hono")             { return Box::new(HonoAdapter); }
    if has_dep(&pkg, "@trpc/server")     { return Box::new(TrpcAdapter); }
    if has_dep(&pkg, "express")          { return Box::new(ExpressAdapter); }

    // Fall back to generic Express-style if no match
    Box::new(ExpressAdapter)
}

fn detect_rust_framework(root: &Path) -> Box<dyn FrameworkAdapter> {
    let cargo = read_cargo_toml(root);

    if has_crate(&cargo, "axum")         { return Box::new(AxumAdapter); }
    if has_crate(&cargo, "actix-web")    { return Box::new(ActixAdapter); }
    if has_crate(&cargo, "rocket")       { return Box::new(RocketAdapter); }
    if has_crate(&cargo, "warp")         { return Box::new(WarpAdapter); }
    if has_crate(&cargo, "tonic")        { return Box::new(TonicAdapter); }  // gRPC — no HTTP

    Box::new(AxumAdapter) // most common modern Rust web framework
}

fn detect_go_framework(root: &Path) -> Box<dyn FrameworkAdapter> {
    let go_mod = read_go_mod(root);

    if go_mod.contains("github.com/gin-gonic/gin")      { return Box::new(GinAdapter); }
    if go_mod.contains("github.com/labstack/echo")      { return Box::new(EchoAdapter); }
    if go_mod.contains("github.com/go-chi/chi")         { return Box::new(ChiAdapter); }
    if go_mod.contains("github.com/gorilla/mux")        { return Box::new(GorillaMuxAdapter); }
    if go_mod.contains("github.com/gofiber/fiber")      { return Box::new(FiberAdapter); }

    Box::new(NetHttpAdapter) // standard library fallback
}

fn read_package_json(root: &Path) -> serde_json::Value {
    let candidates = ["package.json", "apps/api/package.json", "backend/package.json"];
    for c in &candidates {
        if let Ok(s) = fs::read_to_string(root.join(c)) {
            if let Ok(v) = serde_json::from_str(&s) { return v; }
        }
    }
    serde_json::Value::Null
}

fn has_dep(pkg: &serde_json::Value, name: &str) -> bool {
    ["dependencies", "devDependencies", "peerDependencies"].iter().any(|key| {
        pkg[key].as_object()
            .map_or(false, |deps| deps.contains_key(name))
    })
}
```

---

## TypeScript / JavaScript Framework Adapters

### Express & Fastify

Both use the same route registration style: `app.METHOD(path, handler)`.

**New file: `frensense-runtime/src/adapters/express.rs`**

```rust
pub struct ExpressAdapter;

impl FrameworkAdapter for ExpressAdapter {
    fn name(&self) -> &'static str { "Express" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // Matches:  app.get('/path', handler)
        //           router.post('/path', async (req, res) => { ... })
        //           app.all('/path', handler)
        let route_re = regex::Regex::new(
            r#"(?:app|router|server)\.(get|post|put|delete|patch|all)\s*\(\s*['"`]([^'"`]+)['"`]"#
        ).unwrap();

        route_re.captures_iter(source).map(|cap| {
            let method = http_method_from_str(&cap[1]);
            let path   = cap[2].to_string();
            RouteBinding {
                method,
                path_pattern: path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: String::new(), // resolved separately
                injection_points: Vec::new(),
                framework: Framework::Express,
            }
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // Express: req.body.X, req.query.X, req.params.X, req.headers.X, req.cookies.X
        extract_by_patterns(body, &[
            (r"req\.body\.(\w+)",    ParameterLocation::Body),
            (r"req\.query\.(\w+)",   ParameterLocation::Query),
            (r"req\.params\.(\w+)",  ParameterLocation::PathParam),
            (r"req\.headers\[?['\"]?(\w[\w-]*)['\"]?\]?",  ParameterLocation::Header),
            (r"req\.cookies\.(\w+)", ParameterLocation::Cookie),
            // Destructured: const { cmd } = req.body
            (r"req\.body\b",         ParameterLocation::Body),  // whole body
        ])
    }

    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        // Try common entry points in order
        for entry in &["src/index.ts", "src/app.ts", "index.ts", "app.ts", "server.ts"] {
            if root.join(entry).exists() {
                return Some(vec![
                    "npx".to_string(), "ts-node".to_string(), entry.to_string()
                ]);
            }
        }
        // Fallback: use package.json start script
        Some(vec!["npm".to_string(), "run".to_string(), "start".to_string()])
    }

    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}

// FastifyAdapter is identical except the route pattern also matches:
// fastify.get('/path', async (request, reply) => { ... })
// request.body.X, request.query.X, request.params.X
pub struct FastifyAdapter;
impl FrameworkAdapter for FastifyAdapter {
    fn name(&self) -> &'static str { "Fastify" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        extract_by_patterns(body, &[
            (r"request\.body\.(\w+)",   ParameterLocation::Body),
            (r"request\.query\.(\w+)",  ParameterLocation::Query),
            (r"request\.params\.(\w+)", ParameterLocation::PathParam),
            (r"request\.headers\.(\w+)", ParameterLocation::Header),
        ])
    }
    // routes and startup same as Express with fastify.METHOD instead
    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        let route_re = regex::Regex::new(
            r#"(?:fastify|server|app)\.(get|post|put|delete|patch)\s*\(\s*['"`]([^'"`]+)['"`]"#
        ).unwrap();
        route_re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[1]),
            path_pattern: cap[2].to_string(),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::Fastify,
        }).collect()
    }
    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "start".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Next.js — File-System Routing

Next.js has no explicit route registration — the file path IS the route.

**New file: `frensense-runtime/src/adapters/nextjs.rs`**

```rust
pub struct NextJsAdapter;

impl FrameworkAdapter for NextJsAdapter {
    fn name(&self) -> &'static str { "Next.js" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "tsx", "js"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // Next.js App Router:  app/api/users/route.ts  →  GET/POST /api/users
        // Next.js Pages Router: pages/api/users.ts     →  ANY /api/users

        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/app/api/") {
            // App Router
            let rel = &path_str[pos + 4..]; // strip up to /api/
            rel.replace("/route.ts", "").replace("/route.js", "")
               .replace("[", ":").replace("]", "") // [id] → :id
        } else if let Some(pos) = path_str.find("/pages/api/") {
            // Pages Router
            let rel = &path_str[pos + 11..];
            format!("/api/{}", rel.replace(".ts", "").replace(".js", "")
                                  .replace("[", ":").replace("]", ""))
        } else {
            return Vec::new(); // not an API route
        };

        // Detect exported HTTP methods: export async function GET(req) { ... }
        let method_re = regex::Regex::new(
            r"export\s+(?:async\s+)?function\s+(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)"
        ).unwrap();

        let mut routes = Vec::new();
        for cap in method_re.captures_iter(source) {
            routes.push(RouteBinding {
                method: http_method_from_str(&cap[1].to_lowercase()),
                path_pattern: route_path.clone(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: cap[1].to_string(),
                injection_points: Vec::new(),
                framework: Framework::NextJs,
            });
        }

        // App Router without explicit named exports still has a default POST/GET
        if routes.is_empty() && path_str.contains("/app/api/") {
            routes.push(RouteBinding {
                method: HttpMethod::Post,
                path_pattern: route_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "handler".to_string(),
                injection_points: Vec::new(),
                framework: Framework::NextJs,
            });
        }
        routes
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // Next.js App Router: request.json(), searchParams.get('x'), params.id
        // Next.js Pages Router: req.body.x, req.query.x
        let mut points = extract_by_patterns(body, &[
            (r"req\.body\.(\w+)",              ParameterLocation::Body),
            (r"req\.query\.(\w+)",             ParameterLocation::Query),
            (r"searchParams\.get\(['\"](\w+)['\"]", ParameterLocation::Query),
            (r"params\.(\w+)",                 ParameterLocation::PathParam),
            (r"request\.json\(\)",             ParameterLocation::Body),
        ]);
        // If request.json() is found, add a generic body point
        if body.contains("request.json()") || body.contains("await req.json()") {
            points.push(InjectionPoint {
                location: ParameterLocation::Body,
                name: "_body".to_string(),
                taint_origin: Some("user_input".to_string()),
            });
        }
        points
    }

    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        Some(vec!["npx".to_string(), "next".to_string(), "dev".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Remix — Loader/Action Pattern

Remix uses `loader` (GET) and `action` (POST/PUT/DELETE) exports.

**New file: `frensense-runtime/src/adapters/remix.rs`**

```rust
pub struct RemixAdapter;

impl FrameworkAdapter for RemixAdapter {
    fn name(&self) -> &'static str { "Remix" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "tsx", "js"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // Remix: app/routes/api.users.ts → /api/users
        // File convention: dots become slashes, $ becomes :
        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/routes/") {
            let rel = &path_str[pos + 8..];
            let without_ext = rel.replace(".ts", "").replace(".tsx", "").replace(".js", "");
            format!("/{}", without_ext
                .replace('.', "/")          // api.users → api/users
                .replace('$', ":"))         // $id → :id
        } else {
            return Vec::new();
        };

        let has_loader = source.contains("export async function loader")
            || source.contains("export function loader");
        let has_action = source.contains("export async function action")
            || source.contains("export function action");

        let mut routes = Vec::new();
        if has_loader {
            routes.push(RouteBinding {
                method: HttpMethod::Get,
                path_pattern: route_path.clone(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "loader".to_string(),
                injection_points: Vec::new(),
                framework: Framework::Remix,
            });
        }
        if has_action {
            routes.push(RouteBinding {
                method: HttpMethod::Post,
                path_pattern: route_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: "action".to_string(),
                injection_points: Vec::new(),
                framework: Framework::Remix,
            });
        }
        routes
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // Remix: const url = new URL(request.url); url.searchParams.get('x')
        //        const { x } = await request.json()
        //        const formData = await request.formData(); formData.get('x')
        let mut points = extract_by_patterns(body, &[
            (r"searchParams\.get\(['\"](\w+)['\"]", ParameterLocation::Query),
            (r"params\.(\w+)",                      ParameterLocation::PathParam),
            (r#"formData\.get\(['"]\w+['"]\)"#,     ParameterLocation::FormData),
        ]);
        if body.contains("request.json()") { points.push(body_point()); }
        if body.contains("request.formData()") { points.push(form_point()); }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npx".to_string(), "remix".to_string(), "dev".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::SessionCookie("__session".to_string()) }
}
```

### NestJS — Decorator-Based Routing

NestJS uses `@Get()`, `@Post()` decorators and `@Param()`, `@Body()`, `@Query()` for injection.

**New file: `frensense-runtime/src/adapters/nestjs.rs`**

```rust
pub struct NestJsAdapter;

impl FrameworkAdapter for NestJsAdapter {
    fn name(&self) -> &'static str { "NestJS" }
    fn extensions(&self) -> &'static [&'static str] { &["ts"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // @Controller('users')  +  @Get(':id')  →  GET /users/:id
        let controller_re = regex::Regex::new(
            r#"@Controller\s*\(\s*['"`]?([^'"`\)]*?)['"`]?\s*\)"#
        ).unwrap();
        let method_re = regex::Regex::new(
            r#"@(Get|Post|Put|Delete|Patch)\s*\(\s*['"`]?([^'"`\)]*?)['"`]?\s*\)"#
        ).unwrap();

        let base_path = controller_re
            .captures(source)
            .map(|c| c[1].trim().to_string())
            .unwrap_or_default();

        method_re.captures_iter(source).map(|cap| {
            let method     = http_method_from_str(&cap[1].to_lowercase());
            let sub_path   = cap[2].trim().to_string();
            let full_path  = format!("/{}/{}", base_path, sub_path)
                .replace("//", "/")
                .replace(':',  ":"); // NestJS already uses :param syntax
            RouteBinding {
                method,
                path_pattern: full_path,
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: String::new(),
                injection_points: Vec::new(),
                framework: Framework::NestJs,
            }
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // NestJS uses @Param('id'), @Body(), @Query('q'), @Headers('x')
        // These appear in method signatures, not function body
        extract_by_patterns(body, &[
            (r"@Param\(['\"](\w+)['\"]",  ParameterLocation::PathParam),
            (r"@Query\(['\"](\w+)['\"]",  ParameterLocation::Query),
            (r"@Headers\(['\"](\w+)['\"]", ParameterLocation::Header),
            (r"@Body\(\)",                 ParameterLocation::Body),
            // Also handle destructured body in handler body
            (r"body\.(\w+)",               ParameterLocation::Body),
        ])
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "start:dev".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Hono — Lightweight TS Framework

```rust
pub struct HonoAdapter;
impl FrameworkAdapter for HonoAdapter {
    fn name(&self) -> &'static str { "Hono" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // app.get('/path', (c) => { ... })
        let route_re = regex::Regex::new(
            r#"(?:app|router)\.(get|post|put|delete|patch)\s*\(\s*['"`]([^'"`]+)['"`]"#
        ).unwrap();
        // same as express but with Hono framework tag
        route_re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[1]),
            path_pattern: cap[2].to_string(),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::Hono,
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // Hono: c.req.param('id'), c.req.query('q'), await c.req.json()
        //       c.req.header('X-Custom')
        let mut points = extract_by_patterns(body, &[
            (r"c\.req\.param\(['\"](\w+)['\"]",  ParameterLocation::PathParam),
            (r"c\.req\.query\(['\"](\w+)['\"]",  ParameterLocation::Query),
            (r"c\.req\.header\(['\"]([^'\"]+)['\"]", ParameterLocation::Header),
        ]);
        if body.contains("c.req.json()") || body.contains("c.req.parseBody()") {
            points.push(body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "dev".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### SvelteKit — Endpoint Files

```rust
pub struct SvelteKitAdapter;
impl FrameworkAdapter for SvelteKitAdapter {
    fn name(&self) -> &'static str { "SvelteKit" }
    fn extensions(&self) -> &'static [&'static str] { &["ts", "js"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // src/routes/api/users/+server.ts  →  /api/users
        // Exported: export const GET, export const POST
        let path_str = file_path.to_string_lossy();
        let route_path = if let Some(pos) = path_str.find("/routes/") {
            let rel = &path_str[pos + 8..];
            rel.replace("+server.ts", "").replace("+server.js", "")
               .replace("[", ":").replace("]", "")
               .trim_end_matches('/').to_string()
        } else {
            return Vec::new();
        };

        let method_re = regex::Regex::new(
            r"export\s+(?:const|async function)\s+(GET|POST|PUT|DELETE|PATCH)"
        ).unwrap();

        method_re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[1].to_lowercase()),
            path_pattern: format!("/{}", route_path),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: cap[1].to_string(),
            injection_points: Vec::new(),
            framework: Framework::SvelteKit,
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // SvelteKit: event.url.searchParams.get('q'), event.params.id, await event.request.json()
        let mut points = extract_by_patterns(body, &[
            (r"(?:event\.)?url\.searchParams\.get\(['\"](\w+)['\"]", ParameterLocation::Query),
            (r"(?:event\.)?params\.(\w+)", ParameterLocation::PathParam),
        ]);
        if body.contains("event.request.json()") || body.contains("request.json()") {
            points.push(body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["npm".to_string(), "run".to_string(), "dev".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::SessionCookie("session".to_string()) }
}
```

---

## Rust Framework Adapters

### Axum — Type-Level Routing

Axum routes via `.route("/path", get(handler))` and injects via extractors in
handler function signatures: `Path<(String,)>`, `Query<Params>`, `Json<Body>`.

**New file: `frensense-runtime/src/adapters/axum.rs`**

```rust
pub struct AxumAdapter;
impl FrameworkAdapter for AxumAdapter {
    fn name(&self) -> &'static str { "Axum" }
    fn extensions(&self) -> &'static [&'static str] { &["rs"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // .route("/path", get(handler).post(handler))
        let route_re = regex::Regex::new(
            r#"\.route\s*\(\s*"([^"]+)"\s*,\s*([\w\s\.\(\),]+)\)"#
        ).unwrap();
        let method_re = regex::Regex::new(
            r"\b(get|post|put|delete|patch)\s*\("
        ).unwrap();

        route_re.captures_iter(source).flat_map(|cap| {
            let path = cap[1].replace(":param", ":param"); // Axum uses :param
            let methods_str = cap[2].to_string();
            method_re.captures_iter(&methods_str).map(move |m| {
                RouteBinding {
                    method: http_method_from_str(&m[1]),
                    path_pattern: path.clone(),
                    handler_file: file_path.to_string_lossy().to_string(),
                    handler_function: String::new(),
                    injection_points: Vec::new(),
                    framework: Framework::Axum,
                }
            }).collect::<Vec<_>>()
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // Axum extractors appear in function signatures (already in original_content):
        // Path<(String,)>  Query<Params>  Json<Body>  HeaderMap
        // Also look for variable usage patterns: path.0, query.name, payload.field
        let mut points = Vec::new();
        if body.contains("Json<") || body.contains("payload.") || body.contains("body.") {
            points.push(body_point());
        }
        if body.contains("Query<") || body.contains("query.") {
            points.push(InjectionPoint {
                location: ParameterLocation::Query,
                name: "_query".to_string(),
                taint_origin: Some("user_input".to_string()),
            });
        }
        if body.contains("Path<") || body.contains("path.") {
            points.push(InjectionPoint {
                location: ParameterLocation::PathParam,
                name: "_path".to_string(),
                taint_origin: Some("user_input".to_string()),
            });
        }
        // For Axum, field-level extraction requires resolving the extractor type —
        // fall back to injecting into all detected points with category-appropriate payloads
        points
    }

    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        Some(vec!["cargo".to_string(), "run".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Actix-web

```rust
pub struct ActixAdapter;
impl FrameworkAdapter for ActixAdapter {
    fn name(&self) -> &'static str { "Actix-web" }
    fn extensions(&self) -> &'static [&'static str] { &["rs"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // .route("/path", web::get().to(handler))
        // #[get("/path")]  above an async fn
        let macro_re = regex::Regex::new(
            r#"#\[(get|post|put|delete|patch)\s*\(\s*"([^"]+)"\s*\)\]"#
        ).unwrap();
        let builder_re = regex::Regex::new(
            r#"\.route\s*\(\s*"([^"]+)"\s*,\s*web::(get|post|put|delete|patch)"#
        ).unwrap();

        let mut routes: Vec<RouteBinding> = macro_re.captures_iter(source).map(|cap| {
            RouteBinding {
                method: http_method_from_str(&cap[1]),
                path_pattern: cap[2].to_string(),
                handler_file: file_path.to_string_lossy().to_string(),
                handler_function: String::new(),
                injection_points: Vec::new(),
                framework: Framework::Actix,
            }
        }).collect();

        routes.extend(builder_re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[2]),
            path_pattern: cap[1].to_string(),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::Actix,
        }));
        routes
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // web::Path<(String,)>, web::Query<T>, web::Json<T>, web::Form<T>
        let mut points = Vec::new();
        if body.contains("web::Json") || body.contains(".into_inner()") { points.push(body_point()); }
        if body.contains("web::Query") { points.push(query_point()); }
        if body.contains("web::Path")  { points.push(path_point()); }
        if body.contains("web::Form")  { points.push(form_point()); }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["cargo".to_string(), "run".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

---

## Go Framework Adapters

### Go net/http (Standard Library)

```rust
pub struct NetHttpAdapter;
impl FrameworkAdapter for NetHttpAdapter {
    fn name(&self) -> &'static str { "Go/net-http" }
    fn extensions(&self) -> &'static [&'static str] { &["go"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // http.HandleFunc("/path", handler)
        // mux.HandleFunc("/path", handler)
        // http.Handle("/path", handler)
        let re = regex::Regex::new(
            r#"(?:http|mux|r|router)\.Handle(?:Func)?\s*\(\s*"([^"]+)""#
        ).unwrap();
        re.captures_iter(source).map(|cap| RouteBinding {
            method: HttpMethod::All, // net/http doesn't constrain by method at registration
            path_pattern: cap[1].to_string(),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::GoNetHttp,
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // r.URL.Query().Get("param") — URL query param
        // r.FormValue("name")        — form body
        // r.PostFormValue("name")    — POST form body specifically
        // r.PathValue("name")        — Go 1.22+ path params (ServeMux)
        let mut points = extract_by_patterns(body, &[
            (r#"\.Query\(\)\.Get\(['"]([\w-]+)['"]\)"#,  ParameterLocation::Query),
            (r#"\.FormValue\(['"]([\w-]+)['"]\)"#,        ParameterLocation::FormData),
            (r#"\.PostFormValue\(['"]([\w-]+)['"]\)"#,    ParameterLocation::Body),
            (r#"\.PathValue\(['"]([\w-]+)['"]\)"#,        ParameterLocation::PathParam),
            (r#"r\.Header\.Get\(['"]([\w-]+)['"]\)"#,    ParameterLocation::Header),
        ]);
        // Also detect json.Decode(&struct) — body injection
        if body.contains("json.NewDecoder") || body.contains("json.Decode") || body.contains("io.ReadAll(r.Body)") {
            points.push(body_point());
        }
        points
    }

    fn startup_command(&self, root: &Path) -> Option<Vec<String>> {
        // Find the main package file
        Some(vec!["go".to_string(), "run".to_string(), ".".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Go Gin

```rust
pub struct GinAdapter;
impl FrameworkAdapter for GinAdapter {
    fn name(&self) -> &'static str { "Go/Gin" }
    fn extensions(&self) -> &'static [&'static str] { &["go"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // r.GET("/path", handler)  r.POST("/path", handler)
        // authorized.GET("/orders", listOrders)
        let re = regex::Regex::new(
            r#"(?:\w+)\.(GET|POST|PUT|DELETE|PATCH|Any)\s*\(\s*"([^"]+)""#
        ).unwrap();
        re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[1].to_lowercase()),
            path_pattern: cap[2].replace(":", ":"), // Gin already uses :param
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::GoGin,
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // c.Query("name"), c.Param("id"), c.PostForm("field"), c.GetHeader("X-H")
        // c.ShouldBindJSON(&struct), c.ShouldBind(&struct)
        let mut points = extract_by_patterns(body, &[
            (r#"c\.Query\(['"]([\w-]+)['"]\)"#,      ParameterLocation::Query),
            (r#"c\.DefaultQuery\(['"]([\w-]+)['"]\)"#, ParameterLocation::Query),
            (r#"c\.Param\(['"]([\w-]+)['"]\)"#,       ParameterLocation::PathParam),
            (r#"c\.PostForm\(['"]([\w-]+)['"]\)"#,    ParameterLocation::Body),
            (r#"c\.GetHeader\(['"]([\w-]+)['"]\)"#,   ParameterLocation::Header),
            (r#"c\.Cookie\(['"]([\w-]+)['"]\)"#,      ParameterLocation::Cookie),
        ]);
        if body.contains("ShouldBindJSON") || body.contains("BindJSON") || body.contains("ShouldBind") {
            points.push(body_point());
        }
        points
    }

    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["go".to_string(), "run".to_string(), ".".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

### Go Echo

```rust
pub struct EchoAdapter;
impl FrameworkAdapter for EchoAdapter {
    fn name(&self) -> &'static str { "Go/Echo" }
    fn extensions(&self) -> &'static [&'static str] { &["go"] }

    fn extract_routes(&self, file_path: &Path, source: &str) -> Vec<RouteBinding> {
        // e.GET("/path", handler)
        let re = regex::Regex::new(
            r#"(?:e|g|echo)\.(GET|POST|PUT|DELETE|PATCH)\s*\(\s*"([^"]+)""#
        ).unwrap();
        re.captures_iter(source).map(|cap| RouteBinding {
            method: http_method_from_str(&cap[1].to_lowercase()),
            path_pattern: cap[2].replace(":", ":"),
            handler_file: file_path.to_string_lossy().to_string(),
            handler_function: String::new(),
            injection_points: Vec::new(),
            framework: Framework::GoEcho,
        }).collect()
    }

    fn extract_injection_points(&self, body: &str) -> Vec<InjectionPoint> {
        // c.QueryParam("name"), c.Param("id"), c.FormValue("f"), c.Request().Header.Get("H")
        let mut points = extract_by_patterns(body, &[
            (r#"c\.QueryParam\(['"]([\w-]+)['"]\)"#, ParameterLocation::Query),
            (r#"c\.Param\(['"]([\w-]+)['"]\)"#,      ParameterLocation::PathParam),
            (r#"c\.FormValue\(['"]([\w-]+)['"]\)"#,  ParameterLocation::FormData),
        ]);
        if body.contains("c.Bind(") || body.contains("c.Request().Body") {
            points.push(body_point());
        }
        points
    }
    fn startup_command(&self, _root: &Path) -> Option<Vec<String>> {
        Some(vec!["go".to_string(), "run".to_string(), ".".to_string()])
    }
    fn auth_convention(&self) -> AuthConvention { AuthConvention::BearerToken }
}
```

---

## Python and PHP — Future Adapters (Stub Definitions)

Python and PHP do not yet exist in the Frensense corpus (`ls corpus/targets/`
confirms no `py_*` or `php_*` patterns). When corpus patterns are added, the
adapters follow the same trait. Stub signatures for completeness:

### Python adapters (when corpus added)

```rust
// Flask:   @app.route('/path', methods=['GET','POST'])
//          request.args.get('q'), request.json['field'], request.form['f']
pub struct FlaskAdapter;

// FastAPI:  @app.get('/path')  async def handler(q: str = Query(...)):
//           JSON body via Pydantic model param
pub struct FastApiAdapter;

// Django:  path('users/', views.user_list)  in urls.py
//          request.GET['q'], request.POST['field'], request.data (DRF)
pub struct DjangoAdapter;
```

### PHP adapters (when corpus added)

```rust
// Laravel: Route::get('/path', [Controller::class, 'method'])
//          $request->input('field'), $request->query('q'), $request->route('id')
pub struct LaravelAdapter;

// Symfony: #[Route('/path', methods: ['POST'])]
//          $request->request->get('field'), $request->query->get('q')
pub struct SymfonyAdapter;
```

---

## Non-HTTP Bug Classes — Different Strategy

A significant portion of the corpus covers bugs that have **no HTTP interface**:

| Rule prefix | Examples | Nature |
|---|---|---|
| `rust_async_*` | blocking_io, mutex_sync, select_without_biased | Async runtime bugs |
| `tsx_useeffect_*` | missing_dependency, infinite_loop, cleanup_race | React browser runtime |
| `rust_race_*` | concurrent access | Race conditions |
| `ts_toctou_*` | time-of-check/time-of-use | Filesystem races |
| `rust_mem_*` | memory patterns | Memory safety |
| `rust_crypto_*` | weak algorithms | Output analysis |

These **cannot be probed via HTTP**. Three strategies apply:

### Strategy A — Process-Level Observation (Rust async, race conditions)

For `rust_async_blocking_io`, `rust_async_mutex_sync` etc., the bug manifests
as thread starvation or deadlock. The runtime probe is a **concurrent request
flood** rather than a payload injection:

```rust
pub struct ConcurrentStressProber {
    pub concurrency: usize,     // 50 simultaneous requests
    pub duration_ms: u64,       // run for 2000ms
    pub expect_degradation: bool,
}

/// For async blocking bugs: send 50 concurrent requests to the same endpoint.
/// If the async runtime blocks on I/O, response times will degrade exponentially.
/// Oracle: p99 latency > 10× p50 latency → confirmed blocking.
pub async fn probe_concurrency_degradation(
    client: &reqwest::Client,
    url: &str,
    prober: &ConcurrentStressProber,
) -> ConcurrencyVerdict { ... }
```

### Strategy B — Browser-Based Observation (React hook bugs)

For `tsx_useeffect_infinite_loop`, `tsx_useeffect_missing_dependency` etc.,
the bug is visible only in a browser runtime. This requires integration with
a headless browser. Mark these as requiring `--browser-probe` mode:

```rust
pub enum ProbeStrategy {
    Http(ProbeTemplate),
    ConcurrentStress(ConcurrentStressProber),
    BrowserScript(BrowserProbeScript),  // requires --browser-probe (chromium)
    CannotProbeAtRuntime { reason: &'static str },
}

fn strategy_for_rule_id(rule_id: &str) -> ProbeStrategy {
    let category = category_from_rule_id(rule_id);
    match category {
        "cmdi" | "sqli" | "ssrf" | "redirect" | "path_traversal"
        | "xss" | "idor" | "auth" | "cors" | "csrf" => {
            ProbeStrategy::Http(template_for_category(category).unwrap())
        }
        _ if rule_id.starts_with("rust_async") || rule_id.starts_with("rust_race") => {
            ProbeStrategy::ConcurrentStress(ConcurrentStressProber {
                concurrency: 50,
                duration_ms: 3000,
                expect_degradation: true,
            })
        }
        _ if rule_id.starts_with("tsx_use") => {
            ProbeStrategy::BrowserScript(BrowserProbeScript {
                action: "render_and_observe_console_errors",
            })
        }
        _ => ProbeStrategy::CannotProbeAtRuntime {
            reason: "No HTTP surface; static analysis only."
        },
    }
}
```

### Strategy C — Output Analysis (crypto, serialization)

For `rust_crypto_md5_password`, the bug is observable in the stored output.
These need **known-plaintext probing**: send a known value, read it back,
check if the stored hash matches MD5/SHA1:

```rust
ProbeStrategy::OutputAnalysis {
    send_value: "frensense_known_plaintext",
    read_back_endpoint: None, // may not be directly observable
    oracle: OracleKind::CanaryInBody {
        canary: md5("frensense_known_plaintext"), // known MD5 output
    },
}
```

---

## The Complete Multi-Language Runtime Flow

```
1. Read static report  →  Advisory { rule_id: "ts_cmdi_exec_direct",
                                     file_path: "src/routes/run.ts",
                                     enclosing_symbol: "runCommand",
                                     original_content: "const cmd = req.body.cmd; exec(cmd);" }

2. Detect language     →  "ts" prefix → TypeScript project
                          Check package.json → has "express" dep → ExpressAdapter

3. Extract routes      →  ExpressAdapter::extract_routes("src/routes/run.ts")
                          Finds: router.post('/api/run', runCommand)
                          → RouteBinding { POST /api/run }

4. Extract inj. points →  ExpressAdapter::extract_injection_points(original_content)
                          Finds: req.body.cmd → InjectionPoint { Body, "cmd" }

5. Select strategy     →  category "cmdi" → ProbeStrategy::Http
                          template = cmdi::template()

6. Capture baseline    →  POST /api/run {"cmd": "hello"}
                          → BehavioralTrace { 200, 45ms, body_hash: 0xabc }

7. Run probes          →  Probe 1: POST /api/run {"cmd": "; echo FRENSENSE_A1B2 #"}
                          → BehavioralTrace { 200, 47ms, body: "...FRENSENSE_A1B2..." }

8. Oracle fires        →  OracleKind::CanaryInBody → CONFIRMED (confidence: 0.97)

9. Emit advisory       →  RuntimeAdvisory {
                            status: Confirmed,
                            evidence: "Canary FRENSENSE_A1B2 found in response body",
                            probe: "; echo FRENSENSE_A1B2 #",
                            combined_confidence: sqrt(0.85 × 0.97) = 0.91
                          }
```

---

## Framework-to-Injection-Syntax Reference Table

| Framework | Body | Query | Path Param | Header |
|---|---|---|---|---|
| Express/TS | `req.body.X` | `req.query.X` | `req.params.X` | `req.headers.X` |
| Fastify/TS | `request.body.X` | `request.query.X` | `request.params.X` | `request.headers.X` |
| Next.js App | `await request.json()` | `searchParams.get('X')` | `params.X` | `request.headers.get('X')` |
| Next.js Pages | `req.body.X` | `req.query.X` | `req.query.X` | `req.headers.X` |
| Remix | `await request.json()` | `url.searchParams.get('X')` | `params.X` | `request.headers.get('X')` |
| NestJS | `@Body()` | `@Query('X')` | `@Param('X')` | `@Headers('X')` |
| Hono | `await c.req.json()` | `c.req.query('X')` | `c.req.param('X')` | `c.req.header('X')` |
| SvelteKit | `await event.request.json()` | `url.searchParams.get('X')` | `event.params.X` | `event.request.headers.get('X')` |
| Actix-web/Rust | `web::Json<T>` | `web::Query<T>` | `web::Path<T>` | `HeaderMap` |
| Axum/Rust | `Json<T>` | `Query<T>` | `Path<T>` | `HeaderMap` |
| Rocket/Rust | `Json<T>` | `&str` query guard | `<id>` route param | `Header<T>` |
| Go net/http | `json.Decode(r.Body)` | `r.URL.Query().Get("X")` | `r.PathValue("X")` | `r.Header.Get("X")` |
| Go Gin | `c.ShouldBindJSON()` | `c.Query("X")` | `c.Param("X")` | `c.GetHeader("X")` |
| Go Echo | `c.Bind(&T)` | `c.QueryParam("X")` | `c.Param("X")` | `c.Request().Header.Get("X")` |

---

## Request Building Per Framework

When sending a probe, the adapter dictates how to format the request:

```rust
pub fn build_request(
    client: &reqwest::Client,
    target: &ProbeTarget,
    payload: &str,
    point: &InjectionPoint,
    framework: Framework,
) -> reqwest::RequestBuilder {
    let url = format!("{}{}", target.base_url, target.route.path_pattern);

    match point.location {
        ParameterLocation::Body => {
            // All frameworks: JSON body for API routes
            let body = serde_json::json!({ &point.name: payload });
            client.request(target.route.method.into(), &url).json(&body)
        }
        ParameterLocation::Query => {
            client.request(target.route.method.into(), &url)
                .query(&[(&point.name, payload)])
        }
        ParameterLocation::PathParam => {
            // Substitute :param in the route pattern
            let url_with_param = url.replace(&format!(":{}", point.name), payload);
            client.request(target.route.method.into(), &url_with_param)
        }
        ParameterLocation::Header => {
            client.request(target.route.method.into(), &url)
                .header(&point.name, payload)
        }
        ParameterLocation::Cookie => {
            client.request(target.route.method.into(), &url)
                .header("Cookie", format!("{}={}", point.name, payload))
        }
        ParameterLocation::FormData => {
            // multipart/form-data or application/x-www-form-urlencoded
            let params = [(&point.name, payload)];
            client.request(target.route.method.into(), &url).form(&params)
        }
    }
}
```

---

## What Cannot Be Probed at Runtime (and Why)

| Rule category | Why not probeable | What to do instead |
|---|---|---|
| `rust_async_blocking_io` | No user-controlled HTTP parameter; bug is in runtime scheduling | Concurrent flood test (Strategy A) |
| `tsx_useeffect_*` | React is client-side; no server HTTP endpoint | Headless browser (Strategy B, future) |
| `rust_mem_*`, `rust_transmute_*` | Memory safety; no observable HTTP surface | Static analysis only |
| `rust_clone_in_loop` | Performance bug; observable only via profiling | Concurrent stress + latency analysis |
| `ts_hardcoded_*` | Secret in source code; not injectable | Static analysis only; confirm by grepping env |
| `ts_package_*` | Supply chain; not runtime | Audit lockfile |
| `rust_edition2024_*` | Compiler correctness; not runtime | Static only |

These should be labelled `CannotProbeAtRuntime` in the output rather than
silently skipped, so the analyst knows the static finding is all they have.

---

## Build Order for Multi-Language Support

1. `adapters/mod.rs` — `FrameworkAdapter` trait, `AuthConvention`, `Framework` enum
2. `adapters/detector.rs` — `detect_framework()` reading package.json / Cargo.toml / go.mod
3. `adapters/express.rs` — Reference implementation; all other TS adapters extend its patterns
4. `adapters/nextjs.rs` + `adapters/remix.rs` + `adapters/nestjs.rs` + `adapters/hono.rs` + `adapters/sveltekit.rs`
5. `adapters/axum.rs` + `adapters/actix.rs` — Rust adapters
6. `adapters/nethttp.rs` + `adapters/gin.rs` + `adapters/echo.rs` — Go adapters
7. Update `route_extractor.rs` to delegate to `FrameworkAdapter` instead of hardcoding Express patterns
8. Update `scheduler.rs` to call `strategy_for_rule_id()` before building probe list
9. Python and PHP adapters added when corpus patterns exist for those languages
