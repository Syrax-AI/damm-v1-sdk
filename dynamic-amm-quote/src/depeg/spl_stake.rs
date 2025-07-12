use anchor_spl::token_interface::spl_token_metadata_interface::borsh::BorshDeserialize;
use prog_dynamic_amm::constants::depeg;
use spl_stake_pool::state::StakePool;
use std::convert::TryInto;

pub fn get_virtual_price(bytes: &[u8]) -> Option<u64> {
    let mut bytes = bytes;
    let stake = StakePool::deserialize(&mut bytes).ok()?;

    let virtual_price = (stake.total_lamports as u128)
        .checked_mul(depeg::PRECISION as u128)?
        .checked_div(stake.pool_token_supply as u128)?;

    virtual_price.try_into().ok()
}
