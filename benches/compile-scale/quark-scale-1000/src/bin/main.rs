fn main() {
    let reg = quark_scale_1000::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
