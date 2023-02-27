#![no_std]

mod instruction;

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};
pub use instruction::*;
pub use mt_storage_io::TokenId;
use primitive_types::H256;

pub struct MTLogicMetadata;

impl Metadata for MTLogicMetadata {
    type Init = In<InitMTLogic>;
    type Handle = InOut<MTLogicAction, MTLogicEvent>;
    type Others = InOut<Action, ()>;
    type Reply = ();
    type Signal = ();
    type State = MTLogicState;
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Copy)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct MTLogicState {
    pub admin: ActorId,
    pub mtoken_id: ActorId,
    pub transaction_status: Vec<(H256, TransactionStatus)>,
    pub instructions: Vec<(H256, (Instruction, Instruction))>,
    pub storage_code_hash: H256,
    pub id_to_storage: Vec<(String, ActorId)>,
    pub token_nonce: TokenId,
    pub token_uris: Vec<(TokenId, String)>,
    pub token_total_supply: Vec<(TokenId, u128)>,
    pub token_creators: Vec<(TokenId, ActorId)>,
}

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
        is_nft: bool,
    },
    MintBatchFT {
        token_id: TokenId,
        to: Vec<ActorId>,
        amounts: Vec<u128>,
    },
    MintBatchNFT {
        token_id: TokenId,
        to: Vec<ActorId>,
    },
    BurnBatchFT {
        token_id: TokenId,
        burn_from: Vec<ActorId>,
        amounts: Vec<u128>,
    },
    BurnNFT {
        token_id: TokenId,
        from: ActorId,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitMTLogic {
    pub admin: ActorId,
    pub storage_code_hash: H256,
}
