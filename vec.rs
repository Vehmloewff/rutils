pub struct ConsumedVec<'a, T>(&'a mut Vec<T>);

impl<'a, T> ConsumedVec<'_, T> {
	pub fn new(vec: &mut Vec<T>) -> ConsumedVec<'_, T> {
		vec.reverse();
		ConsumedVec(vec)
	}
}

impl<T> Iterator for ConsumedVec<'_, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

pub trait ConsumableVec<T> {
	fn consume(&mut self) -> ConsumedVec<'_, T>;
}

impl<T> ConsumableVec<T> for Vec<T> {
	fn consume(&mut self) -> ConsumedVec<'_, T> {
		ConsumedVec::new(self)
	}
}
