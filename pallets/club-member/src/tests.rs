use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn add_member_to_club() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ClubMember::add_member(Origin::root(), 1));
	})
}

#[test]
fn member_already_added_to_club() {
	new_test_ext().execute_with(|| {
		assert_ok!(ClubMember::add_member(Origin::root(), 1));

		assert_noop!(ClubMember::add_member(Origin::root(), 1),Error::<Test>::AlreadyMember);
	});
}
