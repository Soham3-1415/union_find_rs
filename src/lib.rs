use std::{cmp, fmt, hash, result};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[cfg(test)]
mod tests {
	use super::{DisjointSet, UnionFind};

	#[test]
	fn create_disjoint_sets() {
		DisjointSet::from(b"This is a test.");
		DisjointSet::<u8>::default();
	}
}

type Result<T> = result::Result<T, UnionFindError>;

#[derive(Debug)]
pub enum UnionFindError {
	ElementNotFound,
	DuplicateElement,
	ElementsAlreadyInSameSet,
}

impl Display for UnionFindError {
	fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
		write!(
			f,
			"{}",
			match self {
				UnionFindError::ElementNotFound => "The provided element was not found.",
				UnionFindError::DuplicateElement => "The provided element is already in a set.",
				UnionFindError::ElementsAlreadyInSameSet => "The provided elements are already in the same set."
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
	fn size(&self) -> usize;
}

pub struct DisjointSet<'a, T> {
	ver: u32,
	map: HashMap<&'a T, usize>,
	set: Vec<usize>,
}

impl<T> fmt::Debug for DisjointSet<'_, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
		unimplemented!()
	}
}

impl<T> UnionFind<T> for DisjointSet<'_, T> {
	fn add(&self, elem: &T) -> Result<()> {
		unimplemented!()
	}

	fn union(&self, elem_a: &T, elem_b: &T) -> Result<()> {
		unimplemented!()
	}

	fn find(&self, elem: &T) -> Result<Group<&T>> {
		unimplemented!()
	}

	fn group(&self, elem: &T) -> Result<Vec<&T>> {
		unimplemented!()
	}

	fn all_groups(&self) -> Result<Vec<Vec<&T>>> {
		unimplemented!()
	}

	fn are_same(&self, elem_a: &T, elem_b: &T) -> Result<bool> {
		unimplemented!()
	}

	fn size(&self) -> usize {
		unimplemented!()
	}
}

impl<'a, T> Default for DisjointSet<'a, T> {
	fn default() -> DisjointSet<'a, T> {
		unimplemented!()
	}
}

impl<T> DisjointSet<'_, T>
	where T: hash::Hash + cmp::Eq {
	pub fn from(source: &[T]) -> DisjointSet<T> {
		let mut map = HashMap::new();
		let mut set = Vec::new();

		source.iter()
			.for_each(
				|elem| {
					map.entry(elem)
						.or_insert_with(
							|| {
								let len = set.len();
								set.push(len);
								len
							}
						);
				}
			);

		DisjointSet { ver: 0, map, set }
	}
}