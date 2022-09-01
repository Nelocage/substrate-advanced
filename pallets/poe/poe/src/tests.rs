use frame_support::{assert_noop, assert_ok};
// use frame_system::Origin;  //type Origin<T> = RawOrigin<<T as Config>::AccountId>; 类型别名
use sp_runtime::traits::BadOrigin;
use crate::{mock::*};
// use sp_runtime::BoundedVec;
use super::*;
// use crate::{Error, mock::*, Proofs};

#[test]
fn test_create_claim() {
	new_test_ext().execute_with(|| {
		// let claim = vec![0, 1];
		// assert_ok!(Poe::create_claim(Origin::signed(1),claim.clone()));
		// let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxBytesInHash>::try_from(claim.clone())
		// 	.unwrap();
		// assert_eq!(Proofs::<Test>::get(&bounded_claim),
		// 		   Some((1, frame_system::Pallet::<Test>::block_number())));
		// create_claim must be called by account signed .

		//claim 即是一个vec ,所以只要变成一个vec就可以
		assert_noop!(
            Poe::create_claim(
                Origin::root(),
                "substrate advanced".to_string().as_bytes().to_vec()   //该部分借鉴了其他的代码
            ),
            BadOrigin   //系统内置的错误消息  应该自定义的错误也可以
        );
		//
		assert_ok!(Poe::create_claim(
            Origin::signed(1), //先进行签名
            "substrate advanced".to_string().as_bytes().to_vec()
        ));
		//
		// // if claim is existed, call create_claim will failed.
		assert_noop!(
            Poe::create_claim(
                Origin::signed(1),
				// claim.clone()
                "substrate advanced".to_string().as_bytes().to_vec()
            ),
            Error::<Test>::ProofAlreadyClaimed  //left==right
        );
	});
}


// #[test]  该部分代码不是很理解
// fn test_create_claim_exceed_size_limit() {
// 	let limit_size = ClaimSizeLimit::get() as usize;
// 	new_test_ext().execute_with(|| {
// 		// test claim with large size.
// 		assert_noop!(
//             Poe::create_claim(
//                 Origin::signed(1),
//                 vec![0u8; limit_size.checked_add(1).unwrap()]
//             ),
//             Error::<Test>::ClaimSizeOverflow
//         );
//
// 		assert_ok!(Poe::create_claim(
//             Origin::signed(1),
//             "substrate advanced".to_string().as_bytes().to_vec()
//         ));
// 	});
// }
//
#[test]
fn test_revoke_claim() {
	new_test_ext().execute_with(|| {
		// Create claim "substrate advanced".
		assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "substrate advanced".to_string().as_bytes().to_vec()
        ));
		// revoke_claim must be called by account signed .
		assert_noop!(
            Poe::revoke_claim(
                Origin::root(),
                "substrate advanced".to_string().as_bytes().to_vec()
            ),
            BadOrigin
        );

		// revoke claim must be owner.
		assert_noop!(
            Poe::revoke_claim(
                Origin::signed(2),
                "substrate advanced".to_string().as_bytes().to_vec()
            ),
            Error::<Test>::NotProofOwner
        );

		// revoke claim.
		assert_ok!(Poe::revoke_claim(
            Origin::signed(1),
            "substrate advanced".to_string().as_bytes().to_vec()
        ));
	});
}

#[test]
fn test_transfer_claim() {
	new_test_ext().execute_with(|| {
		// Create claim "substrate advanced".
		assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "substrate advanced".to_string().as_bytes().to_vec()
        ));

		// transfer_claim must be called by account signed .
		assert_noop!(
            Poe::transfer(
                Origin::root(),
                "substrate advanced".to_string().as_bytes().to_vec(),
                2
            ),
            BadOrigin
        );

		// transfer_claim must be called by the owner.
		assert_noop!(
            Poe::transfer(
                Origin::signed(3),
                "substrate advanced".to_string().as_bytes().to_vec(),
                2
            ),
            Error::<Test>::NotProofOwner
        );

		// transfer claim from 1 to 2.
		assert_ok!(Poe::transfer(
            Origin::signed(1),
            "substrate advanced".to_string().as_bytes().to_vec(),
            2
        ));
	});
}
