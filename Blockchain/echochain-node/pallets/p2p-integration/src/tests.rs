use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn register_p2p_node_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(P2PIntegration::register_p2p_node(RuntimeOrigin::signed(1)));
        System::assert_last_event(Event::P2PNodeRegistered(1).into());
    });
}

#[test]
fn report_p2p_activity_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(P2PIntegration::report_p2p_activity(RuntimeOrigin::signed(1), 100, 50));
        System::assert_last_event(Event::P2PActivityReported(1, 100, 50).into());
    });
}

#[test]
fn trigger_compute_job_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(P2PIntegration::trigger_compute_job(RuntimeOrigin::signed(1), 1, 101, vec![1,2,3]));
        System::assert_has_event(Event::ComputeJobTriggered(1, 1, 101).into());
    });
}
