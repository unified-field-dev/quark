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
inventory::submit! { ScaleDescriptor { key: "item_00010", index: 10 } }
inventory::submit! { ScaleDescriptor { key: "item_00011", index: 11 } }
inventory::submit! { ScaleDescriptor { key: "item_00012", index: 12 } }
inventory::submit! { ScaleDescriptor { key: "item_00013", index: 13 } }
inventory::submit! { ScaleDescriptor { key: "item_00014", index: 14 } }
inventory::submit! { ScaleDescriptor { key: "item_00015", index: 15 } }
inventory::submit! { ScaleDescriptor { key: "item_00016", index: 16 } }
inventory::submit! { ScaleDescriptor { key: "item_00017", index: 17 } }
inventory::submit! { ScaleDescriptor { key: "item_00018", index: 18 } }
inventory::submit! { ScaleDescriptor { key: "item_00019", index: 19 } }
inventory::submit! { ScaleDescriptor { key: "item_00020", index: 20 } }
inventory::submit! { ScaleDescriptor { key: "item_00021", index: 21 } }
inventory::submit! { ScaleDescriptor { key: "item_00022", index: 22 } }
inventory::submit! { ScaleDescriptor { key: "item_00023", index: 23 } }
inventory::submit! { ScaleDescriptor { key: "item_00024", index: 24 } }
inventory::submit! { ScaleDescriptor { key: "item_00025", index: 25 } }
inventory::submit! { ScaleDescriptor { key: "item_00026", index: 26 } }
inventory::submit! { ScaleDescriptor { key: "item_00027", index: 27 } }
inventory::submit! { ScaleDescriptor { key: "item_00028", index: 28 } }
inventory::submit! { ScaleDescriptor { key: "item_00029", index: 29 } }
inventory::submit! { ScaleDescriptor { key: "item_00030", index: 30 } }
inventory::submit! { ScaleDescriptor { key: "item_00031", index: 31 } }
inventory::submit! { ScaleDescriptor { key: "item_00032", index: 32 } }
inventory::submit! { ScaleDescriptor { key: "item_00033", index: 33 } }
inventory::submit! { ScaleDescriptor { key: "item_00034", index: 34 } }
inventory::submit! { ScaleDescriptor { key: "item_00035", index: 35 } }
inventory::submit! { ScaleDescriptor { key: "item_00036", index: 36 } }
inventory::submit! { ScaleDescriptor { key: "item_00037", index: 37 } }
inventory::submit! { ScaleDescriptor { key: "item_00038", index: 38 } }
inventory::submit! { ScaleDescriptor { key: "item_00039", index: 39 } }
inventory::submit! { ScaleDescriptor { key: "item_00040", index: 40 } }
inventory::submit! { ScaleDescriptor { key: "item_00041", index: 41 } }
inventory::submit! { ScaleDescriptor { key: "item_00042", index: 42 } }
inventory::submit! { ScaleDescriptor { key: "item_00043", index: 43 } }
inventory::submit! { ScaleDescriptor { key: "item_00044", index: 44 } }
inventory::submit! { ScaleDescriptor { key: "item_00045", index: 45 } }
inventory::submit! { ScaleDescriptor { key: "item_00046", index: 46 } }
inventory::submit! { ScaleDescriptor { key: "item_00047", index: 47 } }
inventory::submit! { ScaleDescriptor { key: "item_00048", index: 48 } }
inventory::submit! { ScaleDescriptor { key: "item_00049", index: 49 } }
inventory::submit! { ScaleDescriptor { key: "item_00050", index: 50 } }
inventory::submit! { ScaleDescriptor { key: "item_00051", index: 51 } }
inventory::submit! { ScaleDescriptor { key: "item_00052", index: 52 } }
inventory::submit! { ScaleDescriptor { key: "item_00053", index: 53 } }
inventory::submit! { ScaleDescriptor { key: "item_00054", index: 54 } }
inventory::submit! { ScaleDescriptor { key: "item_00055", index: 55 } }
inventory::submit! { ScaleDescriptor { key: "item_00056", index: 56 } }
inventory::submit! { ScaleDescriptor { key: "item_00057", index: 57 } }
inventory::submit! { ScaleDescriptor { key: "item_00058", index: 58 } }
inventory::submit! { ScaleDescriptor { key: "item_00059", index: 59 } }
inventory::submit! { ScaleDescriptor { key: "item_00060", index: 60 } }
inventory::submit! { ScaleDescriptor { key: "item_00061", index: 61 } }
inventory::submit! { ScaleDescriptor { key: "item_00062", index: 62 } }
inventory::submit! { ScaleDescriptor { key: "item_00063", index: 63 } }
inventory::submit! { ScaleDescriptor { key: "item_00064", index: 64 } }
inventory::submit! { ScaleDescriptor { key: "item_00065", index: 65 } }
inventory::submit! { ScaleDescriptor { key: "item_00066", index: 66 } }
inventory::submit! { ScaleDescriptor { key: "item_00067", index: 67 } }
inventory::submit! { ScaleDescriptor { key: "item_00068", index: 68 } }
inventory::submit! { ScaleDescriptor { key: "item_00069", index: 69 } }
inventory::submit! { ScaleDescriptor { key: "item_00070", index: 70 } }
inventory::submit! { ScaleDescriptor { key: "item_00071", index: 71 } }
inventory::submit! { ScaleDescriptor { key: "item_00072", index: 72 } }
inventory::submit! { ScaleDescriptor { key: "item_00073", index: 73 } }
inventory::submit! { ScaleDescriptor { key: "item_00074", index: 74 } }
inventory::submit! { ScaleDescriptor { key: "item_00075", index: 75 } }
inventory::submit! { ScaleDescriptor { key: "item_00076", index: 76 } }
inventory::submit! { ScaleDescriptor { key: "item_00077", index: 77 } }
inventory::submit! { ScaleDescriptor { key: "item_00078", index: 78 } }
inventory::submit! { ScaleDescriptor { key: "item_00079", index: 79 } }
inventory::submit! { ScaleDescriptor { key: "item_00080", index: 80 } }
inventory::submit! { ScaleDescriptor { key: "item_00081", index: 81 } }
inventory::submit! { ScaleDescriptor { key: "item_00082", index: 82 } }
inventory::submit! { ScaleDescriptor { key: "item_00083", index: 83 } }
inventory::submit! { ScaleDescriptor { key: "item_00084", index: 84 } }
inventory::submit! { ScaleDescriptor { key: "item_00085", index: 85 } }
inventory::submit! { ScaleDescriptor { key: "item_00086", index: 86 } }
inventory::submit! { ScaleDescriptor { key: "item_00087", index: 87 } }
inventory::submit! { ScaleDescriptor { key: "item_00088", index: 88 } }
inventory::submit! { ScaleDescriptor { key: "item_00089", index: 89 } }
inventory::submit! { ScaleDescriptor { key: "item_00090", index: 90 } }
inventory::submit! { ScaleDescriptor { key: "item_00091", index: 91 } }
inventory::submit! { ScaleDescriptor { key: "item_00092", index: 92 } }
inventory::submit! { ScaleDescriptor { key: "item_00093", index: 93 } }
inventory::submit! { ScaleDescriptor { key: "item_00094", index: 94 } }
inventory::submit! { ScaleDescriptor { key: "item_00095", index: 95 } }
inventory::submit! { ScaleDescriptor { key: "item_00096", index: 96 } }
inventory::submit! { ScaleDescriptor { key: "item_00097", index: 97 } }
inventory::submit! { ScaleDescriptor { key: "item_00098", index: 98 } }
inventory::submit! { ScaleDescriptor { key: "item_00099", index: 99 } }
