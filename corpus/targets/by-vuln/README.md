# Corpus Organization: by Vulnerability → API Version

This directory organizes corpus patterns by **vulnerability class**, then by **API version**.
This structure teaches the auto-filter to generalize across different APIs for the same bug.

## Structure

```
by-vuln/
├── ssrf/                        # Server-Side Request Forgery (CWE-918)
│   ├── node-fetch/              #   modern: import fetch from "node-fetch"
│   │   ├── *_positive.ts
│   │   └── *_negative.ts
│   ├── request/                 #   legacy: require("request")
│   ├── axios/                   #   modern: import axios from "axios"
│   ├── http-get/                #   legacy: require("http").get
│   └── got/                     #   modern: import got from "got"
├── sqli/                        # SQL Injection (CWE-89)
│   ├── pg/                      #   npm pg Pool
│   ├── mysql2/                  #   mysql2 driver
│   ├── sequelize/               #   Sequelize ORM
│   ├── prisma/                  #   Prisma ORM
│   └── mongo/                   #   MongoDB/Mongoose
├── cmdi/                        # Command Injection (CWE-78)
│   ├── exec/                    #   child_process.exec
│   ├── execfile/                #   child_process.execFile (safe)
│   ├── spawn/                   #   child_process.spawn
│   └── promisified/             #   util.promisify + exec
├── xss/                         # Cross-Site Scripting (CWE-79)
│   ├── innerhtml/               #   element.innerHTML =
│   ├── react/                   #   dangerouslySetInnerHTML
│   └── template/                #   template string injection
├── path-traversal/              # Path Traversal (CWE-22)
│   ├── readfile/                #   fs.readFile
│   ├── writefile/               #   fs.writeFile
│   └── join/                    #   path.join
├── open-redirect/               # Open Redirect (CWE-601)
│   ├── express/                 #   res.redirect
│   └── nextjs/                  #   next redirect
├── eval/                        # Code Injection (CWE-95)
│   ├── direct/                  #   eval()
│   ├── vm/                      #   vm.runInContext
│   └── setTimeout/              #   setTimeout(string)
└── injection/                   # Other Injection (various CWEs)
    ├── ldap/                    # LDAP Injection (CWE-90)
    ├── xpath/                   # XPath Injection (CWE-643)
    ├── ssti/                    # Template Injection (CWE-1336)
    └── nosqli/                  # NoSQL Injection (CWE-943)
```

## How to Add a Multi-API Pattern

1. Choose a vulnerability from the list above
2. Pick an API version not yet covered (e.g., `got` for SSRF)
3. Create: `{vuln}/{api}/{name}_positive.ts`
4. Create: `{vuln}/{api}/{name}_negative.ts`
5. Create: `{vuln}/{api}/{name}_negative2.ts` (alternate fix)
6. Ensure the `[frensense]` block has `cwe:`, `cvss:`, `owasp:`, `runtime_probe:`
7. Run `corpus-quality` to verify score

## Why This Works

The auto-filter learns `contains_call_to` by finding calls present in most positives
and absent from most negatives. When SSRF has 5 API variants (`fetch`, `request`,
`axios`, `got`, `http.get`), no single API reaches 100% frequency. The category-level
constraint relaxes from "requires `fetch`" to "requires one of these", letting
the scorer generalize to codebases using any of them.

## Existing Files

High-quality patterns also exist in `../route-handlers/`, `../config/`, and `../`.
Eventually all patterns should migrate into `by-vuln/`.
