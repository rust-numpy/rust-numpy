use pyo3::class::impl_::{PyClassImpl, ThreadCheckerStub};
use pyo3::pyclass::PyClass;
use pyo3::pyclass_slots::PyClassDummySlot;
use pyo3::{ffi, type_object, types::PyAny, PyCell};

pub(crate) struct SliceBox<T> {
    pub(crate) data: Box<[T]>,
}

impl<T> SliceBox<T> {
    pub(crate) fn new(data: Box<[T]>) -> Self {
        Self { data }
    }
}

impl<T> PyClass for SliceBox<T>
where
    T: Send,
{
    type Dict = PyClassDummySlot;
    type WeakRef = PyClassDummySlot;
    type BaseNativeType = PyAny;
}

impl<T> PyClassImpl for SliceBox<T>
where
    T: Send,
{
    const DOC: &'static str = "Memory store for PyArray using rust's Box<[T]> \0";

    type BaseType = PyAny;
    type Layout = PyCell<Self>;
    type ThreadChecker = ThreadCheckerStub<Self>;
}

unsafe impl<T> type_object::PyTypeInfo for SliceBox<T>
where
    T: Send,
{
    type AsRefTarget = PyCell<Self>;
    const NAME: &'static str = "SliceBox";
    const MODULE: Option<&'static str> = Some("_rust_numpy");

    #[inline]
    fn type_object_raw(py: pyo3::Python) -> *mut ffi::PyTypeObject {
        use pyo3::type_object::LazyStaticType;
        static TYPE_OBJECT: LazyStaticType = LazyStaticType::new();
        TYPE_OBJECT.get_or_init::<Self>(py)
    }
}
