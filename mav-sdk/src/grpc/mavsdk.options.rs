#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AsyncType {
    Async = 0,
    Sync = 1,
    Both = 2,
}
impl AsyncType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AsyncType::Async => "ASYNC",
            AsyncType::Sync => "SYNC",
            AsyncType::Both => "BOTH",
        }
    }
}
