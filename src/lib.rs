use std::{fmt, hash, iter, result};
use std::collections::HashSet;

pub mod hash_disjoint_set;

#[cfg(test)]
mod tests {
	use std::collections::HashSet;
	use std::iter::FromIterator;

	use crate::{UnionFind, UnionFindError};
	use crate::hash_disjoint_set::HashDisjointSet;

	#[test]
	fn create() {
		HashDisjointSet::from_iter(b"This is a test.");
		HashDisjointSet::<u8>::default();
	}

	#[test]
	fn define() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");
		assert_eq!(Err(UnionFindError::DuplicateElement), set.define(&b'T'));
		assert_eq!(Ok(()), set.define(&b'Q'));
		assert_eq!(Err(UnionFindError::DuplicateElement), set.define(&b'Q'));
	}

	#[test]
	fn subset_count() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");
		assert_eq!(9, set.subset_count());

		set.define(&b'P').unwrap();
		assert_eq!(10, set.subset_count());

		set.define(&b'h').unwrap_err();
		assert_eq!(10, set.subset_count());

		set.union(&b'h', &b'i').unwrap();
		assert_eq!(9, set.subset_count());

		set.union(&b'T', &b'i').unwrap();
		assert_eq!(8, set.subset_count());

		set.union(&b'h', &b'T').unwrap();
		assert_eq!(8, set.subset_count());

		set.union(&b'Q', &b'h').unwrap_err();
		assert_eq!(8, set.subset_count());
	}

	#[test]
	fn subset_size() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");
		assert_eq!(1, set.subset_size(&b'T').unwrap());

		set.define(&b'P').unwrap();
		assert_eq!(1, set.subset_size(&b'P').unwrap());

		set.define(&b'h').unwrap_err();
		assert_eq!(1, set.subset_size(&b'h').unwrap());

		set.union(&b'h', &b'i').unwrap();
		assert_eq!(2, set.subset_size(&b'i').unwrap());

		set.union(&b'T', &b'i').unwrap();
		assert_eq!(3, set.subset_size(&b'h').unwrap());

		set.union(&b'h', &b'T').unwrap();
		assert_eq!(3, set.subset_size(&b'T').unwrap());

		set.union(&b'Q', &b'h').unwrap_err();
		assert_eq!(3, set.subset_size(&b'h').unwrap());

		assert_eq!(Err(UnionFindError::ElementNotDefined), set.subset_size(&b'Q'));
	}

	#[test]
	fn find() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");

		// simple
		let ticket1 = set.find(&b'T').unwrap();
		let ticket2 = set.find(&b't').unwrap();
		let ticket3 = set.find(&b'T').unwrap();
		assert_ne!(ticket1, ticket2);
		assert_eq!(ticket1, ticket3);
		assert_ne!(ticket1,
				   HashDisjointSet::from_iter(b"This is a test.")
					   .find(&b'T')
					   .unwrap()); // group set must be different

		// union
		set.union(&b'T', &b't').unwrap();
		let ticket4 = set.find(&b'T').unwrap();
		let ticket5 = set.find(&b't').unwrap();
		assert_ne!(ticket1, ticket4); // group version must change
		assert_eq!(ticket4, ticket5);

		// error return
		let group6 = set.find(&b'Q');
		assert_eq!(Err(UnionFindError::ElementNotDefined), group6);

		// path compression
		set.union(&b's', &b't').unwrap();
		set.union(&b'e', &b'T').unwrap();
		set.union(&b' ', &b'i').unwrap();
		set.find(&b's').unwrap();
		let ticket7 = set.find(&b's').unwrap();
		let ticket8 = set.find(&b'e').unwrap();
		let ticket9 = set.find(&b'i').unwrap();
		assert_eq!(ticket7, ticket8);
		assert_ne!(ticket7, ticket9);
	}

	#[test]
	fn same_subset() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");

		assert!(!set.same_subset(&b'.', &b'T').unwrap());
		assert!(set.same_subset(&b'T', &b'T').unwrap());
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'T'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'T', &b'A'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'A'));

		set.union(&b'T', &b'.').unwrap();
		set.define(&b'S').unwrap();
		set.union(&b'S', &b'.').unwrap();
		set.union(&b'e', &b's').unwrap();
		assert!(!set.same_subset(&b'e', &b'S').unwrap());
		assert!(!set.same_subset(&b'h', &b'T').unwrap());
		assert!(set.same_subset(&b'.', &b'S').unwrap());
		assert!(set.same_subset(&b'T', &b'T').unwrap());
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'T'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'T', &b'A'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.same_subset(&b'A', &b'A'));
	}

	#[test]
	fn union() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");

		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b't', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Q', &b't'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Z', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Q', &b'Q'));

		assert_eq!(Ok(()), set.union(&b't', &b'i'));

		set.define(&b'p').unwrap();
		assert_eq!(Ok(()), set.union(&b't', &b'p'));

		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b't', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Q', &b't'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Z', &b'Q'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.union(&b'Q', &b'Q'));
	}

	#[test]
	fn subset_containing() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");

		let subset1 = set.subset_containing(&b'a').unwrap();
		assert!(subset1.contains(&b'a'));
		assert_eq!(Err(UnionFindError::ElementNotDefined), set.subset_containing(&b'Q'));

		set.union(&b'a', &b's').unwrap();
		set.define(&b'Q').unwrap();
		set.union(&b'Q', &b'e').unwrap();
		set.union(&b'e', &b'a').unwrap();
		let subset2 = set.subset_containing(&b'a').unwrap();
		[&b'a', &b's', &b'Q', &b'e'].to_vec()
			.into_iter()
			.for_each(|elem| assert!(subset2.contains(elem)));
	}

	#[test]
	fn all_subsets() {
		let mut set = HashDisjointSet::from_iter(b"This is a test.");

		fn verifier(subsets: &[HashSet<&u8>], expected_subsets: &[HashSet<&u8>]) -> HashSet<usize> {
			let mut used_i = HashSet::with_capacity(expected_subsets.len());
			for subset in subsets.iter() {
				for (i, expected_subset) in expected_subsets.iter().enumerate() {
					if subset == expected_subset && !used_i.contains(&i) {
						used_i.insert(i);
					}
				}
			}
			used_i
		}

		let subsets = set.all_subsets();
		let expected_subsets = [
			HashSet::<&u8>::from_iter(b"T"),
			HashSet::<&u8>::from_iter(b"h"),
			HashSet::<&u8>::from_iter(b"i"),
			HashSet::<&u8>::from_iter(b"s"),
			HashSet::<&u8>::from_iter(b" "),
			HashSet::<&u8>::from_iter(b"a"),
			HashSet::<&u8>::from_iter(b"t"),
			HashSet::<&u8>::from_iter(b"e"),
			HashSet::<&u8>::from_iter(b"."),
		];
		let used_i = verifier(&subsets, &expected_subsets);
		assert_eq!(expected_subsets.len(), subsets.len());
		assert_eq!(expected_subsets.len(), used_i.len());

		set.union(&b'a', &b's').unwrap();
		set.define(&b'Q').unwrap();
		set.union(&b'Q', &b'e').unwrap();
		set.union(&b'e', &b'a').unwrap();

		let subsets = set.all_subsets();
		let expected_subsets = [
			HashSet::<&u8>::from_iter(b"T"),
			HashSet::<&u8>::from_iter(b"h"),
			HashSet::<&u8>::from_iter(b"i"),
			HashSet::<&u8>::from_iter(b"Qeas"),
			HashSet::<&u8>::from_iter(b" "),
			HashSet::<&u8>::from_iter(b"t"),
			HashSet::<&u8>::from_iter(b"."),
		];
		let used_i = verifier(&subsets, &expected_subsets);
		assert_eq!(expected_subsets.len(), subsets.len());
		assert_eq!(expected_subsets.len(), used_i.len());
	}
}

