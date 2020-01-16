use std::{fmt, result};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

type Result<T> = result::Result<T, UnionFindError>;

#[derive(Debug)]
pub enum UnionFindError {
	ElementNotFound,
	DuplicateElement,
}

impl Display for UnionFindError {
	fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
		write!(
			f,
			"{}",
			match self {
				UnionFindError::ElementNotFound => "The provided element was not found.",
				UnionFindError::DuplicateElement => "The provided element is already in the set.",
			}
		)
	}
}

impl std::error::Error for UnionFindError {}

#[derive(Eq, PartialEq)]
pub struct Group<T> {
	id: usize,
	ver: u32,
	// creating more than 2^32 groups could result in collisions
	phantom: PhantomData<T>,
}

pub trait UnionFind<T> {
	fn add(&self, elem: &T) -> Result<()>;
	fn union(&self, elem_a: &T, elem_b: &T) -> Result<()>;
	fn find(&self, elem: &T) -> Result<Group<&T>>;
	fn group(&self, elem: &T) -> Result<Vec<&T>>;
	fn all_groups(&self) -> Result<Vec<Vec<&T>>>;
	fn are_same(&self, elem_a: &T, elem_b: &T) -> Result<bool>;
	fn size() -> usize;
}

pub struct DisjointSet<T> {
	ver: u32,
	map: HashMap<T, usize>,
	set: Vec<usize>,
}

impl<T> fmt::Debug for DisjointSet<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
		unimplemented!()
	}
}

impl<T> UnionFind<T> for DisjointSet<T> {
	fn add(&self, elem: &T) -> Result<()> {
		unimplemented!()
	}

	fn union(&self, elem_a: &T, elem_b: &T) -> Result<()> {
		unimplemented!()
	}

	fn find(&self, elem: &T) -> Result<Group<&T>> {
		unimplemented!()
	}

	fn are_same(&self, elem_a: &T, elem_b: &T) -> Result<bool> {
		unimplemented!()
	}

	fn size() -> usize {
		unimplemented!()
	}

	fn group(&self, elem: &T) -> Result<Vec<&T>> {
		unimplemented!()
	}

	fn all_groups(&self) -> Result<Vec<Vec<&T>>> {
		unimplemented!()
	}
}

impl<T> Default for DisjointSet<T> {
	fn default() -> DisjointSet<T> {
		unimplemented!()
	}
}

impl<T> DisjointSet<T> {
	pub fn from(source: &[T]) -> DisjointSet<T> {
		unimplemented!()
	}
}