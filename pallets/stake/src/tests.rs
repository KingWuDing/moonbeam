//! Unit testing
use crate::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use sp_runtime::DispatchError;

#[test]
fn genesis_config_works() {
	genesis().execute_with(|| {
		assert!(Sys::events().is_empty());
		// validators
		assert_eq!(Balances::reserved_balance(&1), 500);
		assert_eq!(Balances::free_balance(&1), 500);
		assert!(Stake::is_candidate(&1));
		assert_eq!(Balances::reserved_balance(&2), 200);
		assert_eq!(Balances::free_balance(&2), 100);
		assert!(Stake::is_candidate(&2));
		// nominators
		for x in 3..7 {
			assert!(Stake::is_nominator(&x));
			assert_eq!(Balances::free_balance(&x), 0);
			assert_eq!(Balances::reserved_balance(&x), 100);
		}
		// uninvolved
		for x in 7..10 {
			assert!(!Stake::is_nominator(&x));
		}
		assert_eq!(Balances::free_balance(&7), 100);
		assert_eq!(Balances::reserved_balance(&7), 0);
		assert_eq!(Balances::free_balance(&8), 9);
		assert_eq!(Balances::reserved_balance(&8), 0);
		assert_eq!(Balances::free_balance(&9), 4);
		assert_eq!(Balances::reserved_balance(&9), 0);
	});
}

#[test]
fn online_offline_behaves() {
	genesis().execute_with(|| {
		roll_to(4);
		assert_noop!(
			Stake::go_offline(Origin::signed(3)),
			Error::<Test>::CandidateDNE
		);
		roll_to(11);
		assert_noop!(
			Stake::go_online(Origin::signed(3)),
			Error::<Test>::CandidateDNE
		);
		assert_noop!(
			Stake::go_online(Origin::signed(2)),
			Error::<Test>::AlreadyActive
		);
		assert_ok!(Stake::go_offline(Origin::signed(2)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorWentOffline(3, 2))
		);
		roll_to(21);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		let mut expected = vec![
			RawEvent::ValidatorChosen(2, 1, 700),
			RawEvent::ValidatorChosen(2, 2, 400),
			RawEvent::NewRound(5, 2, 2, 1100),
			RawEvent::ValidatorChosen(3, 1, 700),
			RawEvent::ValidatorChosen(3, 2, 400),
			RawEvent::NewRound(10, 3, 2, 1100),
			RawEvent::ValidatorWentOffline(3, 2),
			RawEvent::ValidatorChosen(4, 1, 700),
			RawEvent::NewRound(15, 4, 1, 700),
			RawEvent::ValidatorChosen(5, 1, 700),
			RawEvent::NewRound(20, 5, 1, 700),
		];
		assert_eq!(events, expected);
		assert_noop!(
			Stake::go_offline(Origin::signed(2)),
			Error::<Test>::AlreadyOffline
		);
		assert_ok!(Stake::go_online(Origin::signed(2)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorBackOnline(5, 2))
		);
		expected.push(RawEvent::ValidatorBackOnline(5, 2));
		roll_to(26);
		expected.push(RawEvent::ValidatorChosen(6, 1, 700));
		expected.push(RawEvent::ValidatorChosen(6, 2, 400));
		expected.push(RawEvent::NewRound(25, 6, 2, 1100));
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		assert_eq!(events, expected);
	});
}

#[test]
fn join_validator_candidates_works() {
	genesis().execute_with(|| {
		assert_noop!(
			Stake::join_candidates(Origin::signed(1), Perbill::from_percent(2), 11u128,),
			Error::<Test>::CandidateExists
		);
		assert_noop!(
			Stake::join_candidates(Origin::signed(3), Perbill::from_percent(2), 11u128,),
			Error::<Test>::NominatorExists
		);
		assert_noop!(
			Stake::join_candidates(Origin::signed(7), Perbill::from_percent(2), 9u128,),
			Error::<Test>::ValBondBelowMin
		);
		assert_noop!(
			Stake::join_candidates(Origin::signed(8), Perbill::from_percent(2), 10u128,),
			DispatchError::Module {
				index: 0,
				error: 3,
				message: Some("InsufficientBalance")
			}
		);
		assert_noop!(
			Stake::join_candidates(Origin::signed(7), Perbill::from_percent(51), 10u128,),
			Error::<Test>::FeeOverMax
		);
		assert!(Sys::events().is_empty());
		assert_ok!(Stake::join_candidates(
			Origin::signed(7),
			Perbill::from_percent(3),
			10u128,
		));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::JoinedValidatorCandidates(7, 10u128, 1110u128))
		);
	});
}

