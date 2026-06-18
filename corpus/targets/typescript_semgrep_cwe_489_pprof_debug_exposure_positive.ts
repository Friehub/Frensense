// Vulnerable: The profiling 'pprof' endpoint is automatically exposed on /debug/pprof. This could leak information about the server. Instead, use `import "net/http/pprof"`. See https://www.farsightsecurity.com/blog/txt-record/go-remote-profiling-20161028/ for more information and mitigation.
// Pattern: http.ListenAndServe(...)
function vulnerable() {
  // TODO: implement pattern match
}
