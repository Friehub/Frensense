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
    ("SINK_EXEC", &["exec", "spawn", "child_process"]),
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
];
