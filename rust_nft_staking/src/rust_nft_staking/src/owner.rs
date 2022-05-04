use crate::STAKING_STATE;

pub fn is_owner() -> Result<(), String> {
    STAKING_STATE.with(|staking_service|
        staking_service.borrow().is_owner()
    )
}
