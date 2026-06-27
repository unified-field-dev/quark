fn main() {
    let reg = quark_scale_10000::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
