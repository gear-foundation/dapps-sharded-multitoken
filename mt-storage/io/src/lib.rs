#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub type TokenId = u128;

pub struct MTStorageMetadata;

impl Metadata for MTStorageMetadata {
    type Init = ();
    type Handle = InOut<MTStorageAction, MTStorageEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = MTStorageState;
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub struct MTStorageState {
    pub mt_logic_id: ActorId,
    pub transaction_status: Vec<(H256, bool)>,
    pub balances: Vec<(TokenId, Vec<(ActorId, u128)>)>,
    pub approvals: Vec<(ActorId, Vec<(ActorId, bool)>)>,
}

#[derive(Encode, Decode, Debug, Clone, TypeInfo)]
pub enum MTStorageAction {
    GetBalance {
        token_id: TokenId,
        account: ActorId,
    },
    GetApproval {
        account: ActorId,
        approval_target: ActorId,
    },
    Transfer {
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    },
    Approve {
        transaction_hash: H256,
        msg_source: ActorId,
        account: ActorId,
        approve: bool,
    },
    ClearTransaction(H256),
    IncreaseBalance {
        transaction_hash: H256,
        token_id: TokenId,
        account: ActorId,
        amount: u128,
    },
    DecreaseBalance {
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: ActorId,
        account: ActorId,
        amount: u128,
    },
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum MTStorageEvent {
    Ok,
    Err,
    Balance(u128),
    Approval(bool),
}
