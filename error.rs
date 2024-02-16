#[macro_export]
macro_rules! errors {
	($($x:ident),+) => {
		#[derive(Debug)]
		pub enum Error {
			$($x),+
		}

		impl std::fmt::Display for Error {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				match self {
					$(Error::$x => write!(f, stringify!($x))),+
				}
			}
		}

		impl error_stack::Context for Error {}

		pub type Result<T> = error_stack::Result<T, Error>;
	};
}
