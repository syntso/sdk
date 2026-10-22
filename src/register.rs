#[macro_export]
macro_rules! register_extension {
    ($extension:ty) => {
        type SyntExtensionAdapter = $crate::Adapter<$extension>;

        $crate::wit::export!(SyntExtensionAdapter with_types_in $crate::wit);
    };
}
