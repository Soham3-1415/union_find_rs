use std::{collections::HashSet, error, fmt, hash, iter, result};

//TODO: Use a bidirectional hash map that is stable
use bimap::BiHashMap;

#[cfg(test)]
mod tests {
	use std::collections::HashSet;
	use std::iter::FromIterator;

	use crate::DisjointSetError;

	use super::{DisjointSet, UnionFind};

	#[test]
	fn create() {
		DisjointSet::from_iter(b"This is a test.");
		DisjointSet::<u8>::default();
	}

	#[test]
	fn define() {
		let mut set = DisjointSet::from_iter(b"This is a test.");
		assert_eq!(Err(DisjointSetError::DuplicateElement), set.define(&b'T'));
		assert_eq!(Ok(()), set.define(&b'Q'));
		assert_eq!(Err(DisjointSetError::DuplicateElement), set.define(&b'Q'));
	}

	#[test]
	fn subset_count() {
		let mut set = DisjointSet::from_iter(b"This is a test.");
		assert_eq!(9, set.subset_count());

		set.union(&b'h', &b'i').unwrap();
		set.union(&b'T', &b'i').unwrap();
		set.union(&b'h', &b'T').unwrap();
		assert_eq!(7, set.subset_count());
	}

	// TODO: Examine usefulness
//	#[test]
//	fn find() {
//		let mut set = DisjointSet::from_iter(b"This is a test.");
//
//		// simple
//		let ticket1 = set.find(&b'T').unwrap();
//		let ticket2 = set.find(&b't').unwrap();
//		let ticket3 = set.find(&b'T').unwrap();
//		assert_ne!(ticket1, ticket2);
//		assert_eq!(ticket1, ticket3);
//		assert_ne!(ticket1,
//				   DisjointSet::from(b"This is a test.")
//					   .find(&b'T')
//					   .unwrap()); // group set must be different
//
//		// union
//		set.union(&b'T', &b't').unwrap();
//		let ticket4 = set.find(&b'T').unwrap();
//		let ticket5 = set.find(&b't').unwrap();
//		assert_ne!(ticket1, ticket4); // group version must change
//		assert_eq!(ticket4, ticket5);
//
//		// error return
//		let group6 = set.find(&b't');
//		assert_eq!(Err(DisjointSetError::ElementNotDefined), group6);
//
//		// path compression
//		set.union(&b's', &b't').unwrap();
//		set.union(&b'e', &b'T').unwrap();
//		set.union(&b' ', &b'i').unwrap();
//		set.find(&b's').unwrap();
//		let ticket7 = set.find(&b's').unwrap();
//		let ticket8 = set.find(&b'e').unwrap();
//		let ticket9 = set.find(&b'i').unwrap();
//		assert_eq!(ticket7, ticket8);
//		assert_ne!(ticket7, ticket9);
//	}

	#[test]
	fn same_subset() {
		let mut set = DisjointSet::from_iter(b"This is a test.");

		assert!(!set.same_subset(&b'.', &b'T').unwrap());
		assert!(set.same_subset(&b'T', &b'T').unwrap());
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'T'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'T', &b'A'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'A'));

		set.union(&b'T', &b'.').unwrap();
		set.define(&b'S').unwrap();
		set.union(&b'S', &b'.').unwrap();
		set.union(&b'e', &b's').unwrap();
		assert!(!set.same_subset(&b'e', &b'S').unwrap());
		assert!(!set.same_subset(&b'h', &b'T').unwrap());
		assert!(set.same_subset(&b'.', &b'S').unwrap());
		assert!(set.same_subset(&b'T', &b'T').unwrap());
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'T'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'T', &b'A'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.same_subset(&b'A', &b'A'));
	}

	#[test]
	fn union() {
		let mut set = DisjointSet::from_iter(b"This is a test.");

		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b't', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Q', &b't'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Z', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Q', &b'Q'));

		assert_eq!(Ok(()), set.union(&b't', &b'i'));

		set.define(&b'p').unwrap();
		assert_eq!(Ok(()), set.union(&b't', &b'p'));

		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b't', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Q', &b't'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Z', &b'Q'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.union(&b'Q', &b'Q'));
	}

	#[test]
	fn subset_containing() {
		let mut set = DisjointSet::from_iter(b"This is a test.");

		let subset1 = set.subset_containing(&b'a').unwrap();
		assert!(subset1.contains(&b'a'));
		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.subset_containing(&b'Q'));

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
		let mut set = DisjointSet::from_iter(b"This is a test.");

		let target = set.subset_count();
		let answer_set: HashSet<u8> = b"This ate.".iter().cloned().collect();
		let mut used_answer_set: HashSet<u8> = HashSet::new();

		fn verifier(set: &mut DisjointSet<u8>, answer_set: &HashSet<u8>, used_answer_set: &mut HashSet<u8>) -> usize {
			let mut count = 0;
			set.all_subsets()
				.into_iter()
				.for_each(
					|mut set| {
						answer_set.iter().for_each(
							|possible_answer| {
								if set.contains(possible_answer) && !used_answer_set.contains(possible_answer)
								{
									count += 1;
									used_answer_set.insert(*possible_answer);
								}
							}
						)
					}
				);
			count
		};

		let count = verifier(&mut set, &answer_set, &mut used_answer_set);
		assert_eq!(set.subset_count(), count);

		set.union(&b'a', &b's').unwrap();
		set.define(&b'Q').unwrap();
		set.union(&b'Q', &b'e').unwrap();
		set.union(&b'e', &b'a').unwrap();

		let count = verifier(&mut set, &answer_set, &mut used_answer_set);
		assert_eq!(set.subset_count(), count);
	}
}

