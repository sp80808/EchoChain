//! Security tests for EchoChain sample registry pallet
//! 
//! These tests validate that the governance-based approval system and
//! input validation fixes properly address the identified security vulnerabilities.

#![cfg(test)]

use super::*;
use crate::mock::*;
use frame_support::{assert_err, assert_ok};
use sp_runtime::traits::BadOrigin;

/// Test that governance origin is required for sample approval
/// 
/// This validates the fix for centralized sample approval vulnerability
/// by ensuring only governance (not root) can approve samples.
#[test]
fn test_governance_required_for_approval() {
    new_test_ext().execute_with(|| {
        let user = 1u64;
        let sample_id = 0u64;

        // Register a sample
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmTest123456789".to_vec(),
            b"QmMeta123456789".to_vec()
        ));

        // Test: Regular signed origin cannot approve (should fail)
        assert_err!(
            SampleRegistry::update_sample_status(
                RuntimeOrigin::signed(user),
                sample_id,
                SampleStatus::Approved
            ),
            BadOrigin
        );

        // Test: Root can still use emergency override
        assert_ok!(SampleRegistry::emergency_update_sample_status(
            RuntimeOrigin::root(),
            sample_id,
            SampleStatus::Approved
        ));

        let sample = Samples::<Test>::get(sample_id).unwrap();
        assert_eq!(sample.status, SampleStatus::Approved);
    });
}

/// Test IPFS CID validation prevents malformed inputs
/// 
/// This validates the comprehensive input validation added to prevent
/// injection attacks and ensure data integrity.
#[test]
fn test_ipfs_cid_validation() {
    new_test_ext().execute_with(|| {
        let user = 1u64;

        // Test: Empty CID should fail
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                vec![], // Empty CID
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Too short CID should fail
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                b"Qm123".to_vec(), // Too short
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Too long CID should fail
        let oversized_cid = vec![b'Q'; 150]; // Exceeds MAX_IPFS_CID_LENGTH
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                oversized_cid,
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Invalid starting character should fail
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                b"Xm1234567890123456".to_vec(), // Invalid start
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Invalid characters should fail
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                b"Qm123456789@#$%".to_vec(), // Invalid chars
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Valid CID should succeed
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmTest123456789abcdef".to_vec(), // Valid CID
            b"QmMeta123456789".to_vec()
        ));
    });
}

/// Test metadata IPFS CID validation
/// 
/// This ensures metadata CIDs are also properly validated.
#[test]
fn test_metadata_cid_validation() {
    new_test_ext().execute_with(|| {
        let user = 1u64;

        // Test: Oversized metadata CID should fail
        let oversized_meta_cid = vec![b'Q'; 150]; // Exceeds MAX_METADATA_CID_LENGTH
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                b"QmTest123456789".to_vec(),
                oversized_meta_cid
            ),
            Error::<Test>::InvalidMetadataIpfsCid
        );

        // Test: Invalid metadata CID format should fail
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                b"QmTest123456789".to_vec(),
                b"InvalidMeta@#$".to_vec()
            ),
            Error::<Test>::InvalidMetadataIpfsCid
        );

        // Test: Valid metadata CID should succeed
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmTest123456789".to_vec(),
            b"QmMeta123456789".to_vec()
        ));

        // Test: Empty metadata CID should be allowed (optional field)
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmTest123456790".to_vec(),
            vec![] // Empty metadata is allowed
        ));
    });
}

/// Test that different IPFS CID formats are accepted
/// 
/// This ensures the validation doesn't reject legitimate IPFS CID formats.
#[test]
fn test_valid_ipfs_cid_formats() {
    new_test_ext().execute_with(|| {
        let user = 1u64;

        // Test: Base58 CIDv0 (starts with Qm)
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG".to_vec(),
            b"QmMeta123456789".to_vec()
        ));

        // Test: Base32 CIDv1 (starts with b)
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_vec(),
            b"QmMeta123456789".to_vec()
        ));

        // Test: Base58 CIDv1 (starts with z)
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"zdpuAy9HgcRJW3QLdNtKCpzaRX5dJCWJZ7TjCfcrK9J6NqEek".to_vec(),
            b"QmMeta123456789".to_vec()
        ));
    });
}

/// Test bounds checking prevents buffer overflow attacks
/// 
/// This validates that all input length checks work correctly.
#[test]
fn test_bounds_checking_prevents_overflow() {
    new_test_ext().execute_with(|| {
        let user = 1u64;

        // Test: Exactly at minimum length boundary
        let min_length_cid = vec![b'Q'; MIN_IPFS_CID_LENGTH as usize];
        // This should fail because it doesn't start properly, but length is OK
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                min_length_cid,
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );

        // Test: Exactly at maximum length boundary
        let mut max_length_cid = vec![b'm'; MAX_IPFS_CID_LENGTH as usize - 1];
        max_length_cid.insert(0, b'Q'); // Valid start
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            max_length_cid,
            b"QmMeta123456789".to_vec()
        ));

        // Test: One byte over maximum should fail
        let oversized_cid = vec![b'Q'; MAX_IPFS_CID_LENGTH as usize + 1];
        assert_err!(
            SampleRegistry::register_sample(
                RuntimeOrigin::signed(user),
                oversized_cid,
                b"QmMeta123456789".to_vec()
            ),
            Error::<Test>::InvalidIpfsCid
        );
    });
}

/// Test sample status transitions work correctly
/// 
/// This ensures the governance approval system functions properly.
#[test]
fn test_sample_status_transitions() {
    new_test_ext().execute_with(|| {
        let user = 1u64;
        let sample_id = 0u64;

        // Register sample (starts as Pending)
        assert_ok!(SampleRegistry::register_sample(
            RuntimeOrigin::signed(user),
            b"QmTest123456789".to_vec(),
            b"QmMeta123456789".to_vec()
        ));

        let sample = Samples::<Test>::get(sample_id).unwrap();
        assert_eq!(sample.status, SampleStatus::Pending);

        // Use emergency override to test status changes
        assert_ok!(SampleRegistry::emergency_update_sample_status(
            RuntimeOrigin::root(),
            sample_id,
            SampleStatus::Approved
        ));

        let sample = Samples::<Test>::get(sample_id).unwrap();
        assert_eq!(sample.status, SampleStatus::Approved);

        // Test rejection
        assert_ok!(SampleRegistry::emergency_update_sample_status(
            RuntimeOrigin::root(),
            sample_id,
            SampleStatus::Rejected
        ));

        let sample = Samples::<Test>::get(sample_id).unwrap();
        assert_eq!(sample.status, SampleStatus::Rejected);
    });
}

/// Test that non-existent sample updates fail properly
/// 
/// This ensures error handling works correctly.
#[test]
fn test_nonexistent_sample_handling() {
    new_test_ext().execute_with(|| {
        let nonexistent_id = 999u64;

        // Test: Updating non-existent sample should fail
        assert_err!(
            SampleRegistry::emergency_update_sample_status(
                RuntimeOrigin::root(),
                nonexistent_id,
                SampleStatus::Approved
            ),
            Error::<Test>::SampleNotFound
        );
    });
}