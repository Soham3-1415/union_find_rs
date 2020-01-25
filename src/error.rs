use std::fmt::{Display, Formatter, Result};

/// Error type used by `hash_disjoint_set`
#[derive(Debug, PartialEq)]
pub enum HashDisjointSetError {
	/// returned when a method tries to look for an element that does not exist in the set (any of the disjoint subsets).
	ElementNotDefined,
	/// returned when a method tries to add an element to the set, and the element is already defined in the set (any one of the disjoint subsets).
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
