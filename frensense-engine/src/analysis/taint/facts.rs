// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Task 4.3 completion: **sink signatures**, per-API argument-position facts.
//!
//! A blanket "every argument of a sink is dangerous" model causes the largest
//! FP class in the e2e report: `pool.query(sql, [email])` (parameterized,
//! safe) alerts because the tainted value sits in arg 1, the *safe* channel.
//!
//! A [`SinkSignature`] fixes this without engine surgery: it lists which
//! argument slots of a named call are dangerous, and whether a *binding*
//! parameter array (the parameterized-query safe channel) must be absent for
//! the call to be dangerous.
//!
//! Signatures come from the **fact tables** (`frensense-lang` built-ins merged
//! with bundler-extracted learned facts via a [`FactTable`]). The engine only
//! consumes them; updating a framework never touches engine code.

use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;

use crate::analysis::taint::config::TaintConfig;

/// Per-argument classification for one sink API.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SinkSignature {
    /// Call name matched against `CallStatic.func` / `CallVirtual.method`
    /// (last segment of a member chain).
    pub call: String,
    /// Argument slots that are dangerous when tainted. Empty = every slot.
    pub dangerous_args: BTreeSet<usize>,
    /// When `true`, a tainted argument in a *non-dangerous* slot (e.g. the
    /// params array of a parameterized query) is treated as **sanitized**
    /// (the API's binding channel makes it safe), not just ignored. Without
    /// this, `query(sql, [tainted])` silently degrades to no-op instead of a
    /// definite "safe" verdict, same outcome for alerts, different for
    /// verdicts/statistics.
    pub binding_args_safe: bool,
    /// Object-literal keys whose presence in a tainted argument marks it as
    /// an IDOR-class *object payload* (e.g. `{ where: { id: ... } }` reaching
    /// `findOne`) rather than a raw injection string. Empty for sinks whose
    /// dangerous shape is always a raw value.
    pub idor_keys: Vec<String>,
    /// What this sink does with its input (drives severity ranking).
    pub role: crate::analysis::taint::role::SinkRole,
}

impl SinkSignature {
    /// A signature where every argument is dangerous (the legacy default).
    pub fn all_args(call: &str) -> Self {
        Self {
            call: call.to_string(),
            dangerous_args: BTreeSet::new(),
            binding_args_safe: false,
            idor_keys: Vec::new(),
            role: crate::analysis::taint::role::SinkRole::Other,
        }
    }

    /// A signature with explicit dangerous slots.
    pub fn with_args(call: &str, slots: &[usize]) -> Self {
        Self {
            call: call.to_string(),
            dangerous_args: slots.iter().copied().collect(),
            binding_args_safe: false,
            idor_keys: Vec::new(),
            role: crate::analysis::taint::role::SinkRole::Other,
        }
    }

    /// Is argument slot `slot` dangerous when tainted?
    pub fn is_dangerous(&self, slot: usize) -> bool {
        self.dangerous_args.is_empty() || self.dangerous_args.contains(&slot)
    }

    /// Is this slot an explicitly-safe binding channel?
    pub fn is_binding(&self, slot: usize) -> bool {
        self.binding_args_safe && !self.is_dangerous(slot)
    }
}

/// One learned/built-in sanitizer fact: a call that neutralizes taint.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct SanitizerFact {
    pub call: String,
    /// Per-kind strength label (advisory; the engine treats all kinds as cut).
    /// e.g. "encode", "validate", "allowlist", "parameterize".
    pub kind: String,
    /// Argument slots whose taint is removed. Empty = all arguments.
    pub sanitizes_args: BTreeSet<usize>,
    /// If `true`, a *guard-style* sanitizer: the call only protects values on
    /// paths where it returned true / threw (e.g. `SAFE_RE.test(x)`).
    /// The engine currently treats guard-style like an ordinary sanitizer on
    /// the explored branch (sound under flow-insensitive reading of the
    /// branch).
    pub guard_style: bool,
}

