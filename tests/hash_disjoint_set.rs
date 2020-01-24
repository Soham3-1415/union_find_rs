use std::iter::FromIterator;

use union_find::hash_disjoint_set::HashDisjointSet;
use union_find::{SubsetTicket, UnionFind, UnionFindError};

#[test]
fn ticket_from_moved_set() {
	fn move_set(mut set: HashDisjointSet<u8>) -> SubsetTicket {
		set.find(&b't').unwrap()
	}

	let mut set = HashDisjointSet::from_iter(b"This is a test.");
	set.union(&b't', &b'T').unwrap();

	let ticket1 = set.find(&b'T').unwrap();
	let ticket2 = move_set(set);

	assert_eq!(ticket1, ticket2);
}
