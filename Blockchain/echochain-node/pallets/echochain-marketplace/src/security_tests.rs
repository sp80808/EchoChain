//! Security tests for EchoChain marketplace pallet
//! 
//! These tests validate that the critical security fixes properly prevent
//! the vulnerabilities identified in the security audit.

#![cfg(test)]

use super::*;
use crate::mock::*;
use frame_support::{
    assert_err, assert_ok,
    traits::{Currency, ReservableCurrency},
};
use sp_runtime::traits::{BadOrigin, Zero};

/// Test that bid reservation prevents phantom bids
/// 
/// This test validates the fix for the critical vulnerability where
/// bids could be placed without actual fund backing, enabling market manipulation.
#[test]
fn test_bid_reservation_prevents_phantom_bids() {
    new_test_ext().execute_with(|| {
        let seller = 1u64;
        let bidder = 2u64;
        let item_id = 1u64;
        let starting_price = 100u128;
        let bid_amount = 150u128;

        // Setup: Give seller some balance and list item for auction
        let _ = Balances::deposit_creating(&seller, 1000);
        
        // List item first
        assert_ok!(EchochainMarketplace::list_item(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            b"test item".to_vec(),
            None,
            None
        ));

        // Start auction
        assert_ok!(EchochainMarketplace::start_auction(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            10u32 // duration
        ));

        // Test: Bidder with insufficient funds cannot place bid
        // This prevents phantom bids that were possible before the fix
        assert_err!(
            EchochainMarketplace::place_bid(
                RuntimeOrigin::signed(bidder),
                item_id,
                bid_amount
            ),
            Error::<Test>::InsufficientFunds
        );

        // Give bidder sufficient funds
        let _ = Balances::deposit_creating(&bidder, 200);

        // Now bid should succeed and funds should be reserved
        let initial_free_balance = Balances::free_balance(&bidder);
        
        assert_ok!(EchochainMarketplace::place_bid(
            RuntimeOrigin::signed(bidder),
            item_id,
            bid_amount
        ));

        // Verify funds are reserved
        let final_free_balance = Balances::free_balance(&bidder);
        let reserved_balance = Balances::reserved_balance(&bidder);
        
        assert_eq!(reserved_balance, bid_amount);
        assert_eq!(final_free_balance, initial_free_balance - bid_amount);
    });
}

/// Test that higher bids properly unreserve previous bids
/// 
/// This ensures the escrow system works correctly when multiple bids are placed.
#[test]
fn test_bid_unreservation_on_higher_bid() {
    new_test_ext().execute_with(|| {
        let seller = 1u64;
        let bidder1 = 2u64;
        let bidder2 = 3u64;
        let item_id = 1u64;
        let starting_price = 100u128;

        // Setup balances
        let _ = Balances::deposit_creating(&seller, 1000);
        let _ = Balances::deposit_creating(&bidder1, 500);
        let _ = Balances::deposit_creating(&bidder2, 500);

        // List and start auction
        assert_ok!(EchochainMarketplace::list_item(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            b"test item".to_vec(),
            None,
            None
        ));

        assert_ok!(EchochainMarketplace::start_auction(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            10u32
        ));

        // First bid
        assert_ok!(EchochainMarketplace::place_bid(
            RuntimeOrigin::signed(bidder1),
            item_id,
            150u128
        ));

        assert_eq!(Balances::reserved_balance(&bidder1), 150u128);

        // Second higher bid should unreserve first bidder's funds
        assert_ok!(EchochainMarketplace::place_bid(
            RuntimeOrigin::signed(bidder2),
            item_id,
            200u128
        ));

        // Verify first bidder's funds are unreserved
        assert_eq!(Balances::reserved_balance(&bidder1), 0u128);
        // Verify second bidder's funds are reserved
        assert_eq!(Balances::reserved_balance(&bidder2), 200u128);
    });
}

/// Test input validation prevents attacks
/// 
/// This validates the comprehensive input validation added to prevent
/// various attack vectors through malformed inputs.
#[test]
fn test_input_validation_prevents_attacks() {
    new_test_ext().execute_with(|| {
        let seller = 1u64;
        let _ = Balances::deposit_creating(&seller, 1000);

        // Test price validation - zero price should fail
        assert_err!(
            EchochainMarketplace::list_item(
                RuntimeOrigin::signed(seller),
                1u64,
                0u128, // Invalid zero price
                b"test".to_vec(),
                None,
                None
            ),
            Error::<Test>::InvalidPrice
        );

        // Test description length validation - oversized description should fail
        let oversized_description = vec![b'a'; 2000]; // Exceeds MAX_DESCRIPTION_LENGTH
        assert_err!(
            EchochainMarketplace::list_item(
                RuntimeOrigin::signed(seller),
                1u64,
                100u128,
                oversized_description,
                None,
                None
            ),
            Error::<Test>::DescriptionTooLong
        );

        // Test valid inputs work
        assert_ok!(EchochainMarketplace::list_item(
            RuntimeOrigin::signed(seller),
            1u64,
            100u128,
            b"valid description".to_vec(),
            None,
            None
        ));
    });
}

/// Test auction end properly handles reserved funds
/// 
/// This ensures the escrow system completes properly when auctions end.
#[test]
fn test_auction_end_handles_reserved_funds() {
    new_test_ext().execute_with(|| {
        let seller = 1u64;
        let bidder = 2u64;
        let item_id = 1u64;
        let starting_price = 100u128;
        let bid_amount = 150u128;

        // Setup
        let _ = Balances::deposit_creating(&seller, 1000);
        let _ = Balances::deposit_creating(&bidder, 500);

        // List and start auction
        assert_ok!(EchochainMarketplace::list_item(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            b"test item".to_vec(),
            None,
            None
        ));

        assert_ok!(EchochainMarketplace::start_auction(
            RuntimeOrigin::signed(seller),
            item_id,
            starting_price,
            1u32 // Short duration for testing
        ));

        // Place bid
        assert_ok!(EchochainMarketplace::place_bid(
            RuntimeOrigin::signed(bidder),
            item_id,
            bid_amount
        ));

        // Advance time to end auction
        System::set_block_number(System::block_number() + 2);

        let seller_initial_balance = Balances::free_balance(&seller);

        // End auction
        assert_ok!(EchochainMarketplace::end_auction(
            RuntimeOrigin::signed(seller),
            item_id
        ));

        // Verify funds are unreserved from bidder and transferred to seller
        assert_eq!(Balances::reserved_balance(&bidder), 0u128);
        assert_eq!(
            Balances::free_balance(&seller),
            seller_initial_balance + bid_amount
        );
    });
}

/// Test that the marketplace rejects invalid price ranges
/// 
/// This prevents economic attacks through extreme price values.
#[test]
fn test_price_bounds_validation() {
    new_test_ext().execute_with(|| {
        let seller = 1u64;
        let _ = Balances::deposit_creating(&seller, 1000);

        // Test maximum price rejection (if implementation includes max check)
        // This would prevent overflow attacks
        let max_price = u128::MAX;
        assert_err!(
            EchochainMarketplace::list_item(
                RuntimeOrigin::signed(seller),
                1u64,
                max_price,
                b"test".to_vec(),
                None,
                None
            ),
            Error::<Test>::InvalidPrice
        );

        // Test that reasonable prices work
        assert_ok!(EchochainMarketplace::list_item(
            RuntimeOrigin::signed(seller),
            1u64,
            1000u128,
            b"test".to_vec(),
            None,
            None
        ));
    });
}