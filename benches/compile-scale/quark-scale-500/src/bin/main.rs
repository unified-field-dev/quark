fn main() {
    let reg = quark_scale_500::ScaleRegistry::auto_discover();
    println!("discovered {}", reg.len());
}
