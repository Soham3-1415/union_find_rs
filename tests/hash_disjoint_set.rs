use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

use union_find::hash_disjoint_set::{HashDisjointSet, HashDisjointSetError};
use union_find::{SubsetTicket, UnionFind};

#[test]
fn create_from_iter() {
	HashDisjointSet::from_iter(b"This is a test.");
}

#[test]
fn create_default() {
	HashDisjointSet::<u8>::default();
}

#[test]
fn define_err() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::DuplicateElement),
		set.insert(&b'T')
	);
}

#[test]
fn define_ok() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(Ok(()), set.insert(&b'Q'));
}

#[test]
fn default_subset_count() {
	let set: HashDisjointSet<u8> = HashDisjointSet::default();
	assert_eq!(0, set.subset_count());
}

#[test]
fn from_iter_subset_count() {
	let set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(9, set.subset_count());
}

#[test]
fn define_ok_subset_count() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.insert(&b'P').unwrap();
	assert_eq!(10, set.subset_count());
}

#[test]
fn define_err_subset_count() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.insert(&b'h').unwrap_err();
	assert_eq!(9, set.subset_count());
}

#[test]
fn union_ok_change_subset_count() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'h', &b'i').unwrap();
	set.union(&b'T', &b'i').unwrap();
	assert_eq!(7, set.subset_count());
}

#[test]
fn union_ok_no_change_subset_count() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'h', &b'i').unwrap();
	set.union(&b'T', &b'i').unwrap();
	set.union(&b'h', &b'T').unwrap();
	assert_eq!(7, set.subset_count());
}

#[test]
fn union_err_subset_count() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'Q', &b'h').unwrap_err();
	assert_eq!(9, set.subset_count());
}

#[test]
fn no_op_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(1, set.subset_size(&b'T').unwrap());
}

#[test]
fn insert_ok_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.insert(&b'P').unwrap();
	assert_eq!(1, set.subset_size(&b'P').unwrap());
}

#[test]
fn insert_err_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.insert(&b'h').unwrap_err();
	assert_eq!(1, set.subset_size(&b'h').unwrap());
}

#[test]
fn union_ok_change_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'h', &b'i').unwrap();
	set.union(&b'T', &b'i').unwrap();
	assert_eq!(3, set.subset_size(&b'h').unwrap());
}

#[test]
fn union_ok_no_change_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'h', &b'i').unwrap();
	set.union(&b'T', &b'i').unwrap();
	set.union(&b'h', &b'T').unwrap();
	assert_eq!(3, set.subset_size(&b'T').unwrap());
}

#[test]
fn union_err_subset_size() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b'h', &b'Q').unwrap_err();
	assert_eq!(1, set.subset_size(&b'h').unwrap());
}

#[test]
fn subset_size_err() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.subset_size(&b'Q')
	);
}

#[test]
fn simple_ne_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_ne!(set.find(&b'T').unwrap(), set.find(&b't').unwrap());
}

#[test]
fn simple_eq_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(set.find(&b'T').unwrap(), set.find(&b'T').unwrap());
}

#[test]
fn different_set_find() {
	let mut set1 = HashDisjointSet::from_iter(b"This is a test.");
	let mut set2 = HashDisjointSet::from_iter(b"This is a test.");
	assert_ne!(set1.find(&b'T').unwrap(), set2.find(&b'T').unwrap());
}

#[test]
fn different_ver_insert_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	let ticket1 = set.find(&b'T').unwrap();
	set.insert(&b'Q').unwrap();
	let ticket2 = set.find(&b'T').unwrap();

	assert_ne!(ticket1, ticket2);
}

#[test]
fn different_ver_union_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	let ticket1 = set.find(&b'T').unwrap();
	set.union(&b'T', &b't').unwrap();
	let ticket2 = set.find(&b'T').unwrap();

	assert_ne!(ticket1, ticket2);
}

#[test]
fn same_ver_union_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	set.union(&b'T', &b's').unwrap();
	set.union(&b'i', &b't').unwrap();
	set.union(&b'i', &b's').unwrap();
	let ticket1 = set.find(&b'T').unwrap();
	set.union(&b'T', &b't').unwrap();
	let ticket2 = set.find(&b'T').unwrap();

	assert_eq!(ticket1, ticket2);
}

#[test]
fn moved_set_find() {
	fn move_set(mut set: HashDisjointSet<u8>) -> SubsetTicket<u8> {
		set.find(&b'T').unwrap()
	}

	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	let ticket1 = set.find(&b'T').unwrap();
	let ticket2 = move_set(set);

	assert_eq!(ticket1, ticket2);
}

