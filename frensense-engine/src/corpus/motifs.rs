// SPDX-License-Identifier: MIT

//! Sink/source motif definitions.
//!
//! A motif is a named group of semantically equivalent API calls.
//! At fingerprint time, calls are hashed under the motif name instead of
//! (or in addition to) their literal name. This makes patterns trained on
//! `exec()` automatically match `spawn()`, `Command::new()` etc.

use rustc_hash::FxHashMap;
use std::sync::LazyLock;

/// A motif: a canonical name plus the set of concrete calls it covers.
#[derive(Debug, Clone)]
pub struct Motif {
    pub name: &'static str,
    pub members: &'static [&'static str],
}

/// All registered motifs.
pub static MOTIFS: &[Motif] = &[
    // --- Source motifs ---
    // These mark where user-controlled data enters a function.
    // Used by flow_fingerprint.rs to seed tainted variable tracking.
    Motif {
        name: "UserInputSource",
        members: &[
            // Express / Fastify / Hono request properties
            "req.body",
            "req.query",
            "req.params",
            "req.headers",
            "req.cookies",
            "request.body",
            "request.query",
            "request.params",
            // Common parameter names that carry user data
            "body",
            "query",
            "params",
            "userInput",
            "input",
            "data",
            // Form / URL data
            "formData",
            "searchParams",
            "URLSearchParams",
        ],
    },
    // --- Sink motifs ---
    Motif {
        name: "CommandExecutionSink",
        members: &[
            "exec",
            "execSync",
            "execFile",
            "execFileSync",
            "spawn",
            "spawnSync",
            "Command::new",
            "ProcessBuilder",
            "Runtime.exec",
            "Runtime.getRuntime",
            "popen",
            "system",
            "ShellExecute",
        ],
    },
    Motif {
        name: "DbQuerySink",
        members: &[
            "query",
            "execute",
            "raw",
            "raw_query",
            "sql_query",
            "Statement.execute",
            "execute_query",
            "prepare",
            "db.run",
            "knex.raw",
            "find",
            "findOne",
            "update",
            "updateOne",
            "aggregate",
            "insert",
            "insertOne",
        ],
    },
    Motif {
        name: "HttpOutboundSink",
        members: &[
            "fetch",
            "axios",
            "axios.get",
            "axios.post",
            "request",
            "got",
            "node-fetch",
            "reqwest::get",
            "reqwest::Client",
            "http.get",
            "https.get",
            "attohttpc",
            "isahc",
        ],
    },
    Motif {
        name: "FileReadSink",
        members: &[
            "readFile",
            "readFileSync",
            "createReadStream",
            "fs::read",
            "read_to_string",
            "File::open",
            "open",
            "fopen",
        ],
    },
    Motif {
        name: "FileWriteSink",
        members: &[
            "writeFile",
            "writeFileSync",
            "createWriteStream",
            "fs::write",
            "write_all",
            "File::create",
        ],
    },
    Motif {
        name: "DeserializeSink",
        members: &[
            "JSON.parse",
            "from_str",
            "loads",
            "deserialize",
            "serde_json::from_str",
            "yaml.load",
            "pickle.loads",
            "bincode::deserialize",
        ],
    },
    Motif {
        name: "EvalSink",
        members: &["eval", "new Function", "Function(", "vm.runInContext"],
    },
    Motif {
        name: "HttpResponseSink",
        members: &[
            "res.send",
            "res.json",
            "res.redirect",
            "res.render",
            "res.status",
            "res.end",
            "res.write",
            "response.send",
            "Response::json",
            "Response::error",
        ],
    },
    Motif {
        name: "CryptoWeakSink",
        members: &["md5", "sha1", "Md5", "Sha1", "createHash"],
    },
    Motif {
        name: "PasswordHashing",
        members: &[
            "bcrypt.hash",
            "bcrypt.hashSync",
            "bcrypt.compare",
            "bcrypt.compareSync",
            "argon2",
            "pbkdf2",
            "scrypt",
            "bcrypt.genSalt",
            "bcrypt.genSaltSync",
        ],
    },
    Motif {
        name: "SecurityMiddleware",
        members: &["helmet", "csurf", "csrf", "rateLimit"],
    },
    Motif {
        name: "SessionManagement",
        members: &["session", "express-session", "cookie-session", "cookie"],
    },
    Motif {
        name: "MarkupParser",
        members: &["marked", "showdown", "markdown-it", "DOMPurify.sanitize"],
    },
    Motif {
        name: "TemplateEngineConfig",
        members: &["swig.setDefaults", "swig.init", "nunjucks.configure", "handlebars.registerHelper", "setDefaults", "configure"],
    },
];

/// Build a lookup table from call name → motif canonical name.
fn build_motif_lookup() -> FxHashMap<String, &'static str> {
    let mut map = FxHashMap::default();
    for motif in MOTIFS {
        for &member in motif.members {
            map.insert(member.to_string(), motif.name);
            // Also insert the last segment (e.g. "new" from "Command::new")
            if let Some(pos) = member.rfind("::").or_else(|| member.rfind('.')) {
                let seg = &member[pos + 1..];
                map.entry(seg.to_string()).or_insert(motif.name);
            }
        }
    }
    map
}

/// Cached motif lookup table, built once at first access.
pub static MOTIF_LOOKUP: LazyLock<FxHashMap<String, &'static str>> =
    LazyLock::new(build_motif_lookup);
