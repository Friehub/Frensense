fn main() {
    let bytes = std::fs::read("frensense-corpus.frc").unwrap();
    let bundle = frensense_engine::corpus::bundle::load_bundle(&bytes).unwrap();
    for (cat, weights) in bundle.category_weights.iter() {
        if cat == "_global" || cat == "ts_idor" || cat == "ts_sqli" {
            println!("Cat {}: {:.3?}", cat, weights);
        }
    }
}