#[test]
fn validator_exit_executes_after_delay() {
	genesis().execute_with(|| {
		roll_to(4);
		assert_noop!(
			Stake::leave_candidates(Origin::signed(3)),
			Error::<Test>::CandidateDNE
		);
		roll_to(11);
		assert_ok!(Stake::leave_candidates(Origin::signed(2)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorScheduledExit(3, 2, 5))
		);
		let info = Stake::candidates(&2).unwrap();
		assert_eq!(info.state, ValidatorStatus::Leaving(5));
		roll_to(21);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		// we must exclude leaving validators from rewards while
		// holding them retroactively accountable for previous faults
		// (within the last T::SlashingWindow blocks)
		let expected = vec![
			RawEvent::ValidatorChosen(2, 1, 700),
			RawEvent::ValidatorChosen(2, 2, 400),
			RawEvent::NewRound(5, 2, 2, 1100),
			RawEvent::ValidatorChosen(3, 1, 700),
			RawEvent::ValidatorChosen(3, 2, 400),
			RawEvent::NewRound(10, 3, 2, 1100),
			RawEvent::ValidatorScheduledExit(3, 2, 5),
			RawEvent::ValidatorChosen(4, 1, 700),
			RawEvent::NewRound(15, 4, 1, 700),
			RawEvent::ValidatorLeft(2, 400, 700),
			RawEvent::ValidatorChosen(5, 1, 700),
			RawEvent::NewRound(20, 5, 1, 700),
		];
		assert_eq!(events, expected);
	});
}

#[test]
fn validator_selection_chooses_top_candidates() {
	genesis2().execute_with(|| {
		roll_to(4);
		roll_to(8);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		// should choose top MaxValidators (5), in order
		let expected = vec![
			RawEvent::ValidatorChosen(2, 1, 100),
			RawEvent::ValidatorChosen(2, 2, 90),
			RawEvent::ValidatorChosen(2, 3, 80),
			RawEvent::ValidatorChosen(2, 4, 70),
			RawEvent::ValidatorChosen(2, 5, 60),
			RawEvent::NewRound(5, 2, 5, 400),
		];
		assert_eq!(events, expected);
		assert_ok!(Stake::leave_candidates(Origin::signed(6)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorScheduledExit(2, 6, 4))
		);
		roll_to(21);
		assert_ok!(Stake::join_candidates(
			Origin::signed(6),
			Perbill::from_percent(2),
			69u128
		));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::JoinedValidatorCandidates(6, 69u128, 469u128))
		);
		roll_to(27);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		// should choose top MaxValidators (5), in order
		let expected = vec![
			RawEvent::ValidatorChosen(2, 1, 100),
			RawEvent::ValidatorChosen(2, 2, 90),
			RawEvent::ValidatorChosen(2, 3, 80),
			RawEvent::ValidatorChosen(2, 4, 70),
			RawEvent::ValidatorChosen(2, 5, 60),
			RawEvent::NewRound(5, 2, 5, 400),
			RawEvent::ValidatorScheduledExit(2, 6, 4),
			RawEvent::ValidatorChosen(3, 1, 100),
			RawEvent::ValidatorChosen(3, 2, 90),
			RawEvent::ValidatorChosen(3, 3, 80),
			RawEvent::ValidatorChosen(3, 4, 70),
			RawEvent::ValidatorChosen(3, 5, 60),
			RawEvent::NewRound(10, 3, 5, 400),
			RawEvent::ValidatorLeft(6, 50, 400),
			RawEvent::ValidatorChosen(4, 1, 100),
			RawEvent::ValidatorChosen(4, 2, 90),
			RawEvent::ValidatorChosen(4, 3, 80),
			RawEvent::ValidatorChosen(4, 4, 70),
			RawEvent::ValidatorChosen(4, 5, 60),
			RawEvent::NewRound(15, 4, 5, 400),
			RawEvent::ValidatorChosen(5, 1, 100),
			RawEvent::ValidatorChosen(5, 2, 90),
			RawEvent::ValidatorChosen(5, 3, 80),
			RawEvent::ValidatorChosen(5, 4, 70),
			RawEvent::ValidatorChosen(5, 5, 60),
			RawEvent::NewRound(20, 5, 5, 400),
			RawEvent::JoinedValidatorCandidates(6, 69, 469),
			RawEvent::ValidatorChosen(6, 1, 100),
			RawEvent::ValidatorChosen(6, 2, 90),
			RawEvent::ValidatorChosen(6, 3, 80),
			RawEvent::ValidatorChosen(6, 4, 70),
			RawEvent::ValidatorChosen(6, 6, 69),
			RawEvent::NewRound(25, 6, 5, 409),
		];
		assert_eq!(events, expected);
	});
}

