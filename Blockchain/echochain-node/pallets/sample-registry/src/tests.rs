use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn register_sample_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![1,2,3], vec![4,5,6]));
        assert_eq!(SampleRegistry::next_sample_id(), 1);
        assert_eq!(SampleRegistry::samples(0).unwrap().owner, 1);
        assert_eq!(SampleRegistry::samples(0).unwrap().ipfs_cid, vec![1,2,3]);
        assert_eq!(SampleRegistry::samples(0).unwrap().metadata_ipfs_cid, vec![4,5,6]);
        assert_eq!(SampleRegistry::samples(0).unwrap().status, SampleStatus::Pending);
        System::assert_last_event(Event::SampleRegistered(0, 1).into());
    });
}

#[test]
fn update_sample_status_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![1,2,3], vec![4,5,6]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 0, SampleStatus::Approved));
        assert_eq!(SampleRegistry::samples(0).unwrap().status, SampleStatus::Approved);
        System::assert_last_event(Event::SampleStatusUpdated(0, SampleStatus::Approved).into());
    });
}

#[test]
fn update_sample_status_sample_not_found() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SampleRegistry::update_sample_status(RuntimeOrigin::root(), 0, SampleStatus::Approved),
            Error::<Test>::SampleNotFound
        );
    });
}

#[test]
fn get_approved_sample_count_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![1,2,3], vec![4,5,6]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 0, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![7,8,9], vec![10,11,12]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 1, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(2), vec![13,14,15], vec![16,17,18]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 2, SampleStatus::Pending));

        assert_eq!(SampleRegistry::get_approved_sample_count(&1), 2);
        assert_eq!(SampleRegistry::get_approved_sample_count(&2), 0);
    });
}