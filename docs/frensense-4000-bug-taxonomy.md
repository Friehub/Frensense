# Frensense — 4,000 Bug Type Corpus Taxonomy
## Target Coverage Map for Generalising Across Any Codebase

**Notation:**
- ✅ = corpus pair exists  
- ⚠️ = partial / similar pair exists  
- ❌ = not yet in corpus  
- `[M]` = mutation count (variants to write per base type)  
- **Bold** = highest detection value, write first  

**How to reach 4,000:**  
~200 base patterns × 5 language targets × 4 mutations per pattern = 4,000 unique corpus pairs.  
The learning engine generalises within a language, so you need fewer pairs per mutation than a rule-based tool would need rules. Priority is breadth-first across categories, then depth on high-frequency bugs.

---

## Category 1 — SQL Injection (`CWE-89`)
**Target: 120 corpus pairs**

### 1.1 String Concatenation
| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_sqli_concat_direct` | `"SELECT " + userId` in prepare/query | direct, via ternary, after null-coalesce | TS | ⚠️ |
| `ts_sqli_template_literal` | `` `SELECT WHERE id=${userId}` `` | single field, multiple fields, subquery | TS | ✅ |
| `ts_sqli_intermediate_var` | Taint through intermediate variable | 1-hop, 2-hop, 3-hop, array join | TS | ❌ |
| `ts_sqli_function_built` | SQL built inside helper function, used in caller | same-file, cross-file | TS | ❌ |
| `ts_sqli_table_name` | Table or column name from user input | table, column, ORDER BY direction | TS | ❌ |
| `ts_sqli_multiline_build` | SQL assembled across multiple lines | push+join, reduce, += | TS | ❌ |
| `rust_sqli_format_macro` | `format!("SELECT WHERE id={}", id)` in query | sqlx, diesel, postgres | Rust | ❌ |
| `rust_sqli_string_push` | SQL built with `.push_str(user_val)` | push_str, + operator | Rust | ❌ |

### 1.2 ORM-Specific Raw Query
| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_sqli_prisma_query_raw_unsafe`** | `prisma.$queryRawUnsafe(interpolated)` | direct, via var, via helper | TS | ❌ |
| `ts_sqli_prisma_execute_raw_unsafe` | `prisma.$executeRawUnsafe(interpolated)` | all mutations | TS | ❌ |
| `ts_sqli_sequelize_raw` | `sequelize.query("SELECT " + input)` | query, queryInterface | TS | ❌ |
| `ts_sqli_sequelize_where_object` | `where: { [req.body.field]: value }` | computed key, spread from body | TS | ❌ |
| `ts_sqli_typeorm_query_builder` | `.createQueryBuilder().where(rawInput)` | where, having, orderBy | TS | ❌ |
| `ts_sqli_typeorm_query` | `manager.query("SELECT " + input)` | manager, connection, direct | TS | ❌ |
| `ts_sqli_knex_raw` | `knex.raw("SELECT " + input)` | knex.raw, knex.schema.raw | TS | ❌ |
| `ts_sqli_drizzle_sql_template` | `sql.raw(interpolatedString)` | direct interpolation | TS | ❌ |

