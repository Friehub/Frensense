# Next Session Tasks

## 1. Rebuild Bundle (wait for background process)
```bash
# Check if done:
tail -f /tmp/bundle_build_final.log

# When done, rebuild binary:
touch src/bin/frensense.rs && cargo build --bin frensense

# Benchmark:
./target/debug/frensense "/home/oxisrael/Friehub/Taas/benchmarks/NodeGoat" --threshold 0.5
```

## 2. Migrate Patterns to by-vuln/ Structure
Move existing high-quality patterns from `route-handlers/` into `by-vuln/`:
- `ts_ssrf_fetch_direct` → `by-vuln/ssrf/node-fetch/`
- `ts_sqli_concat_direct` → `by-vuln/sqli/pg/`
- `ts_cmdi_exec_direct` → `by-vuln/cmdi/exec/`
- `ts_xss_reflected_response` → `by-vuln/xss/template/`
- `ts_open_redirect` → `by-vuln/open-redirect/express/`
- `ts_cmdi_exec_shell` → `by-vuln/cmdi/execfile/`

## 3. Create Missing API Variants (Priority Order)

### CMDI — need `spawn` + `promisified` variants
- `by-vuln/cmdi/spawn/ts_cmdi_spawn_positive.ts` (child_process.spawn)
- `by-vuln/cmdi/promisified/ts_cmdi_promisified_positive.ts` (util.promisify(exec))

### XSS — need `innerhtml` + `react` variants
- `by-vuln/xss/innerhtml/ts_xss_innerhtml_positive.ts` (element.innerHTML = userInput)
- `by-vuln/xss/react/ts_xss_react_positive.ts` (dangerouslySetInnerHTML)

### Path Traversal — need `readfile` + `join` variants
- `by-vuln/path-traversal/readfile/ts_path_readfile_positive.ts` (fs.readFile)
- `by-vuln/path-traversal/join/ts_path_join_positive.ts` (path.join)

### Open Redirect — need `nextjs` variant
- `by-vuln/open-redirect/nextjs/ts_nextjs_redirect_positive.ts` (Next.js redirect)

## 4. Run OWASP Benchmark
```bash
git clone https://github.com/OWASP/Benchmark
frensense ./Benchmark --corpus corpus/targets/ --threshold 0.5 --sarif
```

## 5. Run Corpus Quality Score
```bash
cargo run --bin corpus-quality -- corpus/targets/ | tee quality_report.tsv
# Focus on patterns scoring <50
```

## 6. Track TP/FP Rate
| Pattern Category | TP | FP | Notes |
|-----------------|----|----|-------|
| Open Redirect | 1 | 0 | index.js:70 confirmed TP |
| Security Headers | 2 | 0 | helmet disabled |
| Swig XSS | 2 | 0 | NodeGoat-specific |
| Role checks | 0 | 2 | FP — needs fix |
