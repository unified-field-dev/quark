fn main() {
    let reg = quark_scale_100::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
