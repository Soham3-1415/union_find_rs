use std::collections::HashSet;
use std::error::Error;
use std::iter;

mod error;

pub mod hash_disjoint_set;

pub type Result<T> = result::Result<T, UnionFindError>;

pub trait UnionFind<'a, T: 'a>
where
	Self: iter::FromIterator<&'a T>,
	T: Hash + Eq,
{
	type UnionFindError: Error;

	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<(), Self::UnionFindError>;
	fn find(&mut self, elem: &'a T) -> Result<SubsetTicket<T>, Self::UnionFindError>;
	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&'a T>, Self::UnionFindError>;
	fn all_subsets(&mut self) -> Vec<HashSet<&'a T>>;
	fn same_subset(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<bool, Self::UnionFindError>;
	fn subset_count(&self) -> usize;
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