/// One corpus-verified non-dataflow check, installed by a `.frc` bundle.
///
/// This is the learned counterpart of the built-in seed checks in
/// `crate::checks`: the engine supplies the *mechanism* (call matching over
/// the lowered IR), the bundle supplies the *conclusion* (which call is a
/// violation, under which rule, with what advisory text). A rule fires when
/// the function contains a call whose last segment matches `call`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct LearnedCheckFact {
    /// Stable rule id, e.g. `"redirect_substring_guard"`.
    pub rule: String,
    /// Trigger: a call whose last segment matches this name fires the rule.
    pub call: String,
    /// Advisory text shown to the user (bundle-authored).
    pub message: String,
    /// Advisory severity hint: "warning" or "critical".
    pub severity: String,
    /// Optional guard qualification (bundle-authored): the rule only fires
    /// when NO call whose last segment matches this name appears in the
    /// same function. This expresses "trigger without enforcement", e.g.
    /// a privileged tool action executed without its policy-check helper.
    /// `None` = plain presence trigger.
    #[serde(default)]
    pub unless_guard: Option<String>,
    /// Optional range-check qualification (bundle-authored): the rule only
    /// fires when the trigger's guarded argument is NOT compared against a
    /// literal bound in the function. A negative enforcing policy inline
    /// (`if (discount < 0 || discount > MAX) return …`) contains a
    /// comparison of the guarded var against a literal; the positive has
    /// none. This expresses enforcement without requiring a named helper.
    /// The value is the operator set accepted as a bound check
    /// (e.g. `["<", ">", "<=", ">="]`); `None` = no range qualification.
    #[serde(default)]
    pub unless_range_check: Option<Vec<String>>,
}

/// The merged fact table: built-in language tables + bundle-learned facts.
///
/// Built from a [`TaintConfig`] (name sets, backward compatible) plus
/// optional signatures/sanitizer facts supplied by `frensense-lang` specs or
/// a `.frc` bundle.
#[derive(Debug, Clone, Default)]
pub struct FactTable {
    /// By call name. Absent = default "all args dangerous" for configured
    /// sinks.
    pub sink_signatures: FxHashMap<String, SinkSignature>,
    pub sanitizer_facts: FxHashMap<String, SanitizerFact>,
    /// Corpus-verified non-dataflow checks (order-preserving; dedup on
    /// `(rule, call)` at merge time).
    pub learned_checks: Vec<LearnedCheckFact>,
    /// Last segments that come ONLY from dotted client sinks (`got.get`,
    /// `axios.post`, ...). These verbs are ambiguous, `Map.get`, router
    /// `app.post`, LRU `.put` all share the names, so they match
    /// receiver-aware: the call only counts as a sink when the receiver's
    /// root is a known client (see [`client_roots`]).
    pub verb_sinks: FxHashSet<String>,
    /// Receiver roots that make [`verb_sinks`] real client calls, derived
    /// from the first segment of dotted verb sinks (`got`, `axios`, `http`).
    pub client_roots: FxHashSet<String>,
    /// Receiver-specific roles for dotted sinks that share their last
    /// segment with a different-role bare entry:
    /// `(receiver_root, method) -> role`.
    ///
    /// Example: `KVNamespace.put` is StorageWrite while bare `run` is
    /// SqlInjection; `shell.run` is CommandInjection. When the engine sees
    /// `kv.put(...)` and the receiver root resolves to `kv`'s declared
    /// dotted root, THIS role wins over the bare entry's, the receiver is
    /// the disambiguator, exactly like [`verb_sinks`] for match/no-match.
    pub receiver_roles: FxHashMap<(String, String), crate::analysis::taint::role::SinkRole>,
    /// Receiver roots of trusted session stores (`authenticatedUsers`):
    /// `store.get(token)` returns a server-issued session object or
    /// undefined, values read off the result are not attacker-controlled
    /// (see [`SanitizerKind::SessionTrust`]).
    pub session_roots: FxHashSet<String>,
}

impl FactTable {
    /// Build from a plain [`TaintConfig`]: every configured sink gets the
    /// all-args signature; every configured sanitizer a default fact.
    pub fn from_config(config: &TaintConfig) -> Self {
        let mut t = Self::default();
        for s in &config.sinks {
            t.sink_signatures
                .insert(s.clone(), SinkSignature::all_args(s));
        }
        for s in &config.sanitizers {
            t.sanitizer_facts.insert(
                s.clone(),
                SanitizerFact {
                    call: s.clone(),
                    kind: "encode".into(),
                    sanitizes_args: Default::default(),
                    guard_style: false,
                },
            );
        }
        t
    }

