pub use dummy_trait_derive::DummyTrait;

pub trait DummyTrait {}

impl DummyTrait for () {}

impl DummyTrait for u8 {}

impl DummyTrait for u16 {}

impl DummyTrait for u32 {}

impl DummyTrait for u64 {}

impl<T: DummyTrait> DummyTrait for Option<T> {}

impl<T: DummyTrait, const N: usize> DummyTrait for [T; N] {}
