contract;

use std::{
    call_frames::{
        msg_asset_id,
    },
    context::{
        balance_of,
        msg_amount,
    },
    contract_id::ContractId,
    hash::*,
    hash::Hash,
    revert::require,
    storage::*,
    token::*,
};

const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

abi MyContract {

    #[storage(write)]
    fn set_recievable(asset_id: ContractId);

    #[storage(read)]
    fn mint_to_id(amount: u64, address: Identity);

    #[storage(read), payable]
    fn on_recieve() -> bool;

    #[storage(read, write)]
    fn set_asset_id(asset_id: AssetId);
}

storage {
    asset_contract_id: ContractId = ContractId::from(ZERO_B256),
    asset_id: AssetId = AssetId::from(ZERO_B256),
}

impl MyContract for Contract {
     #[storage(write)]
    fn set_recievable(asset_contract: ContractId) {
        storage.asset_contract_id.write(asset_contract);
        storage.asset_id.write(get_default_asset_id(asset_contract));
    }

    #[storage(read)]
    fn mint_to_id(amount: u64, address: Identity) {
        mint_to(address, ZERO_B256, amount);
    }
    
    #[storage(read), payable]
    fn on_recieve() -> bool {
        require(msg_asset_id() == storage.asset_id.read(), "Asset ID must be correct");
        true
    }

    #[storage(read,write)]
    fn set_asset_id(asset_id: AssetId) {
        storage.asset_id.write(asset_id);
    }
}

pub fn get_default_asset_id(temp_contract: ContractId) -> AssetId {
    AssetId::from(sha256((temp_contract, ZERO_B256)))
}
