fn main() {
    let reg = quark_scale_1::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
