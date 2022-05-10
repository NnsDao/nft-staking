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
fn get_something() -> u32 {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_something()
    )
}
#[query]
#[candid::candid_method(query)]
pub async fn set_and_get_something(caller: Principal) -> u128 {
    
    get_ndp_weights(caller).await


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
fn get_ic_cdk_caller(sth : String) -> Option<Principal>
{
    let id = Principal::from_text(sth);
    match id{
        Ok(id) => return Some(id),
        Err(err) => return Some(ic_cdk::caller())
    }
    
}



// get the nft info of all NFTs
// #[import(canister_id = "？", candid_path = "？.did")]
// struct NFTCanister;

/// Print all staking info
// #[query]
// #[candid::candid_method(query)]
// pub fn print_nft_staking_list() -> Vec<staking::StakingListItem> {
//     STAKING_STATE.with(|staking_service| 
//         staking_service.borrow().print_nft_staking_list())
// }
// #[query]
// #[candid::candid_method(query)]
// pub fn print_nft_staking_pools() -> Vec<staking::StakingPoolItem> {
//     STAKING_STATE.with(|staking_service| 
//         staking_service.borrow().print_nft_staking_pools())
// }
//print_nft_staking_list : () -> (vec StakingListItem) query;
//print_nft_staking_pools : () -> (vec StakingPoolItem) query;

// dfx canister user_id token_id staking_time
#[update]
#[candid::candid_method]
pub fn stake(caller: Principal, nft: String, token: String, time: u32) -> ()
{
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().call_staking(caller, nft, token, time);
    })
}

#[query]
#[candid::candid_method(query)]
pub fn get_nft_list() -> Vec<String> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_nft_list())
}

#[query]
#[candid::candid_method(query)]
pub fn get_user_nft(id: Principal) -> Vec<staking::Nft> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_user_nft(id))
}

// dfx canister user_id  -> nfts staking list
//

use ic_ledger_types::AccountIdentifier;
use crate::canisters::{ canister::*};

#[update]
#[candid::candid_method]
pub async fn get_ndp_weights(caller: Principal) -> u128 {

    if let ext::BalanceResponse::ok(ndp_balance) = get_balance(caller).await {
        return ndp_balance;
    }
    return 0
}

async fn get_balance(caller: Principal) -> ext::BalanceResponse {
    let addr = AccountIdentifier::new(&caller, &ic_ledger_types::DEFAULT_SUBACCOUNT);
    let arg = ext::BalanceRequest {
        token: get_canister_id(CanisterEnmu::Ndp),
        user: ext::User::address(addr.to_string()),
    };
    CanisterExtClient::new(get_canister_id(CanisterEnmu::Ndp))
        .balance(arg)
        .await
        .unwrap()
        .0
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    __export_service()
}
