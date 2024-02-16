pub trait OptionExt<Type> {
	fn map_with_result<NewType, Error, Func>(self, func: Func) -> Result<Option<NewType>, Error>
	where
		Self: Sized,
		Func: FnOnce(Type) -> Result<NewType, Error>;
}

impl<T> OptionExt<T> for Option<T> {
	fn map_with_result<NewType, Error, Func>(self, func: Func) -> Result<Option<NewType>, Error>
	where
		Self: Sized,
		Func: FnOnce(T) -> Result<NewType, Error>,
	{
		match self {
			Some(value) => match func(value) {
				Ok(new_value) => Ok(Some(new_value)),
				Err(err) => Err(err),
			},
			None => Ok(None),
		}
	}
}