#[test]
fn path_compression_find() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b's', &b'e').unwrap();
	set.union(&b't', &b'T').unwrap();
	set.union(&b'e', &b'T').unwrap();
	set.find(&b's').unwrap();

	assert_eq!(set.find(&b's').unwrap(), set.find(&b'T').unwrap());
}

#[test]
fn find_err() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.find(&b'Q')
	);
}

#[test]
fn insert_union() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.insert(&b'Q').unwrap();
	assert_eq!(Ok(()), set.union(&b'Q', &b'T'));
}

#[test]
fn diff_union_ok() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(Ok(()), set.union(&b't', &b'T'));
}

#[test]
fn same_union_ok() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(Ok(()), set.union(&b'T', &b'T'));
}

#[test]
fn union_err_left() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.union(&b'Q', &b'T'),
	);
}

#[test]
fn union_err_right() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.union(&b'T', &b'Q'),
	);
}

#[test]
fn union_err_both() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.union(&b'Q', &b'Q'),
	);
}

#[test]
fn same_subset_err_left() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.same_subset(&b'Q', &b'T'),
	);
}

#[test]
fn same_subset_err_right() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.same_subset(&b'T', &b'Q'),
	);
}

#[test]
fn same_subset_err_both() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.same_subset(&b'Q', &b'Q'),
	);
}

#[test]
fn same_same_subset() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(true, set.same_subset(&b'T', &b'T').unwrap());
}

#[test]
fn diff_same_subset() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(false, set.same_subset(&b't', &b'T').unwrap());
}

#[test]
fn union_same_subset() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b't', &b'T').unwrap();
	assert_eq!(true, set.same_subset(&b't', &b'T').unwrap());
}

#[test]
fn create_subset_containing() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	let subset = set.subset_containing(&b't').unwrap();
	assert!(subset.contains(&b't'));
	assert_eq!(1, subset.len());
}

#[test]
fn subset_containing() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	set.union(&b't', &b'T').unwrap();
	set.insert(&b'Q').unwrap();
	set.union(&b'Q', &b'e').unwrap();
	set.union(&b'e', &b't').unwrap();

	let subset = set.subset_containing(&b'Q').unwrap();
	let expected = [&b't', &b'Q', &b'e', &b'T'];

	assert_eq!(expected.len(), subset.len());

	expected
		.to_vec()
		.into_iter()
		.for_each(|elem| assert!(subset.contains(elem)));
}

#[test]
fn subset_containing_err() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	assert_eq!(
		Err(HashDisjointSetError::ElementNotDefined),
		set.subset_containing(&b'Q'),
	);
}

#[test]
fn create_all_subsets() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	let actual = set.all_subsets();
	let actual = Subsets(&actual[..]);
	let expected = [
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
	let expected = Subsets(&expected);

	assert_eq!(expected, actual);
}

#[test]
fn all_subsets() {
	let mut set = HashDisjointSet::from_iter(b"This is a test.");

	set.union(&b'a', &b's').unwrap();
	set.insert(&b'Q').unwrap();
	set.union(&b'Q', &b'e').unwrap();
	set.union(&b'e', &b'a').unwrap();

	let actual = set.all_subsets();
	let actual = Subsets(&actual[..]);
	let expected = [
		HashSet::<&u8>::from_iter(b"T"),
		HashSet::<&u8>::from_iter(b"h"),
		HashSet::<&u8>::from_iter(b"i"),
		HashSet::<&u8>::from_iter(b"Qeas"),
		HashSet::<&u8>::from_iter(b" "),
		HashSet::<&u8>::from_iter(b"t"),
		HashSet::<&u8>::from_iter(b"."),
	];
	let expected = Subsets(&expected);

	assert_eq!(expected, actual);
}

#[derive(Debug, Eq)]
struct Subsets<'a, T>(&'a [HashSet<&'a T>])
where
	T: Hash + Eq;

impl<T> PartialEq for Subsets<'_, T>
where
	T: Hash + Eq,
{
	fn eq(&self, other: &Self) -> bool {
		let expected_subsets = &self.0;
		let subsets = &other.0;

		let mut used_i = HashSet::with_capacity(expected_subsets.len());
		for subset in subsets.iter() {
			for (i, expected_subset) in expected_subsets.iter().enumerate() {
				if subset == expected_subset && !used_i.insert(i) {
					return false;
				}
			}
		}
		expected_subsets.len() == used_i.len() && used_i.len() == subsets.len()
	}
}
