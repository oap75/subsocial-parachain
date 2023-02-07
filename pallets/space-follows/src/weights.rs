//! Autogenerated weights for pallet_space_follows
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
    // ./scripts/../target/release/subsocial-collator
    // benchmark
    // pallet
    // --chain
    // dev
    // --execution
    // wasm
    // --wasm-execution
    // Compiled
    // --pallet
    // pallet_space_follows
    // --extrinsic
    // *
    // --steps
    // 50
    // --repeat
    // 20
    // --heap-pages
    // 4096
    // --output
    // pallets/space-follows/src//weights.rs
    // --template
    // ./.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_space_follows.
pub trait WeightInfo {
    fn follow_space() -> Weight;
    fn unfollow_space() -> Weight;
}

/// Weights for pallet_space_follows using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
        impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:1)
            // Storage: Spaces SpaceById (r:1 w:0)
            // Storage: SpaceFollows SpaceFollowers (r:1 w:1)
            // Storage: SpaceFollows SpacesFollowedByAccount (r:1 w:1)
        fn follow_space() -> Weight {
        (40_968_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
        }
            // Storage: Spaces SpaceById (r:1 w:0)
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:1)
            // Storage: SpaceFollows SpacesFollowedByAccount (r:1 w:1)
            // Storage: SpaceFollows SpaceFollowers (r:1 w:1)
        fn unfollow_space() -> Weight {
        (44_833_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
        }
    }

    // For backwards compatibility and tests
    impl WeightInfo for () {
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:1)
            // Storage: Spaces SpaceById (r:1 w:0)
            // Storage: SpaceFollows SpaceFollowers (r:1 w:1)
            // Storage: SpaceFollows SpacesFollowedByAccount (r:1 w:1)
        fn follow_space() -> Weight {
        (40_968_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
        }
            // Storage: Spaces SpaceById (r:1 w:0)
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:1)
            // Storage: SpaceFollows SpacesFollowedByAccount (r:1 w:1)
            // Storage: SpaceFollows SpaceFollowers (r:1 w:1)
        fn unfollow_space() -> Weight {
        (44_833_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
        }
    }
