use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn submit_report_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(NetworkRewards::submit_report(RuntimeOrigin::signed(1), 100, 50));
        assert_eq!(NetworkRewards::reports(1).unwrap().bytes_uploaded, 100);
        System::assert_last_event(Event::ReportSubmitted(1).into());
    });
}

#[test]
fn submit_report_already_reported() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(NetworkRewards::submit_report(RuntimeOrigin::signed(1), 100, 50));
        assert_noop!(
            NetworkRewards::submit_report(RuntimeOrigin::signed(1), 100, 50),
            Error::<Test>::AlreadyReportedThisPeriod
        );
    });
}

#[test]
fn distribute_network_rewards_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(NetworkRewards::submit_report(RuntimeOrigin::signed(1), 100, 50));
        assert_ok!(NetworkRewards::submit_report(RuntimeOrigin::signed(2), 200, 100));

        assert_ok!(NetworkRewards::distribute_network_rewards(RuntimeOrigin::root()));

        System::assert_has_event(Event::NetworkRewardDistributed(1, 500).into());
        System::assert_has_event(Event::NetworkRewardDistributed(2, 500).into());
        assert_eq!(NetworkRewards::reports(1), None);
        assert_eq!(NetworkRewards::reports(2), None);
    });
}

#[test]
fn distribute_network_rewards_no_reports() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(NetworkRewards::distribute_network_rewards(RuntimeOrigin::root()));
        System::assert_no_events();
    });
}