    /// Merge bundle-learned facts over the current table (learned wins on
    /// name collision, bundle facts are more specific than built-ins).
    ///
    /// All-args signatures never overwrite slot-restricted ones on collision:
    /// a slot-restricted signature is strictly more specific knowledge, so an
    /// all-args merge (e.g. another language spec declaring the same bare
    /// call name) must not widen it back to "everything dangerous".
    pub fn merge(&mut self, other: &FactTable) {
        for (k, v) in &other.sink_signatures {
            let widens = v.dangerous_args.is_empty()
                && !v.binding_args_safe
                && self
                    .sink_signatures
                    .get(k)
                    .map(|cur| !cur.dangerous_args.is_empty() || cur.binding_args_safe)
                    .unwrap_or(false);
            if widens {
                continue;
            }
            self.sink_signatures.insert(k.clone(), v.clone());
        }
        for (k, v) in &other.sanitizer_facts {
            self.sanitizer_facts.insert(k.clone(), v.clone());
        }
        // Learned checks accumulate: a bundle may install many rules, and
        // merging a second bundle must not drop the first's. Dedup on
        // (rule, call): the same fact arriving twice (shared dependencies,
        // re-merge) must not duplicate findings.
        for c in &other.learned_checks {
            if !self
                .learned_checks
                .iter()
                .any(|e| e.rule == c.rule && e.call == c.call)
            {
                self.learned_checks.push(c.clone());
            }
        }
        // Verb-sink bookkeeping accumulates too: dotted client entries
        // (`got.get`, `axios.post`) from any merged spec/bundle widen the
        // receiver-aware sets.
        self.verb_sinks.extend(other.verb_sinks.iter().cloned());
        self.client_roots.extend(other.client_roots.iter().cloned());
        // Receiver-specific roles accumulate like the other receiver-aware
        // sets: two merged specs can declare different dotted roots for the
        // same method, and both disambiguation rules must survive.
        for (k, v) in &other.receiver_roles {
            self.receiver_roles.entry(k.clone()).or_insert(*v);
        }
        self.session_roots
            .extend(other.session_roots.iter().cloned());
    }

    /// Is `last(receiver_root)` a trusted session-store accessor?
    /// Receiver-aware like [`Self::is_sink_call`]: `authenticatedUsers.get(t)`
    /// is a session read; `myMap.get(t)` is not.
    pub fn is_session_accessor(&self, last: &str, receiver_root: Option<&str>) -> bool {
        self.sanitizer_facts
            .get(last)
            .map(|f| f.kind == "session")
            .unwrap_or(false)
            || match receiver_root {
                Some(root) => self.session_roots.contains(root) && last == "get",
                None => false,
            }
    }

    /// Session-accessor check over the receiver's full access path.
    /// Session stores are usually namespaced (`security.authenticatedUsers`),
    /// so ANY dotted segment matching a declared root qualifies:
    /// `security.authenticatedUsers.get(t)` hits root `authenticatedUsers`.
    pub fn is_session_path(&self, last: &str, receiver_path: Option<&str>) -> bool {
        if last != "get" {
            return false;
        }
        let Some(path) = receiver_path else {
            return false;
        };
        path.split('.').any(|seg| self.session_roots.contains(seg))
    }