pub type Result<T> = result::Result<T, UnionFindError>;

pub trait UnionFind<'a, T: 'a>
	where Self: iter::FromIterator<&'a T> {
	fn define(&mut self, elem: &'a T) -> Result<()>;
	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<()>;
	fn find(&mut self, elem: &'a T) -> Result<SubsetTicket>;
	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&T>>;
	fn all_subsets(&mut self) -> Vec<HashSet<&T>>;
	fn same_subset(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<bool>;
	fn subset_count(&self) -> usize;
	fn subset_size(&mut self, elem: &'a T) -> Result<usize>;
}

#[derive(Debug, PartialEq)]
pub enum UnionFindError {
	ElementNotDefined,
	DuplicateElement,
}

impl fmt::Display for UnionFindError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				UnionFindError::ElementNotDefined => "The provided element is not defined in this set.",
				UnionFindError::DuplicateElement => "The element is already defined in this set.",
			}
		)
	}
}

impl std::error::Error for UnionFindError {}

#[derive(Eq, PartialEq)]
pub struct SubsetTicket {
	id: usize,
	ver: usize,
	set_id: usize,
}

impl fmt::Debug for SubsetTicket {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
		write!(f, "id: {}, version: {}, set: {:?}", self.id, self.ver, self.set_id)
	}
}

impl hash::Hash for SubsetTicket {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
		self.ver.hash(state);
		self.set_id.hash(state);
	}
}