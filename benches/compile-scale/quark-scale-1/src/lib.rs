use quark::Registrable;

pub struct ScaleDescriptor {
    pub key: &'static str,
    pub index: usize,
}

impl Registrable for ScaleDescriptor {
    fn registry_key(&self) -> &str {
        self.key
    }
}

inventory::collect!(ScaleDescriptor);

quark::define_registry! {
    pub struct ScaleRegistry for ScaleDescriptor;
}

inventory::submit! { ScaleDescriptor { key: "item_00000", index: 0 } }
