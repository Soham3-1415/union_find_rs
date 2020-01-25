use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum HashDisjointSetError {
	ElementNotDefined,
	DuplicateElement,
}

impl Display for HashDisjointSetError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(
			f,
			"{}",
			match self {
				HashDisjointSetError::ElementNotDefined =>
					"The provided element is not defined in this set.",
				HashDisjointSetError::DuplicateElement =>
					"The element is already defined in this set.",
			}
		)
	}
}

impl std::error::Error for HashDisjointSetError {}
