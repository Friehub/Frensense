fn main() {
    let bytes = std::fs::read("../frensense-corpus.frc").unwrap();
    let bundle = frensense_engine::corpus::bundle::load_bundle(&bytes).unwrap();
    println!("Bundle loaded: {} patterns", bundle.patterns.len());
}