    /// Resolve the receiver's full member-access path from the IR by
    /// walking LoadField defs and joining named segments:
    /// `security.authenticatedUsers.get(...)` →
    /// `Some("security.authenticatedUsers")`.
    ///
    /// Mirrors [`FactTable::receiver_root`] but keeps every named hop, so
    /// namespaced session stores match their declared root segment.
    pub fn receiver_access_path(
        ir: &crate::ir::function::FunctionIR,
        var: crate::ir::function::VarId,
    ) -> Option<String> {
        let mut segments: Vec<String> = Vec::new();
        let mut cur = var;
        for _ in 0..8 {
            if let Some(meta) = ir.var_metadata.get(&cur)
                && let Some(name) = &meta.source_name
            {
                segments.push(name.clone());
                break;
            }
            // Find the LoadField that defines `cur`, continue through its base.
            let mut found = None;
            'outer: for b in ir.blocks.values() {
                for instr in b.instructions.iter() {
                    if let crate::ir::function::Instruction::LoadField {
                        dest, base, field, ..
                    } = instr
                        && *dest == cur
                    {
                        segments.push(field.clone());
                        found = Some(*base);
                        break 'outer;
                    }
                }
            }
            cur = found?;
        }
        if segments.is_empty() {
            return None;
        }
        segments.reverse();
        Some(segments.join("."))
    }

    /// Signature for a sink call; `None` if the call is not a configured sink.
    pub fn sink_signature(&self, call: &str) -> Option<&SinkSignature> {
        self.sink_signatures.get(call)
    }

    /// Role for a sink call, receiver-aware.
    ///
    /// When the receiver root matches a dotted entry's declared root (e.g.
    /// `KVNamespace` for `kv.put(t)`), that entry's role wins over the bare
    /// last-segment entry's. Falls back to [`Self::sink_signature`] when no
    /// receiver-specific role exists.
    pub fn role_for_call(
        &self,
        last: &str,
        receiver_root: Option<&str>,
    ) -> Option<crate::analysis::taint::role::SinkRole> {
        if let Some(root) = receiver_root
            && let Some(role) = self
                .receiver_roles
                .get(&(root.to_string(), last.to_string()))
        {
            return Some(*role);
        }
        self.sink_signature(last).map(|s| s.role)
    }

    /// Sanitizer fact for a call; `None` if not a sanitizer.
    pub fn sanitizer_fact(&self, call: &str) -> Option<&SanitizerFact> {
        self.sanitizer_facts.get(call)
    }

    /// Receiver-aware sink check for virtual calls. `last` is the method
    /// name; `receiver_root` the first segment of the receiver's access
    /// path (e.g. `got` for `got.get(url)`), `None` when unresolvable.
    ///
    /// - Non-verb names match as before (name-configured sinks).
    /// - Verb names (`get`, `post`, ...) only match when they were sourced
    ///   from dotted client entries AND the receiver root is a known
    ///   client, `m.get(t)` on an arbitrary Map is never a sink, while
    ///   `got.get(taint)` is.
    pub fn is_sink_call(&self, last: &str, receiver_root: Option<&str>) -> bool {
        if !self.verb_sinks.contains(last) {
            // Ordinary sink: presence in the signature table decides.
            return self.sink_signatures.contains_key(last);
        }
        match receiver_root {
            Some(root) => self.client_roots.contains(root),
            // Unresolvable receiver: over-approximate (sound, noisier).
            None => true,
        }
    }

    /// Walk a receiver var to the root of its member-access chain via the
    /// IR: `const c = got; got.get(x)` and direct `got.get(x)` both
    /// resolve to root `got`. Returns `None` when the def is not a
    /// LoadField chain grounded in a named var.
    pub fn receiver_root(
        ir: &crate::ir::function::FunctionIR,
        var: crate::ir::function::VarId,
    ) -> Option<String> {
        let mut cur = var;
        for _ in 0..8 {
            if let Some(meta) = ir.var_metadata.get(&cur)
                && let Some(name) = &meta.source_name
            {
                return Some(name.split('.').next().unwrap_or(name).to_string());
            }
            // Find the LoadField that defines `cur`, continue through its base.
            let mut found = None;
            'outer: for b in ir.blocks.values() {
                for instr in b.instructions.iter() {
                    if let crate::ir::function::Instruction::LoadField { dest, base, .. } = instr
                        && *dest == cur
                    {
                        found = Some(*base);
                        break 'outer;
                    }
                }
            }
            cur = found?;
        }
        None
    }

    /// Learned checks whose trigger call's last segment matches `call`.
    /// The engine matches calls by last segment, so a bundle rule written
    /// for `security.hash` also fires on bare `hash`, same over-approximate
    /// semantics as built-in seed checks.
    pub fn learned_checks_for(&self, call: &str) -> Vec<&LearnedCheckFact> {
        let seg = call.rsplit('.').next().unwrap_or(call);
        self.learned_checks
            .iter()
            .filter(|c| c.call.rsplit('.').next() == Some(seg))
            .collect()
    }
}

