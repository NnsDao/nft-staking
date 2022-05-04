use ic_cdk_macros::init;
use crate::STAKING_STATE;

#[init]
fn init() {
    ic_cdk::setup();
    STAKING_STATE.with(|staking_service| 
        staking_service.borrow_mut().set_owner(ic_cdk::caller())
    )
}