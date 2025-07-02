use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn distribute_rewards_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Register some samples and approve them for user 1
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![1], vec![1]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 0, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![2], vec![2]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 1, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![3], vec![3]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 2, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![4], vec![4]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 3, SampleStatus::Approved));
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(1), vec![5], vec![5]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 4, SampleStatus::Approved));

        // User 2 has less than MinApprovedSamples
        assert_ok!(SampleRegistry::register_sample(RuntimeOrigin::signed(2), vec![6], vec![6]));
        assert_ok!(SampleRegistry::update_sample_status(RuntimeOrigin::root(), 5, SampleStatus::Approved));

        assert_ok!(ContentRewards::distribute_rewards(RuntimeOrigin::root()));

        // User 1 should receive rewards
        System::assert_has_event(Event::RewardDistributed(1, 100).into());
        // User 2 should not receive rewards
        System::assert_has_no_event(Event::RewardDistributed(2, 100).into());
    });
}