/// One learned fact as persisted in a `.frc` bundle. A tagged union over the
/// two fact kinds, serde-friendly, decodable into a [`FactTable`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum LearnedFactEntry {
    Sink {
        call: String,
        /// Dangerous argument slots; empty = all slots dangerous.
        dangerous_args: BTreeSet<usize>,
        /// Whether non-dangerous slots are safe binding channels.
        binding_args_safe: bool,
    },
    Sanitizer {
        call: String,
        kind: String,
        guard_style: bool,
    },
    /// Install a corpus-verified non-dataflow check.
    Check {
        rule: String,
        call: String,
        message: String,
        severity: String,
        /// Fire only when this guard call is absent from the function
        /// ("trigger without enforcement"). `None` = presence-only.
        #[serde(default)]
        unless_guard: Option<String>,
        /// Fire only when the trigger's argument is NOT compared against a
        /// literal bound with one of these operators (inline enforcement).
        /// `None` = no range qualification.
        #[serde(default)]
        unless_range_check: Option<Vec<String>>,
    },
}

impl LearnedFactEntry {
    /// Insert this fact into a [`FactTable`].
    pub fn apply(&self, table: &mut FactTable) {
        match self {
            LearnedFactEntry::Sink {
                call,
                dangerous_args,
                binding_args_safe,
            } => {
                // Preserve a spec-derived role (and IDOR keys) if the built-in
                // table already classifies this call, learned facts refine
                // *slots*, they don't reclassify *semantics*.
                let prior = table.sink_signatures.get(call);
                let role = prior
                    .map(|s| s.role)
                    .unwrap_or(crate::analysis::taint::role::SinkRole::Other);
                let idor_keys = prior.map(|s| s.idor_keys.clone()).unwrap_or_default();
                table.sink_signatures.insert(
                    call.clone(),
                    SinkSignature {
                        call: call.clone(),
                        dangerous_args: dangerous_args.clone(),
                        binding_args_safe: *binding_args_safe,
                        idor_keys,
                        role,
                    },
                );
            }
            LearnedFactEntry::Sanitizer {
                call,
                kind,
                guard_style,
            } => {
                table.sanitizer_facts.insert(
                    call.clone(),
                    SanitizerFact {
                        call: call.clone(),
                        kind: kind.clone(),
                        sanitizes_args: Default::default(),
                        guard_style: *guard_style,
                    },
                );
            }
            LearnedFactEntry::Check {
                rule,
                call,
                message,
                severity,
                unless_guard,
                unless_range_check,
            } => {
                let fact = LearnedCheckFact {
                    rule: rule.clone(),
                    call: call.clone(),
                    message: message.clone(),
                    severity: severity.clone(),
                    unless_guard: unless_guard.clone(),
                    unless_range_check: unless_range_check.clone(),
                };
                if !table
                    .learned_checks
                    .iter()
                    .any(|e| e.rule == fact.rule && e.call == fact.call)
                {
                    table.learned_checks.push(fact);
                }
            }
        }
    }
}

/// Build a [`FactTable`] from a list of learned bundle facts.
pub fn fact_table_from_entries(entries: &[LearnedFactEntry]) -> FactTable {
    let mut t = FactTable::default();
    for e in entries {
        e.apply(&mut t);
    }
    t
}

