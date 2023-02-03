#![no_std]

use gstd::{prelude::*, ActorId};
use mt_logic_io::TokenId;
use primitive_types::H256;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum MTokenAction {
    Message {
        transaction_id: u64,
        payload: Vec<u8>,
    },
    UpdateLogicContract {
        mt_logic_code_hash: H256,
        storage_code_hash: H256,
    },
    GetBalance {
        token_id: TokenId,
        account: ActorId,
    },
    GetApproval {
        account: ActorId,
        approval_target: ActorId,
    },
    Clear(H256),
    MigrateStorageAddresses,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MTokenEvent {
    Ok,
    Err,
    Balance(u128),
    Approval(bool),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitMToken {
    pub storage_code_hash: H256,
    pub mt_logic_code_hash: H256,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MTokenState {
    TransactionStatus(ActorId, u64),
    MTLogicId,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MTokenStateReply {
    TransactionStatus(Option<TransactionStatus>),
    MTLogicId(ActorId),
}

#[derive(Encode, Decode, TypeInfo, Copy, Clone, Debug)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}
