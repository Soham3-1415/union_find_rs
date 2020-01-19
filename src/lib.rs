use std::{collections::{HashMap, HashSet}, error, fmt, hash, iter, mem, result};

//TODO: Use a bidirectional hash map that is stable
use bimap::BiHashMap;

#[cfg(test)]
mod tests {
	use std::collections::HashSet;
	use std::iter::FromIterator;

	use super::{DisjointSet, DisjointSetError, UnionFind};

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
		let mut set = DisjointSet::from_iter(b"This is a test.");
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

		assert_eq!(Err(DisjointSetError::ElementNotDefined), set.subset_size(&b'Q'));
	}

	#[test]
	fn find() {
		let mut set = DisjointSet::from_iter(b"This is a test.");

		// simple
		let ticket1 = set.find(&b'T').unwrap();
		let ticket2 = set.find(&b't').unwrap();
		let ticket3 = set.find(&b'T').unwrap();
		assert_ne!(ticket1, ticket2);
		assert_eq!(ticket1, ticket3);
		assert_ne!(ticket1,
				   DisjointSet::from_iter(b"This is a test.")
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
		assert_eq!(Err(DisjointSetError::ElementNotDefined), group6);

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

type Result<T> = result::Result<T, DisjointSetError>;

pub trait UnionFind<'a, T: 'a>
	where Self: iter::FromIterator<&'a T> {
	type UnionFindImplementation: UnionFind<'a, T>;
	type UnionFindError: error::Error;

	fn define(&mut self, elem: &'a T) -> result::Result<(), Self::UnionFindError>;
	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> result::Result<(), Self::UnionFindError>;
	fn find(&mut self, elem: &'a T) -> result::Result<SubsetTicket<Self::UnionFindImplementation>, Self::UnionFindError>;
	fn subset_containing(&mut self, elem: &'a T) -> result::Result<HashSet<&T>, Self::UnionFindError>;
	fn all_subsets(&mut self) -> Vec<HashSet<&T>>;
	fn same_subset(&mut self, elem_a: &'a T, elem_b: &'a T) -> result::Result<bool, Self::UnionFindError>;
	fn subset_count(&self) -> usize;
	fn subset_size(&mut self, elem: &'a T) -> result::Result<usize, Self::UnionFindError>;
}

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

#[derive(Eq)]
pub struct SubsetTicket<T> {
	id: usize,
	ver: usize,
	set: *const T,
}

impl<T> fmt::Debug for SubsetTicket<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
		write!(f, "id: {}, version: {}", self.id, self.ver)
	}
}

impl<T> PartialEq for SubsetTicket<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id &&
			self.ver == other.ver &&
			self.set as *const _ == other.set as *const _
	}
}

impl<T> hash::Hash for SubsetTicket<T> {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state);
		self.ver.hash(state);
		self.set.hash(state);
	}
}

pub struct DisjointSet<'a, T>
	where T: hash::Hash + Eq {
	ver: usize,
	map: BiHashMap<&'a T, usize>,
	set: Vec<Unit>,
	subset_count: usize,
}

struct Unit {
	size: usize,
	parent: usize,
}

impl<'a, T: 'a> UnionFind<'a, T> for DisjointSet<'a, T>
	where T: hash::Hash + Eq {
	type UnionFindImplementation = Self;
	type UnionFindError = DisjointSetError;

	// TODO: define conditions to invalidate a ticket
	fn define(&mut self, elem: &'a T) -> Result<()> {
		let set = &mut self.set;

		self.map.insert_no_overwrite(elem, set.len())
			.map_err(|_| DisjointSetError::DuplicateElement)?;

		set.push(Unit { size: 1, parent: set.len() });
		self.subset_count += 1;

		Ok(())
	}

	// TODO: define conditions to invalidate a ticket
	fn union(&mut self, elem_a: &T, elem_b: &T) -> Result<()> {
		let a_i = self.index(elem_a)?;
		let b_i = self.index(elem_b)?;

		let mut root_a = Self::find_internal(&mut self.set, a_i);
		let mut root_b = Self::find_internal(&mut self.set, b_i);

		if root_a != root_b {
			if self.set[root_a].size < self.set[root_b].size {
				mem::swap(&mut root_a, &mut root_b);
			}

			self.set[root_b].parent = root_a;
			self.set[root_a].size += self.set[root_b].size;

			self.subset_count -= 1;
			self.ver += 1;
		}

		Ok(())
	}

	fn find(&mut self, elem: &T) -> Result<SubsetTicket<Self>> {
		let i = self.index(elem)?;
		let root = Self::find_internal(&mut self.set, i);

		Ok(SubsetTicket { ver: self.ver, id: root, set: self as *const Self })
	}

	fn subset_containing(&mut self, elem: &T) -> Result<HashSet<&T>> {
		let i = self.index(elem)?;
		let root = Self::find_internal(&mut self.set, i);
		let avg_set_size = self.set.len() / self.subset_count;
		let mut subset = HashSet::with_capacity(avg_set_size);

		let set = &mut self.set;
		self.map.iter()
			.filter(|(_, &i)| root == Self::find_internal(set, i))
			.for_each(|(&elem, _)| { subset.insert(elem); });

		Ok(subset)
	}

	fn all_subsets(&mut self) -> Vec<HashSet<&T>> {
		let avg_set_size = self.set.len() / self.subset_count;
		let mut subset_map = HashMap::with_capacity(self.subset_count);
		let mut subsets = Vec::with_capacity(self.subset_count);

		let set = &mut self.set;
		self.map.iter()
			.for_each(
				|(&elem, &i)| {
					let root = Self::find_internal(set, i);
					let entry = subset_map.entry(root).or_insert_with(
						|| {
							subsets.push(HashSet::with_capacity(avg_set_size));
							subsets.len() - 1
						}
					);
					subsets[*entry].insert(elem);
				}
			);

		subsets
	}

	fn same_subset(&mut self, elem_a: &T, elem_b: &T) -> Result<bool> {
		let a_i = self.index(elem_a)?;
		let b_i = self.index(elem_b)?;

		let root_a = Self::find_internal(&mut self.set, a_i);
		let root_b = Self::find_internal(&mut self.set, b_i);

		Ok(root_a == root_b)
	}

	fn subset_count(&self) -> usize {
		self.subset_count
	}

	fn subset_size(&mut self, elem: &T) -> Result<usize> {
		let i = self.index(elem)?;
		let root = Self::find_internal(&mut self.set, i);
		Ok(self.set[root].size)
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
					set.push(Unit { size: 1, parent: set.len() });
				}
			}
		);

		DisjointSet { ver: 0, set, subset_count: map.len(), map }
	}
}

impl<T> DisjointSet<'_, T>
	where T: hash::Hash + Eq {
	fn find_internal(set: &mut Vec<Unit>, elem: usize) -> usize {
		let mut elem = elem;
		while set[elem].parent != elem {
			let grandparent = set[elem].parent;
			set[elem].parent = set[grandparent].parent;
			elem = grandparent;
		}
		elem
	}

	fn index(&self, elem: &T) -> Result<usize> {
		Ok(*self.map.get_by_left(&elem).ok_or(DisjointSetError::ElementNotDefined)?)
	}
}