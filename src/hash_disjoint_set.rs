use std::{
	collections::{hash_map::Entry, HashMap, HashSet},
	fmt,
	fmt::Debug,
	hash, iter,
	marker::PhantomData,
	mem, result,
	sync::atomic::{AtomicUsize, Ordering},
};

pub use crate::error::HashDisjointSetError;
use crate::{SubsetTicket, UnionFind};

type Result<T> = result::Result<T, HashDisjointSetError>;

static SET_ID: AtomicUsize = AtomicUsize::new(0);

/// Uses a `HashMap` and `Vec` to do meet the requirements for the `UnionFind` trait.
///
/// Path splitting is used. The union operation is done by size.
pub struct HashDisjointSet<'a, T>
where
	T: hash::Hash + Eq,
{
	ver: usize,
	map: HashMap<&'a T, usize>,
	set: Vec<Unit>,
	subset_count: usize,
	set_id: usize,
}

struct Unit {
	size: usize,
	parent: usize,
}

impl<'a, T: 'a> UnionFind<'a, T> for HashDisjointSet<'a, T>
where
	T: hash::Hash + Eq,
{
	type UnionFindError = HashDisjointSetError;

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

	fn find(&mut self, elem: &T) -> Result<SubsetTicket<T>> {
		let i = self.index(elem)?;
		let root = Self::find_internal(&mut self.set, i);

		Ok(SubsetTicket {
			ver: self.ver,
			id: root,
			set_id: self.set_id,
			phantom: PhantomData,
		})
	}

	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&'a T>> {
		let i = self.index(elem)?;
		let root = Self::find_internal(&mut self.set, i);
		let avg_set_size = self.set.len() / self.subset_count;
		let mut subset = HashSet::with_capacity(avg_set_size);

		let set = &mut self.set;
		self.map
			.iter()
			.filter(|(_, &i)| root == Self::find_internal(set, i))
			.for_each(|(&elem, _)| {
				subset.insert(elem);
			});

		Ok(subset)
	}

	fn all_subsets(&mut self) -> Vec<HashSet<&'a T>> {
		let avg_set_size = self.set.len() / self.subset_count;
		let mut subset_map = HashMap::with_capacity(self.subset_count);
		let mut subsets = Vec::with_capacity(self.subset_count);

		let set = &mut self.set;
		self.map.iter().for_each(|(&elem, &i)| {
			let root = Self::find_internal(set, i);
			let entry = subset_map.entry(root).or_insert_with(|| {
				subsets.push(HashSet::with_capacity(avg_set_size));
				subsets.len() - 1
			});
			subsets[*entry].insert(elem);
		});

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

impl<'a, T> Default for HashDisjointSet<'_, T>
where
	T: hash::Hash + Eq,
{
	fn default() -> Self {
		let disjoint_set = HashDisjointSet {
			ver: 0,
			map: HashMap::new(),
			set: Vec::new(),
			subset_count: 0,
			set_id: SET_ID.load(Ordering::SeqCst),
		};
		SET_ID.fetch_add(1, Ordering::SeqCst);
		disjoint_set
	}
}

impl<'a, T> iter::FromIterator<&'a T> for HashDisjointSet<'a, T>
where
	T: hash::Hash + Eq,
{
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = &'a T>,
	{
		let mut map = HashMap::new();
		let mut set = Vec::new();

		iter.into_iter().for_each(|elem| {
			map.entry(elem).or_insert_with(|| {
				let len = set.len();
				set.push(Unit {
					size: 1,
					parent: len,
				});
				len
			});
		});

		let disjoint_set = HashDisjointSet {
			ver: 0,
			set,
			subset_count: map.len(),
			map,
			set_id: SET_ID.load(Ordering::SeqCst),
		};
		SET_ID.fetch_add(1, Ordering::SeqCst);
		disjoint_set
	}
}

impl<'a, T> HashDisjointSet<'a, T>
where
	T: hash::Hash + Eq,
{
	/// Adds an element to the `HashDisjointSet`.
	/// The added element is considered part of a new disjoint subset
	/// containing only that element.
	///
	/// # Examples
	///
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.insert(&b'Q').unwrap();
	///
	/// assert_eq!(result, ());
	/// assert_eq!(10, set.subset_count());
	/// ```
	///
	/// # Failures
	/// An error is returned if the provided element is not in the set.
	/// ```
	/// # use union_find::hash_disjoint_set::{HashDisjointSet, HashDisjointSetError};
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.insert(&b'T').unwrap_err();
	///
	/// assert_eq!(result, HashDisjointSetError::DuplicateElement);
	/// ```
	pub fn insert(&mut self, elem: &'a T) -> Result<()> {
		let set = &mut self.set;

		if let Entry::Vacant(entry) = self.map.entry(elem) {
			entry.insert(set.len());
			Ok(())
		} else {
			Err(HashDisjointSetError::DuplicateElement)
		}?;

		set.push(Unit {
			size: 1,
			parent: set.len(),
		});
		self.subset_count += 1;
		self.ver += 1;

		Ok(())
	}

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
		Ok(*self
			.map
			.get(&elem)
			.ok_or(HashDisjointSetError::ElementNotDefined)?)
	}
}

impl<'a, T> HashDisjointSet<'a, T>
where
	T: hash::Hash + Eq + Debug,
{
	/// Pretty prints a `HashDisjointSet` for debugging purposes.
	///
	/// The order of each set in the list is arbitrary,
	/// and the order of each element in the sets are also arbitrary.
	///
	/// The `Debug` trait cannot be used because
	/// efficiently finding elements requires access to `&mut self`.
	/// However, the method signature is similar to the method signature for `Debug::fmt(..)`.
	pub fn fmt(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.all_subsets())
	}
}
