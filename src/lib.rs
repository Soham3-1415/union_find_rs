use std::collections::HashSet;
use std::error::Error;
use std::iter;

mod error;

pub mod hash_disjoint_set;

pub type Result<T> = result::Result<T, UnionFindError>;

pub trait UnionFind<'a, T: 'a>
where
	Self: iter::FromIterator<&'a T>,
{
	type UnionFindError: Error;

	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<(), Self::UnionFindError>;
	fn find(&mut self, elem: &'a T) -> Result<SubsetTicket, Self::UnionFindError>;
	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&'a T>, Self::UnionFindError>;
	fn all_subsets(&mut self) -> Vec<HashSet<&'a T>>;
	fn same_subset(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<bool, Self::UnionFindError>;
	fn subset_count(&self) -> usize;
	fn subset_size(&mut self, elem: &'a T) -> Result<usize, Self::UnionFindError>;
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SubsetTicket {
	id: usize,
	ver: usize,
	set_id: usize,
}
