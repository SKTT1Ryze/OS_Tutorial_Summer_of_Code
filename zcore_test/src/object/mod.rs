use alloc::string::String;

/// trait for kernel object
pub trait KernelObject: Send + Sync {
    /// get id of kernel object
    fn id(&self) -> KoID;
    /// get type of kernel object
    fn type_name(&self) -> &str;
    /// get name of kernel object
    fn name(&self) -> String;
    /// set name of kernel object
    fn set_name(&self, name: &str);

}

pub type KoID = u64;

mod dummyobject;