/// Build a [`FactTable`] from a `frensense-lang` [`LanguageSpec`], the
/// built-in per-language knowledge (known sinks, sanitizers).
///
/// Every known sink name becomes an all-args signature; every sanitizer the
/// spec classifies becomes a [`SanitizerFact`]. Bundle-learned facts can then
/// be merged *over* this table (learned wins on collision).
pub fn fact_table_from_spec(spec: &dyn frensense_lang::spec::LanguageSpec) -> FactTable {
    let mut t = FactTable::default();
    for (call, label) in spec.known_sink_names() {
        // Dotted names ("res.json") are keyed by full path AND by last
        // segment: the engine matches by last segment at the call site, so
        // `res.json(...)` must resolve to the same signature (and role) as
        // the dotted entry.
        //
        // The role is resolved from THIS entry's own label, not a
        // last-segment-shared map: two entries can share a last segment
        // with different roles (`decode` = JwtUnsafeDecode/Validation,
        // `msgpack.decode` = UnsafeDeserialize/Execution). A shared map let
        // whichever dotted entry came last silently re-role the bare name.
        // Bare (non-dotted) entries register first and win the shared
        // last-segment key below; dotted aliases only fill unclaimed keys.
        let key = (*call).to_string();
        let last = call.rsplit('.').next().unwrap_or(call).to_string();
        let role = crate::analysis::taint::role::SinkRole::from_label(*label);
        t.sink_signatures
            .entry(key)
            .or_insert_with(|| SinkSignature {
                role,
                ..SinkSignature::all_args(call)
            });
        if last != *call {
            t.sink_signatures
                .entry(last.clone())
                .or_insert_with(|| SinkSignature {
                    role,
                    ..SinkSignature::all_args(call)
                });
            // Dotted entry sharing its last segment with a DIFFERENT-role
            // bare (or earlier-dotted) entry: record a receiver-specific
            // role so `kv.put(...)` can resolve Storage while `axios.put`
            // stays Ssrf. Root = first segment of the dotted name.
            if let Some(root) = call.split('.').next() {
                t.receiver_roles
                    .entry((root.to_string(), last.clone()))
                    .or_insert(role);
            }
        }
        // A dotted entry whose last segment is a bare HTTP verb is an
        // AMBIGUOUS verb sink: `Map.get`, router `app.post`, LRU `.put` all
        // share the name. Record it so sink matching can require that the
        // receiver actually resolves to the client root (got/axios/http...).
        if last != *call
            && matches!(
                last.as_str(),
                "get" | "post" | "put" | "delete" | "patch" | "head" | "options" | "request"
            )
        {
            t.verb_sinks.insert(last.clone());
            if let Some(root) = call.split('.').next() {
                t.client_roots.insert(root.to_string());
            }
        }
    }
    // Per-slot danger facts override the all-args default for the sinks that
    // declare them (jwt.verify(token, secret), parameterized query(sql, params)).
    for (call, dangerous_slots, binding_safe) in spec.known_sink_signatures() {
        // Finder/updater sinks carry the default IDOR key set: a tainted
        // argument arriving as an object literal with these keys is an
        // access-control query payload, not an injection string.
        let idor_keys: Vec<String> = if crate::analysis::forward::IDOR_FINDER_SINKS.contains(call) {
            crate::analysis::forward::DEFAULT_IDOR_KEYS
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };
        // Per-slot entries carry no label of their own: inherit the role
        // from the all-args entry the first loop registered for this call
        // (same spec, so it is present), falling back to Other.
        let role = t
            .sink_signatures
            .get(*call)
            .map(|s| s.role)
            .unwrap_or(crate::analysis::taint::role::SinkRole::Other);
        let entry = SinkSignature {
            call: (*call).to_string(),
            dangerous_args: dangerous_slots.iter().copied().collect(),
            binding_args_safe: *binding_safe,
            idor_keys,
            role,
        };
        t.sink_signatures.insert((*call).to_string(), entry.clone());
        let last = call.rsplit('.').next().unwrap_or(call);
        if last != *call {
            t.sink_signatures.entry(last.to_string()).or_insert(entry);
        }
    }
    for root in spec.known_session_roots() {
        t.session_roots.insert((*root).to_string());
    }
    // Spec sanitizers are declared per-language via `classify_sanitizer`;
    // probe the common method-name space is impractical, so we rely on the
    // runtime `is_sanitizer_use` path that calls `classify_sanitizer`
    // directly. No static table needed here beyond configured names.
    t
}

