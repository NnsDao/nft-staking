use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_types::messages::HasCanisterId;
use ledger_canister::protobuf::Hash;
use ic_cdk::api::time;

use std::{collections::HashMap, borrow::Borrow};
use std::vec::Vec;

use crate::init;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Nft {

    // nft type
    pub canister_id: String,

    // nft info
    pub token_id: String,      
    pub nri: u32,

    pub owner_id: Option<Principal>,

    // staking state
    pub is_staking: bool,   
    
    pub start_time: u64,
    pub end_time: u64,

    pub ndp_cache: u128,

    pub weight: u128,
    pub nri_weight: u128,
    pub ndp_weight: u128,
    pub time_weight: u128,
    pub earned_profit: u128,

    pub staking_level: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct User {

    // nft type
    //pub canister_id: String,  

    // nft pool
    pub nfts: Vec<Nft>, 

    // total bonus
    pub bonus: u128,

    // weights to calc bonus
    pub bonus_weights: u128,
}

impl User
{
    pub fn init() -> User
    {
        User
        {
            nfts : Vec::new(),
            bonus : 0,
            bonus_weights : 0,
        }
    }
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
    //pub users: Vec<Principal>,
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
            //users : Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct StakingBonus {

    // bonus pool
    // icp ndp icm ...
    pub stable_benefit: HashMap<String, u128>,
    pub temp_benefit: HashMap<String, u128>,

    pub weights: u128,
}

impl StakingBonus
{
    pub fn init() -> StakingBonus
    {
        StakingBonus
        {
            temp_benefit : HashMap::new(),
            stable_benefit : HashMap::new(),

            weights : 0,
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

    // to query all nft staking
    pub nft_token_list: Vec<String>,
    //pub total_user_list: Vec<String>,

    // id -> nft pool
    pub nft_staking_pools: HashMap<String, StakingPool>,
    pub nft_staking_bonus: HashMap<String, StakingBonus>,

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

    // init
    pub fn init_service(&mut self) -> ()
    {
        self.service_id = 0;

        self.nft_staking_list.clear();
        self.nft_staking_list = Vec::new();

        self.nft_token_list.clear();
        self.nft_token_list = Vec::new();

        self.nft_staking_pools.clear();
        self.nft_staking_pools = HashMap::new();

        self.user_list.clear();
        self.user_list = HashMap::new();
    }

    // add a serial of nft into staking service
    pub fn add_staking(&mut self, canister_id: String) -> () {

        self.service_id = self.service_id + 1;

        self.nft_staking_list.push(canister_id.clone());

        let pool = StakingPool::init(self.service_id, canister_id.clone());
        self.nft_staking_pools.insert(canister_id.clone(), pool.clone());

        let bonus = StakingBonus::init();
        self.nft_staking_bonus.insert(canister_id.clone(), bonus.clone());

    }
    pub fn get_nft_list(&self) -> Vec<String> {
        self.nft_staking_list
            .clone()
            .into_iter()
            .collect()
    }


    // User
    pub fn add_user(&mut self, id: Principal)
    {
        //let mut user = self.user_list.get(&caller);
        let mut user_instance = User{
            //canister_id: String::from("user name"),
            nfts: Vec::new(), 
            bonus: 0,
            bonus_weights: 0,
        };

        self.user_list.insert(id, user_instance);
    }
    pub fn get_user_nft(&self, id: Principal) -> Vec<Nft>
    {
        let user = self.user_list.get(id.borrow()).unwrap();
        user.clone().nfts
    }
    pub fn get_user_bonus(&self, id: Principal) -> u128
    {
        let user = self.user_list.get(id.borrow()).unwrap();
        user.clone().bonus
    }

    

    pub fn call_staking(&mut self, 
        caller: Principal,  // user param - id
        nft: String,        // nft param - type
        token: String,      // nft param - id
        nri: u32,           // nft param - nri
        ndp_count: u128,  // staking param - ndp
        staking_time: u64) -> ()    // staking param - time
    {
        if self.nft_staking_list.is_empty()
        {
            return;
        }

        // nft type check in service
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft.clone())
        {
            // user check in white_list
            if self.user_list.contains_key(&caller)
            {
                let now = time();
                let end_time = now + staking_time;

                let user = self.user_list.get(&caller);
                match user{
                    Some(u) => {

                        // if success
                        // update msg in service
                        
                        // use nri ndp and staking time calc nft weight
                        let E : f64 = 2.718281828459;
                        let nri : f64 = nri.into();
                        
                        let nri_weight = E.powf(nri / 10000.0);

                        let mut ndp_weight : f64 = 1.0;
                        if ndp_count > 100_000_000
                        {
                            ndp_weight = 1.2;
                        }
                        if ndp_count > 100_000_000 * 50000
                        {
                            ndp_weight = 1.3;
                        }
                        if ndp_count > 100_000_000 * 100000
                        {
                            ndp_weight = 1.4;
                        }
                        if ndp_count > 100_000_000 * 200000
                        {
                            ndp_weight = 1.5;
                        }
                        if ndp_count > 100_000_000 * 300000
                        {
                            ndp_weight = 1.6;
                        }

                        let time_weight = match staking_time
                        {
                            30 => 1.300,
                            90 => 1.900,
                            180 => 3.250,
                            360 => 6.475,
                            (_) => 1.300,
                        };

                        let weight = nri_weight * ndp_weight * time_weight;

                        let weight:u128 = (weight * 1000.0) as u128;
                        let ndp_weight: u128 = (ndp_weight * 1000.0) as u128;
                        let nri:u32 = nri as u32;
                        let nri_weight : u128 = (nri_weight * 1000.0) as u128;
                        let time_weight : u128 = (time_weight * 1000.0) as u128;

                        // new a nft
                        let nft_instance = Nft{
                            canister_id: nft.clone(),
                            token_id: token.clone(),      
                            nri: nri,
                            owner_id: Some(caller),
                            is_staking:true,  
                            start_time:now,
                            end_time:end_time,
                            ndp_cache:ndp_count,
                            weight:weight,
                            nri_weight:nri_weight,
                            ndp_weight:ndp_weight,
                            time_weight:time_weight,
                            earned_profit:0,
                            staking_level:staking_time,
                        };
                        // // add this nft into service pool vector 
                        // self.nft_token_list.push(token.clone());
                        // // and add into pool vector
                        // let pool = self.nft_staking_pools.get(&nft);
                        // match pool{
                        //     Some(p) => 
                        //     {
                        //         p.nfts.push(nft_instance);
                        //     }
                        //     None => ()
                        // }
                        // let pool = self.nft_staking_pools.entry(nft.clone()).or_insert(StakingPool::init(self.service_id, nft.clone()));
                        // pool.nfts.push(nft_instance.clone());

                        // add into service bonus weight
                        // let bonus_pool = self.nft_staking_bonus.get(&nft);
                        // match bonus_pool{
                        //     Some(p) => 
                        //     {
                        //         p.weights += weight;
                        //     }
                        //     None => ()
                        // }

                        let bonus = self.nft_staking_bonus.entry(nft.clone()).or_insert(StakingBonus::init());
                        bonus.weights += weight;

                        // let mut n = u.clone();
                        // //n.canister_id = String::from("user id");
                        // n.nfts.push(nft_instance.clone());
                        // //n.bonus += 1; // move to bonus function
                        // n.bonus_weights += weight;

                        // self.user_list.insert(caller.clone(), n);

                        let u = self.user_list.entry(caller).or_insert(User::init());
                        u.nfts.push(nft_instance.clone());
                        u.bonus_weights += weight;
                    }
                    None => ()
                }
            }
        }
    }

    // add benefit into benefit pool
    // which pool
    // which token icp
    // temp -> total
    pub fn add_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        self.add_temp_benefit(nft_canister, coin_id, volumn);
    }

    pub fn add_benefit_test(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        self.add_temp_benefit(nft_canister.clone(), coin_id.clone(), volumn);
        self.add_stable_benefit(nft_canister.clone(), coin_id.clone(), volumn);
    }
    pub fn get_temp_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128) -> u128
    {
        // have pool
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            let value = pool.temp_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a + volumn,
                None => 0
            };
            b_new
        }
        else {
            0
        }
    }
    pub fn get_stable_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128) -> u128
    {
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            let value = pool.stable_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a + volumn,
                None => 0
            };
            b_new
        }
        else {
            0
        }       
    }

    // add benefit into temp_benefit
    pub fn add_temp_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        // have pool
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            // let pool = self.nft_staking_bonus.get(&nft_canister);
            // match pool{
            //     Some(p) => {
            //         let mut b_new = 0;
            //         let bonus = p.temp_benefit.get(&coin_id);
            //         match bonus{
            //             Some(p) => 
            //             {
            //                 b_new = p + volumn;
            //             }
            //             None => ()
            //         }
                    
            //         p.temp_benefit.insert(coin_id.clone(), b_new);
            //     }
            //     None => ()
            // }

            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            
            let value = pool.temp_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a + volumn,
                None => 0
            };
            pool.temp_benefit.insert(coin_id.clone(), b_new);
            // let bonus = pool.temp_benefit.entry(coin_id.clone()).or_insert(0);
            // let mut v = volumn;
            // bonus = bonus + v;
        }
    }

    pub fn reduce_temp_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        // have pool
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            
            let value = pool.temp_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a - volumn,
                None => 0
            };
            pool.temp_benefit.insert(coin_id.clone(), b_new);
        }
    }
    
    pub fn add_stable_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            
            let value = pool.stable_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a + volumn,
                None => 0
            };
            pool.stable_benefit.insert(coin_id.clone(), b_new);
        }
    }


    pub fn reduce_stable_benefit(&mut self, nft_canister: String, coin_id: String, volumn: u128)
    {
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            
            let value = pool.stable_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a - volumn,
                None => 0
            };
            pool.stable_benefit.insert(coin_id.clone(), b_new);
        }
    }

    // calc bonus every day/hours
    pub fn calc_benefit(&mut self)
    {
        for (key, value) in self.nft_staking_pools.clone() {
            // get all users info and his every nft benifit weights in this pool
            let bonus = self.nft_staking_bonus.entry(key.clone()).or_insert(StakingBonus::init());
            let weights  = bonus.weights;

            let ndp_token = String::from("vgqnj-miaaa-aaaal-qaapa-cai");
            // ndp only
            let temp_balance = bonus.temp_benefit.get(&ndp_token).unwrap();
            let stable_balance = bonus.stable_benefit.get(&ndp_token).unwrap() / 2;
            let balance = temp_balance + stable_balance;
            let single_bonus = balance / weights;

            for (p, user) in &mut self.user_list
            {            
                for nft in &mut user.nfts
                {
                    nft.earned_profit += single_bonus * nft.weight;
                    user.bonus += nft.earned_profit;
                }
            }

            // update pool bonus
            self.update_benefit(key, ndp_token);
        }
    }

    pub fn update_benefit(&mut self, nft_canister: String, coin_id: String)
    {
        // have pool
        if self.nft_staking_list.iter().any(|i| i.to_string()== nft_canister.clone())
        {
            let pool = self.nft_staking_bonus.entry(nft_canister.clone()).or_insert(StakingBonus::init());
            
            let value = pool.stable_benefit.get(&coin_id);
            let b_new = match value{
                Some(a) => a / 2,
                None => 0
            };
            pool.temp_benefit.insert(coin_id.clone(), 0);
            pool.stable_benefit.insert(coin_id.clone(), b_new);
        }
    }

    /* user command */
    //withdraw the benefit of his owned
    pub fn call_withdraw(&mut self, 
        caller: Principal,  // user param - id
        nft: String,        // nft param - type
        token: String      // nft param - id
    )
    {
        // todo : send nft

        // update info

    }

}