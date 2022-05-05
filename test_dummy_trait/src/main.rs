use dummy_trait::DummyTrait;

#[derive(DummyTrait)]
pub enum TestEnum<'a, A: Copy, B, C, const N: usize>
where
	C: Copy,
{
	VarA(&'a A),
	VarB(u8, B),
	VarC { nums: [u8; N], c: Option<C> },
}

#[derive(DummyTrait)]
pub struct TestUnitStruct;

#[derive(DummyTrait)]
pub struct TestTupleStruct<'a, A: Copy, B>(&'a A, B)
where
	B: Copy;

#[derive(DummyTrait)]
pub struct TestStruct<'a, A: Copy, B>
where
	B: Copy,
{
	field_a: &'a A,
	field_b: Option<B>,
}

fn main() {}
