use std::path::Path;
use std::fs;

use super::{express::ExpressAdapter, FrameworkAdapter, UnknownAdapter};

pub fn detect_framework(project_root: &Path, advisory_rule_id: &str) -> Box<dyn FrameworkAdapter> {
    let lang_prefix = advisory_rule_id.split('_').next().unwrap_or("");

    match lang_prefix {
        "ts" | "tsx" | "nodejs" => detect_ts_framework(project_root),
        "rust" => Box::new(super::express::FastifyAdapter),
        "go" => Box::new(super::express::FastifyAdapter),
        _ => Box::new(UnknownAdapter),
    }
}

fn detect_ts_framework(root: &Path) -> Box<dyn FrameworkAdapter> {
    let pkg = read_package_json(root);

    if has_dep(&pkg, "next") { return Box::new(super::nextjs::NextJsAdapter); }
    if has_dep(&pkg, "@remix-run/node") || has_dep(&pkg, "@remix-run/react") {
        return Box::new(super::remix::RemixAdapter);
    }
    if has_dep(&pkg, "@sveltejs/kit") { return Box::new(super::sveltekit::SvelteKitAdapter); }
    if has_dep(&pkg, "astro") { return Box::new(ExpressAdapter); }
    if has_dep(&pkg, "@nestjs/core") { return Box::new(super::nestjs::NestJsAdapter); }
    if has_dep(&pkg, "fastify") { return Box::new(super::express::FastifyAdapter); }
    if has_dep(&pkg, "hono") { return Box::new(super::hono::HonoAdapter); }
    if has_dep(&pkg, "@trpc/server") { return Box::new(ExpressAdapter); }
    if has_dep(&pkg, "express") { return Box::new(ExpressAdapter); }

    Box::new(ExpressAdapter)
}

fn read_package_json(root: &Path) -> serde_json::Value {
    let candidates = ["package.json", "apps/api/package.json", "backend/package.json"];
    for c in &candidates {
        if let Ok(s) = fs::read_to_string(root.join(c)) {
            if let Ok(v) = serde_json::from_str(&s) { return v; }
        }
    }
    serde_json::Value::Null
}

fn has_dep(pkg: &serde_json::Value, name: &str) -> bool {
    ["dependencies", "devDependencies", "peerDependencies"].iter().any(|key| {
        pkg[key].as_object().map_or(false, |deps| deps.contains_key(name))
    })
}
