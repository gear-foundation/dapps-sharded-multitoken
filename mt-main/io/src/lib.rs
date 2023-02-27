#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
use mt_logic_io::TokenId;
use primitive_types::H256;

pub struct MTMainMetadata;

impl Metadata for MTMainMetadata {
    type Init = In<InitMToken>;
    type Handle = InOut<MTokenAction, MTokenEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = MTokenState;
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct MTokenState {
    pub admin: ActorId,
    pub mt_logic_id: ActorId,
    pub transactions: Vec<(H256, TransactionStatus)>,
}

#[derive(Encode, Decode, TypeInfo, Copy, Clone, Debug)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

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
