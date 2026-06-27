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
inventory::submit! { ScaleDescriptor { key: "item_00001", index: 1 } }
inventory::submit! { ScaleDescriptor { key: "item_00002", index: 2 } }
inventory::submit! { ScaleDescriptor { key: "item_00003", index: 3 } }
inventory::submit! { ScaleDescriptor { key: "item_00004", index: 4 } }
inventory::submit! { ScaleDescriptor { key: "item_00005", index: 5 } }
inventory::submit! { ScaleDescriptor { key: "item_00006", index: 6 } }
inventory::submit! { ScaleDescriptor { key: "item_00007", index: 7 } }
inventory::submit! { ScaleDescriptor { key: "item_00008", index: 8 } }
inventory::submit! { ScaleDescriptor { key: "item_00009", index: 9 } }
