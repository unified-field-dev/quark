fn main() {
    let reg = quark_scale_5000::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
