use std::collections::HashSet;
use std::{fmt, iter, result};

pub mod hash_disjoint_set;

pub type Result<T> = result::Result<T, UnionFindError>;

pub trait UnionFind<'a, T: 'a>
where
	Self: iter::FromIterator<&'a T> + Default,
{
	fn define(&mut self, elem: &'a T) -> Result<()>;
	fn union(&mut self, elem_a: &'a T, elem_b: &'a T) -> Result<()>;
	fn find(&mut self, elem: &'a T) -> Result<SubsetTicket>;
	fn subset_containing(&mut self, elem: &'a T) -> Result<HashSet<&'a T>>;
	fn all_subsets(&mut self) -> Vec<HashSet<&'a T>>;
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
				UnionFindError::ElementNotDefined =>
					"The provided element is not defined in this set.",
				UnionFindError::DuplicateElement => "The element is already defined in this set.",
			}
		)
	}
}

impl std::error::Error for UnionFindError {}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SubsetTicket {
	id: usize,
	ver: usize,
	set_id: usize,
}
