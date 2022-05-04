pub use dummy_trait_derive::DummyTrait;

pub trait DummyTrait {
	fn foo(&self);
}

impl DummyTrait for () {
	fn foo(&self) {}
}

impl DummyTrait for u8 {
	fn foo(&self) {}
}

impl DummyTrait for u16 {
	fn foo(&self) {}
}

impl DummyTrait for u32 {
	fn foo(&self) {}
}

impl DummyTrait for u64 {
	fn foo(&self) {}
}

impl<T: DummyTrait> DummyTrait for Option<T> {
	fn foo(&self) {}
}

impl<T: DummyTrait, const N: usize> DummyTrait for [T; N] {
	fn foo(&self) {}
}
