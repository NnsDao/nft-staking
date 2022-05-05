//! This is the contract for IC Market staking
mod canisters;

pub mod staking;
use staking::StakingService;

mod init;

mod owner;
use owner::is_owner;

use std::cell::RefCell;
use std::vec::Vec;

use canisters::{canister::*, ext};

use ic_cdk_macros::*;
use ic_cdk::export::candid::Principal;

thread_local! {
    static STAKING_STATE: RefCell<StakingService> = RefCell::default();
}

#[query]
#[candid::candid_method(query)]
fn get_owner() -> Option<Principal> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_owner()
    )
}

#[query]
#[candid::candid_method(query)]
fn get_id() -> u32 {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_id()
    )
}

// todo: auth error
// #[update(guard = "is_owner")]
#[update]
#[candid::candid_method]
fn add_staking(id : String) -> () {
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().add_staking(id.clone());
    })
}

#[query]
#[candid::candid_method(query)]
fn get_ic_cdk_caller() -> Option<Principal>
{
    Some(ic_cdk::caller())
}

// get the nft info of all NFTs
// #[import(canister_id = "？", candid_path = "？.did")]
// struct NFTCanister;

/// Print all staking info
#[query]
#[candid::candid_method(query)]
pub fn print_nft_staking_list() -> Vec<staking::StakingListItem> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().print_nft_staking_list())
}
#[query]
#[candid::candid_method(query)]
pub fn print_nft_staking_pools() -> Vec<staking::StakingPoolItem> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().print_nft_staking_pools())
}