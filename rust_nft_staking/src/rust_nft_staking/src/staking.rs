use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_types::messages::HasCanisterId;

use std::collections::HashMap;
use std::vec::Vec;

use crate::init;

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
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct StakingPool {

    // nft type
    pub canister_id: String,    

    // index in staking service
    pub service_id: u32,
    
    // nft pool
    pub nfts: Vec<Nft>,         
    
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
            benefit_pool : Vec::new(),
            temp_benefit : Vec::new()
        }
    }
}

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct StakingService {
    pub owner: Option<Principal>,
    pub id: u32,
    
    // id -> nft canister_id
    pub nft_staking_list: HashMap<u32, String>,

    // id -> nft pool
    pub nft_staking_pools: HashMap<u32, StakingPool>,

    // todo: staking record

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

    // temp functions
    pub fn get_id(& self) -> (u32)
    {
        self.id
    }

    // common functions
    // check nft is in service
    fn is_in_service() -> (bool)
    {
        true
    }

    // 

    /* admin command */

    // init
    pub fn init_service(&mut self) -> ()
    {
        //self.id = Some(candid::Nat::from(0));
        self.id = 0;
        self.nft_staking_list.clear();
        self.nft_staking_pools.clear();
        
    }

    // add a serial of nft into staking service
    pub fn add_staking(&mut self, canister_id: String) -> () {
        self.id = self.id + 1;
        self.nft_staking_list.insert(self.id, canister_id.clone());
        let pool = StakingPool::init(self.id, canister_id.clone());
        self.nft_staking_pools.insert(self.id, pool.clone());
    }

    // add benefit into benefit pool

    // add benefit into temp_benefit

    // temp_benefit -> benefit pool


    /* user command */
    // add one of nft into it's staking pool

    // withdraw one nft out of pool

    // withdraw the benefit of his owned


}