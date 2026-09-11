# frensense-runtime: Runtime Verification (WIP)

> **⚠️ Status: Unfinished / Work In Progress**

`frensense-runtime` is an experimental layer designed to close the gap between Static Application Security Testing (SAST) and Dynamic Application Security Testing (DAST) via **Corpus-Driven Runtime Verification**.

## The Vision

Static analysis inherently suffers from false positives (e.g., tracing a taint path that looks exploitable but is mitigated by an unmodeled sanitize function or environmental block). 

`frensense-runtime` is built to take high-confidence SAST findings from the `frensense-engine` and attempt to actively confirm them in a live or staging environment.

### Planned Capabilities
- **Automated Payload Generation:** Uses the vulnerability classification (e.g., SQLi, XSS) from the `.frc` corpus to generate targeted, harmless test payloads.
- **Active Probing:** Sends HTTP/WebSocket requests tracing the identified static entrypoints (sources) to trigger the vulnerable sink.
- **Verification Feedback Loop:** If the exploit succeeds, the finding is promoted to `CRITICAL` with a 100% True Positive guarantee. If it fails, the finding is demoted or flagged as heavily mitigated.

*Note: This crate is currently in early development and is not yet integrated into the main `frensense` CLI workflow.*
