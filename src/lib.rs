//! This crate allows users to work with the union and find operations for disjoint sets.
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use std::iter;
use std::marker::PhantomData;

mod error;

pub mod hash_disjoint_set;

/// This trait should be applied to set structures
/// that store disjoint subsets and can find information
/// on the subsets based on provided elements.
/// The implementation should also be able perform the union operation
/// on subsets.
pub trait UnionFind<'a, T: 'a>
where
	Self: iter::FromIterator<&'a T>,
	T: Hash + Eq,
{
	/// Some error type is needed for all implementors
	type UnionFindError: Error;

	/// Combine the subsets containing each element.
	/// If the two elements are already part of the same set, no change occurs.
	///
	/// # Examples
	/// Union of elements in different subsets.
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.union(&b'T',&b't').unwrap();
	///
	/// assert_eq!((), result);
	/// assert_eq!(8, set.subset_count());
	/// ```
	///  Union of elements in the same subset.
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.union(&b'T',&b'T').unwrap();
	///
	/// assert_eq!((), result);
	/// assert_eq!(9, set.subset_count());
	/// ```
	///
	/// # Failures
	/// An error is returned if at least one of the provided elements are not in the set.
	/// ```
	/// # use union_find::hash_disjoint_set::{HashDisjointSet, HashDisjointSetError};
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.union(&b'T',&b'Q').unwrap_err();
	///
	/// assert_eq!(HashDisjointSetError::ElementNotDefined, result);
	///```
	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<(), Self::UnionFindError>;

	/// Identify the subset of an element.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// use std::collections::HashSet;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result1 = set.find(&b't').unwrap();
	/// let result2 = set.find(&b'T').unwrap();
	/// let result3 = set.find(&b't').unwrap();
	///
	/// let mut found_set = HashSet::new();
	///
	/// assert!(found_set.insert(result1));
	/// assert!(found_set.insert(result2));
	/// assert!(!found_set.insert(result3));
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
	/// let result = set.find(&b'Q').unwrap_err();
	///
	/// assert_eq!(HashDisjointSetError::ElementNotDefined, result);
	/// ```
	fn find(&mut self, elem: &'a T) -> Result<SubsetTicket<T>, Self::UnionFindError>;

	/// Get all the elements in the same subset as the provided element. The provided element is included.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.subset_containing(&b't').unwrap();
	///
	/// assert!(result.contains(&b't'));
	/// assert!(!result.contains(&b'T'));
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
	/// let result = set.subset_containing(&b'Q').unwrap_err();
	///
	/// assert_eq!(HashDisjointSetError::ElementNotDefined, result);
	/// ```
	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&'a T>, Self::UnionFindError>;

	/// Get a list of all the subsets in the disjoint set.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.all_subsets();
	///
	/// // the code to rigorously check if the result is correct is too long for this example
	/// // it appears in the tests directory
	/// println!("{:?}", result);
	/// ```
	fn all_subsets(&mut self) -> Vec<HashSet<&'a T>>;

	/// Determine if two elements are in the same subset.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.same_subset(&b't',&b'a').unwrap();
	///
	/// assert!(!result);
	/// ```
	///
	/// # Failures
	/// An error is returned if at least one of the provided elements are not in the set.
	/// ```
	/// # use union_find::hash_disjoint_set::{HashDisjointSet, HashDisjointSetError};
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.same_subset(&b't',&b'Q').unwrap_err();
	///
	/// assert_eq!(HashDisjointSetError::ElementNotDefined, result);
	/// ```
	fn same_subset(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<bool, Self::UnionFindError>;

	/// Get the number of disjoint subsets in the set.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.subset_count();
	///
	/// assert_eq!(9, result);
	/// ```
	fn subset_count(&self) -> usize;

	/// Get the number of elements in the subsetset containing the provided element.
	///
	/// # Examples
	/// ```
	/// # use union_find::hash_disjoint_set::HashDisjointSet;
	/// # use std::iter::FromIterator;
	/// # use union_find::UnionFind;
	/// #
	/// let mut set = HashDisjointSet::from_iter(b"This is a test.");
	/// let result = set.subset_size(&b't').unwrap();
	///
	/// assert_eq!(1, result);
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
	/// let result = set.subset_size(&b'Q').unwrap_err();
	///
	/// assert_eq!(HashDisjointSetError::ElementNotDefined, result);
	/// ```
	fn subset_size(&mut self, elem: &'a T) -> Result<usize, Self::UnionFindError>;
}

/// A type returned by the `find(..)` function to allow checking if elements are in the same group
///
/// Two SubsetTickets will not be equal if they originate from the different instances of an implementor of the UnionFind trait.
/// If an implementor of UnionFind is modified, then SubsetTickets created from that instance of the implementor after the modification will never equal SubsetTickets created before.
/// A union operation that combines two elements already in the same subset is not considered a modification.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SubsetTicket<T> {
	id: usize,
	ver: usize,
	set_id: usize,
	phantom: PhantomData<T>,
}
