fn main() {
    let reg = quark_scale_10::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
