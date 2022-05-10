use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_types::messages::HasCanisterId;

use std::{collections::HashMap, borrow::Borrow};
use std::vec::Vec;

use crate::init;

static E : f64 = 2.718281828459;

// #[derive(Clone, Debug, Default, CandidType, Deserialize)]
// pub enum StakingTime {
//     #[default] month,    // 1.3
//     #[default] quarter,  // 1.9
//     #[default] half,     // 3.25
//     #[default] year,     // 6.475
// }

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Nft {

    // nft type
    pub canister_id: String,

    // nft info
    pub token_id: String,      
    pub nri: u32,

    pub owner_id: String,

    // staking state
    pub is_staking: bool,   
    
    pub start_time: u64,
    pub end_time: u64,

    pub earned_profit: u64,

    pub staking_level: u32,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {

    // nft type
    pub canister_id: String,  

    // nft pool
    pub nfts: Vec<Nft>, 

    // total bonus
    pub bonus: u128,

    // weights to calc bonus
    pub bonus_weights: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct StakingPool {

    // nft type
    pub canister_id: String,

    // index in staking service
    pub service_id: u32,
    
    // todo : white list

    // nft pool
    pub nfts: Vec<Nft>,         

    // nft user list
    pub users: Vec<User>,
    
    // bonus pool
    // 1 = icp, 2 = ndp, 3 = icm...
    pub benefit_pool: Vec<u64>,
    pub temp_benefit: Vec<u64>,
}

impl StakingPool
{
    pub fn init(service_id : u32, canister_id : String) -> StakingPool
    {
        StakingPool
        {
            service_id : service_id,
            canister_id : canister_id,
            nfts : Vec::new(),
            users : Vec::new(),
            benefit_pool : Vec::new(),
            temp_benefit : Vec::new()
        }
    }
}

// item for print debug
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct StakingListItem {
    pub id: u32,
    pub addr: String,
}
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct StakingPoolItem {
    pub id: u32,
    pub pool: StakingPool,
}

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct StakingService {
    pub owner: Option<Principal>,
    pub service_id: u32,
    
    // id -> nft canister_id
    pub nft_staking_list: Vec<String>,

    pub nft_token_list: Vec<String>,
    //pub total_user_list: Vec<String>,

    // id -> nft pool
    pub nft_staking_pools: HashMap<String, StakingPool>,

    // todo: staking record

    // 
    pub user_id: u32,
    pub user_list: HashMap<Principal, User>,
}

impl StakingService {

    // owner
    pub fn set_owner(&mut self, principal: Principal) -> () {
        self.owner = Some(principal);
    }
    pub fn get_owner(&self) -> Option<Principal> {
        self.owner
    }
    pub fn is_owner(&self) -> Result<(), String> {
        if self.owner.unwrap() != ic_cdk::caller() {
            return  Err("no auth".to_owned());
         }
         Ok(())
    }

    // print functions for debug
    pub fn get_something(& self) -> (u32)
    {
        self.service_id
    }

    // pub fn print_nft_staking_list(&self) -> Vec<StakingListItem> {
    //     self.nft_staking_list
    //         .clone()
    //         .into_iter()
    //         .map(|(id, addr)| StakingListItem { id, addr })
    //         .collect()
    // }
    // pub fn print_nft_staking_pools(&self) -> Vec<StakingPoolItem> {
    //     self.nft_staking_pools
    //         .clone()
    //         .into_iter()
    //         .map(|(id, pool)| StakingPoolItem { id, pool })
    //         .collect()
    // }

    // inner functions
    // check nft is in service
    fn is_in_service() -> (bool)
    {
        true
    }

    fn calcucate_benefit()
    {

    }

    /* admin command */

    // init
    pub fn init_service(&mut self) -> ()
    {
        //self.id = Some(candid::Nat::from(0));
        self.service_id = 0;
        self.nft_staking_list.clear();
        self.nft_staking_pools.clear();
        
    }

    // add a serial of nft into staking service
    pub fn add_staking(&mut self, canister_id: String) -> () {
        self.service_id = self.service_id + 1;
        self.nft_staking_list.push(canister_id.clone());

        let pool = StakingPool::init(self.service_id, canister_id.clone());
        self.nft_staking_pools.insert(canister_id, pool.clone());

    }
    pub fn get_nft_list(&self) -> Vec<String> {
        self.nft_staking_list
            .clone()
            .into_iter()
            .collect()
    }

    pub fn get_user_nft(&self, id: Principal) -> Vec<Nft>
    {
        let user = self.user_list.get(id.borrow()).unwrap();
        user.clone().nfts
    }

    //
    pub fn call_staking(&mut self, 
        caller: Principal, 
        nft: String, 
        token: String, 
        time: u32) -> () 
    {
        if self.nft_staking_list.is_empty()
        {
            return;
        }

        // nft type check
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft.clone())
        {
            // user check in white_list

            // lock nft

            // send nft

            // if success
            // update msg in service
            //let mut user = self.user_list.get(&caller);
            let mut user_instance = User{
                canister_id: String::from("123456"),
                nfts: Vec::new(), 
                bonus: 123,
                bonus_weights: 1234,
            };

            // new a nft
            let nft_instance = Nft{
                canister_id: nft,
                token_id: token,      
                nri: 100,
                owner_id: String::from("rex"),
                is_staking:true,  
                start_time:1000,
                end_time:2000,
                earned_profit:3000,
                staking_level:40,
            };

            user_instance.nfts.push(nft_instance.clone());

            self.user_list.insert(caller, user_instance);
            
            // add into user struct
            
            // calc bonus weights

            // update user bonus weights
        }
    }

    // add benefit into benefit pool
    // which pool
    // which token icp?
    // temp -> total
    pub fn add_benefit()
    {
        
    }
    // add benefit into temp_benefit

    // temp_benefit -> benefit pool


    /* user command */
    // add one of nft into it's staking pool
    pub fn add_nft_into_pool() -> ()
    {
        
    }

    // withdraw one nft out of pool

    // withdraw the benefit of his owned


}

