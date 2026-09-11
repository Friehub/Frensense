// SPDX-License-Identifier: MIT

/// Universal semantic tokens mapped to framework-specific API prefixes.
/// This allows structural matching to generalize across frameworks
/// by boosting similarity when two different APIs map to the same token.

pub const JS_SEMANTIC_MAPPINGS: &[(&str, &[&str])] = &[
    (
        "SINK_DB_READ",
        &[
            "prisma.",
            "db.prepare",
            "mongoose.model.find",
            "mongoose.model.findOne",
            "collection.find",
            "allocationsCol.find",
            "benefitsCol.find",
            "contributionsCol.find",
            "researchCol.find",
            "memosCol.find",
            "userCol.find",
            "db.collection",
            "findBy",
            "getBy",
        ],
    ),
    (
        "SINK_DB_WRITE",
        &[
            "prisma.",
            "db.prepare",
            "mongoose.model.save",
            "mongoose.model.update",
            "collection.insert",
            "collection.update",
            "allocationsCol.update",
            "benefitsCol.update",
            "contributionsCol.update",
            "researchCol.update",
            "memosCol.update",
            "userCol.update",
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
    (
        "SINK_NOSQL_QUERY",
        &["$where", "allocationsCol.find", "db.collection"],
    ),
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
