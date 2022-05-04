use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn should_not_throw_errors() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic
		// create identity "prasad" for accountId 1
		let identity = "srinivas".as_bytes().to_vec();
		assert_ok!(TemplateModule::create_identity(Origin::signed(1), "srinivas".as_bytes().to_vec() ));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::get_identity(&identity), Some(1));

		let attribute_key = "name".as_bytes().to_vec();
		let attribute_value = "srinivas relangi".as_bytes().to_vec();

		// add attribute name => prasad kumkar
		assert_ok!(TemplateModule::add_attribute(Origin::signed(1), "srinivas".as_bytes().to_vec(), "name".as_bytes().to_vec(), "srinivas relangi".as_bytes().to_vec()));
		// check attribute value
		assert_eq!(TemplateModule::get_attribute((&identity, &attribute_key)), attribute_value);

		// Remmove attribute
		assert_ok!(TemplateModule::remove_attribute(Origin::signed(1), "srinivas".as_bytes().to_vec(), "name".as_bytes().to_vec()));
		// after removing, attribute value should be blank
		assert_eq!(TemplateModule::get_attribute((identity, attribute_key)), "".as_bytes().to_vec());

	});

}

#[test]
fn check_for_errors() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let identity = "srinivas".as_bytes().to_vec();
		assert_ok!(TemplateModule::create_identity(Origin::signed(1), "srinivas".as_bytes().to_vec() ));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::get_identity(&identity), Some(1));

		// Should throw error as identity "prasad" is already claimed
		let identity = "srinivas".as_bytes().to_vec();
		assert_noop!(
			TemplateModule::create_identity(
				Origin::signed(2), 
				"srinivas".as_bytes().to_vec()
			),
			Error::<Test>::IdentityAlreadyClaimed
		);

		// add_attribute signed by different identity (2)
		// should throw NotAuthrized error
		assert_noop!(
			TemplateModule::add_attribute(
				Origin::signed(2), 
				"srinivas".as_bytes().to_vec(), 
				"name".as_bytes().to_vec(), 
				"srinivas relangi".as_bytes().to_vec()
			), 
			Error::<Test>::NotAuthorized
		);

		// Attribute value should be blank
		assert_eq!(TemplateModule::get_attribute((identity, "name".as_bytes().to_vec())), "".as_bytes().to_vec());
	});
}