### 1.3 D1 / SQLite (Cloudflare Workers)
| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_sqli_d1_prepare_template`** | `env.DB.prepare(\`SELECT ${userId}\`)` | direct, via var | TS | ❌ |
| `ts_sqli_d1_missing_bind` | `env.DB.prepare("SELECT ? ").first()` — no .bind() | all(), first(), run() | TS | ❌ |
| `ts_sqli_better_sqlite3_raw` | `db.prepare("SELECT " + input).run()` | run, get, all | TS | ❌ |

### 1.4 Second-Order Injection
| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_sqli_second_order_stored` | Value stored then used in later query | same request, cross-request | TS | ❌ |
| `ts_sqli_second_order_cache` | Tainted value cached, retrieved and used in SQL | KV, Redis, in-memory | TS | ❌ |

---

## Category 2 — NoSQL Injection (`CWE-943`)
**Target: 60 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_nosqli_mongo_where`** | `{ $where: userInput }` | function, string expression | TS | ⚠️ |
| **`ts_nosqli_mongo_computed_key`** | `{ [req.body.operator]: value }` | direct, destructured | TS | ❌ |
| `ts_nosqli_mongo_regex` | `{ field: { $regex: userInput } }` | direct regex, flag injection | TS | ❌ |
| `ts_nosqli_mongo_query_selector` | Query object constructed from `req.body` directly | spread, Object.assign | TS | ❌ |
| `ts_nosqli_mongoose_populate_path` | `.populate(req.query.path)` | string, object | TS | ❌ |
| `ts_nosqli_redis_eval` | `redis.eval(userScript, ...)` | Lua script injection | TS | ❌ |
| `ts_nosqli_dynamodb_filter_expr` | FilterExpression with unparameterised value | direct, via var | TS | ❌ |
| `ts_nosqli_firestore_where_injection` | `.where(field, op, userValue)` where `op` from user | operator from user | TS | ❌ |
| `ts_nosqli_couchdb_view` | CouchDB view with user-controlled key | startkey, endkey | TS | ❌ |
| `ts_nosqli_elasticsearch_query` | Elasticsearch query built from user body | match, bool, script | TS | ❌ |

---

## Category 3 — Command Injection (`CWE-78`)
**Target: 80 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_cmdi_exec_direct`** | `exec(userInput)` | exec, execSync | TS | ✅ |
| **`ts_cmdi_template_literal`** | `` exec(`convert ${filename} output.pdf`) `` | exec, spawn, execa | TS | ❌ |
| `ts_cmdi_spawn_args` | `spawn(cmd, [userArg])` | spawn, spawnSync | TS | ❌ |
| `ts_cmdi_shell_true` | `spawn(userCmd, { shell: true })` | any shell:true | TS | ❌ |
| `ts_cmdi_env_injection` | `spawn(cmd, { env: { PATH: userInput } })` | PATH, LD_PRELOAD | TS | ❌ |
| `ts_cmdi_third_party_ffmpeg` | `exec(\`ffmpeg -i ${userFile}\`)` | ffmpeg, imagemagick, ghostscript | TS | ❌ |
| `ts_cmdi_third_party_git` | `exec(\`git clone ${userUrl}\`)` | git, svn, curl | TS | ❌ |
| `ts_cmdi_zip_unzip` | `exec(\`unzip ${userFile} -d ${userDir}\`)` | zip, tar, 7z | TS | ❌ |
| `ts_cmdi_indirect_via_file` | Write user input to file, execute file | shell script, config | TS | ❌ |
| **`rust_cmdi_command_new`** | `Command::new(user_input).spawn()` | new, arg, args | Rust | ✅ |
| `rust_cmdi_command_arg_injection` | `Command::new("ls").arg(user_arg)` | arg, args, env | Rust | ❌ |
| `rust_cmdi_shell_command` | `Command::new("sh").arg("-c").arg(user_cmd)` | sh, bash, cmd.exe | Rust | ❌ |

---

## Category 4 — Template Injection / SSTI (`CWE-94`)
**Target: 50 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_ssti_ejs_render`** | `ejs.render(userTemplate, data)` | render, renderFile | TS | ❌ |
| `ts_ssti_handlebars_compile` | `Handlebars.compile(userTemplate)` | compile, precompile | TS | ❌ |
| `ts_ssti_pug_render` | `pug.render(userTemplate)` | render, renderFile | TS | ❌ |
| `ts_ssti_nunjucks_render` | `nunjucks.renderString(userTemplate, ctx)` | renderString | TS | ❌ |
| `ts_ssti_lodash_template` | `_.template(userStr)(data)` | direct call | TS | ❌ |
| `ts_ssti_mustache_render` | `Mustache.render(userTemplate, view)` | template from user | TS | ❌ |
| `ts_ssti_js_template_function` | `new Function("return " + userCode)()` | new Function, eval | TS | ❌ |
| `ts_ssti_react_jsx_injection` | `eval("React.createElement(" + userCode + ")")` | via eval | TS | ❌ |

---

## Category 5 — Log Injection (`CWE-117`)
**Target: 40 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_log_injection_newline` | Newline in log message fakes log entries | console.log, logger | TS | ❌ |
| `ts_log_injection_json_key` | User controls key in structured log object | JSON.stringify | TS | ❌ |
| `ts_log_injection_level` | User can set log level field | severity injection | TS | ❌ |
| `ts_log_injection_pii` | PII (email, password) directly logged | console.log, logger | TS | ✅ |
| `ts_log_injection_token` | Auth token logged | console.log, logger | TS | ✅ |
| `ts_log_injection_credit_card` | Credit card number in log | regex should catch | TS | ❌ |
| `rust_log_injection_format` | `log::info!("{}", user_input)` with ANSI | info!, debug!, warn! | Rust | ❌ |

---

## Category 6 — Header Injection (`CWE-113`)
**Target: 40 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_header_injection_set_header` | `res.setHeader(userKey, userValue)` | setHeader, set | TS | ❌ |
| `ts_header_injection_content_disposition` | Filename in Content-Disposition from user | attachment filename | TS | ❌ |
| `ts_header_injection_location` | `res.setHeader("Location", userUrl)` | open redirect variant | TS | ❌ |
| `ts_header_injection_crlf` | CRLF in cookie value or header value | Set-Cookie, Location | TS | ❌ |
| `ts_email_header_injection` | Newline in To/CC/BCC/Subject | nodemailer, sendmail | TS | ❌ |

---

## Category 7 — LDAP Injection (`CWE-90`)
**Target: 20 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_ldap_search_filter` | `ldap.search("(uid=" + userInput + ")")` | direct, template | TS | ❌ |
| `ts_ldap_dn_injection` | DN constructed from user input | bind, add, delete | TS | ❌ |
| `ts_ldap_attribute_injection` | Attribute name from user | search, compare | TS | ❌ |

---

## Category 8 — XPath Injection (`CWE-91`)
**Target: 20 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_xpath_select` | `xpath.select("/users/user[name/text()='" + user + "']", doc)` | concat, template | TS | ❌ |
| `ts_xpath_evaluate` | `doc.evaluate(userXpath, ...)` | direct user expression | TS | ❌ |

---

## Category 9 — Code Injection (`CWE-94`)
**Target: 60 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| **`ts_eval_direct`** | `eval(userCode)` | eval, setTimeout(string) | TS | ✅ |
| **`ts_eval_function_constructor`** | `new Function(userCode)()` | new Function, Function() | TS | ❌ |
| `ts_eval_vm_run` | `vm.runInNewContext(userCode)` | runInNewContext, runInThisContext | TS | ❌ |
| `ts_eval_vm_script` | `new vm.Script(userCode).runInContext(ctx)` | Script, createContext | TS | ❌ |
| `ts_dynamic_require` | `require(userPath)` | require, createRequire | TS | ❌ |
| `ts_dynamic_import` | `` import(userModule) `` | dynamic import | TS | ❌ |
| `ts_settimeout_string` | `setTimeout(userString, delay)` | setTimeout, setInterval | TS | ❌ |
| `ts_node_vm_user_module` | Loading user-provided module string | Module._compile | TS | ❌ |
| `rust_proc_macro_injection` | Macro input from external source | proc-macro, quote! | Rust | ❌ |

---

## Category 10 — Expression / Query Injection
**Target: 40 corpus pairs**

| ID | Name | Mutations | Lang | Status |
|----|------|-----------|------|--------|
| `ts_jsonpath_injection` | `jsonpath.query(obj, userPath)` | direct path | TS | ❌ |
| `ts_jmespath_injection` | `jmespath.search(data, userExpr)` | direct expression | TS | ❌ |
| `ts_odata_injection` | OData filter string from user | $filter, $orderby | TS | ❌ |
| `ts_graphql_query_injection` | GraphQL query built from user string | query, mutation | TS | ❌ |
| `ts_cel_injection` | CEL expression from user (Firebase rules) | direct | TS | ❌ |
| `ts_regex_injection_user_pattern` | `new RegExp(userPattern)` — ReDoS | direct, flags from user | TS | ❌ |
| `ts_regex_redos` | Catastrophic backtracking pattern | (a+)+ style, nested groups | TS | ✅ |

---

## Category 11-15 — Cross-Site Scripting (`CWE-79`)
**Target: 120 corpus pairs**

### 11 — Reflected XSS
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_xss_reflected_response` | User input echoed in HTML response body | res.send, template render | ❌ |
| `ts_xss_reflected_error_message` | User-controlled error message in page | error handler | ❌ |
| `ts_xss_reflected_url_param` | URL parameter reflected in HTML | query string, hash | ❌ |
| `ts_xss_reflected_search` | Search term in results page | results header, highlight | ❌ |
| `ts_xss_reflected_json_html` | JSON value rendered as HTML elsewhere | API response used in template | ❌ |

### 12 — Stored XSS
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_xss_stored_db_to_html` | DB value rendered without encoding | direct, via template | ❌ |
| `ts_xss_stored_user_profile` | User profile field (name, bio) in HTML | name, bio, website URL | ❌ |
| `ts_xss_stored_comment_system` | Comment rendered without sanitisation | body, title, author | ❌ |
| `ts_xss_stored_admin_panel` | User data shown in admin panel without encoding | table cell, detail view | ❌ |
| `ts_xss_stored_email_template` | User data in HTML email template | to name, subject, body | ❌ |

### 13 — DOM XSS
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_xss_dom_innerhtml` | `element.innerHTML = location.hash` | innerHTML, outerHTML | ❌ |
| `ts_xss_dom_document_write` | `document.write(location.search)` | write, writeln | ❌ |
| `ts_xss_dom_jquery_html` | `$(el).html(userInput)` | html(), append(), prepend() | ❌ |
| `ts_xss_dom_eval_atob` | `eval(atob(location.hash))` | eval with decoded input | ❌ |
| `ts_xss_dom_postmessage` | `element.innerHTML = event.data` without origin check | postMessage handler | ❌ |
| `ts_xss_dom_href_javascript` | `element.href = "javascript:" + userInput` | href, src, action | ❌ |
| `ts_xss_dom_cookie_reflection` | Reading document.cookie into innerHTML | cookie to DOM | ❌ |

### 14 — React / JSX XSS
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`tsx_dangerously_set_inner_html`** | `dangerouslySetInnerHTML={{ __html: userInput }}` | direct, via prop, via state | TS | ✅ |
| `tsx_xss_href_javascript` | `<a href={userUrl}>` without protocol check | href, src, action | TS | ❌ |
| `tsx_xss_ref_dom_write` | `ref.current.innerHTML = userInput` | innerHTML via ref | TS | ❌ |
| `tsx_xss_ssr_raw` | `__html` from server-side rendered prop | SSR injection | TS | ❌ |
| `tsx_xss_styled_component` | CSS injection via styled-component props | css prop, style tag | TS | ❌ |

### 15 — XSS via Upload / Indirect
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_xss_svg_upload` | SVG file with `<script>` tag served inline | image/svg+xml | TS | ❌ |
| `ts_xss_html_upload` | HTML file uploaded, served with wrong MIME | text/html | TS | ❌ |
| `ts_xss_filename_in_response` | Filename reflected in download header without encoding | Content-Disposition | TS | ❌ |
| `ts_xss_pdf_javascript` | PDF with embedded JavaScript action | annotation, form | TS | ❌ |

---

## Category 16-18 — Server-Side Request Forgery (`CWE-918`)
**Target: 80 corpus pairs**

### 16 — Direct SSRF
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_ssrf_fetch_direct`** | `fetch(userUrl)` without allowlist | fetch, axios.get | TS | ✅ |
| **`ts_ssrf_fetch_constructed`** | URL built from user parts used in fetch | path join, protocol + host | TS | ✅ |
| `ts_ssrf_webhook_registration` | Storing user-provided URL, calling it later | webhook, callback | TS | ❌ |
| `ts_ssrf_redirect_follow` | Following redirect to user-controlled URL | 301/302 handling | TS | ❌ |
| `ts_ssrf_import_url` | `import(userUrl)` — dynamic import from URL | node --experimental | TS | ❌ |
| `ts_ssrf_dns_rebinding_vector` | Hostname resolved twice, no IP validation | DNS rebind prevention missing | TS | ❌ |
| `rust_ssrf_reqwest` | `reqwest::get(user_url).await` | get, post | Rust | ❌ |
| `rust_ssrf_hyper_uri` | `Uri::from_str(user_url)` in client | hyper, ureq | Rust | ❌ |

### 17 — Indirect SSRF
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_ssrf_pdf_generation` | PDF generator fetches user-controlled URL | wkhtmltopdf, puppeteer | TS | ❌ |
| `ts_ssrf_image_processing` | Image library fetches URL from user | sharp with URL, jimp | TS | ❌ |
| `ts_ssrf_headless_browser` | Puppeteer/Playwright navigates to user URL | goto(userUrl) | TS | ❌ |
| `ts_ssrf_xml_external_entity` | XXE as SSRF vector | DOCTYPE fetch | TS | ❌ |
| `ts_ssrf_graphql_url_directive` | GraphQL @external directive to user URL | schema stitching | TS | ❌ |

### 18 — SSRF to Cloud Metadata
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_ssrf_aws_metadata` | URL not blocked, allows 169.254.169.254 | AWS, GCP, Azure metadata | TS | ❌ |
| `ts_ssrf_internal_network` | URL allows RFC1918 addresses | 10.x, 172.16.x, 192.168.x | TS | ❌ |
| `ts_ssrf_localhost` | URL allows localhost / 127.0.0.1 | localhost, 0.0.0.0 | TS | ❌ |

---

## Category 19-21 — Path Traversal (`CWE-22`)
**Target: 80 corpus pairs**

### 19 — Read Traversal
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_path_traversal_readfile`** | `fs.readFile(basePath + userPath)` | readFile, readFileSync | TS | ✅ |
| `ts_path_traversal_join_no_check` | `path.join(base, userPath)` without prefix verification | join, resolve | TS | ✅ |
| `ts_path_traversal_unicode_bypass` | Unicode normalization bypass (..%2F, %252F) | encoded traversal | TS | ❌ |
| `ts_path_traversal_null_byte` | Null byte termination bypass | %00 suffix | TS | ❌ |
| `ts_path_traversal_symlink_follow` | Symlink followed outside allowed directory | lstat not checked | TS | ❌ |
| `rust_path_traversal_fs_read` | `fs::read_to_string(base.join(user_path))` | read_to_string, read, File::open | Rust | ❌ |

### 20 — Write Traversal
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_path_traversal_writefile` | `fs.writeFile(basePath + userFilename, data)` | writeFile, appendFile | TS | ❌ |
| `ts_path_traversal_upload_filename` | File saved with user-provided filename | multer, formidable | TS | ❌ |
| **`ts_zip_slip`** | Zip entry path extracted to user-controlled dir | zip, tar, 7z | TS | ❌ |
| `ts_path_traversal_log_write` | Log file path from user config | winston, pino | TS | ❌ |

### 21 — Execute / Delete Traversal
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_path_traversal_unlink` | `fs.unlink(base + userPath)` | unlink, rm, rmdir | TS | ❌ |
| `ts_path_traversal_require` | `require(userPath)` with traversal | relative, absolute | TS | ❌ |
| `ts_path_traversal_exec_file` | `execFile(basePath + userFile)` | execFile | TS | ❌ |

---

## Category 22-30 — Authentication & Session (`CWE-287, 384`)
**Target: 200 corpus pairs**

### 22 — Broken Authentication
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_auth_timing_attack_compare` | `password === storedHash` (non-constant-time) | ==, ===, indexOf | TS | ❌ |
| `ts_auth_md5_password` | MD5 used for password hashing | md5, sha1, sha256 | TS | ❌ |
| `ts_auth_no_hash` | Password stored in plaintext | direct DB write | TS | ❌ |
| `ts_auth_low_bcrypt_rounds` | `bcrypt.hash(password, 4)` (rounds < 10) | rounds 1-9 | TS | ❌ |
| `ts_auth_bcrypt_truncation` | bcrypt silently truncates at 72 bytes | long password issue | TS | ❌ |
| `ts_auth_predictable_reset_token` | Reset token using `Math.random()` | Math.random, Date.now | TS | ✅ |

### 23 — Session Management
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_session_fixation` | Session ID not regenerated after login | express-session, custom | TS | ❌ |
| `ts_session_not_invalidated_logout` | Session token not deleted on logout | JWT without blacklist, session store | TS | ❌ |
| `ts_session_in_url` | Session token in URL parameter | query string, redirect | TS | ❌ |
| `ts_session_long_expiry` | JWT/cookie expiry > 30 days with no revocation | 365d, no-expiry | TS | ❌ |
| `ts_session_weak_secret` | Session secret too short or predictable | "secret", short string | TS | ❌ |

### 24 — JWT Vulnerabilities
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_jwt_decode_vs_verify`** | `jwt.decode()` instead of `jwt.verify()` | decode without verify | TS | ✅ |
| `ts_jwt_algorithm_none` | `alg: "none"` accepted in verification | none, NONE | TS | ❌ |
| `ts_jwt_algorithm_confusion` | RS256 key used to verify HS256 | symmetric/asymmetric swap | TS | ❌ |
| `ts_jwt_missing_expiry_check` | JWT verified but `exp` claim not checked | manual verify | TS | ❌ |
| `ts_jwt_missing_audience_check` | `aud` claim not verified | service-to-service misuse | TS | ❌ |
| `ts_jwt_weak_secret` | JWT signed with short/guessable secret | "secret", empty string | TS | ❌ |
| `ts_jwt_in_localstorage` | JWT stored in localStorage (XSS-accessible) | localStorage.setItem | TS | ❌ |
| `ts_jwt_sensitive_payload` | PII or secret in JWT payload (base64 only) | email, role, SSN in claims | TS | ❌ |

### 25 — OAuth / OIDC
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_oauth_missing_state` | OAuth callback without state parameter CSRF check | state missing, not verified | TS | ❌ |
| `ts_oauth_redirect_uri_not_validated` | Redirect URI not checked against whitelist | open redirect variant | TS | ❌ |
| `ts_oauth_token_in_referrer` | Access token in URL, leaked via Referer header | implicit flow, fragment | TS | ❌ |
| `ts_oidc_missing_nonce` | Nonce not verified in ID token | replay attack vector | TS | ❌ |
| `ts_oauth_overpermissive_scope` | Requesting all scopes instead of minimum | scope minimization | TS | ❌ |
| `ts_oauth_pkce_missing` | PKCE not used in public client | code_challenge missing | TS | ❌ |

### 26 — API Key Management
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_apikey_in_url` | API key in URL query parameter | ?key=, ?apiKey= | TS | ❌ |
| `ts_apikey_in_client_bundle` | API key embedded in frontend JS bundle | process.env in client | TS | ❌ |
| **`ts_apikey_hardcoded`** | API key string literal in source code | const KEY = "sk-..." | TS | ✅ |
| `ts_apikey_in_log` | API key printed to log | console.log, logger | TS | ❌ |
| `ts_apikey_no_scoping` | API key grants full access, no scope | admin key used for user ops | TS | ❌ |
| `ts_apikey_third_party_forward` | Own API key forwarded to third party | fetch with own key | TS | ✅ |

### 27 — MFA Bypass
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_mfa_code_no_expiry` | OTP valid indefinitely | no expiry check | TS | ❌ |
| `ts_mfa_code_reuse` | OTP can be used multiple times | no invalidation after use | TS | ❌ |
| `ts_mfa_rate_limit_missing` | No limit on OTP attempts | brute-force possible | TS | ✅ |
| `ts_mfa_bypass_direct_endpoint` | MFA can be skipped by calling post-MFA endpoint directly | auth flow bypass | TS | ❌ |
| `ts_mfa_backup_codes_unhashed` | Backup codes stored unhashed | plaintext storage | TS | ❌ |

### 28 — Password Reset
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_reset_token_weak_random`** | `Math.random().toString(36)` as reset token | Math.random, Date.now | TS | ✅ |
| `ts_reset_token_no_expiry` | Reset token valid forever | no expiry stored | TS | ❌ |
| `ts_reset_user_enumeration` | Different response for existing vs non-existing user | timing, message | TS | ❌ |
| `ts_reset_token_reuse` | Same token can reset password multiple times | no invalidation | TS | ❌ |
| `ts_reset_no_email_verify` | Password reset without email confirmation | direct reset | TS | ❌ |

### 29 — Account Lockout
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_lockout_missing` | No failed-attempt limit on login | brute force possible | TS | ❌ |
| `ts_lockout_ip_only` | Lockout by IP only, bypassable via rotation | IP-based only | TS | ❌ |
| `ts_lockout_not_atomic` | Lockout state check outside transaction | race condition on lockout | TS | ❌ |

### 30 — Credential Stuffing Vectors
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_login_no_rate_limit` | No rate limit on login endpoint | missing rate limit | TS | ❌ |
| `ts_login_username_enumeration_timing` | Timing difference for valid vs invalid user | bcrypt short-circuit | TS | ❌ |
| `ts_login_username_enumeration_message` | "User not found" vs "Wrong password" messages | distinct error msgs | TS | ❌ |
| `ts_captcha_bypassable` | CAPTCHA not server-side verified | client-side only | TS | ❌ |

---

## Category 31-40 — Authorization & IDOR (`CWE-639, 284`)
**Target: 250 corpus pairs**

### 31 — IDOR Direct
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_idor_url_resource_id`** | `GET /items/:id` — no ownership check before query | numeric, UUID | TS | ✅ |
| **`ts_idor_body_resource_id`** | `POST /update` with `resourceId` from body, no ownership check | update, delete | TS | ✅ |
| `ts_idor_sequential_int_id` | Sequential integer IDs (enumerable) | auto-increment | TS | ❌ |
| `ts_idor_pre_query_no_check` | Resource fetched before ownership verified | fetch then check | TS | ✅ |

### 32 — IDOR Indirect
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_idor_child_resource`** | Child resource accessible without parent ownership | comment → post, item → order | TS | ✅ |
| `ts_idor_aggregate_leak` | Aggregate endpoint leaks per-user data | totals, averages | TS | ❌ |
| `ts_idor_batch_endpoint` | Batch operation processes IDs without per-ID ownership check | bulk update | TS | ❌ |
| `ts_idor_filter_existence_leak` | Filter reveals existence of resources from other users | filter error vs empty | TS | ❌ |

### 33 — IDOR Write
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_idor_update_no_ownership`** | `UPDATE SET ... WHERE id=?` — id from user, no ownership | DB update | TS | ✅ |
| `ts_idor_delete_no_ownership` | `DELETE WHERE id=?` — no ownership | DB delete | TS | ❌ |
| `ts_idor_transfer_no_check` | Transfer resource to another user without verification | ownership change | TS | ❌ |
| `ts_idor_share_no_check` | Share resource without being the owner | public sharing | TS | ❌ |

### 34 — IDOR Functional
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_idor_workflow_action`** | Approve/reject action on any ID without role check | workflow IDOR | TS | ✅ |
| `ts_idor_assign_resource` | Assign resource to user without manager role | team management | TS | ❌ |
| `ts_idor_publish_without_author_check` | Publish/unpublish content without authorship verify | CMS pattern | TS | ❌ |

### 35 — Broken Access Control
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_bac_admin_endpoint_no_check`** | Admin endpoint accessible without admin role | /admin/* | TS | ✅ |
| `ts_bac_internal_api_exposed` | Internal service API callable externally | service-to-service API | TS | ❌ |
| `ts_bac_debug_endpoint_production` | Debug/dev endpoint reachable in production | /debug, /info, /metrics | TS | ❌ |
| `ts_bac_http_method_bypass` | Mutation operation accessible via GET | GET vs POST for writes | TS | ❌ |
| `ts_bac_feature_flag_bypass` | Feature flag checked client-side only | feature flag bypass | TS | ❌ |

### 36 — Privilege Escalation
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_privesc_role_mass_assignment` | Role field in update body processed without filter | mass assignment | TS | ❌ |
| **`ts_privesc_hardcoded_role_in_token`** | `{ role: "user" }` hardcoded in JWT/session | hardcoded privilege | TS | ✅ |
| `ts_privesc_admin_url_param` | `?admin=true` parameter grants privilege | param-based escalation | TS | ❌ |
| `ts_privesc_token_scope_expansion` | Token issued with broader scope than requested | OAuth scope escalation | TS | ❌ |

### 37 — RBAC Bypass
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_rbac_role_without_status_check`** | Role check passes but account status not verified | suspended user | TS | ✅ |
| `ts_rbac_check_after_action` | Authorization check after sensitive action taken | check too late | TS | ❌ |
| `ts_rbac_stale_cached_role` | Role checked from cached session, not fresh DB value | stale role | TS | ❌ |
| `ts_rbac_and_or_confusion` | Multiple roles checked with wrong AND/OR logic | AND vs OR bug | TS | ❌ |
| `ts_rbac_missing_on_update` | Create requires role, update does not | asymmetric check | TS | ❌ |

### 38 — Tenant Isolation
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_tenant_id_from_request`** | Tenant ID taken from request body/header instead of session | body.tenantId | TS | ❌ |
| `ts_tenant_cross_tenant_data` | Query not filtered by tenant ID from session | missing WHERE tenantId | TS | ❌ |
| `ts_tenant_shared_cache_key` | Cache key not namespaced per tenant | Redis, KV shared key | TS | ❌ |
| `ts_tenant_url_id_not_verified` | Tenant ID in URL not verified against session | /tenant/:id not checked | TS | ❌ |

### 39 — API Authorization Patterns
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_auth_unauthenticated_db_write`** | DB write in handler with no auth check | any DB write, no session | TS | ✅ |
| **`ts_auth_fail_open`** | `try { session = verify() } catch { session = {} }` | catch gives access | TS | ✅ |
| **`ts_auth_no_rejection`** | Auth function returns `null` but caller never checks | CSA pattern | TS | ✅ |
| `ts_auth_missing_on_delete` | GET and PUT protected, DELETE is not | HTTP verb gap | TS | ❌ |
| `ts_auth_cors_private_api` | CORS allows external origins on internal API | CORS bypass | TS | ✅ |

### 40 — GraphQL Authorization
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_graphql_missing_field_auth` | Field resolver has no auth check | query, mutation field | TS | ❌ |
| `ts_graphql_introspection_production` | Schema introspection enabled in production | __schema, __type | TS | ❌ |
| `ts_graphql_batch_attack` | No limit on operation count per request | batching, aliases | TS | ❌ |
| `ts_graphql_depth_limit_missing` | Query depth not limited | nested query DoS | TS | ❌ |
| `ts_graphql_complexity_missing` | Query complexity not limited | expensive fields | TS | ❌ |

---

## Category 41-55 — Business Logic (`CWE-840`)
**Target: 350 corpus pairs**

### 41 — State Machine Bypass
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_state_machine_direct_update`** | Direct DB status update bypassing state machine | order, subscription, return | TS | ✅ |
| **`ts_state_machine_payment_bypass`** | Payment creates completed order bypassing confirmation | payment webhook | TS | ❌ |
| `ts_state_machine_invalid_transition` | Transition from terminal state (COMPLETED → PENDING) | invalid backward transition | TS | ❌ |
| `ts_state_machine_concurrent_transition` | Two simultaneous state transitions race condition | both pass guard check | TS | ❌ |
| `ts_state_machine_skip_approval` | Required approval step skipped | multi-step workflow | TS | ❌ |

### 42 — Financial — Price Manipulation
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_price_from_client` | Total price taken from request body, not calculated server-side | checkout price | TS | ❌ |
| `ts_price_negative` | Negative price allowed in checkout | -1 item, negative total | TS | ❌ |
| `ts_price_integer_overflow` | Price × quantity overflows integer type | u32, i32 overflow | TS | ❌ |
| `ts_price_currency_confusion` | Price in USD compared/added to price in NGN | currency mismatch | TS | ❌ |
| `ts_price_discount_exceeds_total` | Discount can exceed item price | no floor at 0 | TS | ❌ |
| `ts_price_free_with_coupon` | 100% discount coupon accepted | free checkout | TS | ❌ |

### 43 — Financial — Refund Abuse
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_refund_without_return_verification` | Refund issued without verifying returned item | refund before return | TS | ❌ |
| `ts_refund_double_refund` | Same order refunded twice (no idempotency) | race, direct endpoint | TS | ❌ |
| `ts_refund_amount_from_client` | Refund amount from request body | client-controlled amount | TS | ❌ |
| `ts_refund_missing_ledger` | Wallet credited without ledger debit | wallet + ledger mismatch | TS | ❌ |
| `ts_refund_race_condition` | Refund check outside transaction | TOCTOU refund | TS | ❌ |

### 44 — Financial — Payment Gate
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_missing_payment_gate`** | Expensive operation without tier/credit check | LLM call, sandbox, premium feature | TS | ✅ |
| `ts_payment_check_after_action` | Payment deducted after expensive operation completes | check too late | TS | ❌ |
| `ts_expired_subscription_still_active` | Subscription end date not checked, feature granted | expired but allowed | TS | ❌ |
| `ts_trial_abuse_multiple_accounts` | Trial period enforced only by email, not device/IP | new email = new trial | TS | ❌ |
| `ts_premium_feature_via_url_param` | `?premium=true` unlocks feature | param-based unlock | TS | ❌ |

### 45 — Financial — Ledger Consistency
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_missing_ledger_after_wallet_fund`** | Wallet funded, ledger entry not created | fundWallet without ledger | TS | ❌ |
| `ts_ledger_double_counting` | Aggregation sums both DEBIT and CREDIT entries | revenue calculation | TS | ❌ |
| `ts_ledger_missing_debit` | Credit recorded, corresponding debit not | incomplete double-entry | TS | ❌ |
| `ts_ledger_currency_mismatch` | USD ledger entry for NGN transaction | multi-currency bug | TS | ❌ |
| `ts_ledger_refund_not_recorded` | Refund given but not in seller ledger | seller over-payout | TS | ❌ |

### 46 — Coupon / Discount
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_coupon_scope_bypass`** | Coupon scoped to one seller, discount applied to full order | scope mismatch | TS | ✅ |
| `ts_coupon_per_user_limit_missing` | Coupon with per-user limit not enforced | usage count | TS | ❌ |
| `ts_coupon_global_limit_missing` | Total coupon usage limit not enforced | max redemptions | TS | ❌ |
| `ts_coupon_expiry_not_checked` | Expired coupon accepted | date comparison | TS | ❌ |
| `ts_coupon_min_order_not_checked` | Minimum order requirement not validated | amount threshold | TS | ❌ |
| `ts_coupon_stacking` | Multiple coupons stacked beyond allowed | stacking limit | TS | ❌ |
| `ts_coupon_category_bypass` | Coupon limited to category, applied to any item | category filter | TS | ❌ |
| `ts_coupon_wrong_user` | Coupon limited to one user, usable by any | user ownership | TS | ❌ |

### 47 — Inventory
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_inventory_oversell`** | Stock check outside transaction — race condition | concurrent purchases | TS | ❌ |
| **`ts_inventory_release_missing_on_cancel`** | Order cancelled, stock reservation not released | missing releaseStock | TS | ❌ |
| `ts_inventory_negative_stock` | Stock can go below zero | no floor at 0 | TS | ❌ |
| `ts_inventory_wrong_id_to_release` | `releaseStock(orderId)` instead of `releaseStock(reservationId)` | wrong ID type | TS | ❌ |
| `ts_inventory_reservation_not_confirmed` | Reservation created, never confirmed on payment | zombie reservation | TS | ❌ |

### 48 — Rate Limiting
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_ratelimit_missing_sensitive` | No rate limit on login / OTP / reset endpoint | brute force surface | TS | ❌ |
| `ts_ratelimit_ip_only_bypassable` | Rate limit by IP only (X-Forwarded-For spoofable) | IP header bypass | TS | ✅ |
| `ts_ratelimit_bypass_header` | Rate limit bypassed via X-Real-IP, CF-Connecting-IP | header spoofing | TS | ✅ |
| `ts_ratelimit_not_on_expensive_op` | No rate limit on LLM, compute, or heavy DB operation | cost amplification | TS | ❌ |
| `ts_ratelimit_per_endpoint_not_global` | Per-endpoint limits but no global user limit | endpoint bypass | TS | ❌ |

### 49 — Quota Management
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_quota_fail_open`** | `catch { return { allowed: true } }` — quota fails open | error grants access | TS | ✅ |
| `ts_quota_race_condition` | Quota check outside transaction — concurrent bypass | double-spend variant | TS | ✅ |
| `ts_quota_side_channel_not_counted` | Credits for MCP tools not deducted from quota | side channel bypass | TS | ❌ |
| `ts_quota_per_user_not_per_account` | Quota per user-row, not per account | sub-account bypass | TS | ❌ |
| `ts_quota_not_enforced_on_retry` | Quota checked first call, not on retry | retry bypass | TS | ❌ |

### 50 — Subscription / Entitlement
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_subscription_not_revoked_on_cancel` | Entitlements remain after subscription cancelled | lazy revocation | TS | ❌ |
| `ts_subscription_downgrade_bypass` | Downgrade doesn't revoke premium features | feature still available | TS | ❌ |
| **`ts_subscription_hardcoded_feature_flag`** | `const isPremium = true` hardcoded | hardcoded flag | TS | ❌ |
| `ts_subscription_plan_check_missing` | Feature used without checking current plan | plan not verified | TS | ❌ |
| `ts_subscription_concurrent_activation` | Two subscriptions activated simultaneously | duplicate subscription | TS | ❌ |

### 51 — Loyalty / Points
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_points_awarded_before_fulfillment` | Points credited before order fulfilled/delivered | premature award | TS | ❌ |
| `ts_points_race_condition` | Points check and deduction not atomic | double-spend points | TS | ❌ |
| `ts_points_overflow` | Points balance can overflow integer type | large accumulation | TS | ❌ |
| `ts_points_self_referral` | User can earn referral bonus from own referral | self-referral | TS | ❌ |

### 52 — Affiliate / Referral
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_affiliate_commission_on_cancelled` | Commission paid on orders that are later cancelled | refund without commission reversal | TS | ❌ |
| **`ts_affiliate_wrong_status_filter`** | Commission for DELIVERED orders, but check uses COMPLETED | status name mismatch | TS | ❌ |
| `ts_affiliate_self_referral` | User can refer themselves for bonus | identity check missing | TS | ❌ |
| `ts_affiliate_commission_race` | Commission credited twice for same order | idempotency missing | TS | ❌ |

### 53-55 — Order/Booking/Auction
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_order_modification_after_fulfil` | Order modifiable after shipped/delivered | state check missing | TS | ❌ |
| `ts_booking_double_book` | No lock prevents two bookings for same slot | concurrent booking | TS | ❌ |
| `ts_auction_bid_without_balance_check` | Bid placed without verifying user has funds | balance not checked | TS | ❌ |
| `ts_bundle_component_price` | Bundle price components manipulable individually | component extraction | TS | ❌ |
| `ts_timezone_scheduling` | Timezone not normalised in booking system | UTC vs local | TS | ❌ |

---

## Category 56-62 — Race Conditions (`CWE-362`)
**Target: 150 corpus pairs**

### 56 — TOCTOU Database
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_race_condition_read_check_write`** | Balance/quota check, then deduct, outside transaction | read-check-write | TS | ✅ |
| **`ts_idempotency_check_outside_txn`** | Idempotency key checked, then used — outside transaction | webhook duplicate | TS | ❌ |
| `ts_stock_check_outside_txn` | `if (stock >= qty)` then `UPDATE stock - qty` without lock | inventory race | TS | ❌ |
| `ts_session_creation_race` | Two concurrent logins create two sessions | duplicate session | TS | ❌ |
| **`ts_credit_double_spend`** | Credits deducted twice in concurrent requests | atomic deduction missing | TS | ✅ |

### 57 — TOCTOU File System
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_toctou_file_exist_then_open` | `fs.existsSync(path)` then `fs.readFile(path)` | TOCTOU via replace | TS | ❌ |
| `ts_toctou_permission_check_then_access` | Permission checked, then file accessed separately | symlink swap | TS | ❌ |
| `ts_toctou_temp_file_race` | Temp file with predictable name, race on creation | race to create | TS | ❌ |

### 58-62 — Other Race Conditions
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_race_cache_double_population` | Cache miss triggers two DB fetches, both write | Redis SETNX missing | TS | ❌ |
| `ts_race_job_deduplication` | Queue job processed twice (no dedup) | message queue | TS | ❌ |
| `ts_race_token_revocation` | Token used between invalidation and propagation | async revocation gap | TS | ❌ |
| `ts_race_subscription_concurrent_activation` | Two subscription activations race | concurrent payment | TS | ❌ |
| `rust_race_static_mut` | `static mut` accessed from multiple threads | static mut unsafe | Rust | ❌ |
| `rust_race_arc_refcell` | `Arc<RefCell<T>>` used across threads | not Send | Rust | ❌ |

---

## Category 63-70 — Cryptography (`CWE-327, 338`)
**Target: 150 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_crypto_weak_hash_md5` | MD5 used for security-sensitive hash | md5(), createHash("md5") | TS | ❌ |
| `ts_crypto_weak_hash_sha1` | SHA1 for signatures or password hashing | sha1(), createHash("sha1") | TS | ❌ |
| **`ts_insecure_random`** | `Math.random()` for security token | token, nonce, ID | TS | ✅ |
| `ts_insecure_random_date` | `Date.now()` as security nonce | timestamp as nonce | TS | ❌ |
| `ts_weak_id_entropy` | Sequential or low-entropy IDs for security objects | id++, short random | TS | ✅ |
| `ts_hardcoded_secret` | Secret key as string literal | API key, JWT secret, password | TS | ✅ |
| `ts_crypto_ecb_mode` | ECB cipher mode for block cipher | createCipheriv("aes-128-ecb") | TS | ❌ |
| `ts_crypto_iv_reuse` | Static IV for AES-CBC | hardcoded IV | TS | ❌ |
| `ts_crypto_key_in_code` | Encryption key hardcoded in source | AES key, RSA key | TS | ❌ |
| `ts_tls_verify_disabled` | `rejectUnauthorized: false` in HTTPS | TLS bypass | TS | ❌ |
| `ts_crypto_timing_not_safe_equal` | Non-constant-time comparison for secrets | == vs crypto.timingSafeEqual | TS | ❌ |
| `rust_rand_thread_rng_crypto` | `thread_rng()` used for cryptographic purpose | should be OsRng | Rust | ❌ |
| `rust_openssl_no_verify` | `SslConnector::builder` with verification disabled | Rust TLS bypass | Rust | ❌ |
| `rust_crypto_transmute_key` | Crypto key transmitted via transmute | unsafe key handling | Rust | ❌ |

---

## Category 71-80 — Memory Safety — Rust (`CWE-119, 416, 190`)
**Target: 150 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`rust_transmute`** | `std::mem::transmute(userValue)` | transmute, transmute_copy | Rust | ✅ |
| `rust_from_utf8_unchecked` | `str::from_utf8_unchecked(user_bytes)` | from_utf8_unchecked | Rust | ❌ |
| `rust_slice_from_raw_parts` | `slice::from_raw_parts(ptr, user_len)` | raw parts len from user | Rust | ❌ |
| **`rust_unwrap_in_handler`** | `.unwrap()` in request handler (panics → 500) | unwrap, expect | Rust | ❌ |
| `rust_integer_overflow_add` | `a + b` without `checked_add` in financial calc | addition, multiplication | Rust | ❌ |
| `rust_integer_cast_truncation` | `user_value as u32` truncates large u64 | cast narrowing | Rust | ❌ |
| `rust_vec_set_len` | `v.set_len(n)` without initializing elements | set_len unsafe | Rust | ❌ |
| **`rust_blocking_io_in_async`** | `std::fs::read` / `thread::sleep` in async fn | blocking in tokio | Rust | ✅ |
| **`rust_network_in_txn`** | Network call inside database transaction | HTTP inside BEGIN | Rust | ✅ |
| **`rust_connection_leak`** | DB connection acquired but not returned on error | pool exhaustion | Rust | ✅ |
| **`rust_clone_in_loop`** | Expensive `.clone()` of large struct per iteration | performance + allocation | Rust | ✅ |
| `rust_deadlock_lock_order` | Two mutexes acquired in different orders across threads | lock ordering | Rust | ❌ |
| `rust_deadlock_async_mutex` | `std::sync::Mutex` held across `.await` point | blocking hold | Rust | ❌ |
| `rust_resource_file_handle_leak` | `File::open` result used, no drop guarantee on error | error path drop | Rust | ❌ |
| **`rust_mutate_after_response`** | State mutated after response sent to client | post-response mutation | Rust | ✅ |

---

## Category 81-88 — Async / Promise Misuse (`CWE-662`)
**Target: 120 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_promise_missing_catch`** | Promise chain with no `.catch()` | .then without .catch | TS | ✅ |
| `ts_await_inside_foreach` | `await` inside `.forEach()` — not actually awaited | forEach, map without Promise.all | TS | ❌ |
| `ts_promise_fire_forget_side_effect` | State-modifying async function not awaited | fire-and-forget mutation | TS | ❌ |
| `ts_unhandled_rejection_catch_swallows` | `catch(err => {})` — error silently ignored | empty catch | TS | ✅ |
| `ts_promise_all_no_error_handling` | `Promise.all([...])` without catch | parallel ops | TS | ❌ |
| `ts_closure_stale_loop_variable` | `var` in loop captured by async callback | var vs let in loop | TS | ❌ |
| `ts_event_listener_not_removed` | EventEmitter listener added repeatedly without removal | memory leak | TS | ❌ |
| **`ts_workers_missing_wait_until`** | `ctx.waitUntil()` not used for async work | CF Workers lifetime | TS | ❌ |
| **`ts_module_level_state_leak`** | Module-level mutable state shared across requests | request isolation | TS | ✅ |
| `ts_setinterval_not_cleared` | `setInterval` started on request, never cleared | memory/CPU leak | TS | ❌ |
| `rust_await_in_sync_fn` | `.await` used without `async fn` | compilation error pattern | Rust | ✅ |
| `rust_spawn_without_join` | `tokio::spawn(task)` without `.await` on handle | detached task | Rust | ❌ |
| `rust_block_on_in_async` | `Runtime::block_on()` called inside async | async nested runtime | Rust | ✅ |

---

## Category 89-95 — Information Disclosure (`CWE-200, 209`)
**Target: 120 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_debug_stack_leak`** | `res.json({ error: err.stack })` | stack, message, raw Error | TS | ✅ |
| **`ts_unfiltered_json_response`** | Full DB row returned (includes passwordHash, etc.) | findUnique → json | TS | ✅ |
| `ts_user_enumeration_error_message` | "User not found" vs "Wrong password" | auth endpoint | TS | ❌ |
| `ts_debug_endpoint_production` | `/debug`, `/env`, `/config` accessible in production | debug routes | TS | ❌ |
| `ts_version_header_leak` | `X-Powered-By` or `Server` header exposes version | response headers | TS | ❌ |
| `ts_console_log_sensitive` | PII, token, or secret printed via console.log | debug log | TS | ✅ |
| `ts_response_internal_path` | File system path in error response | path in message | TS | ❌ |
| `ts_git_folder_accessible` | `.git/` directory served by static file server | .git, .env served | TS | ❌ |
| `ts_sourcemap_production` | Source maps served in production bundle | .map files | TS | ❌ |
| `ts_graphql_introspection_prod` | GraphQL introspection not disabled | __schema accessible | TS | ❌ |

---

## Category 96-100 — Deserialization (`CWE-502`)
**Target: 80 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_prototype_pollution_merge`** | Deep merge from user input: `merge(target, req.body)` | lodash.merge, Object.assign | TS | ✅ |
| `ts_prototype_pollution_json_proto` | JSON with `__proto__` or `constructor.prototype` | JSON.parse of user body | TS | ❌ |
| `ts_deserialization_yaml_load` | `yaml.load(userString)` — not yaml.safeLoad | js-yaml load | TS | ✅ |
| `ts_deserialization_eval_json` | `eval("(" + userJson + ")")` instead of JSON.parse | eval for JSON | TS | ❌ |
| `ts_xxe_xml_external_entity` | XML parser with external entity expansion enabled | XXE via DOCTYPE | TS | ❌ |
| `ts_xxe_libxml2_external` | libxml2 external entity in node-xml | SSRF via XML | TS | ❌ |
| `rust_bincode_from_network` | `bincode::deserialize(user_bytes)` | bincode, rmp (MessagePack) | Rust | ✅ |
| `rust_serde_arbitrary_enum` | Serde deserializing arbitrary user-controlled enum | tag-based enum | Rust | ❌ |

---

## Category 101-108 — File Operations (`CWE-434, 73`)
**Target: 120 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_file_upload_mime_not_checked` | MIME type from Content-Type not validated server-side | client-provided MIME | TS | ❌ |
| `ts_file_upload_extension_not_checked` | File extension not validated | .php, .js, .sh upload | TS | ❌ |
| `ts_file_upload_content_not_validated` | File content not inspected (magic bytes) | content check | TS | ❌ |
| `ts_file_upload_user_provided_name` | File stored with user-provided filename | path traversal via name | TS | ❌ |
| `ts_file_upload_no_size_limit` | No file size limit (resource exhaustion) | disk fill | TS | ❌ |
| **`ts_zip_slip`** | Archive extracted to path containing `../` | zip, tar, jar | TS | ❌ |
| `ts_archive_bomb` | Zip bomb not detected before extraction | decompression bomb | TS | ❌ |
| `ts_temp_file_predictable_name` | Temp file `/tmp/upload_${userId}.tmp` — predictable | predictable temp name | TS | ❌ |
| `ts_temp_file_not_cleaned` | Temp file created, never deleted on error | cleanup on error | TS | ❌ |
| `ts_directory_listing_enabled` | Static file server serves directory listing | no index.html, autoindex | TS | ❌ |

---

## Category 109-115 — Web Security Headers (`CWE-346, 352`)
**Target: 120 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_cors_wildcard_credentials`** | `Access-Control-Allow-Origin: *` + `Access-Control-Allow-Credentials: true` | wildcard + credentials | TS | ✅ |
| **`ts_insecure_cors_origin_reflection`** | Origin header reflected without validation | reflect all origins | TS | ✅ |
| `ts_cors_null_origin_accepted` | `Origin: null` accepted | null origin | TS | ❌ |
| **`ts_csrf_token_missing`** | State-changing endpoint has no CSRF token | forms, API | TS | ✅ |
| `ts_csrf_token_weak` | CSRF token is predictable (timestamp-based) | weak entropy | TS | ❌ |
| `ts_csrf_token_not_validated` | CSRF token in request but not checked server-side | validation missing | TS | ❌ |
| **`ts_cookie_security_missing_flags`** | Cookie without HttpOnly, Secure, or SameSite | session cookie | TS | ✅ |
| `ts_security_header_xframe_missing` | Missing X-Frame-Options or CSP frame-ancestors | clickjacking | TS | ❌ |
| `ts_security_header_content_type` | Missing X-Content-Type-Options: nosniff | MIME sniffing | TS | ❌ |
| `ts_security_header_hsts_missing` | HSTS not set on HTTPS response | downgrade attack | TS | ❌ |
| `ts_csp_unsafe_inline` | CSP includes `unsafe-inline` or `unsafe-eval` | CSP bypass | TS | ❌ |
| `ts_referrer_policy_missing` | No Referrer-Policy — URL leaks via Referer | URL leak | TS | ❌ |
| **`ts_ip_spoofing_x_forwarded_for`** | Trust in `X-Forwarded-For` without proxy verification | IP spoofing | TS | ✅ |

---

## Category 116-122 — AI / LLM Specific
**Target: 100 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_prompt_injection_user_input`** | User input directly in system/user prompt | direct injection | TS | ❌ |
| `ts_prompt_injection_document` | RAG document content injected into prompt | indirect via retrieved doc | TS | ❌ |
| `ts_prompt_injection_filename` | Filename from upload injected into prompt | metadata injection | TS | ❌ |
| `ts_llm_output_code_execution` | LLM output `eval()`'d or exec'd | code from model | TS | ❌ |
| `ts_llm_output_sql_executed` | LLM output used in DB query without validation | model-generated SQL | TS | ❌ |
| `ts_llm_output_url_followed` | URL from LLM output fetched (SSRF via LLM) | model-generated URL | TS | ❌ |
| `ts_llm_output_file_path_used` | File path from LLM output passed to fs | path from model | TS | ❌ |
| **`ts_llm_api_key_hardcoded`** | `const OPENAI_KEY = "sk-..."` in source | hardcoded AI key | TS | ✅ |
| `ts_llm_no_token_limit` | LLM call without max_tokens parameter | unbounded cost | TS | ❌ |
| `ts_llm_model_from_user` | Model name taken from user request | model selection bypass | TS | ❌ |
| **`ts_llm_fallback_auth`** | LLM response used to make auth decisions | AI-based gate | TS | ✅ |
| `ts_llm_system_prompt_in_client` | System prompt visible in client-side bundle | prompt exposure | TS | ❌ |
| `ts_agent_tool_no_auth` | Agent tool call not authorized before execution | tool IDOR | TS | ❌ |
| `ts_agent_recursive_call` | Agent output triggers another agent call without limit | infinite loop | TS | ❌ |
| **`ts_llm_any_parameter`** | Function parameters typed as `any` — LLM output flows everywhere | type erasure | TS | ✅ |

---

## Category 123-130 — Cloud / Serverless
**Target: 130 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_kv_unauthenticated_write`** | `env.KV.put()` without auth check | KV write, KV delete | TS | ❌ |
| `ts_r2_unauthenticated_write` | `env.BUCKET.put()` without auth check | R2 write | TS | ❌ |
| `ts_do_unauthenticated` | Durable Object called without validating caller | DO state write | TS | ❌ |
| **`ts_workers_missing_wait_until`** | Long task not wrapped in `ctx.waitUntil()` | async work timeout | TS | ❌ |
| **`ts_module_level_mutable_state`** | Module-level `let state = {}` mutated per-request | isolation bug | TS | ✅ |
| `ts_s3_presigned_url_no_expiry` | Pre-signed URL with very long or no expiry | S3, R2 presigned | TS | ❌ |
| `ts_s3_bucket_public_read` | Bucket ACL set to public-read | S3, GCS, Azure Blob | TS | ❌ |
| `ts_cloud_fn_overprivileged_iam` | Lambda / Cloud Function with admin IAM role | least privilege | TS | ❌ |
| `ts_env_var_not_validated_startup` | Required environment variable used without existence check | missing env | TS | ❌ |
| `ts_secret_in_env_log` | `process.env.SECRET` logged at startup | env var leak | TS | ❌ |
| `ts_cold_start_data_leak` | Data from previous request visible in cold start cache | isolation | TS | ❌ |
| `rust_aws_sdk_no_verify` | AWS SDK with credential exposure | SDK misconfiguration | Rust | ❌ |

---

## Category 131-137 — Event-Driven / Missing Side Effects
**Target: 130 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_missing_event_after_state_change`** | Status updated but domain event not published | order, payment, subscription | TS | ❌ |
| **`ts_missing_ledger_after_wallet_op`** | Wallet operation without ledger record | credit, debit, refund | TS | ❌ |
| `ts_missing_notification_after_refund` | Refund processed, user not notified | email, push, SMS | TS | ❌ |
| **`ts_missing_audit_after_admin_action`** | Admin deletes/modifies without audit log entry | GDPR/SOC2 gap | TS | ❌ |
| **`ts_missing_stock_release_after_cancel`** | Order cancelled without releasing inventory | releaseStock not called | TS | ❌ |
| `ts_missing_cache_invalidation` | DB record updated, related cache not cleared | stale cache | TS | ❌ |
| `ts_missing_session_revoke_on_pw_change` | Password changed but existing sessions not invalidated | stale sessions | TS | ❌ |
| `ts_missing_downstream_cleanup` | User account deleted but related resources remain | cascade delete | TS | ✅ |
| `ts_missing_webhook_event_handling` | Webhook handles only happy-path event, ignores failures | partial handling | TS | ❌ |
| **`ts_missing_webhook_signature`** | Webhook received without HMAC signature verification | any webhook | TS | ✅ |
| **`ts_missing_idempotency_guard`** | Webhook or API handler not idempotent | duplicate processing | TS | ✅ |

---

## Category 138-145 — Validation & Sanitization (`CWE-20`)
**Target: 150 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_csa_validate_unconditional`** | `validate()` always returns `true` regardless of input | CSA pattern | TS | ✅ |
| **`ts_csa_sanitize_passthrough`** | `sanitize(input)` returns input unchanged | CSA pattern | TS | ✅ |
| **`ts_csa_find_never_empty`** | `db.findUser()` result used without null check | CSA pattern | TS | ✅ |
| `ts_validation_email_not_checked` | Email format accepted without validation | any format | TS | ❌ |
| `ts_validation_url_not_checked` | URL accepted without format validation | arbitrary URL | TS | ❌ |
| `ts_validation_integer_range` | Integer accepted without min/max bounds check | quantity, age, amount | TS | ❌ |
| `ts_validation_string_length` | String accepted without length limit | name, bio, comment | TS | ❌ |
| `ts_validation_enum_not_checked` | Enum field accepts any string value | status, role, type | TS | ❌ |
| `ts_type_confusion_string_number` | String ID compared to number ID (loose equality) | == vs === | TS | ❌ |
| `ts_type_confusion_boolean_string` | "false" string treated as truthy | boolean from string | TS | ❌ |
| `ts_integer_overflow_price_calc` | Price × quantity can exceed safe integer | Number.MAX_SAFE_INTEGER | TS | ❌ |
| **`ts_regex_filename_hyphen_drop`** | Character class `[^\s-]` drops hyphenated filenames | regex character class | TS | ✅ |
| `ts_regex_redos_catastrophic` | `(a+)+` style pattern on user input | catastrophic backtracking | TS | ❌ |
| `ts_regex_missing_anchors` | Pattern without `^` and `$` allows partial match | injection via suffix | TS | ❌ |
| `ts_negative_quantity_allowed` | Cart/order allows `quantity: -1` | negative input | TS | ❌ |
| `ts_future_date_for_past_event` | Past event accepts future dates | date validation | TS | ❌ |

---

## Category 146-152 — Third-Party Integration
**Target: 130 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_missing_webhook_signature`** | No HMAC verification on incoming webhook | Stripe, Dodo, GitHub | TS | ✅ |
| `ts_webhook_replay_attack` | Webhook timestamp not checked (replay possible) | timestamp validation | TS | ❌ |
| **`ts_webhook_partial_handling`** | Webhook only handles success event, not failure | incomplete switch | TS | ❌ |
| `ts_payment_amount_from_client` | Payment amount from client request body | Stripe, PayPal amount | TS | ❌ |
| `ts_payment_currency_from_client` | Currency taken from client, not server config | currency injection | TS | ❌ |
| `ts_oauth_token_not_validated` | OAuth access token used without introspection | token passthrough | TS | ❌ |
| **`ts_third_party_token_leak`** | Own service credential sent to external URL | VERCEL_TOKEN to external | TS | ✅ |
| `ts_email_header_injection` | User input in To/Subject without sanitization | nodemailer injection | TS | ❌ |
| `ts_sms_otp_rate_not_limited` | OTP send endpoint not rate-limited | SMS flood | TS | ❌ |
| **`ts_secret_in_cli_args`** | Secret interpolated into shell command string | token in exec() arg | TS | ✅ |
| `ts_api_response_not_validated` | External API response used without schema check | unchecked external data | TS | ❌ |
| `ts_api_timeout_not_set` | External API called without timeout | hanging request | TS | ❌ |

---

## Category 153-157 — Supply Chain
**Target: 50 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_dependency_confusion` | Internal package name without registry scope | @company/pkg not scoped | TS | ❌ |
| `ts_package_version_not_pinned` | `"dependency": "*"` or `"^1.x.x"` in production | unpinned versions | TS | ❌ |
| `ts_no_sri_external_script` | `<script src="cdn">` without integrity attribute | CDN without SRI | TS | ❌ |
| `ts_postinstall_network_call` | `postinstall` script makes network request | supply chain exec | TS | ❌ |
| `ts_lockfile_not_committed` | No package-lock.json or yarn.lock in repo | lock file absent | TS | ❌ |

---

## Category 158-163 — Logging & Monitoring
**Target: 100 corpus pairs**

| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_log_sensitive_credential`** | Auth token or API key in log statement | console.log, logger | TS | ✅ |
| `ts_log_pii` | Email, phone, SSN, or name in log | structured log, format | TS | ❌ |
| `ts_log_credit_card` | Credit card number or CVV in log | payment log | TS | ❌ |
| `ts_log_injection_crlf` | User input with CRLF in log message | log forging | TS | ❌ |
| `ts_missing_login_failure_log` | Failed login attempt not logged | no security log | TS | ❌ |
| `ts_missing_admin_action_log` | Admin action (delete, config change) not logged | audit gap | TS | ❌ |
| `ts_audit_log_deletable` | Audit log records can be deleted by users | tamper possible | TS | ❌ |
| `ts_no_alert_on_repeated_failure` | No monitoring for repeated auth failures | monitoring gap | TS | ❌ |

---

## Category 164-175 — Framework-Specific Patterns
**Target: 250 corpus pairs**

### Express.js
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_express_trust_proxy_missing` | `app.set("trust proxy", true)` not set, X-Forwarded-For untrustworthy | proxy header | TS | ❌ |
| `ts_express_session_secret_weak` | `secret: "keyboard cat"` or short string | express-session | TS | ❌ |
| `ts_express_middleware_order` | Auth middleware registered after route | order bug | TS | ❌ |
| `ts_express_body_size_no_limit` | No body-parser size limit | large payload | TS | ❌ |

### Hono (Cloudflare Workers)
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_hono_kv_unauthenticated`** | `env.KV.put()` in Hono handler without session check | KV write without auth | TS | ❌ |
| `ts_hono_middleware_order` | Middleware applied after route registration | route before auth | TS | ❌ |
| `ts_hono_d1_template_sqli` | D1 prepare with template literal | `env.DB.prepare(\`...\`)` | TS | ❌ |
| `ts_hono_error_handler_missing` | No global error handler in Hono app | unhandled throws | TS | ❌ |

### Next.js
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_nextjs_server_action_no_auth` | Server action mutates data without auth | form action | TS | ❌ |
| `ts_nextjs_api_route_no_auth` | API route handler has no authentication | pages/api/ | TS | ❌ |
| `ts_nextjs_redirect_external` | `redirect()` to user-controlled URL | arbitrary redirect | TS | ❌ |
| `ts_nextjs_env_client_side` | `NEXT_PUBLIC_SECRET_KEY` exposes secret | public env var | TS | ❌ |
| `ts_nextjs_getserversideprops_leak` | `getServerSideProps` returns sensitive server data to props | data leak to client | TS | ❌ |

### NestJS
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_nestjs_guard_missing` | Controller route without `@UseGuards()` decorator | unprotected route | TS | ❌ |
| `ts_nestjs_dto_no_validation` | DTO class without `@IsString()`, `@IsEmail()` decorators | unvalidated body | TS | ❌ |
| `ts_nestjs_interceptor_data_leak` | Interceptor serializes entity including sensitive fields | response filter | TS | ❌ |

### Prisma ORM
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_prisma_query_raw_unsafe` | `prisma.$queryRawUnsafe(userSql)` | all variants | TS | ❌ |
| **`ts_prisma_findmany_no_pagination`** | `prisma.*.findMany()` without `take` limit | unbounded query | TS | ❌ |
| `ts_prisma_upsert_race` | `upsert` on non-unique field can create duplicates | no unique constraint | TS | ❌ |

### Sequelize
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_sequelize_insecure_setter`** | `Model.update(req.body, { where: ... })` — mass assignment | unfiltered body | TS | ✅ |
| **`ts_sequelize_missing_bounds`** | Query without LIMIT on potentially large table | full table scan | TS | ✅ |
| `ts_sequelize_computed_where_key` | `where: { [req.body.field]: val }` — operator injection | computed key | TS | ❌ |

### Mongoose / MongoDB Driver
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| `ts_mongoose_query_selector_injection` | `User.find(req.body)` — body treated as full query | object spread | TS | ❌ |
| `ts_mongoose_populate_path_injection` | `.populate(req.query.path)` | path from user | TS | ❌ |
| `ts_mongo_dollar_where` | `collection.find({ $where: userFn })` | $where | TS | ⚠️ |

### React / JSX
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`tsx_dangerously_set_inner_html`** | `dangerouslySetInnerHTML={{ __html: userContent }}` | direct, via prop | TS | ✅ |
| **`tsx_undefined_api_property`** | Access to property that doesn't exist on API response | null crash | TS | ✅ |
| `tsx_xss_href_javascript` | `<a href={userInput}>` without protocol validation | javascript: protocol | TS | ❌ |
| `tsx_useeffect_missing_dependency` | `useEffect` dependency array incomplete — stale closure | stale state | TS | ❌ |

### Socket.io / WebSocket
| ID | Name | Mutations | Status |
|----|------|-----------|--------|
| **`ts_socket_unvalidated_payload`** | `socket.on("event", (data) => db.query(data.id))` | payload not validated | TS | ✅ |
| `ts_socket_missing_auth` | WebSocket handler processes messages without auth check | unauthenticated WS | TS | ❌ |
| `ts_socket_broadcast_data_leak` | `io.emit(event, sensitiveData)` broadcasts to all clients | over-broadcast | TS | ❌ |

---

## Mutation Matrix

Every base pattern above should be written in these mutation variants to maximise generalisation:

| Mutation Class | Description | Example |
|---|---|---|
| **M1: Direct** | Source flows directly to sink | `exec(req.body.cmd)` |
| **M2: Intermediate Variable** | 1 hop through named variable | `const cmd = req.body.cmd; exec(cmd)` |
| **M3: Multi-hop** | 2–3 hops through multiple assignments | `const a = req.body.cmd; const b = a; exec(b)` |
| **M4: Via Helper Function** | Taint flows through a called function | `const cmd = getCommand(req); exec(cmd)` |
| **M5: Via Template Literal** | Taint via string interpolation | `` exec(`run ${req.body.cmd}`) `` |
| **M6: Via Concatenation** | Taint via `+` concatenation | `exec("run " + req.body.cmd)` |
| **M7: Via Destructuring** | Taint from destructured object | `const { cmd } = req.body; exec(cmd)` |
| **M8: Via Array** | Taint through array element | `exec(req.body.cmds[0])` |
| **M9: Via Object Property** | Taint through object property | `exec(config.cmd)` where `config = req.body` |
| **M10: Async Path** | Taint across await boundary | `const cmd = await getCmd(req); exec(cmd)` |
| **M11: Conditional Path** | Taint on one branch of conditional | `if (x) exec(req.body.cmd)` |
| **M12: Try-Catch Wrap** | Bug inside try block | `try { exec(userInput) } catch {}` |
| **M13: Different Framework Syntax** | Same bug via different API | `c.req.query("cmd")` vs `req.query.cmd` |
| **M14: Renamed Variables** | Different variable names for same pattern | `userCmd`, `shellInput`, `rawCommand` |
| **M15: Async/Await vs .then()** | Same taint via promise chain | `.then(body => exec(body.cmd))` |

**Priority for writing:** Write M1, M2, M5, M7 first (covers ~80% of real cases). Add M3, M4, M6 for depth. M8–M15 are long-tail coverage.

---

## Language Coverage Matrix

| Bug Category | TypeScript | Rust | JavaScript | Go | Python |
|---|---|---|---|---|---|
| SQL Injection | 🎯 Primary | 🎯 Primary | ⚠️ Shared with TS | 🗂️ Backlog | 🗂️ Backlog |
| Command Injection | 🎯 Primary | 🎯 Primary | ⚠️ Shared | 🗂️ Backlog | 🗂️ Backlog |
| SSRF | 🎯 Primary | 🎯 Primary | ⚠️ Shared | 🗂️ Backlog | 🗂️ Backlog |
| Auth/IDOR | 🎯 Primary | ⚠️ Secondary | ⚠️ Shared | 🗂️ Backlog | 🗂️ Backlog |
| Memory Safety | — | 🎯 Primary | — | 🗂️ Backlog | — |
| Business Logic | 🎯 Primary | ⚠️ Secondary | ⚠️ Shared | — | — |
| Race Conditions | 🎯 Primary | 🎯 Primary | ⚠️ Shared | 🗂️ Backlog | — |
| XSS | 🎯 Primary | — | 🎯 Primary | — | — |
| Deserialization | 🎯 Primary | 🎯 Primary | ⚠️ Shared | — | 🗂️ Backlog |
| AI/LLM | 🎯 Primary | ⚠️ Secondary | ⚠️ Shared | — | 🗂️ Backlog |
| Cloud/Serverless | 🎯 Primary (CF Workers) | 🎯 Primary (Axum) | ⚠️ Shared | — | — |

**Build TS and Rust first.** These two languages cover the primary audited codebases and will deliver the most detection value. JS shares the TS AST grammar so TS patterns generalise there automatically. Go and Python are phase-2.

---

## Cumulative Count

| Category Group | Patterns Targeted | Currently in Corpus | Gap |
|---|---|---|---|
| Injection (1–10) | 370 | ~15 | 355 |
| XSS (11–15) | 120 | 3 | 117 |
| SSRF (16–18) | 80 | 3 | 77 |
| Path Traversal (19–21) | 80 | 3 | 77 |
| Auth & Session (22–30) | 200 | 12 | 188 |
| Authorization & IDOR (31–40) | 250 | 22 | 228 |
| Business Logic (41–55) | 350 | 10 | 340 |
| Race Conditions (56–62) | 150 | 5 | 145 |
| Cryptography (63–70) | 150 | 5 | 145 |
| Memory Safety Rust (71–80) | 150 | 14 | 136 |
| Async/Promise (81–88) | 120 | 8 | 112 |
| Information Disclosure (89–95) | 120 | 5 | 115 |
| Deserialization (96–100) | 80 | 4 | 76 |
| File Operations (101–108) | 120 | 2 | 118 |
| Web Headers (109–115) | 120 | 8 | 112 |
| AI/LLM (116–122) | 100 | 7 | 93 |
| Cloud/Serverless (123–130) | 130 | 3 | 127 |
| Event-Driven (131–137) | 130 | 4 | 126 |
| Validation (138–145) | 150 | 8 | 142 |
| Third-Party (146–152) | 130 | 6 | 124 |
| Supply Chain (153–157) | 50 | 0 | 50 |
| Logging (158–163) | 100 | 3 | 97 |
| Framework-Specific (164–175) | 250 | 8 | 242 |
| **Total** | **~4,050** | **~115** | **~3,935** |

---

## Writing Priority (First 500 Corpus Pairs)

Write in this order to maximise detection value per pair written:

**Batch 1 — High-value security, already partially covered (100 pairs):**
Complete the SQL injection family, IDOR family, auth bypass family, command injection family. Write M1–M5 mutations of each existing pattern.

**Batch 2 — Business logic (100 pairs):**
State machine bypass variants, payment gate variants, missing side-effect patterns (ledger, event, stock). These patterns come from real audit findings and have zero coverage.

**Batch 3 — Framework-specific Hono + D1 + KV (75 pairs):**
All Cloudflare Workers-specific patterns: KV/R2/DO unauthenticated writes, D1 SQL injection, module state leak. Directly targets the audited codebase stack.

**Batch 4 — Race conditions and idempotency (75 pairs):**
TOCTOU patterns, idempotency-check-outside-transaction, credit double-spend variants. High precision, directly observed in audit findings.

**Batch 5 — XSS, SSRF, Path Traversal depth (75 pairs):**
Add framework-specific variants for existing categories. React XSS, Next.js SSRF, Hono path traversal.

**Batch 6 — AI/LLM and Cloud (75 pairs):**
Prompt injection variants, LLM output trust patterns, cloud IAM patterns. Growing attack surface.

After 500 pairs: re-benchmark against the deliberate vulnerability apps. Calibrate which categories still have high FP rates and add negatives targeting those specifically before continuing to Batches 7–10.
