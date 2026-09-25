// SPDX-License-Identifier: GPL-3.0-only
// Copyright (c) 2024-2026 Friehub. All rights reserved.
// Commercial use requires a separate license: https://friehub.com/licensing

//! Sink-role model: what a sink *does* with the data it receives.
//!
//! Two sinks that both "receive tainted data" are not equally dangerous:
//! `eval(userInput)` executes it, `db.query({ where: { id } })` controls
//! row selection, and `res.json(userInput)` merely reflects it back,
//! the last is normally not a bug at all unless the client renders it
//! unescaped. Classifying by role lets consumers rank findings by what
//! the sink does instead of treating every flow as Critical.
//!
//! Roles are derived from `frensense-lang`'s per-sink [`SinkLabel`]s at
//! fact-table build time (single source of truth, no parallel table here)
//! and ride on `SinkSignature` so both engines see them.

use frensense_lang::spec::SinkLabel;

/// What the sink does with the value flowing into it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serialize", serde(rename_all = "snake_case"))]
pub enum SinkRole {
    /// Data is interpreted as code: eval, exec, deserialize, template
    /// engines, SQL/NoSQL/LDAP strings. Highest severity.
    Execution,
    /// Data selects or reads a resource (IDOR-class query payloads, path
    /// traversal). Access-control concern, ranked below execution.
    Resource,
    /// Data is written to persistent storage (KV put, file write).
    Storage,
    /// Data goes into a cryptographic operation (sign, cipher, hash).
    Crypto,
    /// Data is reflected into an HTTP response. Usually not a bug by
    /// itself, review-level, the client-side rendering decides.
    Response,
    /// Data enters a validation/verification routine (jwt.verify,
    /// `.test()`, comparators). Consuming tainted data is these APIs'
    /// *job*; a finding here is almost always noise.
    Validation,
    /// Unknown or unlabeled sink, keep the conservative default.
    #[default]
    Other,
}

impl SinkRole {
    /// Map a language-spec label to a role. Labels not listed here keep
    /// `Other` (conservative: treated as execution by the consumer).
    pub fn from_label(label: SinkLabel) -> Self {
        use SinkLabel as L;
        match label {
            L::SqlInjection
            | L::NoSqlInjection
            | L::CommandInjection
            | L::CodeExecution
            | L::TemplateSsti
            | L::UnsafeDeserialize
            | L::LdapInjection
            | L::XpathInjection
            | L::GraphqlInjection
            | L::Xxe
            | L::PrototypePollution
            | L::FormatString => SinkRole::Execution,

            L::Ssrf | L::OpenRedirect | L::PathTraversal | L::Toctou => SinkRole::Resource,

            L::StorageWrite => SinkRole::Storage,

            L::Jwt => SinkRole::Crypto,

            L::Xss
            | L::XssDom
            | L::XssReflected
            | L::ResponseLeak
            | L::HeaderInjection
            | L::CookiePoisoning
            | L::ContentTypeInjection => SinkRole::Response,

            L::JwtUnsafeDecode => SinkRole::Validation,

            L::LogLeak | L::CredentialLeak => SinkRole::Response,

            L::JwtWeakAlgorithm | L::UnsafeMemory | L::BufferOverflow | L::Unknown => {
                SinkRole::Other
            }
        }
    }

    /// Default report level as a stable string (`"critical"`, `"warning"`,
    /// `"info"`). The consumer maps this onto its own `Severity`.
    pub fn default_level(&self) -> &'static str {
        match self {
            SinkRole::Execution | SinkRole::Other => "critical",
            SinkRole::Resource | SinkRole::Storage | SinkRole::Crypto => "warning",
            SinkRole::Response | SinkRole::Validation => "info",
        }
    }

    /// Stable tag for reporting (SARIF properties, CLI JSON).
    pub fn tag(&self) -> &'static str {
        match self {
            SinkRole::Execution => "execution",
            SinkRole::Resource => "resource",
            SinkRole::Storage => "storage",
            SinkRole::Crypto => "crypto",
            SinkRole::Response => "response",
            SinkRole::Validation => "validation",
            SinkRole::Other => "other",
        }
    }
}
