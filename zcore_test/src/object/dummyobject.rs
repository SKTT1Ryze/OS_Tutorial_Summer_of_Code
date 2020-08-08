use spin::Mutex;
use super::*;
use core::sync::atomic::*;
use alloc::sync::Arc;
/// empty object
#[derive(Debug)]
pub struct DummyObject {
    id: KoID,
    inner: Mutex<DummyObjectInner>,
}

/// changable part in `DummyObject`
#[derive(Default,Debug)]
struct DummyObjectInner {
    name: String,
    cooike: usize,
}

impl DummyObject {
    /// create a new `DummyObject`
    pub fn new() -> Arc<Self> {
        Arc::new(DummyObject {
            id: Self::new_koid(),
            inner: Default::default(),
        })
    }

    /// generate a new but only ID
    fn new_koid() -> KoID {
        static NEXT_KOID: AtomicU64 = AtomicU64::new(1024);
        NEXT_KOID.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelObject for DummyObject {
    fn id(&self) -> KoID {
        self.id
    }
    
    fn type_name(&self) -> &str {
        "DummyObject"
    }

    fn name(&self) -> String {
        self.inner.lock().name.clone()
    }

    fn set_name(&self, name: &str) {
        self.inner.lock().name = String::from(name)
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
}