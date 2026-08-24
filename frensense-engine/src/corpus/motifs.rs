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
            // Go
            "c.Param",
            "c.Query",
            "c.PostForm",
            "r.URL.Query",
            "r.FormValue",
            // Rust
            "Query",
            "Path",
            "Form",
            "Json",
            // Java/C#
            "@RequestParam",
            "@PathVariable",
            "@RequestBody",
            "[FromQuery]",
            "[FromBody]",
            "[FromRoute]",
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
            // Go
            "exec.Command",
            "exec.CommandContext",
            "Run",
            "Output",
            "CombinedOutput",
            // Rust
            "Command::new",
            "spawn",
            // C#
            "Process.Start",
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
            // Go
            "QueryRow",
            "Query",
            "Exec",
            "db.First",
            "db.Find",
            "db.Create",
            "db.Save",
            "db.Updates",
            // Rust
            "fetch_one",
            "fetch_optional",
            "fetch_all",
            "get_result",
            "load",
            "execute",
            "insert_into",
            // Java/C#
            "entityManager.persist",
            "entityManager.merge",
            "entityManager.createQuery",
            "DbContext.SaveChanges",
            "DbSet.Add",
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
            "reqwest::Client",
            "http.get",
            "https.get",
            "attohttpc",
            "isahc",
            // Go
            "http.Get",
            "http.Post",
            "http.DefaultClient.Do",
            "client.Do",
            // Java/C#
            "HttpClient.send",
            "RestTemplate.getForObject",
            "RestTemplate.postForObject",
            "HttpClient.GetAsync",
            "HttpClient.PostAsync",
            "WebClient.DownloadString",
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
            // Go
            "os.Open",
            "os.ReadFile",
            "ioutil.ReadFile",
            // Java/C#
            "Files.readAllBytes",
            "Files.readString",
            "File.ReadAllText",
            "File.ReadAllBytes",
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
            // Go
            "os.Create",
            "os.WriteFile",
            "ioutil.WriteFile",
            // Java/C#
            "Files.write",
            "Files.writeString",
            "File.WriteAllText",
            "File.WriteAllBytes",
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
            "yaml.parse",
            "js-yaml.load",
            "js-yaml.safeLoad",
            "pickle.loads",
            "bincode::deserialize",
            "msgpack.decode",
            "msgpack.unpack",
            "php.unserialize",
            // Go
            "json.Unmarshal",
            "xml.Unmarshal",
            "gob.NewDecoder",
            "yaml.Unmarshal",
            // Java/C#
            "ObjectInputStream.readObject",
            "JsonConvert.DeserializeObject",
            "XmlSerializer.Deserialize",
            "BinaryFormatter.Deserialize",
            "JsonSerializer.Deserialize",
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
            // Go
            "http.Error",
            "w.Write",
            "c.JSON",
            "c.String",
            "c.HTML",
            // Rust
            "HttpResponse::Ok",
            "HttpResponse::BadRequest",
            "HttpResponse::InternalServerError",
            "Json",
            // Java/C#
            "ResponseEntity.ok",
            "ResponseEntity.badRequest",
            "Results.Ok",
            "Results.BadRequest",
            "return Ok",
            "return BadRequest",
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
        members: &[
            "swig.setDefaults",
            "swig.init",
            "nunjucks.configure",
            "handlebars.registerHelper",
            "setDefaults",
            "configure",
        ],
    },
    Motif {
        name: "TemplateRenderSink",
        members: &[
            "ejs.render",
            "ejs.renderFile",
            "pug.compile",
            "pug.render",
            "pug.renderFile",
            "handlebars.compile",
            "handlebars.render",
            "handlebars.renderView",
            "nunjucks.render",
            "nunjucks.renderString",
            "nunjucks.renderFile",
            "marko.render",
            "marko.renderToString",
            "eta.render",
            "eta.renderToString",
            "artTemplate.render",
            "swig.render",
            "swig.renderFile",
            "liquid.render",
            "liquid.renderFile",
            "mustache.render",
            "hogan.render",
            "dust.render",
            "jade.render",
            "react-dom/server.renderToString",
            "react-dom/server.renderToStaticMarkup",
            "vue-server-renderer.renderToString",
            // Python
            "Jinja2.render",
            "Jinja2.render_template",
            "Template.render",
            "render_template",
            "render_template_string",
            // Go
            "html/template.Execute",
            "text/template.Execute",
        ],
    },
    Motif {
        name: "PrototypePollutionSink",
        members: &[
            "Object.assign",
            "_.merge",
            "lodash.merge",
            "_.defaultsDeep",
            "_.set",
            "$.extend",
            "jQuery.extend",
            "angular.merge",
            "angular.extend",
            "setPrototypeOf",
        ],
    },
    Motif {
        name: "XmlParserSink",
        members: &[
            "DOMParser",
            "libxml2",
            "SAXParser",
            "XMLReader",
            "xml.etree.ElementTree",
            "lxml.etree",
            "DocumentBuilder",
            "DocumentBuilderFactory",
            "SAXBuilder",
            "SAXReader",
            "XmlDocument",
            "XDocument",
            "XmlTextReader",
            "XmlReader",
            "XmlSerializer",
            "SimpleXML",
            "simplexml_load_string",
            "DOMDocument",
            "DOMDocument::load",
            "DOMDocument::loadXML",
        ],
    },
    Motif {
        name: "JwtOperation",
        members: &[
            "jwt.verify",
            "jwt.decode",
            "jwt.sign",
            "jsonwebtoken.verify",
            "jsonwebtoken.decode",
            "jsonwebtoken.sign",
            "JWT.verify",
            "JWT.decode",
            "jose.JWT.verify",
            "jose.JWT.decode",
        ],
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
