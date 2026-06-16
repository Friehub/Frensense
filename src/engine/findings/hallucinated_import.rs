use crate::Advisory;
use frensense_engine::deps::DependencyResolver;

pub fn find(
    resolver: &mut DependencyResolver,
    snap: &crate::engine::project::FileSnapshot,
) -> Vec<Advisory> {
    resolver.scan_file(&snap.content, &snap.path).into_iter().map(|imp| {
        Advisory::bare("HALLUCINATED_IMPORT", crate::Severity::Warning, snap.id, std::path::Path::new(&imp.file_path), format!("Import '{}' not found in project dependencies.", imp.import_name))
            .with_confidence(0.85)
            .with_line(crate::to_u32(imp.line))
            .with_column(crate::to_u32(imp.column))
            .with_content(&imp.import_name)
            .with_impact("Referencing non-existent packages causes compile errors. LLM-generated code often hallucinates API names.")
            .with_improvement("Verify the import exists in Cargo.toml or package.json.")
            .with_tags(["hallucination", "dependency"])
    }).collect()
}
