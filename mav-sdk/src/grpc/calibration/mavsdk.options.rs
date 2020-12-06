#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AsyncType {
    Async = 0,
    Sync = 1,
    Both = 2,
}
