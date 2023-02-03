#![no_std]

use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};
pub use mt_storage_io::TokenId;
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
    Approve {
        account: ActorId,
        is_approved: bool,
    },
    Create {
        initial_amount: u128,
        uri: String,
    },
    MintBatch {
        token_id: TokenId,
        to: Vec<ActorId>,
        amounts: Vec<u128>,
    },
    BurnBatch {
        token_id: TokenId,
        burn_from: Vec<ActorId>,
        amounts: Vec<u128>,
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
    GetTokenNonce,
    GetTokenURI(TokenId),
    GetTokenTotalSupply(TokenId),
    GetTokenOwner(TokenId),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum MTLogicStateReply {
    Storages(Vec<(String, ActorId)>),
    TokenNonce(TokenId),
    TokenURI(String),
    TokenTotalSupply(u128),
    TokenOwner(ActorId),
}
