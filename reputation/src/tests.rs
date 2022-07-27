// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_increase_reputation() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(Reputations::<Test>::get(2), None);
        assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 5));
        assert_eq!(Reputations::<Test>::get(2).unwrap(), 5);
        assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 7));
        assert_eq!(Reputations::<Test>::get(2).unwrap(), 12);
    });
}

#[test]
fn should_catch_errors() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(Reputation::increase_reputation(Origin::signed(1), 2, 0), Error::<Test>::ReputationTooLow);
        assert_noop!(Reputation::increase_reputation(Origin::signed(1), 2, 20), Error::<Test>::ReputationTooHigh);
    });
}

#[test]
fn should_decrease_reputation() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(Reputations::<Test>::get(2), None);
        assert_ok!(Reputation::increase_reputation(Origin::signed(1), 2, 8));
        assert_eq!(Reputations::<Test>::get(2).unwrap(), 8);
        assert_ok!(Reputation::decrease_reputation(Origin::signed(1), 2, 7));
        assert_eq!(Reputations::<Test>::get(2).unwrap(), 1);
        assert_ok!(Reputation::decrease_reputation(Origin::signed(1), 2, 7));
        assert_eq!(Reputations::<Test>::get(2).unwrap(), 0);
    });
}