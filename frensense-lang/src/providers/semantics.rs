// SPDX-License-Identifier: MIT

/// Universal semantic tokens mapped to framework-specific API substrings.
/// This allows structural matching to generalize across frameworks
/// by boosting similarity when two different APIs map to the same token.

pub const JS_SEMANTIC_MAPPINGS: &[(&str, &[&str])] = &[
    (
        "SINK_DB_READ",
        &[
            "prisma.",
            "db.prepare",
            ".find",
            ".findOne",
            ".findAll",
            ".findBy",
            ".getBy",
            ".count",
            "db.collection",
        ],
    ),
    (
        "SINK_DB_WRITE",
        &[
            ".save",
            ".update",
            ".updateOne",
            ".updateMany",
            ".insert",
            ".insertOne",
            ".insertMany",
            ".destroy",
            ".delete",
            ".deleteOne",
            ".deleteMany",
        ],
    ),
    (
        "SINK_HTTP_OUTBOUND",
        &[
            "fetch",
            "axios",
            "got",
            "needle.",
            "request.",
            "https.request",
            "http.request",
        ],
    ),
    ("SINK_EXEC", &["exec", "spawn", "child_process", "execSync"]),
    ("SINK_NOSQL_QUERY", &["$where"]),
    ("OP_JWT_VERIFY", &["jwt.verify", "jose.jwtVerify"]),
    (
        "SOURCE_HTTP_REQUEST",
        &[
            "req.query",
            "req.body",
            "req.params",
            "c.req.",
            "searchParams.",
        ],
    ),
    // --- Security, Cookies, and Sessions ---
    (
        "OP_SET_COOKIE",
        &["res.cookie", ".cookie(", ".setHeader", "set-cookie"],
    ),
    ("OP_SET_HEADER", &["res.setHeader", "res.header", "res.set"]),
    ("OP_APP_DISABLE", &["app.disable", ".disable("]),
    ("OP_SESSION_CREATE", &["express-session", "session("]),
    (
        "OP_SESSION_REGENERATE",
        &["req.session.regenerate", ".regenerate("],
    ),
    ("OP_CSRF_PROTECT", &["csurf", "csrf"]),
    ("OP_MARKDOWN_PARSE", &["marked(", "marked.parse"]),
    ("OP_HASH_COMPARE", &["bcrypt.compare", "timingSafeEqual"]),
    (
        "OP_HASH_CREATE",
        &["bcrypt.hash", "createHash", "createHmac"],
    ),
    ("OP_HPP_PROTECT", &["hpp()"]),
    ("OP_HELMET_PROTECT", &["helmet()"]),
];
