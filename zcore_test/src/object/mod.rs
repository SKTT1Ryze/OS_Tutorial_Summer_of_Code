use alloc::string::String;

use core::fmt::Debug;
use downcast_rs::{impl_downcast,DowncastSync};
use spin::Mutex;
use core::sync::atomic::*;
use alloc::sync::Arc;
//mod dummyobject;

/// trait for kernel object
pub trait KernelObject: DowncastSync + Debug {
    /// get id of kernel object
    fn id(&self) -> KoID;
    /// get type of kernel object
    fn type_name(&self) -> &str;
    /// get name of kernel object
    fn name(&self) -> String;
    /// set name of kernel object
    fn set_name(&self, name: &str);

}

impl_downcast!(sync KernelObject);

/// base of kernel object
pub struct KObjectBase {
    /// ID
    pub id: KoID,
    /// inner
    inner: Mutex<KObjectBaseInner>,
}

/// changable part of `KObjectBase`
#[derive(Default)]
struct KObjectBaseInner {
    name: String,
}

impl Default for KObjectBase {
    fn default() -> Self {
        KObjectBase {
            id: Self::new_koid(),
            inner: Default::default(),
        }
    }
}

impl KObjectBase {
    fn new_koid() -> KoID {
        static NEXT_KOID: AtomicU64 = AtomicU64::new(1024);
        NEXT_KOID.fetch_add(1, Ordering::SeqCst)
    }

    fn name(&self) -> String {
        self.inner.lock().name.clone()
    }

    fn set_name(&self, name: &str) {
        self.inner.lock().name = String::from(name)
    }
}

pub type KoID = u64;

#[macro_export]
macro_rules! impl_kobject {
    ($class:ident $( $fn:tt )*) => {
        // implement `KernelObject` trait for object
        impl KernelObject for $class {
            fn id(&self) -> KoID {
                self.base.id
            }
            fn type_name(&self) -> &str {
                stringify!($class)
            }
            fn name(&self) -> alloc::string::String {
                self.base.name()
            }
            fn set_name(&self, name: &str) {
                self.base.set_name(name)
            }
            $( $fn )*
        }
        impl core::fmt::Debug for $class {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::result::Result<(),core::fmt::Error> {
                f.debug_tuple(&stringify!($class))
                    .field(&self.id())
                    .field(&self.name())
                    .finish()
            }
        }
    };
}

/// empty object
pub struct DummyObject {
    base: KObjectBase,
}

impl_kobject!(DummyObject);

impl DummyObject {
    /// create a new `DummyObject`
    pub fn new() -> Arc<Self> {
        Arc::new(
            DummyObject {
                base: KObjectBase::default(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dummy_object() {
        let object_0 = DummyObject::new();
        let object_1 = DummyObject::new();
        assert_ne!(object_0.id(), object_1.id());
        assert_eq!(object_0.type_name(), "DummyObject");
        assert_eq!(object_1.name(), "");
        object_1.set_name("object_1");
        assert_eq!(object_1.name(), "object_1");
    }

    #[test]
    fn downcast() {
        let dummy = DummyObject::new();
        let object: Arc<dyn KernelObject> = dummy;
        let _result: Arc<DummyObject> = object.downcast_arc::<DummyObject>().unwrap();
    }

    #[test]
    fn impl_kobject() {
        use alloc::format;
        let dummy = DummyObject::new();
        let object: Arc<dyn KernelObject> = dummy;
        assert_eq!(object.type_name(), "DummyObject");
        assert_eq!(object.name(), "");
        object.set_name("dummy");
        assert_eq!(object.name(), "dummy");
        assert_eq!(
            format!("{:?}",object),
            format!("DummyObject({}, \"dummy\")", object.id())
        );
        let _result: Arc<DummyObject> = object.downcast_arc::<DummyObject>().unwrap();
    }
}