#![allow(clippy::same_length_and_capacity, clippy::too_many_arguments)]

wit_bindgen::generate!({
    path: "wit",
    world: "extension",
    pub_export_macro: true,
    export_macro_name: "export",
    default_bindings_module: "synt_sdk::wit",
    runtime_path: "synt_sdk::wit::rt",
    additional_derives: [PartialEq, Eq],
});

pub use wit_bindgen::rt;
