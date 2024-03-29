//! This is the contract for IC Market staking
mod canisters;

pub mod staking;
use staking::StakingService;
use candid::{Decode, Encode, encode_one, decode_one};
mod init;

mod owner;
use owner::is_owner;

use std::str::FromStr;
use std::{cell::RefCell, borrow::Borrow};
use std::vec::Vec;

use canisters::{canister::*, ext::{self, balance}, dip20};

use ic_cdk_macros::*;
use ic_cdk::{export::candid::Principal, println};

use num_bigint::BigUint;
use num_integer::Integer;

use num_bigint::ToBigUint;

thread_local! {
    static STAKING_STATE: RefCell<StakingService> = RefCell::default();
}

// #[query]
// #[candid::candid_method(query)]
// fn get_owner() -> Option<Principal> {
//     STAKING_STATE.with(|staking_service| 
//         staking_service.borrow().get_owner()
//     )
// }

// #[query]
// #[candid::candid_method(query)]
// fn get_something() -> u32 {
//     STAKING_STATE.with(|staking_service| 
//         staking_service.borrow().get_something()
//     )
// }
// #[query]
// #[candid::candid_method(query)]
// pub async fn set_and_get_something(caller: Principal) -> u128 {
    
//     get_ndp_weights(caller).await
// }

// todo: auth error
// #[update(guard = "is_owner")]
#[update]
#[candid::candid_method]
fn add_staking(id : String) -> () {
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().add_staking(id.clone());
    })
}

#[update]
#[candid::candid_method]
fn add_user(id : Principal) -> () {
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().add_user(id.clone());
    })
}

// #[query]
// #[candid::candid_method(query)]
// fn get_ic_cdk_caller(sth : String) -> Option<Principal>
// {
//     let id = Principal::from_text(sth);
//     match id{
//         Ok(id) => return Some(id),
//         Err(err) => return Some(ic_cdk::caller())
//     }
    
// }



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
pub async fn stake(caller: Principal, nft: String, token: String, time: u64) -> ()
{
    let ndp_weights = get_ndp_weights(caller).await;
    // todo : get nft nri from nft token id and its canister
    let nri : u32 = 10000;

    // todo : lock nft

    // todo : send nft

    // if success         
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().call_staking(caller, nft, token, nri, ndp_weights, time);
    })
}

// get all nft type in service
#[query]
#[candid::candid_method(query)]
pub fn get_nft_list() -> Vec<String> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_nft_list())
}

// get all nft list in user staking
#[query]
#[candid::candid_method(query)]
pub fn get_user_nft(id: Principal) -> Vec<staking::Nft> {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_user_nft(id))
}
#[query]
#[candid::candid_method(query)]
pub fn get_user_bonus(id: Principal) -> u128 {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow().get_user_bonus(id))
}

use ic_ledger_types::AccountIdentifier;
use crate::canisters::{ canister::*};


use ic_cdk::api::call::CallResult;

#[update]
#[candid::candid_method]
pub async fn get_ndp_weights(caller: Principal) -> u128 {
    // pub async fn get_ndp_weights(caller: Principal) -> u128 {

    // if let ext::BalanceResponse::ok(ndp_balance) = get_balance(caller).await {
    //     return ndp_balance;
    // }
    let dip_client =
            dip20::Service::new(Principal::from_text("vgqnj-miaaa-aaaal-qaapa-cai").unwrap());
    let balance_ndp: candid::Nat = dip_client.balanceOf(caller).await.unwrap().0;
    // let balll = balance.0.clone();
    // let ndp = match BigUint::from(balll).to_u64_digits().first()
    // {
    //     Some(balance) => balance,
    //     None => &0
    // };
    
    // let ndp = candid::Nat(balance.0.to_biguint().unwarp());
    // decode_impl!(balance_ndp)
    
    // Decode!()
    // balance_ndp
    // match balance_ndp.0.to_u128() {
    //     Some(ndp) => return ndp,
    //     None => return 0
    // }
    // let ndps = *ndp as u128;

    // ndp
    // ndp as u128

    // let mut readable = Vec::new();
    // Nat::decode(balance_ndp).unwrap().0.to_u128().unwrap()
    // balance_ndp.decode()
    let encode_nat = Encode!(&balance_ndp).unwrap();
    let (ndp_num) = Decode!(&encode_nat, u128).unwrap();
    ndp_num

}



#[update]
#[candid::candid_method]
pub async fn add_benefit_test(nft: String, token: String, volumn: u128) -> ()
{
    // if success         
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().add_benefit_test(nft, token, volumn);
    })
}

#[query]
#[candid::candid_method(query)]
pub fn get_temp_benefit(nft:String, token :String, volumn: u128) -> u128 {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow_mut().get_temp_benefit(nft, token, volumn))
}
#[query]
#[candid::candid_method(query)]
pub fn get_stable_benefit(nft:String, token :String, volumn: u128) -> u128 {
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow_mut().get_stable_benefit(nft, token, volumn))
}

#[update]
#[candid::candid_method]
pub async fn calc_bonus() -> ()
{
    // if success         
    STAKING_STATE.with(|staking_service| {
        staking_service.borrow_mut().calc_benefit();
    })
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