#[test]
fn exit_queue_works() {
	genesis2().execute_with(|| {
		roll_to(4);
		roll_to(8);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		// should choose top MaxValidators (5), in order
		let mut expected = vec![
			RawEvent::ValidatorChosen(2, 1, 100),
			RawEvent::ValidatorChosen(2, 2, 90),
			RawEvent::ValidatorChosen(2, 3, 80),
			RawEvent::ValidatorChosen(2, 4, 70),
			RawEvent::ValidatorChosen(2, 5, 60),
			RawEvent::NewRound(5, 2, 5, 400),
		];
		assert_eq!(events, expected);
		assert_ok!(Stake::leave_candidates(Origin::signed(6)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorScheduledExit(2, 6, 4))
		);
		roll_to(11);
		assert_ok!(Stake::leave_candidates(Origin::signed(5)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorScheduledExit(3, 5, 5))
		);
		roll_to(16);
		assert_ok!(Stake::leave_candidates(Origin::signed(4)));
		assert_eq!(
			last_event(),
			MetaEvent::stake(RawEvent::ValidatorScheduledExit(4, 4, 6))
		);
		roll_to(21);
		let events = Sys::events()
			.into_iter()
			.map(|r| r.event)
			.filter_map(|e| {
				if let MetaEvent::stake(inner) = e {
					Some(inner)
				} else {
					None
				}
			})
			.collect::<Vec<_>>();
		let mut new_events = vec![
			RawEvent::ValidatorScheduledExit(2, 6, 4),
			RawEvent::ValidatorChosen(3, 1, 100),
			RawEvent::ValidatorChosen(3, 2, 90),
			RawEvent::ValidatorChosen(3, 3, 80),
			RawEvent::ValidatorChosen(3, 4, 70),
			RawEvent::ValidatorChosen(3, 5, 60),
			RawEvent::NewRound(10, 3, 5, 400),
			RawEvent::ValidatorScheduledExit(3, 5, 5),
			RawEvent::ValidatorLeft(6, 50, 400),
			RawEvent::ValidatorChosen(4, 1, 100),
			RawEvent::ValidatorChosen(4, 2, 90),
			RawEvent::ValidatorChosen(4, 3, 80),
			RawEvent::ValidatorChosen(4, 4, 70),
			RawEvent::NewRound(15, 4, 4, 340),
			RawEvent::ValidatorScheduledExit(4, 4, 6),
			RawEvent::ValidatorLeft(5, 60, 340),
			RawEvent::ValidatorChosen(5, 1, 100),
			RawEvent::ValidatorChosen(5, 2, 90),
			RawEvent::ValidatorChosen(5, 3, 80),
			RawEvent::NewRound(20, 5, 3, 270),
		];
		expected.append(&mut new_events);
		assert_eq!(events, expected);
	});
}