#![no_std]

use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};
use mt_storage_io::TokenMetadata;
use primitive_types::H256;

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum MTLogicAction {
    Message {
        transaction_hash: H256,
        account: ActorId,
        payload: Vec<u8>,
    },
    GetBalance {
        token_id: u128,
        account: ActorId,
    },
    GetApproval {
        account: ActorId,
        approval_target: ActorId,
    },
    Clear(H256),
    UpdateStorageCodeHash(H256),
    MigrateStorages,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MTLogicEvent {
    Ok,
    Err,
    Balance(u128),
    Approval(bool),
}

#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
pub enum Action {
    Transfer {
        token_id: u128,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    },
    Mint {
        ids: Vec<u128>,
        amounts: Vec<u128>,
        meta: Vec<Option<TokenMetadata>>,
    },
    Burn {
        ids: Vec<u128>,
        amounts: Vec<u128>,
    },
    Approve {
        account: ActorId,
        is_approved: bool,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitMTLogic {
    pub admin: ActorId,
    pub storage_code_hash: H256,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum MTLogicState {
    Storages,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum MTLogicStateReply {
    Storages(Vec<(String, ActorId)>),
}