/// Build a [`TaintConfig`] from a `frensense-lang` [`LanguageSpec`]:
/// known source patterns + known sink names (last segment) + the sanitizer
/// name space derived from the spec's sanitizer classification table.
pub fn config_from_spec(spec: &dyn frensense_lang::spec::LanguageSpec) -> TaintConfig {
    let mut sources: FxHashSet<String> = spec
        .known_source_patterns()
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    // Conventional request-parameter names are sources too: a function
    // parameter named `req`/`input`/`body` carries user data by convention.
    for p in spec.request_param_names() {
        sources.insert((*p).to_string());
    }
    let sinks: FxHashSet<String> = spec
        .known_sink_names()
        .iter()
        .map(|(call, _)| {
            // Engine matching is by last segment of the member chain.
            call.rsplit('.').next().unwrap_or(call).to_string()
        })
        .collect();
    let sanitizers: FxHashSet<String> = spec
        .known_sanitizer_names()
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    TaintConfig {
        sources,
        sinks,
        sanitizers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> TaintConfig {
        TaintConfig {
            sources: ["req.body".to_string()].into_iter().collect(),
            sinks: ["query".to_string(), "exec".to_string()]
                .into_iter()
                .collect(),
            sanitizers: ["escapeHtml".to_string()].into_iter().collect(),
        }
    }

    #[test]
    fn test_fact_table_defaults_all_args_dangerous() {
        let t = FactTable::from_config(&cfg());
        let sig = t.sink_signature("query").unwrap();
        assert!(sig.is_dangerous(0));
        assert!(sig.is_dangerous(1));
        assert!(sig.is_dangerous(9));
    }

    #[test]
    fn test_parameterized_query_signature() {
        let mut t = FactTable::from_config(&cfg());
        let mut sig = SinkSignature::with_args("query", &[0]);
        sig.binding_args_safe = true;
        t.sink_signatures.insert("query".into(), sig);

        let s = t.sink_signature("query").unwrap();
        assert!(s.is_dangerous(0), "sql template slot is dangerous");
        assert!(
            !s.is_dangerous(1),
            "params array slot is the safe binding channel"
        );
        assert!(s.is_binding(1));
    }

    #[test]
    fn test_merge_learns_over_builtins() {
        let mut t = FactTable::from_config(&cfg());
        assert!(t.sink_signature("query").unwrap().is_dangerous(1));

        let mut learned = FactTable::default();
        let mut sig = SinkSignature::with_args("query", &[0]);
        sig.binding_args_safe = true;
        learned.sink_signatures.insert("query".into(), sig);
        t.merge(&learned);

        assert!(!t.sink_signature("query").unwrap().is_dangerous(1));
    }

    #[test]
    fn test_sanitizer_fact_lookup() {
        let t = FactTable::from_config(&cfg());
        let f = t.sanitizer_fact("escapeHtml").unwrap();
        assert_eq!(f.kind, "encode");
        assert!(t.sanitizer_fact("exec").is_none());
    }

    #[test]
    fn test_verb_sink_receiver_aware() {
        // The JS spec carries dotted client sinks (got.get, axios.post, ...);
        // their bare verb segments must be receiver-aware, not blanket sinks.
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let t = fact_table_from_spec(spec);

        assert!(
            t.verb_sinks.contains("get"),
            "dotted got.get registers verb"
        );
        assert!(t.client_roots.contains("got"), "client root from got.get");

        // Known client root: real SSRF channel, sink.
        assert!(t.is_sink_call("get", Some("got")));
        // Arbitrary Map/receiver root: accessor, NOT a sink.
        assert!(!t.is_sink_call("get", Some("authenticatedUsers")));
        // Non-verb sinks match by name regardless of receiver.
        assert!(t.is_sink_call("findOne", Some("anything")));
    }

    #[test]
    fn test_verb_sink_merge_accumulates() {
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let base = fact_table_from_spec(spec);
        let mut merged = FactTable::default();
        merged.merge(&base);
        assert!(merged.verb_sinks.contains("post"));
        assert!(merged.client_roots.contains("axios"));
    }

    #[test]
    fn test_last_segment_role_collision_bare_wins() {
        // `decode` (JwtUnsafeDecode) and `msgpack.decode` (UnsafeDeserialize)
        // share a last segment with different roles. The bare entry must keep
        // Validation; a dotted sibling must never silently re-role it.
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let t = fact_table_from_spec(spec);
        let sig = t.sink_signature("decode").expect("decode registered");
        assert_eq!(sig.role, crate::analysis::taint::role::SinkRole::Validation);
        // The dotted entry keeps its own role under its full-path key.
        let msgpack = t
            .sink_signature("msgpack.decode")
            .expect("msgpack.decode registered");
        assert_eq!(
            msgpack.role,
            crate::analysis::taint::role::SinkRole::Execution
        );
    }

    #[test]
    fn test_per_slot_signature_inherits_label_role() {
        // Per-slot entries (JS_SINK_SIGNATURES) carry no label of their own;
        // they must inherit the role of the same call's all-args entry.
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let t = fact_table_from_spec(spec);
        let query = t.sink_signature("query").expect("query registered");
        assert_eq!(
            query.role,
            crate::analysis::taint::role::SinkRole::Execution
        );
        assert!(
            query.binding_args_safe,
            "parameterized query keeps binding safety"
        );
        let fetch = t.sink_signature("fetch").expect("bare fetch registered");
        assert_eq!(fetch.role, crate::analysis::taint::role::SinkRole::Resource);
    }

    #[test]
    fn test_rust_fetch_single_label() {
        // The Rust spec once declared `fetch` twice (SqlInjection + Ssrf);
        // insertion-order races decided its role. Exactly one label wins now.
        let spec = frensense_lang::spec_for_ext("rs").unwrap();
        let t = fact_table_from_spec(spec);
        let fetch = t.sink_signature("fetch").expect("fetch registered");
        assert_eq!(
            fetch.role,
            crate::analysis::taint::role::SinkRole::Execution
        );
    }

    #[test]
    fn test_receiver_role_disambiguates_dotted_vs_bare() {
        // The JS spec declares dotted KVNamespace.put (StorageWrite) whose
        // last segment collides with the bare HTTP-verb space, and
        // `res.send` (ResponseLeak) colliding with `Queue.send` (Ssrf).
        // A receiver root matching the dotted entry's declared root must
        // flip the role; an unmatched receiver keeps the bare entry's.
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let t = fact_table_from_spec(spec);

        // res.send: receiver `res` matches the dotted root -> ResponseLeak.
        let res_send = t.role_for_call("send", Some("res"));
        assert_eq!(
            res_send,
            Some(crate::analysis::taint::role::SinkRole::Response),
            "res.send resolves via dotted ResponseLeak entry"
        );
        // queue.send: receiver `queue` does NOT match `res`'s root, but
        // Queue.send is its own dotted entry with root `Queue` -> Ssrf.
        let queue_send = t.role_for_call("send", Some("Queue"));
        assert_eq!(
            queue_send,
            Some(crate::analysis::taint::role::SinkRole::Resource),
            "Queue.send resolves via dotted Ssrf entry"
        );
        // Unresolvable receiver: falls back to the bare last-segment entry.
        let bare = t.role_for_call("send", None);
        assert!(bare.is_some(), "bare send still a sink");
    }

    #[test]
    fn test_receiver_roles_survive_merge() {
        let spec = frensense_lang::spec_for_ext("ts").unwrap();
        let base = fact_table_from_spec(spec);
        let mut merged = FactTable::default();
        merged.merge(&base);
        let role = merged.role_for_call("put", Some("KVNamespace"));
        assert_eq!(
            role,
            Some(crate::analysis::taint::role::SinkRole::Storage),
            "KVNamespace.put role survives FactTable::merge"
        );
    }
}

/// Corpus-owned seed facts: deployment/deployment-specific knowledge that
/// is NOT language semantics, framework/session-store names, project
/// conventions, kept OUT of `frensense-lang` so the spec layer stays
/// general and the corpus stays ownable.
///
/// Loaded from a JSON file and merged over the spec-built fact table;
/// entries here win (they are more specific than any language default).
#[cfg(feature = "serialize")]
pub mod seed {
    use super::FactTable;
    use serde::Deserialize;

    /// File schema. Only fields present are applied; unknown fields are
    /// ignored so older engines load newer files without failing.
    #[derive(Debug, Deserialize, Default)]
    pub struct SeedFacts {
        /// Receiver roots of trusted session stores
        /// (`security.authenticatedUsers` -> root `authenticatedUsers`).
        #[serde(default)]
        pub session_roots: Vec<String>,
    }

    impl SeedFacts {
        /// Parse a seed-facts JSON document.
        pub fn parse(json: &str) -> Result<Self, String> {
            serde_json::from_str(json).map_err(|e| format!("seed facts: {e}"))
        }

        /// Load from disk.
        pub fn load(path: &std::path::Path) -> Result<Self, String> {
            let text = std::fs::read_to_string(path)
                .map_err(|e| format!("seed facts {}: {e}", path.display()))?;
            Self::parse(&text)
        }

        /// Merge into a fact table (self wins on collision).
        pub fn apply_to(&self, t: &mut FactTable) {
            for r in &self.session_roots {
                t.session_roots.insert(r.clone());
            }
        }

        /// Load a seed file (if it exists) and merge over `t`.
        pub fn load_and_apply(path: &std::path::Path, t: &mut FactTable) -> Result<(), String> {
            if !path.exists() {
                return Ok(());
            }
            Self::load(path)?.apply_to(t);
            Ok(())
        }
    }
}