type Result<T> = result::Result<T, DisjointSetError>;

#[derive(Debug, PartialEq)]
pub enum DisjointSetError {
	ElementNotDefined,
	DuplicateElement,
}

impl fmt::Display for DisjointSetError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				DisjointSetError::ElementNotDefined => "The provided element is not defined in this set.",
				DisjointSetError::DuplicateElement => "The element is already defined in this set.",
			}
		)
	}
}

impl std::error::Error for DisjointSetError {}

// TODO: Examine usefulness
//pub struct SubsetTicket<T> {
//	id: usize,
//	ver: usize,
//	set: *const T,
//}
//
//impl<T> fmt::Debug for SubsetTicket<T> {
//	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
//		write!(f, "id: {}, version: {}", self.id, self.ver)
//	}
//}
//
//impl<T> PartialEq for SubsetTicket<T> {
//	fn eq(&self, other: &Self) -> bool {
//		self.id == other.id &&
//			self.ver == other.ver &&
//			self.set as *const _ == other.set as *const _
//	}
//}

pub trait UnionFind<'a, T: 'a>
	where Self: iter::FromIterator<&'a T> {
	type UnionFindImplementation: UnionFind<'a, T>;
	type UnionFindError: error::Error;

	fn define(&mut self, elem: &T) -> result::Result<(), Self::UnionFindError>;
	fn union(&mut self, elem_a: &T, elem_b: &T) -> result::Result<(), Self::UnionFindError>;
	//	fn find(&mut self, elem: &T) -> result::Result<SubsetTicket<Self::UnionFindImplementation>, Self::UnionFindError>;
	fn subset_containing(&mut self, elem: &T) -> result::Result<HashSet<&T>, Self::UnionFindError>;
	fn all_subsets(&mut self) -> Vec<HashSet<&T>>;
	fn same_subset(&mut self, elem_a: &T, elem_b: &T) -> result::Result<bool, Self::UnionFindError>;
	fn subset_count(&self) -> usize;
}

pub struct DisjointSet<'a, T>
	where T: hash::Hash + Eq {
	ver: usize,
	map: BiHashMap<&'a T, usize>,
	set: Vec<usize>,
	subset_count: usize,
}

impl<'a, T: 'a> UnionFind<'a, T> for DisjointSet<'a, T>
	where T: hash::Hash + Eq {
	type UnionFindImplementation = Self;
	type UnionFindError = DisjointSetError;

	fn define(&mut self, elem: &T) -> Result<()> {
		unimplemented!()
	}

	fn union(&mut self, elem_a: &T, elem_b: &T) -> Result<()> {
		unimplemented!()
	}

	// TODO: Examine usefulness
//	fn find(&mut self, elem: &T) -> Result<SubsetTicket<DisjointSet<'a, T>>> {
//		let id = self.index(elem)?;
//		let id = self.find_internal(id);
//
//		Ok(SubsetTicket { ver: self.ver, id, set: self as *const DisjointSet<T> })
//	}

	fn subset_containing(&mut self, elem: &T) -> Result<HashSet<&T>> {
		unimplemented!()
	}

	fn all_subsets(&mut self) -> Vec<HashSet<&T>> {
		unimplemented!()
	}

	fn same_subset(&mut self, elem_a: &T, elem_b: &T) -> Result<bool> {
		unimplemented!()
	}

	fn subset_count(&self) -> usize {
		self.subset_count
	}
}

impl<'a, T> Default for DisjointSet<'_, T>
	where T: hash::Hash + Eq {
	fn default() -> Self {
		DisjointSet { ver: 0, map: BiHashMap::new(), set: Vec::new(), subset_count: 0 }
	}
}

impl<'a, T> iter::FromIterator<&'a T> for DisjointSet<'a, T>
	where T: hash::Hash + Eq {
	fn from_iter<I>(iter: I) -> Self
		where I: IntoIterator<Item=&'a T> {
		let mut map = BiHashMap::new();
		let mut set = Vec::new();

		iter.into_iter().for_each(
			|elem| {
				if let Ok(()) = map.insert_no_overwrite(elem, set.len()) {
					set.push(set.len());
				}
			}
		);

		DisjointSet { ver: 0, set, subset_count: map.len(), map }
	}
}

impl<T> DisjointSet<'_, T>
	where T: hash::Hash + Eq {
	fn find_internal(&mut self, elem: usize) -> usize {
		if self.set[elem] == elem {
			return elem;
		}

		let result = self.find_internal(self.set[elem]);
		self.set[elem] = result; // path compression
		result
	}

	fn index(&self, elem: &T) -> Result<usize> {
		Ok(*self.map.get_by_left(&elem).ok_or(DisjointSetError::ElementNotDefined)?)
	}

	fn deindex(&self, index: usize) -> &T {
		self.map.get_by_right(&index).unwrap()
	}
}