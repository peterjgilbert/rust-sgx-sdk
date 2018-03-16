/// EDL descriptor.
///
/// These descriptors are used to build the final EDL definition file by
/// combining multiple EDL files from different crates.
pub struct EDL {
    /// Namespace (e.g., crate name) to avoid name conflicts.
    pub namespace: String,
    /// Name.
    pub name: String,
    /// Contents.
    pub data: String,
}

/// Macro for easier EDL definitions.
///
/// Example use:
/// ```
/// define_edl! {
///     // EDL definitions from external crates.
///     use my_other_edl;
///
///     // Local EDL definitions.
///     "core.edl"
/// }
/// ```
#[macro_export]
macro_rules! define_edl {
    (
        $( use $external_edl:ident ; )*

        $( $local_edl:expr ),* $(,)*
    ) => {
        pub fn edl() -> Vec<$crate::EDL> {
            let mut output = vec![];

            // Imported EDL definitions.
            $(
                output.append(&mut $external_edl::edl());
            )*

            // Local EDL definitions.
            $(
                output.push($crate::EDL {
                    namespace: env!("CARGO_PKG_NAME").to_owned(),
                    name: std::path::Path::new($local_edl).file_name().unwrap().to_str().unwrap().to_owned(),
                    data: include_str!($local_edl).to_owned(),
                });
            )*

            output
        }
    }
}

define_edl! {
    "../../edl/sgx_tstd.edl",
    "../../edl/sgx_stdio.edl",
    "../../edl/sgx_backtrace.edl",
    "../../edl/sgx_time.edl",
}
