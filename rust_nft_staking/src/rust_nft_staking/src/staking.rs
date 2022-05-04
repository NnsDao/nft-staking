use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};

#[derive(CandidType, Clone, Deserialize, Default)]
pub struct StakingService {
    pub owner: Option<Principal>,
    pub id: u32,
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

    // init
    pub fn init_service(&mut self) -> ()
    {
        //self.id = Some(candid::Nat::from(0));
        self.id = 0;
    }
    pub fn get_id(& self) -> (u32)
    {
        self.id
    }

    // add staking
    pub fn add_staking(&mut self) -> () {
        self.id = self.id + 1;
        //self.id.as_mut().unwrap().0 += 1u64;
    }



}