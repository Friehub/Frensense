// SPDX-License-Identifier: MIT

use crate::engine::taint_entry_points::TaintEntryPoint;
use crate::semantics::data_flow::TaintOrigin;
use crate::semantics::data_flow::TaintRegistry;
use regex::Regex;
use tree_sitter::Node;

pub struct TaintSeeder<'a> {
    entry_points: &'a [TaintEntryPoint],
    language: &'a str,
}

impl<'a> TaintSeeder<'a> {
    pub fn new(entry_points: &'a [TaintEntryPoint], language: &'a str) -> Self {
        Self {
            entry_points,
            language,
        }
    }

    pub fn seed_from_function_params(
        &self,
        fn_node: Node<'_>,
        source: &str,
        registry: &mut TaintRegistry,
    ) {
        let Some(params_node) = fn_node.child_by_field_name("parameters") else {
            return;
        };

        let mut cursor = params_node.walk();
        for (idx, param) in params_node
            .children(&mut cursor)
            .filter(|c| !matches!(c.kind(), "(" | ")" | "," | "self" | ";"))
            .enumerate()
        {
            let (param_name, param_type) = self.extract_param_info(param, source);
            if param_name.is_empty() {
                continue;
            }

            // Strip leading `: ` from type annotation text
            let clean_type = param_type.trim_start_matches(':').trim();

            for ep in self.entry_points {
                if ep.language != self.language {
                    continue;
                }
                if let Some(pi) = ep.param_index {
                    if pi != idx {
                        continue;
                    }
                }
                if let Some(ref tp) = ep.type_pattern {
                    if !clean_type.is_empty() {
                        let type_re = match Regex::new(tp) {
                            Ok(r) => r,
                            Err(_) => continue,
                        };
                        if !type_re.is_match(clean_type) {
                            continue;
                        }
                    }
                }
                registry.taint(&param_name, TaintOrigin::UserInput);
                break;
            }
        }
    }

    fn extract_param_info(&self, param: Node<'_>, source: &str) -> (String, String) {
        match self.language {
            "rust" => self.extract_rust_param(param, source),
            "typescript" | "javascript" => self.extract_ts_param(param, source),
            "python" => self.extract_python_param(param, source),
            _ => (String::new(), String::new()),
        }
    }

    fn extract_rust_param(&self, param: Node<'_>, source: &str) -> (String, String) {
        let text = &source[param.start_byte()..param.end_byte()];

        // Pattern: `name: Type` or `name: Type<Inner>`
        // The parameter node has children: pattern, `:`, type
        let mut name = String::new();
        let mut ty = String::new();

        let mut cursor = param.walk();
        for child in param.children(&mut cursor) {
            match child.kind() {
                "identifier" | "shorthand_field_identifier" | "field_identifier" => {
                    if name.is_empty() {
                        name = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                "type_annotation"
                | "scoped_type_identifier"
                | "generic_type"
                | "type_identifier" => {
                    if ty.is_empty() {
                        ty = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                _ => {}
            }
        }

        // Fallback: parse the full text with regex
        if name.is_empty() || ty.is_empty() {
            if let Some(caps) = Regex::new(r"(\w+)\s*:\s*(.+)")
                .ok()
                .and_then(|re| re.captures(text))
            {
                if name.is_empty() {
                    name = caps
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
                if ty.is_empty() {
                    ty = caps
                        .get(2)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
            }
        }

        (name, ty)
    }

    fn extract_ts_param(&self, param: Node<'_>, source: &str) -> (String, String) {
        let mut name = String::new();
        let mut ty = String::new();

        // TypeScript: required_parameter has identifier + type_annotation children
        let mut cursor = param.walk();
        for child in param.children(&mut cursor) {
            match child.kind() {
                "identifier" => {
                    if name.is_empty() {
                        name = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                "type_annotation" => {
                    if ty.is_empty() {
                        ty = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                _ => {}
            }
        }

        // Fallback: regex on full text
        if name.is_empty() || ty.is_empty() {
            let text = &source[param.start_byte()..param.end_byte()];
            if let Some(caps) = Regex::new(r"(\w+)\s*:\s*(.+)")
                .ok()
                .and_then(|re| re.captures(text))
            {
                if name.is_empty() {
                    name = caps
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
                if ty.is_empty() {
                    ty = caps
                        .get(2)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
            }
        }

        (name, ty)
    }

    fn extract_python_param(&self, param: Node<'_>, source: &str) -> (String, String) {
        let mut name = String::new();
        let mut ty = String::new();

        // Python: `name: Type`
        let mut cursor = param.walk();
        for child in param.children(&mut cursor) {
            match child.kind() {
                "identifier" | "dotted_name" => {
                    if name.is_empty() {
                        name = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                "type" | "type_identifier" => {
                    if ty.is_empty() && !name.is_empty() {
                        ty = source[child.start_byte()..child.end_byte()].to_string();
                    }
                }
                _ => {}
            }
        }

        // Fallback: regex
        if name.is_empty() || ty.is_empty() {
            let text = &source[param.start_byte()..param.end_byte()];
            if let Some(caps) = Regex::new(r"(\w+)\s*:\s*(.+)")
                .ok()
                .and_then(|re| re.captures(text))
            {
                if name.is_empty() {
                    name = caps
                        .get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
                if ty.is_empty() {
                    ty = caps
                        .get(2)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                }
            }
        }

        (name, ty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::taint_entry_points::load_entry_points_from_str;

    fn make_seeder(lang: &str, type_pattern: &str) -> (Vec<TaintEntryPoint>, String) {
        let toml = format!(
            r#"
[[entry_points]]
language = "{}"
type_pattern = "{}"
rule_ids = []
"#,
            lang, type_pattern
        );
        let points = load_entry_points_from_str(&toml);
        (points, lang.to_string())
    }

    #[test]
    fn test_rust_axum_params() {
        let (points, lang) = make_seeder("rust", "Json|Query|Form|Path");
        let seeder = TaintSeeder::new(&points, &lang);

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let source = "fn handler(body: Json<CreateUser>, id: Path<String>) { }";
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        seeder.seed_from_function_params(fn_node, source, &mut registry);

        // body should be tainted (Json matches type pattern)
        assert!(
            registry.is_tainted("body"),
            "body with Json type should be tainted"
        );
        // id should be tainted (Path matches type pattern)
        assert!(
            registry.is_tainted("id"),
            "id with Path type should be tainted"
        );
    }

    #[test]
    fn test_rust_non_extractor_param_not_tainted() {
        let (points, lang) = make_seeder("rust", "Json|Query|Form|Path");
        let seeder = TaintSeeder::new(&points, &lang);

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        let source = "fn handler(name: String, count: i32) { }";
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        seeder.seed_from_function_params(fn_node, source, &mut registry);

        assert!(
            !registry.is_tainted("name"),
            "String type should not be tainted"
        );
        assert!(
            !registry.is_tainted("count"),
            "i32 type should not be tainted"
        );
    }

    #[test]
    fn test_typescript_request_param() {
        let (points, lang) = make_seeder("typescript", "Request|IncomingMessage");
        let seeder = TaintSeeder::new(&points, &lang);

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .unwrap();
        let source = "function handler(req: Request, res: Response) { }";
        let tree = parser.parse(source, None).unwrap();
        let fn_node = tree.root_node().child(0).unwrap();

        let mut registry = TaintRegistry::default();
        seeder.seed_from_function_params(fn_node, source, &mut registry);

        // req should be tainted (Request matches, param_index=0)
        assert!(
            registry.is_tainted("req"),
            "req with Request type should be tainted"
        );
    }
}
